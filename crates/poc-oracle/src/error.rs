use std::fmt;

/// Typed POC error. Display never contains secrets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PocError {
    InvalidConfig(String),
    MissingEnv(Vec<String>),
    ReadOnlyViolation,
    EmptyStatement,
    Db { code: u32, message: String },
    Timeout,
    DeadConnection,
    Other(String),
}

impl fmt::Display for PocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PocError::InvalidConfig(m) => write!(f, "invalid config: {m}"),
            PocError::MissingEnv(names) => {
                write!(f, "missing environment variables: {}", names.join(", "))
            }
            PocError::ReadOnlyViolation => f.write_str("statement rejected by read-only guard"),
            PocError::EmptyStatement => f.write_str("empty statement"),
            PocError::Db { code, message } => write!(f, "ORA-{code:05}: {message}"),
            PocError::Timeout => f.write_str("call timeout exceeded"),
            PocError::DeadConnection => f.write_str("connection is dead"),
            PocError::Other(m) => f.write_str(m),
        }
    }
}

impl std::error::Error for PocError {}

/// Replace every occurrence of each non-empty secret with `***`.
pub fn redact(message: &str, secrets: &[&str]) -> String {
    secrets
        .iter()
        .filter(|s| !s.is_empty())
        .fold(message.to_string(), |acc, s| acc.replace(s, "***"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redact_replaces_all_occurrences() {
        let out = redact("login hunter2 failed for hunter2", &["hunter2"]);
        assert_eq!(out, "login *** failed for ***");
    }

    #[test]
    fn redact_ignores_empty_secret() {
        assert_eq!(redact("abc", &[""]), "abc");
    }

    #[test]
    fn display_db_error_has_ora_code() {
        let e = PocError::Db {
            code: 1017,
            message: "invalid username/password".into(),
        };
        assert_eq!(e.to_string(), "ORA-01017: invalid username/password");
    }

    #[test]
    fn display_missing_env_lists_names_only() {
        let e = PocError::MissingEnv(vec!["DBZ_ORACLE_HOST".into()]);
        assert!(e.to_string().contains("DBZ_ORACLE_HOST"));
    }
}
