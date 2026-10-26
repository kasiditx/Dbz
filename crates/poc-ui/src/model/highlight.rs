use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Keyword,
    String,
    Comment,
    Number,
    Bind,
    QuotedIdent,
    Ident,
    Punct,
    Space,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: Kind,
    /// Byte range into the source; tokens tile the whole input contiguously.
    pub range: Range<usize>,
}

/// Oracle-flavoured SQL tokenizer for highlighting only (not a parser).
pub fn tokenize(src: &str) -> Vec<Token> {
    let b = src.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let start = i;
        let c = b[i];
        let kind = if c.is_ascii_whitespace() {
            while i < b.len() && b[i].is_ascii_whitespace() {
                i += 1;
            }
            Kind::Space
        } else if c == b'-' && b.get(i + 1) == Some(&b'-') {
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
            Kind::Comment
        } else if c == b'/' && b.get(i + 1) == Some(&b'*') {
            i += 2;
            while i < b.len() && !(b[i] == b'*' && b.get(i + 1) == Some(&b'/')) {
                i += 1;
            }
            i = (i + 2).min(b.len());
            Kind::Comment
        } else if c == b'\'' {
            i = quoted_end(b, i, b'\'');
            Kind::String
        } else if c == b'"' {
            i = quoted_end(b, i, b'"');
            Kind::QuotedIdent
        } else if c == b':'
            && b.get(i + 1)
                .is_some_and(|n| n.is_ascii_alphanumeric() || *n == b'_')
        {
            i += 1;
            while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                i += 1;
            }
            Kind::Bind
        } else if c.is_ascii_digit() {
            while i < b.len() && (b[i].is_ascii_digit() || b[i] == b'.') {
                i += 1;
            }
            Kind::Number
        } else if c.is_ascii_alphabetic() || c == b'_' || c >= 0x80 {
            // Non-ASCII bytes stay inside identifiers so multi-byte chars are never split.
            while i < b.len()
                && (b[i].is_ascii_alphanumeric()
                    || matches!(b[i], b'_' | b'$' | b'#')
                    || b[i] >= 0x80)
            {
                i += 1;
            }
            if is_keyword(&src[start..i]) {
                Kind::Keyword
            } else {
                Kind::Ident
            }
        } else {
            i += 1;
            Kind::Punct
        };
        out.push(Token {
            kind,
            range: start..i,
        });
    }
    out
}

/// End index (exclusive) of a quoted run starting at `i`; doubled quote is an escape.
fn quoted_end(b: &[u8], mut i: usize, q: u8) -> usize {
    i += 1;
    while i < b.len() {
        if b[i] == q {
            if b.get(i + 1) == Some(&q) {
                i += 2;
                continue;
            }
            return i + 1;
        }
        i += 1;
    }
    b.len()
}

const KEYWORDS: &[&str] = &[
    "SELECT", "FROM", "WHERE", "AND", "OR", "NOT", "NULL", "IN", "IS", "AS", "ON", "JOIN", "LEFT",
    "RIGHT", "INNER", "OUTER", "FULL", "CROSS", "GROUP", "BY", "ORDER", "HAVING", "UNION", "ALL",
    "DISTINCT", "INSERT", "INTO", "VALUES", "UPDATE", "SET", "DELETE", "MERGE", "CREATE", "ALTER",
    "DROP", "TABLE", "VIEW", "INDEX", "WITH", "FETCH", "FIRST", "NEXT", "ROWS", "ONLY", "OFFSET",
    "BEGIN", "END", "DECLARE", "IF", "THEN", "ELSE", "ELSIF", "LOOP", "CASE", "WHEN", "BETWEEN",
    "LIKE", "EXISTS", "ASC", "DESC", "COMMIT", "ROLLBACK", "FOR", "ROWNUM",
];

fn is_keyword(w: &str) -> bool {
    KEYWORDS.iter().any(|k| k.eq_ignore_ascii_case(w))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(src: &str) -> Vec<(Kind, &str)> {
        tokenize(src)
            .into_iter()
            .map(|t| (t.kind, &src[t.range]))
            .collect()
    }

    #[test]
    fn basic_select() {
        assert_eq!(
            kinds("select 1 from dual"),
            vec![
                (Kind::Keyword, "select"),
                (Kind::Space, " "),
                (Kind::Number, "1"),
                (Kind::Space, " "),
                (Kind::Keyword, "from"),
                (Kind::Space, " "),
                (Kind::Ident, "dual"),
            ]
        );
    }

    #[test]
    fn keywords_case_insensitive() {
        assert_eq!(kinds("SeLeCt")[0].0, Kind::Keyword);
    }

    #[test]
    fn line_comment_to_eol_and_block_comment() {
        let k = kinds("a -- hi\nb /* x */ c");
        assert!(k.contains(&(Kind::Comment, "-- hi")));
        assert!(k.contains(&(Kind::Comment, "/* x */")));
    }

    #[test]
    fn unterminated_block_comment_runs_to_end() {
        let k = kinds("a /* never");
        assert_eq!(k.last().unwrap(), &(Kind::Comment, "/* never"));
    }

    #[test]
    fn string_with_escaped_quote_and_thai() {
        let k = kinds("'it''s ไทย' x");
        assert_eq!(k[0], (Kind::String, "'it''s ไทย'"));
    }

    #[test]
    fn bind_and_quoted_identifier() {
        let k = kinds("where \"Mixed Case\" = :id and b = :1");
        assert!(k.contains(&(Kind::QuotedIdent, "\"Mixed Case\"")));
        assert!(k.contains(&(Kind::Bind, ":id")));
        assert!(k.contains(&(Kind::Bind, ":1")));
    }

    #[test]
    fn decimal_number_and_punct() {
        let k = kinds("1.50,(x)");
        assert_eq!(k[0], (Kind::Number, "1.50"));
        assert_eq!(k[1], (Kind::Punct, ","));
    }

    #[test]
    fn tokens_tile_input_and_slice_on_char_boundaries() {
        let src = "select 'ลูกค้า', \"ชื่อ\" -- หมายเหตุ\nfrom t /* ยาว */ where x = :n; ";
        let toks = tokenize(src);
        let mut pos = 0;
        for t in &toks {
            assert_eq!(t.range.start, pos);
            assert!(src.is_char_boundary(t.range.start) && src.is_char_boundary(t.range.end));
            pos = t.range.end;
        }
        assert_eq!(pos, src.len());
    }

    #[test]
    fn empty_input_has_no_tokens() {
        assert!(tokenize("").is_empty());
    }
}
