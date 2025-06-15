//! Error handling tests.
//!
//! Tests for various error conditions and edge cases in command execution,
//! including non-existent commands, exit codes, and error message quality.

use super::Cmd;
use crate::cmd;
use std::ffi::OsString;

/// Tests comprehensive command not found error handling
#[test]
fn test_command_not_found_error() {
    // Test basic command not found
    let result = cmd!("nonexistent_command_12345").no_echo().run();
    assert!(result.is_err());

    // Test that error message is informative
    let error = result.unwrap_err();
    assert!(error.message.contains("Failed to spawn command"));
    assert!(error.message.contains("nonexistent_command_12345"));

    // Test with different non-existent command
    let result = cmd!("this_command_definitely_does_not_exist")
        .no_echo()
        .run();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.message.contains("Failed to spawn command"));
    assert!(
        error
            .message
            .contains("this_command_definitely_does_not_exist")
    );

    // Test with output() method
    let result = cmd!("missing_command").no_echo().output();
    assert!(result.is_err());
}

/// Tests command that exits with non-zero status
#[test]
fn test_exit_code_handling() {
    // Test various exit codes
    for exit_code in [1, 2, 127, 255] {
        let result = cmd!("sh", "-c", &format!("exit {}", exit_code))
            .no_echo()
            .run();
        assert!(
            result.is_err(),
            "Exit code {} should result in error",
            exit_code
        );
    }

    // Test with output() method
    let result = cmd!("sh", "-c", "exit 42").no_echo().output();
    assert!(result.is_err());

    // Test successful command (should not error)
    let result = cmd!("sh", "-c", "exit 0").no_echo().run();
    assert!(result.is_ok());
}

/// Tests empty command handling
#[test]
fn test_empty_command_handling() {
    let cmd = Cmd::new("");
    assert_eq!(cmd.program, OsString::from(""));
    let result = cmd.no_echo().run();
    assert!(result.is_err());

    // Test empty command with output
    let result = Cmd::new("").no_echo().output();
    assert!(result.is_err());
}

/// Tests pipeline error propagation
#[test]
fn test_pipeline_error_propagation() {
    // First command fails
    let result = cmd!("nonexistent_command")
        .pipe(cmd!("cat"))
        .no_echo()
        .run();
    assert!(result.is_err());

    // Second command fails
    let result = cmd!("echo", "test")
        .pipe(cmd!("nonexistent_command"))
        .no_echo()
        .run();
    assert!(result.is_err());

    // First command exits with error
    let result = cmd!("sh", "-c", "exit 1").pipe(cmd!("cat")).no_echo().run();
    assert!(result.is_err());
}
