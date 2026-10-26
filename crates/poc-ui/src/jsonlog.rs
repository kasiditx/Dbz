//! Minimal JSON string escaping for the event log. Rust's `{:?}` is NOT valid JSON for
//! non-ASCII "grapheme extend" chars (it prints `\u{e47}`), which silently broke the first
//! E2E run on Thai text.

/// Quote and escape `s` as a JSON string; non-ASCII characters are emitted as-is (UTF-8).
pub fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_quote_backslash_and_control_chars() {
        assert_eq!(json_string("a\"b\\c\n\t"), "\"a\\\"b\\\\c\\n\\t\"");
        assert_eq!(json_string("\u{1}"), "\"\\u0001\"");
    }

    #[test]
    fn thai_and_combining_marks_pass_through_unescaped() {
        assert_eq!(json_string("ก็ ที่"), "\"ก็ ที่\"");
    }

    #[test]
    fn empty_string() {
        assert_eq!(json_string(""), "\"\"");
    }
}
