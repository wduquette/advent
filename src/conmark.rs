//! conmark -- console markup
//! This module contains code for marking up text for display to the console.

use textwrap::Wrapper;

/// Wraps a text string for display to the console.  The string is wrapped to fit within
/// the console terminal width.  The string is broken at explicit newlines.
pub fn conwrap(text: &str) -> String {
    let wrapper = Wrapper::with_termwidth();

    wrapper.fill(text)
}

/// Reformats the input string using conmark syntax:
///
/// * The string is trimmed.
/// * Explicit newlines are replaced with blanks.
/// * Pipe characters ("|") are replaced with newlines.
///
/// The reason for this syntax is as follows:
///
/// * Rust's multiline string syntax presumes that you want to retain any explicit line breaks,
///   and allows you to escape from that using "\" when necessary.
/// * But this use case is the opposite.  You usually don't want the explicit line breaks, but
///   in rare cases you'll want to escape from that.
/// * Plus, it builds in a mechanism where we can add more interesting stuff in the long run.
pub fn confmt(text: &str) -> String {
    let mut result = String::new();

    for char in text.trim().chars() {
        match char {
            '\n' => result.push(' '),
            '|' => result.push('\n'),
            c => result.push(c),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conwrap() {
        // Can't test this fully, because we don't know the terminal width.
        // But textwrap has its own tests; the real question is whether we
        // get output.
        assert_eq!(conwrap("ab cd"), "ab cd");
    }

    #[test]
    fn test_confmt_trim() {
        assert_eq!(confmt("  abcd  "), "abcd");
    }

    #[test]
    fn test_confmt_flatten() {
        assert_eq!(confmt("ab\ncd"), "ab cd");
    }

    #[test]
    fn test_confmt_newline() {
        assert_eq!(confmt("ab|cd"), "ab\ncd");
    }
}
