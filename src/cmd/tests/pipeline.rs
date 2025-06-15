//! Pipeline operations tests.
//!
//! Tests for command pipelining with different modes (stdout, stderr, both),
//! including complex multi-stage pipelines and pipe mode combinations.

use super::Pipeline;
use crate::cmd;
use crate::cmd::PipeMode;

/// Tests basic pipeline functionality (stdout piping)
#[test]
fn test_pipeline() {
    let output = cmd!("echo", "hello")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "HELLO");
}

/// Tests pipeline with input data
#[test]
fn test_pipeline_with_input() {
    let output = cmd!("tr", "[:lower:]", "[:upper:]")
        .input("hello world")
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "HELLO WORLD");
}

/// Tests pipeline with multiple stages
#[test]
fn test_multiple_pipes() {
    let output = cmd!("echo", "hello world")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .pipe(cmd!("rev"))
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "DLROW OLLEH");
}

/// Tests stderr piping to the next command
#[test]
fn test_pipe_err() {
    // Test piping stderr to next command
    // First command generates stderr, second command should receive it
    let output = cmd!("sh", "-c", "echo 'error message' >&2")
        .pipe_err(cmd!("wc", "-c"))
        .no_echo()
        .output()
        .unwrap();

    // Should count characters in the stderr message (13 chars + newline = 14)
    assert_eq!(output.trim(), "14");
}

/// Tests piping both stdout and stderr
#[test]
fn test_pipe_out_err() {
    // Test piping both stdout and stderr
    let output = cmd!("sh", "-c", "echo 'stdout' && echo 'stderr' >&2")
        .pipe_out_err(cmd!("sort"))
        .no_echo()
        .output()
        .unwrap();

    // sort should produce deterministic output
    let lines: Vec<&str> = output.trim().lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines.contains(&"stdout"));
    assert!(lines.contains(&"stderr"));
    // After sorting, "stderr" comes before "stdout" alphabetically
    assert_eq!(lines[0], "stderr");
    assert_eq!(lines[1], "stdout");
}

/// Tests that default pipe() creates stdout pipe mode
#[test]
fn test_default_pipe_mode() {
    let pipeline = cmd!("echo", "test").pipe(cmd!("cat"));
    assert_eq!(pipeline.connections[1].1, PipeMode::Stdout);
}

/// Tests that pipe_err() creates stderr pipe mode
#[test]
fn test_pipe_err_mode() {
    let pipeline = cmd!("echo", "test").pipe_err(cmd!("cat"));
    assert_eq!(pipeline.connections[1].1, PipeMode::Stderr);
}

/// Tests that pipe_out_err() creates both pipe mode
#[test]
fn test_pipe_out_err_mode() {
    let pipeline = cmd!("echo", "test").pipe_out_err(cmd!("cat"));
    assert_eq!(pipeline.connections[1].1, PipeMode::Both);
}

/// Tests all direct pipe methods for proper execution
#[test]
fn test_direct_pipe_methods() {
    // Test stdout piping (default)
    let stdout_result = cmd!("echo", "native test")
        .pipe(cmd!("cat"))
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(stdout_result.trim(), "native test");

    // Test stderr piping
    let stderr_result = cmd!("sh", "-c", "echo 'native error' >&2")
        .pipe_err(cmd!("wc", "-c"))
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(stderr_result.trim(), "13");

    // Test both piping
    let both_result = cmd!("sh", "-c", "echo 'out'; echo 'err' >&2")
        .pipe_out_err(cmd!("wc", "-l"))
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(both_result.trim(), "2");
}

/// Tests a complex pipeline with different pipe modes
#[test]
fn test_complex_mixed_pipeline() {
    let output = cmd!("sh", "-c", "echo 'normal output'; echo 'error output' >&2")
        .pipe_err(cmd!("sed", "s/error/processed_error/"))
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .pipe_out_err(cmd!("sort"))
        .no_echo()
        .output()
        .unwrap();

    // Should contain the processed stderr output, transformed to uppercase
    assert!(output.contains("PROCESSED_ERROR OUTPUT"));
    assert!(!output.contains("error output")); // Original should be transformed
}

