use std::ops::Range;

/// Byte range of the statement containing `cursor` (trimmed). Statements split on `;`
/// outside strings/comments; `BEGIN`/`DECLARE` blocks end at a line holding only `/`.
pub fn statement_at(text: &str, cursor: usize) -> Range<usize> {
    let spans = statements(text);
    spans
        .iter()
        .rev()
        .find(|r| r.start <= cursor)
        .or(spans.first())
        .cloned()
        .unwrap_or(0..0)
}

fn statements(text: &str) -> Vec<Range<usize>> {
    let mut out = Vec::new();
    let mut pos = 0;
    while pos < text.len() {
        let rest = &text[pos..];
        let skip = rest.len() - rest.trim_start().len();
        let start = pos + skip;
        if start >= text.len() {
            break;
        }
        let first_word: String = text[start..]
            .chars()
            .take_while(|c| c.is_ascii_alphabetic())
            .collect::<String>()
            .to_ascii_uppercase();
        let (end, next) = if first_word == "BEGIN" || first_word == "DECLARE" {
            plsql_end(text, start)
        } else {
            match find_semicolon(text, start) {
                Some(i) => (i, i + 1),
                None => (text.len(), text.len()),
            }
        };
        let body = &text[start..end];
        let trimmed = body.trim_end();
        if !trimmed.is_empty() {
            out.push(start..start + trimmed.len());
        }
        pos = next.max(start + 1).min(text.len());
    }
    out
}

/// Block ends before a line containing only `/`; (end, resume position).
fn plsql_end(text: &str, start: usize) -> (usize, usize) {
    let mut off = start;
    for line in text[start..].split_inclusive('\n') {
        if line.trim() == "/" {
            return (off, off + line.len());
        }
        off += line.len();
    }
    (text.len(), text.len())
}

/// Index of a `;` outside strings, quoted identifiers and comments.
fn find_semicolon(text: &str, from: usize) -> Option<usize> {
    let b = text.as_bytes();
    let mut i = from;
    while i < b.len() {
        match b[i] {
            b';' => return Some(i),
            b'\'' | b'"' => {
                let q = b[i];
                i += 1;
                while i < b.len() && b[i] != q {
                    i += 1;
                }
            }
            b'-' if b.get(i + 1) == Some(&b'-') => {
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            b'/' if b.get(i + 1) == Some(&b'*') => {
                i += 2;
                while i < b.len() && !(b[i] == b'*' && b.get(i + 1) == Some(&b'/')) {
                    i += 1;
                }
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// 1-based (line, column) for a byte offset; column counts chars (Thai-safe).
pub fn line_col(text: &str, cursor: usize) -> (usize, usize) {
    let cursor = cursor.min(text.len());
    let before = &text[..cursor];
    let line = before.matches('\n').count() + 1;
    let col = before.rsplit('\n').next().map_or(0, |l| l.chars().count()) + 1;
    (line, col)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn st(text: &str, cur: usize) -> &str {
        &text[statement_at(text, cur)]
    }

    #[test]
    fn picks_statement_under_cursor() {
        let t = "select 1 from dual;\nselect 2 from dual;";
        assert_eq!(st(t, 3), "select 1 from dual");
        assert_eq!(st(t, 25), "select 2 from dual");
    }

    #[test]
    fn cursor_at_end_picks_last() {
        let t = "select 1;\nselect 2;";
        assert_eq!(st(t, t.len()), "select 2");
    }

    #[test]
    fn semicolon_in_string_or_comment_does_not_split() {
        let t = "select 'a;b' from dual -- x; y\n where 1=1;";
        assert_eq!(st(t, 2), "select 'a;b' from dual -- x; y\n where 1=1");
    }

    #[test]
    fn plsql_block_not_split_on_inner_semicolons() {
        let t = "begin\n  a := 1;\n  b := 2;\nend;\n/\nselect 1 from dual;";
        assert_eq!(st(t, 8), "begin\n  a := 1;\n  b := 2;\nend;");
        assert_eq!(st(t, t.len()), "select 1 from dual");
    }

    #[test]
    fn statement_without_trailing_semicolon() {
        assert_eq!(st("select 1 from dual", 0), "select 1 from dual");
    }

    #[test]
    fn cursor_in_blank_gap_uses_previous_statement() {
        let t = "select 1;\n\n\nselect 2;";
        assert_eq!(st(t, 11), "select 1");
    }

    #[test]
    fn empty_text_is_empty_range() {
        assert_eq!(statement_at("", 0), 0..0);
        assert_eq!(statement_at("  \n ", 2), 0..0);
    }

    #[test]
    fn thai_text_ranges_are_char_boundaries() {
        let t = "select 'ลูกค้า' from dual;\nselect 2;";
        let r = statement_at(t, 10);
        assert!(t.is_char_boundary(r.start) && t.is_char_boundary(r.end));
        assert_eq!(&t[r], "select 'ลูกค้า' from dual");
    }

    #[test]
    fn line_col_counts_chars_not_bytes() {
        assert_eq!(line_col("ab\nลูกค้า x", 0), (1, 1));
        assert_eq!(line_col("ab\nลูกค้า x", 3), (2, 1));
        let t = "ab\nลูกค้า x";
        let idx = t.find('x').unwrap();
        // "ลูกค้า" = 6 chars (consonants + combining marks) + space => x at column 8
        assert_eq!(line_col(t, idx), (2, 8));
    }
}
