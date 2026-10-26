use crate::error::PocError;

/// Exact-value cell. NUMBER is never cast to float.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Null,
    Text(String),
    /// Exact decimal text as produced by the driver.
    Number(String),
    /// Type not mapped in POC; carries native type name.
    Unsupported(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bind {
    Number(String),
    Text(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultPage {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Value>>,
    /// true when more rows existed beyond `max_rows`.
    pub has_more: bool,
}

/// Backend seam: real Oracle adapter or test mock.
pub trait SelectBackend {
    /// Return at most `max_rows + 1` rows so caller can detect `has_more`.
    fn query(&self, sql: &str, bind: &Bind, max_rows: usize) -> Result<ResultPage, PocError>;
}

/// Client-side read-only guard: only SELECT / WITH. Enforcement = client, not server.
pub fn ensure_read_only(sql: &str) -> Result<(), PocError> {
    let code = strip_comments_and_strings(sql);
    let code = code.trim();
    if code.is_empty() {
        return Err(PocError::EmptyStatement);
    }
    let first = code
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .next()
        .unwrap_or("")
        .to_ascii_uppercase();
    if first != "SELECT" && first != "WITH" {
        return Err(PocError::ReadOnlyViolation);
    }
    // Reject a second statement after ';' (trailing ';' alone is tolerated).
    if let Some(i) = code.find(';')
        && !code[i + 1..].trim().is_empty()
    {
        return Err(PocError::ReadOnlyViolation);
    }
    Ok(())
}

/// Drop `--` / `/* */` comments; blank out `'...'` literals so keywords/`;` inside are ignored.
fn strip_comments_and_strings(sql: &str) -> String {
    let b: Vec<char> = sql.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < b.len() {
        match (b[i], b.get(i + 1)) {
            ('-', Some('-')) => {
                while i < b.len() && b[i] != '\n' {
                    i += 1;
                }
            }
            ('/', Some('*')) => {
                i += 2;
                while i + 1 < b.len() && !(b[i] == '*' && b[i + 1] == '/') {
                    i += 1;
                }
                i += 2;
                out.push(' ');
            }
            ('"', _) => {
                i += 1;
                while i < b.len() && b[i] != '"' {
                    i += 1;
                }
                i += 1;
                out.push_str("\"\"");
            }
            ('\'', _) => {
                i += 1;
                while i < b.len() {
                    if b[i] == '\'' {
                        if b.get(i + 1) == Some(&'\'') {
                            i += 2;
                            continue;
                        }
                        break;
                    }
                    i += 1;
                }
                i += 1;
                out.push_str("''");
            }
            (c, _) => {
                out.push(c);
                i += 1;
            }
        }
    }
    out
}

pub fn run_select(
    backend: &dyn SelectBackend,
    sql: &str,
    bind: &Bind,
    max_rows: usize,
) -> Result<ResultPage, PocError> {
    ensure_read_only(sql)?;
    let mut page = backend.query(sql, bind, max_rows)?;
    if page.rows.len() > max_rows {
        page.rows.truncate(max_rows);
        page.has_more = true;
    }
    Ok(page)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Mock(Result<ResultPage, PocError>);
    impl SelectBackend for Mock {
        fn query(&self, _: &str, _: &Bind, _: usize) -> Result<ResultPage, PocError> {
            self.0.clone()
        }
    }

    fn page(n: usize) -> ResultPage {
        ResultPage {
            columns: vec!["N".into()],
            rows: (0..n).map(|i| vec![Value::Number(i.to_string())]).collect(),
            has_more: false,
        }
    }

    #[test]
    fn guard_allows_select_and_with_case_insensitive() {
        for ok in [
            "select 1 from dual",
            "  SELECT 1 FROM dual",
            "with a as (select 1 x from dual) select * from a",
            "-- c\nselect 1 from dual",
            "/* c */ select 1 from dual",
        ] {
            assert!(ensure_read_only(ok).is_ok(), "{ok}");
        }
    }

    #[test]
    fn guard_rejects_dml_ddl_plsql() {
        for bad in [
            "delete from t",
            "UPDATE t SET a=1",
            "insert into t values (1)",
            "drop table t",
            "begin null; end;",
            "merge into t using d on (1=1) when matched then update set a=1",
            "commit",
            "select 1 from dual; delete from t",
        ] {
            assert_eq!(
                ensure_read_only(bad),
                Err(PocError::ReadOnlyViolation),
                "{bad}"
            );
        }
    }

    #[test]
    fn guard_quoted_identifier_with_dashes_is_not_a_comment() {
        let sql = "select 1 from \"a--b\"; delete from t";
        assert_eq!(ensure_read_only(sql), Err(PocError::ReadOnlyViolation));
    }

    #[test]
    fn guard_rejects_empty() {
        assert_eq!(
            ensure_read_only("  -- only comment"),
            Err(PocError::EmptyStatement)
        );
    }

    #[test]
    fn run_select_caps_rows_and_sets_has_more() {
        let out = run_select(
            &Mock(Ok(page(101))),
            "select 1 from dual",
            &Bind::Number("1".into()),
            100,
        )
        .unwrap();
        assert_eq!(out.rows.len(), 100);
        assert!(out.has_more);
    }

    #[test]
    fn run_select_exact_page_has_no_more() {
        let out = run_select(
            &Mock(Ok(page(100))),
            "select 1 from dual",
            &Bind::Number("1".into()),
            100,
        )
        .unwrap();
        assert_eq!(out.rows.len(), 100);
        assert!(!out.has_more);
    }

    #[test]
    fn run_select_blocks_mutation_before_backend() {
        struct Panics;
        impl SelectBackend for Panics {
            fn query(&self, _: &str, _: &Bind, _: usize) -> Result<ResultPage, PocError> {
                panic!("backend must not be called");
            }
        }
        let err = run_select(&Panics, "delete from t", &Bind::Text("x".into()), 10).unwrap_err();
        assert_eq!(err, PocError::ReadOnlyViolation);
    }

    #[test]
    fn run_select_propagates_backend_error() {
        let e = PocError::Db {
            code: 942,
            message: "table or view does not exist".into(),
        };
        let err = run_select(
            &Mock(Err(e.clone())),
            "select * from nope where a = :1",
            &Bind::Text("x".into()),
            10,
        )
        .unwrap_err();
        assert_eq!(err, e);
    }

    #[test]
    fn number_value_keeps_exact_digits() {
        let v = Value::Number("12345678901234567890.123456789".into());
        assert_eq!(v, Value::Number("12345678901234567890.123456789".into()));
    }
}