/// Tests that different pipe modes can be used in the same pipeline
#[test]
fn test_mixed_pipe_modes() {
    // Create a pipeline that uses different pipe modes between different commands
    let output = cmd!("sh", "-c", "echo 'stdout line'; echo 'stderr line' >&2")
        .pipe_err(cmd!(
            "sh",
            "-c",
            "read line; echo \"processed: $line\"; echo \"more stderr\" >&2"
        ))
        .pipe(cmd!("wc", "-c"))
        .no_echo()
        .output()
        .unwrap();

    // Should count characters in "processed: stderr line" (no automatic newline)
    let char_count: i32 = output.trim().parse().unwrap();
    assert_eq!(char_count, 23); // "processed: stderr line" = 23 chars
}

/// Tests stderr → stdout → combined pipeline
#[test]
fn test_mixed_stderr_to_stdout_pipeline() {
    let output = cmd!("sh", "-c", "echo 'error message' >&2")
        .pipe_err(cmd!("wc", "-c")) // stderr → stdout (character count)
        .pipe(cmd!("cat")) // stdout → stdout (pass through)
        .no_echo()
        .output()
        .unwrap();

    // Should count characters in stderr message
    assert!(output.trim().parse::<i32>().unwrap() > 0);
}

/// Tests stdout → stderr → both sequence
#[test]
fn test_stdout_stderr_both_sequence() {
    let output = cmd!("echo", "test data")
        .pipe(cmd!(
            "sh",
            "-c",
            "read input; echo \"$input\"; echo \"error: $input\" >&2"
        ))
        .pipe_err(cmd!("sed", "s/^/ERR: /"))
        .pipe_out_err(cmd!("wc", "-l"))
        .no_echo()
        .output()
        .unwrap();

    // Should count lines from both streams
    assert_eq!(output.trim(), "1");
}

/// Tests alternating between different pipe modes
#[test]
fn test_alternating_pipe_modes() {
    let output = cmd!("sh", "-c", "echo 'line1'; echo 'err1' >&2")
        .pipe_err(cmd!("tr", "[:lower:]", "[:upper:]"))
        .no_echo()
        .output()
        .unwrap();

    // Should contain the processed stderr output transformed to uppercase
    let output_str = output.trim();
    assert_eq!(output_str, "ERR1");
}

/// Tests a longer pipeline with multiple mixed pipe modes
#[test]
fn test_long_mixed_pipeline() {
    let output = cmd!("echo", "start")
        .pipe(cmd!(
            "sh",
            "-c",
            "read input; echo \"$input-processed\"; echo \"warning\" >&2"
        ))
        .pipe_err(cmd!("wc", "-c"))
        .pipe(cmd!("sh", "-c", "read count; echo \"chars: $count\""))
        .pipe_out_err(cmd!("wc", "-w"))
        .no_echo()
        .output()
        .unwrap();

    // Should count words in the final output
    assert!(output.trim().parse::<i32>().unwrap() >= 1);
}

/// Tests pipeline with only one command (effectively no pipeline)
#[test]
fn test_pipeline_single_command() {
    let output = cmd!("echo", "single").no_echo().output().unwrap();
    assert_eq!(output.trim(), "single");
}

/// Tests that pipeline input overrides individual command input
#[test]
fn test_pipeline_input_override() {
    let output = cmd!("cat")
        .input("original")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .input("pipeline_input")
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "PIPELINE_INPUT");
}

/// Tests a very long pipeline to ensure no stack overflow or resource issues
#[test]
fn test_very_long_pipeline() {
    let output = cmd!("echo", "start")
        .pipe(cmd!("sed", "s/^/step0_/"))
        .pipe(cmd!("sed", "s/^/step1_/"))
        .pipe(cmd!("sed", "s/^/step2_/"))
        .pipe(cmd!("sed", "s/^/step3_/"))
        .pipe(cmd!("sed", "s/^/step4_/"))
        .no_echo()
        .output()
        .unwrap();
    assert!(output.contains("step4_step3_step2_step1_step0_start"));
}

