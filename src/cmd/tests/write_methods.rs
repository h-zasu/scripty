//! Tests for write methods (write_to, write_err_to, write_both_to)

use crate::cmd;
use serial_test::serial;
use std::io::Cursor;

#[test]
#[serial]
fn test_write_to_basic() {
    let mut buffer = Vec::new();
    cmd!("echo", "hello world")
        .no_echo()
        .write_to(&mut buffer)
        .unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output.trim(), "hello world");
}

#[test]
#[serial]
fn test_write_to_with_input() {
    let mut buffer = Vec::new();
    cmd!("sort")
        .no_echo()
        .input("zebra\napple\nbanana\n")
        .write_to(&mut buffer)
        .unwrap();

    let output = String::from_utf8(buffer).unwrap();
    let lines: Vec<&str> = output.trim().split('\n').collect();
    assert_eq!(lines, vec!["apple", "banana", "zebra"]);
}

#[test]
#[serial]
fn test_write_err_to_basic() {
    let mut buffer = Vec::new();
    // Use a command that outputs to stderr
    cmd!("sh", "-c", "echo 'error message' >&2")
        .no_echo()
        .write_err_to(&mut buffer)
        .unwrap();

    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output.trim(), "error message");
}

#[test]
#[serial]
fn test_write_both_to_basic() {
    let buffer = Vec::new();
    let cursor = std::io::Cursor::new(buffer);
    // Command that outputs to both stdout and stderr
    cmd!("sh", "-c", "echo 'stdout'; echo 'stderr' >&2")
        .no_echo()
        .write_both_to(cursor)
        .unwrap();

    // Note: This test verifies that write_both_to executes without error
    // Actual output verification would require a more complex setup
}

#[test]
#[serial]
fn test_write_methods_with_pipeline() {
    // Test write_to with pipeline
    let mut stdout_buffer = Vec::new();
    cmd!("echo", "hello")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .no_echo()
        .write_to(&mut stdout_buffer)
        .unwrap();

    let output = String::from_utf8(stdout_buffer).unwrap();
    assert_eq!(output.trim(), "HELLO");
}

#[test]
#[serial]
fn test_write_to_empty_output() {
    let mut buffer = Vec::new();
    cmd!("true") // Command that produces no output
        .no_echo()
        .write_to(&mut buffer)
        .unwrap();

    assert!(buffer.is_empty());
}

#[test]
#[serial]
fn test_write_err_to_no_error_output() {
    let mut buffer = Vec::new();
    cmd!("echo", "only stdout")
        .no_echo()
        .write_err_to(&mut buffer)
        .unwrap();

    assert!(buffer.is_empty());
}

#[test]
#[serial]
fn test_write_both_to_mixed_output() {
    let buffer = Vec::new();
    let cursor = std::io::Cursor::new(buffer);
    // Complex command with both stdout and stderr
    cmd!(
        "sh",
        "-c",
        "echo 'line1'; echo 'error1' >&2; echo 'line2'; echo 'error2' >&2"
    )
    .no_echo()
    .write_both_to(cursor)
    .unwrap();

    // Note: This test verifies that write_both_to executes without error
    // Actual output verification would require a more complex setup
}

#[test]
#[serial]
fn test_write_methods_with_binary_data() {
    // Test with binary input and output
    let binary_input = vec![0u8, 1, 2, 3, 4, 5];
    let mut buffer = Vec::new();

    cmd!("cat")
        .no_echo()
        .input_bytes(&binary_input)
        .write_to(&mut buffer)
        .unwrap();

    assert_eq!(buffer, binary_input);
}

#[test]
#[serial]
fn test_write_to_large_output() {
    // Test with larger output to ensure streaming works
    let mut buffer = Vec::new();
    cmd!("seq", "1", "100")
        .no_echo()
        .write_to(&mut buffer)
        .unwrap();

    let output = String::from_utf8(buffer).unwrap();
    let lines: Vec<&str> = output.trim().split('\n').collect();
    assert_eq!(lines.len(), 100);
    assert_eq!(lines[0], "1");
    assert_eq!(lines[99], "100");
}

#[test]
#[serial]
fn test_write_methods_error_handling() {
    // Test with command that fails
    let mut buffer = Vec::new();
    let result = cmd!("sh", "-c", "echo 'output'; exit 1")
        .no_echo()
        .write_to(&mut buffer);

    // Should capture output even if command fails
    assert!(result.is_err());
    let output = String::from_utf8(buffer).unwrap();
    assert_eq!(output.trim(), "output");
}

#[test]
#[serial]
fn test_write_with_cursor() {
    // Test using cursor as output
    let cursor = Cursor::new(Vec::new());
    cmd!("echo", "test data")
        .no_echo()
        .write_to(cursor)
        .unwrap();

    // Note: This test verifies that write_to executes with a cursor without error
    // For actual output verification, we use the buffer-based tests above
}
