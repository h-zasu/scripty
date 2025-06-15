//! Tests to verify that run() actually outputs to stdout/stderr.
//! This ensures we catch regression bugs where run() doesn't properly inherit stdio.
//!
//! # Why These Tests Are Special
//!
//! Standard unit tests that use `output()` method capture stdout/stderr into a string,
//! but they don't verify that `run()` actually outputs to the terminal. In the past,
//! we had a regression where `run()` wasn't properly inheriting stdout/stderr, causing
//! commands to produce no visible output even though the tests passed.
//!
//! # The Subprocess Approach
//!
//! These tests use a special subprocess approach to verify actual output behavior:
//!
//! 1. Each test checks for a `TEST_SUBPROCESS` environment variable
//! 2. If not set, the test spawns itself as a subprocess with the variable set
//! 3. In the subprocess, the actual `run()` command is executed
//! 4. The parent process captures the subprocess's stdout/stderr
//! 5. The captured output is then verified
//!
//! This approach ensures we're testing the real behavior of `run()` - that it actually
//! outputs to the inherited stdout/stderr streams.
//!
//! # Example Pattern
//!
//! ```rust,ignore
//! #[test]
//! fn test_run_outputs_to_stdout() {
//!     if std::env::var("TEST_SUBPROCESS").is_ok() {
//!         // In subprocess: execute the actual command
//!         cmd!("echo", "hello").run().unwrap();
//!         return;
//!     }
//!     // In parent: spawn subprocess and verify output
//!     let output = Command::new(std::env::current_exe().unwrap())
//!         .env("TEST_SUBPROCESS", "1")
//!         // ... capture stdout/stderr ...
//!     assert!(stdout.contains("hello"));
//! }
//! ```
//!
//! # Important Notes
//!
//! - These tests may include test runner output (e.g., "running 1 test"), so exact
//!   output matching isn't always possible
//! - Binary data tests need to search for the expected bytes within the output
//! - Large output tests should count only the relevant lines, not total line count

use crate::cmd;
use std::process::{Command, Stdio};

#[test]
fn test_run_outputs_to_stdout() {
    // This test will be run in a subprocess
    if std::env::var("TEST_SUBPROCESS").is_ok() {
        // When running in subprocess, execute the actual test
        cmd!("echo", "hello from run").run().unwrap();
        return;
    }

    // Main test: spawn subprocess and verify output
    let output = Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("cmd::tests::run_output_verification::test_run_outputs_to_stdout")
        .arg("--nocapture")
        .env("TEST_SUBPROCESS", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("hello from run"),
        "run() should output to stdout, but got: {}",
        stdout
    );
}

#[test]
fn test_run_outputs_to_stderr() {
    if std::env::var("TEST_SUBPROCESS").is_ok() {
        cmd!("sh", "-c", "echo 'error from run' >&2").run().unwrap();
        return;
    }

    let output = Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("cmd::tests::run_output_verification::test_run_outputs_to_stderr")
        .arg("--nocapture")
        .env("TEST_SUBPROCESS", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("error from run"),
        "run() should output to stderr, but got: {}",
        stderr
    );
}

#[test]
fn test_run_outputs_both_stdout_and_stderr() {
    if std::env::var("TEST_SUBPROCESS").is_ok() {
        cmd!(
            "sh",
            "-c",
            "echo 'stdout message'; echo 'stderr message' >&2"
        )
        .run()
        .unwrap();
        return;
    }

    let output = Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("cmd::tests::run_output_verification::test_run_outputs_both_stdout_and_stderr")
        .arg("--nocapture")
        .env("TEST_SUBPROCESS", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stdout.contains("stdout message"),
        "run() should output to stdout"
    );
    assert!(
        stderr.contains("stderr message"),
        "run() should output to stderr"
    );
}

#[test]
fn test_run_with_pipe_outputs_correctly() {
    if std::env::var("TEST_SUBPROCESS").is_ok() {
        cmd!("echo", "hello")
            .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
            .run()
            .unwrap();
        return;
    }

    let output = Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("cmd::tests::run_output_verification::test_run_with_pipe_outputs_correctly")
        .arg("--nocapture")
        .env("TEST_SUBPROCESS", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("HELLO"),
        "Piped run() should output to stdout"
    );
}

#[test]
fn test_run_vs_output_consistency() {
    // First, get what output() returns
    let output_result = cmd!("echo", "test message").output().unwrap();

    // Now verify run() outputs the same thing
    if std::env::var("TEST_SUBPROCESS").is_ok() {
        cmd!("echo", "test message").run().unwrap();
        return;
    }

    let output = Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("cmd::tests::run_output_verification::test_run_vs_output_consistency")
        .arg("--nocapture")
        .env("TEST_SUBPROCESS", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("test message"),
        "run() should produce same output as output()"
    );
    // Note: We can't compare exact output because test runner adds its own output
    assert!(
        stdout.contains(output_result.trim()),
        "run() output should contain what output() returns"
    );
}

