//! Basic command construction tests.
//!
//! Tests for creating and configuring commands using `Cmd::new()`,
//! the `cmd!` macro, and the builder pattern.

use super::*;
use crate::cmd;
use std::ffi::OsString;

/// Tests basic command creation with `Cmd::new()`
#[test]
fn test_cmd_new() {
    let cmd = Cmd::new("echo");
    assert_eq!(cmd.program, OsString::from("echo"));
    assert!(cmd.args.is_empty());
    assert!(!cmd.suppress_echo);
}

/// Tests command creation using the `cmd!` macro with arguments
#[test]
fn test_cmd_with_args() {
    let cmd = cmd!("echo", "hello", "world");
    assert_eq!(cmd.program, OsString::from("echo"));
    assert_eq!(
        cmd.args,
        vec![OsString::from("hello"), OsString::from("world")]
    );
}

/// Tests the builder pattern for command configuration
#[test]
fn test_cmd_builder() {
    use std::env;
    let temp_dir = env::temp_dir();

    let cmd = Cmd::new("ls")
        .arg("-la")
        .env("TEST", "value")
        .current_dir(&temp_dir)
        .no_echo();

    assert_eq!(cmd.program, OsString::from("ls"));
    assert_eq!(cmd.args, vec![OsString::from("-la")]);
    assert_eq!(
        cmd.envs,
        vec![(OsString::from("TEST"), OsString::from("value"))]
    );
    assert_eq!(cmd.current_dir, Some(temp_dir));
    assert!(cmd.suppress_echo);
}

/// Tests command output capture
#[test]
fn test_cmd_output() {
    let output = cmd!("echo", "test").no_echo().output().unwrap();
    assert_eq!(output.trim(), "test");
}

/// Tests command execution with input
#[test]
fn test_cmd_with_input() {
    let output = cmd!("cat").input("hello world").no_echo().output().unwrap();
    assert_eq!(output.trim(), "hello world");
}

/// Tests the `args()` method for adding multiple arguments at once
#[test]
fn test_args_method() {
    // Test with multiple arguments
    let cmd = Cmd::new("ls").args(vec!["-l", "-a", "-h"]);
    assert_eq!(
        cmd.args,
        vec![
            OsString::from("-l"),
            OsString::from("-a"),
            OsString::from("-h")
        ]
    );

    // Test with empty iterator
    let cmd = Cmd::new("echo").args(Vec::<&str>::new());
    assert!(cmd.args.is_empty());

    // Test combining single arg() and args() methods
    let cmd = Cmd::new("command")
        .arg("first")
        .args(vec!["second", "third"]);
    assert_eq!(
        cmd.args,
        vec![
            OsString::from("first"),
            OsString::from("second"),
            OsString::from("third")
        ]
    );
}

/// Tests that all builder methods work correctly in combination
#[test]
fn test_builder_pattern_completeness() {
    use std::env;
    let temp_dir = env::temp_dir();

    let cmd = Cmd::new("test_program")
        .arg("arg1")
        .args(vec!["arg2", "arg3"])
        .env("VAR1", "value1")
        .env("VAR2", "value2")
        .current_dir(&temp_dir)
        .no_echo();

    assert_eq!(cmd.program, OsString::from("test_program"));
    assert_eq!(
        cmd.args,
        vec![
            OsString::from("arg1"),
            OsString::from("arg2"),
            OsString::from("arg3")
        ]
    );
    assert_eq!(cmd.envs.len(), 2);
    assert_eq!(
        cmd.envs[0],
        (OsString::from("VAR1"), OsString::from("value1"))
    );
    assert_eq!(
        cmd.envs[1],
        (OsString::from("VAR2"), OsString::from("value2"))
    );
    assert_eq!(cmd.current_dir, Some(temp_dir));
    assert!(cmd.suppress_echo);
}
