//! Tests for the `cmd` module.
//!
//! This module contains comprehensive tests for the command execution functionality,
//! organized into separate modules by category for better maintainability.

// Re-export items needed by test modules
use super::*;

// Test modules
mod basic;
mod environment;
mod error_handling;
mod io_patterns;

mod no_echo;
mod pipeline;
mod quoting;
mod run_output_verification;
mod write_methods;