#[test]
fn test_run_with_input_outputs_correctly() {
    if std::env::var("TEST_SUBPROCESS").is_ok() {
        cmd!("cat").input("input data").run().unwrap();
        return;
    }

    let output = Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("cmd::tests::run_output_verification::test_run_with_input_outputs_correctly")
        .arg("--nocapture")
        .env("TEST_SUBPROCESS", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("input data"),
        "run() with input should output to stdout"
    );
}

#[test]
fn test_run_preserves_output_order() {
    if std::env::var("TEST_SUBPROCESS").is_ok() {
        // Generate multiple lines to test order preservation
        cmd!("sh", "-c", "for i in 1 2 3 4 5; do echo \"Line $i\"; done")
            .run()
            .unwrap();
        return;
    }

    let output = Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("cmd::tests::run_output_verification::test_run_preserves_output_order")
        .arg("--nocapture")
        .env("TEST_SUBPROCESS", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Line 1"), "Should contain Line 1");
    assert!(stdout.contains("Line 2"), "Should contain Line 2");
    assert!(stdout.contains("Line 3"), "Should contain Line 3");
    assert!(stdout.contains("Line 4"), "Should contain Line 4");
    assert!(stdout.contains("Line 5"), "Should contain Line 5");

    // Verify order
    let line1_pos = stdout.find("Line 1").unwrap();
    let line2_pos = stdout.find("Line 2").unwrap();
    let line3_pos = stdout.find("Line 3").unwrap();
    let line4_pos = stdout.find("Line 4").unwrap();
    let line5_pos = stdout.find("Line 5").unwrap();

    assert!(line1_pos < line2_pos, "Line 1 should come before Line 2");
    assert!(line2_pos < line3_pos, "Line 2 should come before Line 3");
    assert!(line3_pos < line4_pos, "Line 3 should come before Line 4");
    assert!(line4_pos < line5_pos, "Line 4 should come before Line 5");
}

#[test]
fn test_run_with_no_echo_still_outputs() {
    if std::env::var("TEST_SUBPROCESS").is_ok() {
        cmd!("echo", "silent but visible").no_echo().run().unwrap();
        return;
    }

    let output = Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("cmd::tests::run_output_verification::test_run_with_no_echo_still_outputs")
        .arg("--nocapture")
        .env("TEST_SUBPROCESS", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("silent but visible"),
        "no_echo() should not affect run() output"
    );
}

/// Test that verifies run() properly handles binary output
#[test]
fn test_run_handles_binary_output() {
    if std::env::var("TEST_SUBPROCESS").is_ok() {
        // Use Python for consistent cross-platform binary output
        // Python is widely available and handles binary output consistently
        cmd!(
            "python3",
            "-c",
            "import sys; sys.stdout.buffer.write(b'\\x00\\x01\\x02\\xFF'); sys.stdout.buffer.flush()"
        )
        .run()
        .unwrap();
        return;
    }

    let output = Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("cmd::tests::run_output_verification::test_run_handles_binary_output")
        .arg("--nocapture")
        .env("TEST_SUBPROCESS", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    // Verify binary data is preserved (the output will contain test runner output too)
    let stdout = &output.stdout;
    assert!(
        stdout.len() > 4,
        "Should output more than 4 bytes due to test runner output"
    );

    // Find the binary sequence in the output
    let mut found = false;
    for i in 0..stdout.len() - 3 {
        if stdout[i] == 0x00
            && stdout[i + 1] == 0x01
            && stdout[i + 2] == 0x02
            && stdout[i + 3] == 0xFF
        {
            found = true;
            break;
        }
    }
    assert!(
        found,
        "Binary sequence \\x00\\x01\\x02\\xFF should be present in the output"
    );
}

/// Test that run() works correctly with large outputs
#[test]
fn test_run_with_large_output() {
    if std::env::var("TEST_SUBPROCESS").is_ok() {
        // Generate 1000 lines of output
        cmd!(
            "sh",
            "-c",
            "for i in $(seq 1 1000); do echo \"Line number $i\"; done"
        )
        .run()
        .unwrap();
        return;
    }

    let output = Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("cmd::tests::run_output_verification::test_run_with_large_output")
        .arg("--nocapture")
        .env("TEST_SUBPROCESS", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Count lines that match our pattern (exclude test runner output)
    let matching_lines = stdout
        .lines()
        .filter(|line| line.starts_with("Line number "))
        .count();
    assert_eq!(
        matching_lines, 1000,
        "Should output 1000 'Line number' lines"
    );
    assert!(
        stdout.contains("Line number 1"),
        "Should contain first line"
    );
    assert!(
        stdout.contains("Line number 500"),
        "Should contain middle line"
    );
    assert!(
        stdout.contains("Line number 1000"),
        "Should contain last line"
    );
}
