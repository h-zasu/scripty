//! No echo mode tests.
//!
//! Tests for no echo execution mode functionality, including command echo suppression,
//! pipeline propagation, and inheritance behavior.

use crate::cmd;

/// Tests basic no echo mode functionality
#[test]
fn test_no_echo_mode() {
    // Test that no echo mode doesn't crash with run()
    let result = cmd!("echo", "test").no_echo().run();
    assert!(result.is_ok());

    // Test that no echo mode doesn't crash with output()
    let result = cmd!("echo", "test").no_echo().output();
    assert!(result.is_ok());

    // Test no echo mode with failing command
    let result = cmd!("nonexistent_command").no_echo().run();
    assert!(result.is_err());
}

/// Tests that no echo mode propagates correctly through pipelines
#[test]
fn test_pipeline_no_echo_propagation() {
    let pipeline_no_echo = cmd!("echo", "test")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .no_echo();

    let pipeline_normal = cmd!("echo", "test").pipe(cmd!("tr", "[:lower:]", "[:upper:]"));

    assert!(pipeline_no_echo.suppress_echo);
    assert!(!pipeline_normal.suppress_echo);

    let output_no_echo = pipeline_no_echo.output().unwrap();
    let output_normal = pipeline_normal.output().unwrap();

    assert_eq!(output_no_echo.trim(), "TEST");
    assert_eq!(output_normal.trim(), "TEST");
}

/// Tests that no echo mode is inherited when creating pipelines
#[test]
fn test_no_echo_mode_inheritance() {
    let no_echo_cmd = cmd!("echo", "hello").no_echo();
    let pipeline = no_echo_cmd.pipe(cmd!("cat"));

    assert!(pipeline.suppress_echo);

    let normal_cmd = cmd!("echo", "hello");
    let pipeline2 = normal_cmd.pipe(cmd!("cat"));

    assert!(!pipeline2.suppress_echo);
}

/// Tests no echo mode with various execution methods
#[test]
fn test_no_echo_mode_execution_methods() {
    // Test run() method with no echo
    let result = cmd!("echo", "run_test").no_echo().run();
    assert!(result.is_ok());

    // Test output() method with no echo
    let output = cmd!("echo", "output_test").no_echo().output().unwrap();
    assert_eq!(output.trim(), "output_test");

    // Test with input
    let output = cmd!("cat").input("input_test").no_echo().output().unwrap();
    assert_eq!(output.trim(), "input_test");
}

/// Tests no echo mode with environment variables and working directory
#[test]
fn test_no_echo_mode_with_env_and_dir() {
    use std::env;

    let temp_dir = env::temp_dir();

    let output = cmd!("printenv", "NO_ECHO_TEST_VAR")
        .env("NO_ECHO_TEST_VAR", "no_echo_value")
        .current_dir(&temp_dir)
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "no_echo_value");
}

/// Tests no echo mode with error conditions
#[test]
fn test_no_echo_mode_error_handling() {
    // Test no echo mode with command not found
    let result = cmd!("command_that_does_not_exist_xyz").no_echo().run();
    assert!(result.is_err());

    // Test no echo mode with non-zero exit
    let result = cmd!("sh", "-c", "exit 42").no_echo().run();
    assert!(result.is_err());

    // Test no echo mode with invalid directory
    let result = cmd!("echo", "test")
        .current_dir("/path/that/does/not/exist/xyz")
        .no_echo()
        .run();
    assert!(result.is_err());
}

/// Tests no echo mode behavior consistency
#[test]
fn test_no_echo_mode_consistency() {
    // Multiple executions should have consistent behavior
    for i in 0..5 {
        let output = cmd!("echo", &format!("test_{}", i))
            .no_echo()
            .output()
            .unwrap();
        assert_eq!(output.trim(), format!("test_{}", i));
    }
}

/// Tests no echo mode with complex pipelines
#[test]
fn test_no_echo_mode_complex_pipelines() {
    // Test no echo mode with stderr piping
    let output = cmd!("sh", "-c", "echo 'error_msg' >&2")
        .pipe_err(cmd!("tr", "[:lower:]", "[:upper:]"))
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "ERROR_MSG");

    // Test no echo mode with mixed pipe modes
    let output = cmd!("sh", "-c", "echo 'out'; echo 'err' >&2")
        .pipe_out_err(cmd!("sort"))
        .no_echo()
        .output()
        .unwrap();
    let lines: Vec<&str> = output.trim().lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines.contains(&"err"));
    assert!(lines.contains(&"out"));
}

/// Tests no echo mode flag propagation in builder pattern
#[test]
fn test_no_echo_mode_builder_propagation() {
    // Test that no echo flag is preserved through builder chain
    let cmd = cmd!("echo", "test")
        .arg("extra")
        .env("TEST_VAR", "value")
        .no_echo()
        .arg("more");

    assert!(cmd.suppress_echo);

    // Test pipeline creation from no echo command
    let pipeline = cmd.pipe(cmd!("cat"));
    assert!(pipeline.suppress_echo);
}
