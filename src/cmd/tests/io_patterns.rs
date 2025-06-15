//! Complete I/O Pattern Test Coverage
//!
//! This module tests all 8 possible I/O control patterns (2³ = 8) in scripty:
//!
//! Pattern Coverage:
//! - 000: Basic execution (run/output) - Traditional methods
//! - 100: stdin only (spawn_io_in) - Data input control
//! - 010: stdout only (spawn_io_out) - Output capture
//! - 001: stderr only (spawn_io_err) - Error monitoring
//! - 110: stdin + stdout (spawn_io_in_out) - Interactive processing ⭐
//! - 101: stdin + stderr (spawn_io_in_err) - Debug scenarios ⭐
//! - 011: stdout + stderr (spawn_io_out_err) - Output separation
//! - 111: All I/O (spawn_io_all) - Complete control

use crate::cmd;
use crate::io_ext::ReadExt;
use std::io::{BufRead, BufReader, Cursor, Read, Write};
use std::thread;

/// Test Pattern 000: Basic execution (no I/O control)
#[test]
fn test_pattern_000_basic_execution() {
    // Test basic run()
    let result = cmd!("echo", "hello").no_echo().run();
    assert!(result.is_ok());

    // Test basic output()
    let output = cmd!("echo", "basic_test").no_echo().output().unwrap();
    assert_eq!(output.trim(), "basic_test");

    // Test with pipeline
    let output = cmd!("echo", "world")
        .pipe(cmd!("grep", "world"))
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "world");
}

/// Test Pattern 100: stdin only control
#[test]
fn test_pattern_100_stdin_only() {
    let (handle, stdin) = cmd!("wc", "-l").no_echo().spawn_io_in().unwrap();

    let input_handle = stdin.map(|mut stdin| {
        thread::spawn(move || {
            stdin.write_all(b"line1\nline2\nline3\n").unwrap();
            // stdin auto-closes when dropped
        })
    });

    if let Some(h) = input_handle {
        h.join().unwrap();
    }

    let result = handle.wait();
    assert!(result.is_ok());
}

/// Test Pattern 010: stdout only control
#[test]
fn test_pattern_010_stdout_only() {
    let (handle, stdout) = cmd!("seq", "1", "3").no_echo().spawn_io_out().unwrap();

    let output_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(stdout);
            reader.read_to_end(&mut buffer).unwrap();
            String::from_utf8_lossy(&buffer).trim().to_string()
        })
    });

    handle.wait().unwrap();

    if let Some(h) = output_handle {
        let output = h.join().unwrap();
        assert_eq!(output, "1\n2\n3");
    }
}

/// Test Pattern 001: stderr only control
#[test]
fn test_pattern_001_stderr_only() {
    let (handle, stderr) = cmd!("sh", "-c", "echo 'error message' >&2")
        .no_echo()
        .spawn_io_err()
        .unwrap();

    let error_handle = stderr.map(|stderr| {
        thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(stderr);
            reader.read_to_end(&mut buffer).unwrap();
            String::from_utf8_lossy(&buffer).trim().to_string()
        })
    });

    handle.wait().unwrap();

    if let Some(h) = error_handle {
        let error_output = h.join().unwrap();
        assert_eq!(error_output, "error message");
    }
}

