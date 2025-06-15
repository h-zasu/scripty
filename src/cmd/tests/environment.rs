//! Environment and directory tests.
//!
//! Tests for setting environment variables and working directories
//! for command execution, including error handling for invalid paths.

use crate::cmd;
use std::env;

/// Tests setting environment variables for command execution
#[test]
fn test_environment_variable() {
    let output = cmd!("printenv", "TEST_VAR")
        .env("TEST_VAR", "test_value")
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "test_value");
}

/// Tests setting multiple environment variables
#[test]
fn test_multiple_environment_variables() {
    let output = cmd!("sh", "-c", "echo $VAR1 $VAR2 $VAR3")
        .env("VAR1", "value1")
        .env("VAR2", "value2")
        .env("VAR3", "value3")
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "value1 value2 value3");
}

/// Tests environment variable inheritance and overriding
#[test]
fn test_environment_inheritance() {
    // Set a system environment variable
    // SAFETY: We're setting environment variables in a test environment
    // This is safe because tests run in isolation and we clean up afterwards
    // Note: std::env::set_var became unsafe in Rust 2024 Edition (stable in Rust 1.85.0, Feb 2025)
    unsafe {
        env::set_var("SCRIPTY_TEST_VAR", "original_value");
    }

    // Test that system env vars are inherited
    let output = cmd!("printenv", "SCRIPTY_TEST_VAR")
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "original_value");

    // Test overriding system env var
    let output = cmd!("printenv", "SCRIPTY_TEST_VAR")
        .env("SCRIPTY_TEST_VAR", "overridden_value")
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "overridden_value");

    // Clean up
    // SAFETY: Cleaning up the test environment variable we set above
    // Note: std::env::remove_var became unsafe in Rust 2024 Edition (stable in Rust 1.85.0, Feb 2025)
    unsafe {
        env::remove_var("SCRIPTY_TEST_VAR");
    }
}

/// Tests working directory functionality
#[test]
fn test_working_directory() {
    let temp_dir = env::temp_dir();

    let output = cmd!("pwd")
        .current_dir(&temp_dir)
        .no_echo()
        .output()
        .unwrap();
    let pwd_output = output.trim();

    // Convert both to canonical paths for comparison
    let expected_canonical = temp_dir.canonicalize().unwrap_or_else(|_| temp_dir.clone());
    let actual_path = std::path::PathBuf::from(pwd_output);
    let actual_canonical = actual_path.canonicalize().unwrap_or(actual_path);

    assert_eq!(
        actual_canonical,
        expected_canonical,
        "Working directory should be '{}', but pwd returned '{}'",
        expected_canonical.display(),
        actual_canonical.display()
    );
}

/// Tests environment variables with special characters
#[test]
fn test_environment_special_characters() {
    let special_values = [
        "value with spaces",
        "value\nwith\nnewlines",
        "value\twith\ttabs",
        "value\"with\"quotes",
        "value'with'quotes",
        "value$with$dollars",
        "value=with=equals",
        "value;with;semicolons",
    ];

    for (i, value) in special_values.iter().enumerate() {
        let var_name = format!("SPECIAL_VAR_{}", i);

        let output = cmd!("printenv", &var_name)
            .env(&var_name, value)
            .no_echo()
            .output()
            .unwrap();
        assert_eq!(output.trim(), *value);
    }
}

/// Tests empty and null environment variables
#[test]
fn test_empty_environment_variables() {
    // Test empty value
    let output = cmd!("printenv", "EMPTY_VAR")
        .env("EMPTY_VAR", "")
        .no_echo()
        .output()
        .unwrap();
    assert_eq!(output.trim(), "");

    // Test unsetting a variable (should fail to find it)
    let result = cmd!("printenv", "UNSET_VAR").no_echo().output();
    // printenv should fail for unset variables
    assert!(result.is_err());
}
