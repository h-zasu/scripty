//! Argument quoting and escaping tests.
//!
//! Tests for proper shell argument escaping and quoting functionality,
//! ensuring command arguments are safely passed to the shell.

use super::*;
use std::ffi::OsString;

/// Tests quoting of simple arguments (no special characters)
#[test]
fn test_quote_argument_simple() {
    let arg = OsString::from("simple");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(quoted, "simple");
}

/// Tests quoting of arguments containing spaces
#[test]
fn test_quote_argument_with_spaces() {
    let arg = OsString::from("hello world");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(quoted, "'hello world'");
}

/// Tests quoting of empty arguments
#[test]
fn test_quote_argument_empty() {
    let arg = OsString::from("");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(quoted, "\"\"");
}

/// Tests quoting of arguments containing single quotes
#[test]
fn test_quote_argument_with_single_quotes() {
    let arg = OsString::from("it's a test");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(quoted, "\"it's a test\"");
}

/// Tests quoting of arguments containing double quotes
#[test]
fn test_quote_argument_with_double_quotes() {
    let arg = OsString::from("say \"hello\"");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(quoted, "'say \"hello\"'");
}

/// Tests quoting of arguments containing both single and double quotes
#[test]
fn test_quote_argument_with_mixed_quotes() {
    let arg = OsString::from("it's a \"test\"");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(quoted, "\"it's a \\\"test\\\"\"");
}

/// Tests quoting of arguments with various special shell characters
#[test]
fn test_quote_argument_with_various_characters() {
    let test_cases = vec![
        ("file*.txt", "'file*.txt'"),
        ("$HOME/test", "'$HOME/test'"),
        ("command|grep", "'command|grep'"),
        ("arg&background", "'arg&background'"),
        ("path;command", "'path;command'"),
        ("(group)", "'(group)'"),
        ("redirect>file", "'redirect>file'"),
        ("[pattern]", "'[pattern]'"),
        ("{expansion}", "'{expansion}'"),
        ("back`tick", "'back`tick'"),
        ("hash#comment", "'hash#comment'"),
        ("exclaim!", "'exclaim!'"),
        ("tilde~path", "'tilde~path'"),
    ];

    for (input, expected) in test_cases {
        let arg = OsString::from(input);
        let quoted = Cmd::quote_argument(&arg);
        assert_eq!(quoted, expected, "Failed for input: {}", input);
    }
}

/// Tests quoting of arguments containing control characters
#[test]
fn test_quote_argument_with_control_characters() {
    let arg = OsString::from("line1\nline2");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(quoted, "'line1\\nline2'");

    let arg = OsString::from("tab\there");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(quoted, "'tab\\there'");
}

/// Tests quoting of arguments with backslashes and quotes
#[test]
fn test_quote_argument_with_backslash_and_quotes() {
    let arg = OsString::from("path\\with\"quotes");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(quoted, "'path\\with\"quotes'");
}

/// Tests quoting of arguments with dollar signs and backticks
#[test]
fn test_quote_argument_with_dollar_and_backtick() {
    let arg = OsString::from("$VAR and `command`");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(quoted, "'$VAR and `command`'");
}

/// Tests complex escaping scenarios
#[test]
fn test_quote_argument_complex_escaping() {
    // Test case with single quotes that requires double quote escaping
    let arg = OsString::from("can't use $HOME or `pwd`");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(quoted, "\"can't use $HOME or `pwd`\"");
}

/// Tests quoting with mixed control characters
#[test]
fn test_quote_argument_with_mixed_control_chars() {
    let arg = OsString::from("line1\nhas\ttabs\rand\0null");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(quoted, "'line1\\nhas\\ttabs\\rand\\0null'");
}

/// Tests quoting single quotes combined with control characters
#[test]
fn test_quote_argument_single_quotes_with_control_chars() {
    let arg = OsString::from("can't\nuse\ttabs");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(quoted, "\"can't\\nuse\\ttabs\"");
}

/// Tests complex combinations of control characters
#[test]
fn test_quote_argument_complex_control_combinations() {
    // Test null character
    let arg = OsString::from("test\0null");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(quoted, "'test\\0null'");

    // Test carriage return
    let arg = OsString::from("windows\r\nline");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(quoted, "'windows\\r\\nline'");

    // Test bell character (control character)
    let arg = OsString::from("bell\x07char");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(quoted, "'bell\\x07char'");

    // Test DEL character
    let arg = OsString::from("del\x7fchar");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(quoted, "'del\\x7fchar'");
}

/// Tests the most complex quoting scenario with everything mixed together
#[test]
fn test_quote_argument_mixed_everything() {
    // Test argument with single quotes, control chars, and regular text
    let arg = OsString::from("can't handle\tthis\ncomplex 'string' with\0null");
    let quoted = Cmd::quote_argument(&arg);
    assert_eq!(
        quoted,
        "\"can't handle\\tthis\\ncomplex 'string' with\\0null\""
    );
}