/// Test Pattern 110: stdin + stdout control (Most Important Interactive Pattern)
#[test]
fn test_pattern_110_stdin_stdout_interactive() {
    // Test 1: Simple data transformation
    let (handle, stdin, stdout) = cmd!("tr", "a-z", "A-Z")
        .no_echo()
        .spawn_io_in_out()
        .unwrap();

    let input_handle = stdin.map(|mut stdin| {
        thread::spawn(move || {
            stdin.write_all(b"hello world").unwrap();
        })
    });

    let output_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(stdout);
            reader.read_to_end(&mut buffer).unwrap();
            String::from_utf8_lossy(&buffer).trim().to_string()
        })
    });

    if let Some(h) = input_handle {
        h.join().unwrap();
    }

    handle.wait().unwrap();

    if let Some(h) = output_handle {
        let output = h.join().unwrap();
        assert_eq!(output, "HELLO WORLD");
    }

    // Test 2: Interactive calculator pattern
    let (handle, stdin, stdout) = cmd!("bc", "-l").no_echo().spawn_io_in_out().unwrap();

    let input_handle = stdin.map(|mut stdin| {
        thread::spawn(move || {
            stdin.write_all(b"2+2\nquit\n").unwrap();
        })
    });

    let output_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            let mut results = Vec::new();
            #[allow(clippy::manual_flatten)]
            for line in reader.lines() {
                if let Ok(line) = line {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() && !trimmed.contains("(standard_in)") {
                        results.push(trimmed.to_string());
                    }
                }
            }
            results
        })
    });

    if let Some(h) = input_handle {
        h.join().unwrap();
    }

    handle.wait().unwrap();

    if let Some(h) = output_handle {
        let results = h.join().unwrap();
        assert!(!results.is_empty());
        // First result should be "4"
        assert_eq!(results.first().unwrap_or(&"".to_string()), "4");
    }
}

/// Test Pattern 101: stdin + stderr control (Debug Pattern)
#[test]
fn test_pattern_101_stdin_stderr_debug() {
    // Test 1: Compilation error monitoring
    let (handle, stdin, stderr) = cmd!("rustc", "-").no_echo().spawn_io_in_err().unwrap();

    let input_handle = stdin.map(|mut stdin| {
        thread::spawn(move || {
            // Send invalid Rust code
            stdin
                .write_all(b"fn main() { let x: i32 = \"invalid\"; }")
                .unwrap();
        })
    });

    let error_handle = stderr.map(|stderr| {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            let mut error_count = 0;
            #[allow(clippy::manual_flatten)]
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.contains("error") {
                        error_count += 1;
                    }
                }
            }
            error_count
        })
    });

    if let Some(h) = input_handle {
        h.join().unwrap();
    }

    let _ = handle.wait(); // Expected to fail

    if let Some(h) = error_handle {
        let error_count = h.join().unwrap();
        assert!(error_count > 0, "Should detect compilation errors");
    }

    // Test 2: JSON validation error monitoring
    let (handle, stdin, stderr) = cmd!("jq", ".").no_echo().spawn_io_in_err().unwrap();

    let input_handle = stdin.map(|mut stdin| {
        thread::spawn(move || {
            // Send invalid JSON
            stdin.write_all(b"{\"name\": \"test\", \"age\": }").unwrap();
        })
    });

    let error_handle = stderr.map(|stderr| {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            let mut has_parse_error = false;
            #[allow(clippy::manual_flatten)]
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.contains("parse error") || line.contains("Invalid") {
                        has_parse_error = true;
                    }
                }
            }
            has_parse_error
        })
    });

    if let Some(h) = input_handle {
        h.join().unwrap();
    }

    let _ = handle.wait(); // Expected to fail

    if let Some(h) = error_handle {
        let has_error = h.join().unwrap();
        assert!(has_error, "Should detect JSON parse error");
    }
}

/// Test Pattern 011: stdout + stderr control
#[test]
fn test_pattern_011_stdout_stderr_separation() {
    let (handle, stdout, stderr) =
        cmd!("sh", "-c", "echo 'normal output'; echo 'error message' >&2")
            .no_echo()
            .spawn_io_out_err()
            .unwrap();

    let stdout_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(stdout);
            reader.read_to_end(&mut buffer).unwrap();
            String::from_utf8_lossy(&buffer).trim().to_string()
        })
    });

    let stderr_handle = stderr.map(|stderr| {
        thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(stderr);
            reader.read_to_end(&mut buffer).unwrap();
            String::from_utf8_lossy(&buffer).trim().to_string()
        })
    });

    handle.wait().unwrap();

    if let Some(h) = stdout_handle {
        let stdout_result = h.join().unwrap();
        assert_eq!(stdout_result, "normal output");
    }

    if let Some(h) = stderr_handle {
        let stderr_result = h.join().unwrap();
        assert_eq!(stderr_result, "error message");
    }
}