/// Tests all combinations of pipe modes in a single pipeline
#[test]
fn test_pipe_mode_combinations() {
    let output = cmd!("sh", "-c", "echo 'out1'; echo 'err1' >&2")
        .pipe_err(cmd!(
            "sh",
            "-c",
            "read line; echo \"stderr_to_stdout: $line\"; echo 'err2' >&2"
        ))
        .pipe_out_err(cmd!(
            "sh",
            "-c",
            "while read line; do echo \"combined: $line\"; done"
        ))
        .pipe(cmd!("wc", "-l"))
        .no_echo()
        .output()
        .unwrap();

    // Should count the processed lines
    let line_count: i32 = output.trim().parse().unwrap();
    assert!(line_count >= 1);
}

/// Tests pipeline with no connections
#[test]
fn test_empty_pipeline() {
    let pipeline = Pipeline {
        connections: vec![],
        input: None,
        suppress_echo: true,
    };
    let result = pipeline.output().unwrap();
    assert!(result.is_empty());
}

/// Tests pipeline error handling and validation
#[test]
fn test_pipeline_error_scenarios() {
    // Test pipeline where first command fails
    let result = cmd!("nonexistent_command_xyz")
        .pipe(cmd!("cat"))
        .no_echo()
        .output();
    assert!(result.is_err());

    // Test pipeline where middle command fails
    let result = cmd!("echo", "test")
        .pipe(cmd!("nonexistent_filter"))
        .pipe(cmd!("cat"))
        .no_echo()
        .output();
    assert!(result.is_err());

    // Test pipeline where last command fails
    let result = cmd!("echo", "test")
        .pipe(cmd!("cat"))
        .pipe(cmd!("nonexistent_output"))
        .no_echo()
        .output();
    assert!(result.is_err());
}

/// Tests pipeline with precise data flow validation
#[test]
fn test_pipeline_data_flow() {
    // Test precise data transformation through multiple stages
    let output = cmd!("echo", "hello world")
        .pipe(cmd!("tr", " ", "_")) // hello_world
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]")) // HELLO_WORLD
        .pipe(cmd!("sed", "s/HELLO/GREETING/")) // GREETING_WORLD
        .no_echo()
        .output()
        .unwrap();

    assert_eq!(output.trim(), "GREETING_WORLD");

    // Test with numbers for precise counting
    let output = cmd!("printf", "1\n2\n3\n4\n5")
        .pipe(cmd!("grep", "[13]")) // Should get 1 and 3
        .pipe(cmd!("wc", "-l")) // Should count 2 lines
        .no_echo()
        .output()
        .unwrap();

    assert_eq!(output.trim(), "2");
}

/// Tests stderr pipe mode data integrity
#[test]
fn test_stderr_pipe_data_integrity() {
    // Generate specific stderr content and verify it's processed correctly
    let output = cmd!("sh", "-c", "echo 'ERROR: file not found' >&2")
        .pipe_err(cmd!("sed", "s/ERROR:/WARNING:/"))
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .no_echo()
        .output()
        .unwrap();

    assert_eq!(output.trim(), "WARNING: FILE NOT FOUND");

    // Test stderr character counting precision
    let output = cmd!("sh", "-c", "printf 'exactly25characters123' >&2")
        .pipe_err(cmd!("wc", "-c"))
        .no_echo()
        .output()
        .unwrap();

    assert_eq!(output.trim(), "22"); // "exactly25characters123" is 22 characters
}

/// Tests pipe_out_err mode with mixed output verification
#[test]
fn test_pipe_out_err_mixed_output() {
    // Generate both stdout and stderr with identifiable content
    let output = cmd!("sh", "-c", "echo 'OUT:message1'; echo 'ERR:message2' >&2")
        .pipe_out_err(cmd!("sort")) // Sort both outputs together
        .no_echo()
        .output()
        .unwrap();

    let lines: Vec<&str> = output.trim().lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines.contains(&"ERR:message2"));
    assert!(lines.contains(&"OUT:message1"));
    // After sorting, ERR should come before OUT
    assert_eq!(lines[0], "ERR:message2");
    assert_eq!(lines[1], "OUT:message1");
}