/// Test Pattern 111: Complete I/O control
#[test]
fn test_pattern_111_complete_io_control() {
    let spawn = cmd!("sort").no_echo().spawn_io_all().unwrap();

    // Input management
    let input_handle = spawn.stdin.map(|mut stdin| {
        thread::spawn(move || {
            stdin.write_all(b"zebra\napple\nbanana\n").unwrap();
        })
    });

    // Output management
    let output_handle = spawn.stdout.map(|stdout| {
        thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(stdout);
            reader.read_to_end(&mut buffer).unwrap();
            String::from_utf8_lossy(&buffer).trim().to_string()
        })
    });

    // Error management
    let error_handle = spawn.stderr.map(|stderr| {
        thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(stderr);
            reader.read_to_end(&mut buffer).unwrap();
            buffer.is_empty() // Should be no errors for sort
        })
    });

    if let Some(h) = input_handle {
        h.join().unwrap();
    }

    spawn.handle.wait().unwrap();

    if let Some(h) = output_handle {
        let output = h.join().unwrap();
        let lines: Vec<&str> = output.split('\n').collect();
        assert_eq!(lines, vec!["apple", "banana", "zebra"]);
    }

    if let Some(h) = error_handle {
        let no_errors = h.join().unwrap();
        assert!(no_errors, "Sort command should not produce errors");
    }
}

/// Test classic input/output methods for backward compatibility
#[test]
fn test_classic_io_methods() {
    // Test ReadExt::pipe()
    let data = "apple\nbanana\ncherry\ndate\nfig";
    let cursor = Cursor::new(data.as_bytes());
    let output = cursor.pipe(cmd!("grep", "a")).no_echo().output().unwrap();
    assert!(output.contains("apple"));
    assert!(output.contains("banana"));
    assert!(output.contains("date"));

    // Test write_to()
    let mut buffer = Vec::new();
    cmd!("echo", "test_stream")
        .no_echo()
        .write_to(&mut buffer)
        .unwrap();
    let result = String::from_utf8(buffer).unwrap();
    assert_eq!(result.trim(), "test_stream");

    // Test run_with_io()
    let input_data = "zebra\napple\nbanana\ncherry";
    let input_reader = Cursor::new(input_data);
    let mut output_buffer = Vec::new();
    cmd!("sort")
        .no_echo()
        .run_with_io(input_reader, &mut output_buffer)
        .unwrap();
    let result = String::from_utf8(output_buffer).unwrap();
    assert!(result.contains("apple"));
    assert!(result.contains("banana"));

    // Test run_with_err_io()
    let invalid_rust_code = "fn main() { invalid syntax }";
    let input_reader = Cursor::new(invalid_rust_code);
    let mut error_buffer = Vec::new();
    let _ = cmd!("rustc", "--error-format=short", "-")
        .no_echo()
        .run_with_err_io(input_reader, &mut error_buffer);
    let error_output = String::from_utf8(error_buffer).unwrap();
    // rustc should produce some error output
    assert!(!error_output.is_empty());

    // Test run_with_both_io()
    let input_data = "test data\nmore data";
    let input_reader = Cursor::new(input_data);
    let combined_buffer = Vec::new();
    let cursor = Cursor::new(combined_buffer);
    cmd!("sh", "-c", "cat; echo 'stderr message' >&2")
        .no_echo()
        .run_with_both_io(input_reader, cursor)
        .unwrap();
    // Note: This test verifies that run_with_both_io executes without error
    // Actual output verification would require a more complex setup
}

/// Test binary data handling across patterns
#[test]
fn test_binary_data_patterns() {
    // Test binary data with Pattern 100 (stdin only)
    let binary_data: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0xFF, 0xFE, 0xFD, 0xFC];

    let (handle, stdin) = cmd!("wc", "-c").no_echo().spawn_io_in().unwrap();

    let input_handle = stdin.map(|mut stdin| {
        let data = binary_data.clone();
        thread::spawn(move || {
            stdin.write_all(&data).unwrap();
        })
    });

    if let Some(h) = input_handle {
        h.join().unwrap();
    }

    handle.wait().unwrap();

    // Test binary data with Pattern 110 (stdin + stdout)
    let (handle, stdin, stdout) = cmd!("cat").no_echo().spawn_io_in_out().unwrap();

    let input_handle = stdin.map(|mut stdin| {
        let data = binary_data.clone();
        thread::spawn(move || {
            stdin.write_all(&data).unwrap();
        })
    });

    let output_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(stdout);
            reader.read_to_end(&mut buffer).unwrap();
            buffer
        })
    });

    if let Some(h) = input_handle {
        h.join().unwrap();
    }

    handle.wait().unwrap();

    if let Some(h) = output_handle {
        let output = h.join().unwrap();
        assert_eq!(output, binary_data);
    }
}

/// Test error handling across all patterns
#[test]
fn test_error_handling_patterns() {
    // Test error in Pattern 100
    let result = cmd!("nonexistent_command_12345").no_echo().spawn_io_in();
    assert!(result.is_err());

    // Test error in Pattern 010
    let result = cmd!("nonexistent_command_12345").no_echo().spawn_io_out();
    assert!(result.is_err());

    // Test error in Pattern 110
    let result = cmd!("nonexistent_command_12345")
        .no_echo()
        .spawn_io_in_out();
    assert!(result.is_err());

    // Test error in Pattern 111
    let result = cmd!("nonexistent_command_12345").no_echo().spawn_io_all();
    assert!(result.is_err());
}

/// Test pipeline compatibility with new patterns
#[test]
fn test_pipeline_patterns() {
    // Test Pipeline with Pattern 010 (stdout only)
    let (handle, stdout) = cmd!("echo", "hello")
        .pipe(cmd!("tr", "a-z", "A-Z"))
        .no_echo()
        .spawn_io_out()
        .unwrap();

    let output_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(stdout);
            reader.read_to_end(&mut buffer).unwrap();
            String::from_utf8_lossy(&buffer).trim().to_string()
        })
    });

    handle.wait().unwrap();

    if let Some(h) = output_handle {
        let output = h.join().unwrap();
        assert_eq!(output, "HELLO");
    }

    // Test Pipeline with Pattern 111 (complete control)
    let spawn = cmd!("cat")
        .pipe(cmd!("sort"))
        .no_echo()
        .spawn_io_all()
        .unwrap();

    let input_handle = spawn.stdin.map(|mut stdin| {
        thread::spawn(move || {
            stdin.write_all(b"zebra\napple\nbanana\n").unwrap();
        })
    });

    let output_handle = spawn.stdout.map(|stdout| {
        thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(stdout);
            reader.read_to_end(&mut buffer).unwrap();
            String::from_utf8_lossy(&buffer).trim().to_string()
        })
    });

    if let Some(h) = input_handle {
        h.join().unwrap();
    }

    spawn.handle.wait().unwrap();

    if let Some(h) = output_handle {
        let output = h.join().unwrap();
        let lines: Vec<&str> = output.split('\n').collect();
        assert_eq!(lines, vec!["apple", "banana", "zebra"]);
    }
}

/// Performance and stress test for all patterns
#[test]
fn test_pattern_performance() {
    // Test large data with Pattern 110 (most common)
    let large_data = "x".repeat(10000); // 10KB of data

    let (handle, stdin, stdout) = cmd!("wc", "-c").no_echo().spawn_io_in_out().unwrap();

    let input_handle = stdin.map(|mut stdin| {
        let data = large_data.clone();
        thread::spawn(move || {
            stdin.write_all(data.as_bytes()).unwrap();
        })
    });

    let output_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let mut buffer = Vec::new();
            let mut reader = BufReader::new(stdout);
            reader.read_to_end(&mut buffer).unwrap();
            String::from_utf8_lossy(&buffer).trim().to_string()
        })
    });

    if let Some(h) = input_handle {
        h.join().unwrap();
    }

    handle.wait().unwrap();

    if let Some(h) = output_handle {
        let output = h.join().unwrap();
        let count: usize = output.parse().unwrap();
        assert_eq!(count, 10000);
    }
}
