# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this
repository.

## 🔴 CRITICAL: Implementation Principle

**Keep implementations minimal and focused on the specific request.**

- Avoid excessive implementation beyond what is necessary
- Limit changes to what is sufficient to solve the problem
- Do not add features or improvements not explicitly requested

## 🔴 CRITICAL: Branch-Based Development

**NEVER push directly to main branch.**

- Always create feature branches for changes
- Use Pull Requests for all merges to main
- See CONTRIBUTING.md for detailed workflow

## Essential Development Commands

```bash
# Run tests (various options)
cargo test                    # All tests
cargo test --test <name>      # Specific integration test
cargo test <module>::         # Specific module tests
cargo test -- --nocapture     # Show println! output

# Lint and format
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt

# MANDATORY before every commit
cargo xtask precommit         # Runs tests + clippy + fmt

# Other useful commands
cargo xtask ci                # Full CI validation
cargo readme                  # Regenerate README.md from src/lib.rs
```

## Architecture Overview

**Scripty** is a Rust library that provides shell command execution with a scripting-like interface.
Key architectural decisions:

### Core Module Structure

- **`cmd/`** - Command execution engine
  - `command.rs` - Main `Cmd` struct with builder pattern
  - `pipeline.rs` - Command chaining and piping logic
  - `types.rs` - Shared type definitions
  - `error.rs` - Error types and handling
  - `macros.rs` - The `cmd!` macro implementation

### Key Design Patterns

1. **Builder Pattern**: Commands are built fluently before execution
2. **Streaming I/O**: Uses native `std::io::pipe` for efficient data transfer
3. **Extension Traits**: `ReadExt` allows any reader to pipe into commands
4. **Command Echo**: Automatic visual feedback (suppressible via `NO_ECHO`)

### Critical Implementation Details

- **Platform**: Unix-only (uses fork/exec model)
- **Rust Version**: Requires 1.87.0+ for native pipe support
- **Memory Efficiency**: Streams data rather than buffering entire outputs
- **Error Handling**: Commands that fail exit status return errors by default

## Testing Principles

- **Test Coverage**: When fixing bugs, add tests that would have caught the issue
- **Test Documentation**: Document unusual test approaches within the test file itself
- **Regression Prevention**: For bugs related to output behavior, consider tests that verify actual
  stdout/stderr output, not just captured strings
- **Test Organization**: Tests are grouped by functionality in `src/cmd/tests/`

## Project Information

### Overview

See `Cargo.toml` for official project metadata (name, description, version, etc.)

### Technology Stack

- **Language**: Rust 2024 Edition
- **Target Platform**: Unix-like systems only
- **Dependencies**: Minimal (see Cargo.toml)
- **Testing**: Unit tests + integration tests + doc tests

### Project Structure and Documentation

- See `CONTRIBUTING.md` for detailed project structure and development guidelines
- **AI Responsibility**: When making changes that affect project structure, file organization, or
  development procedures, proactively update `CONTRIBUTING.md` to reflect these changes
- **Documentation Maintenance**: Always verify that documentation matches current project state
  before completing tasks

### Documentation Update Triggers

AI agents should update `CONTRIBUTING.md` when:

- Adding, removing, or moving **source modules or directories** (not individual test files)
- Changing development workflows or build processes
- Modifying testing strategies or CI/CD procedures
- Adding new development tools or dependencies
- Restructuring the codebase organization
- Adding new **categories** of files (e.g., a new test directory, new module type)

### Documentation Best Practices

- **Implementation Details**: Document special implementation approaches, unusual patterns, or
  complex logic within the source files themselves using comments, not in CONTRIBUTING.md
- **Test Documentation**: Special test approaches (e.g., subprocess testing, integration test
  patterns) should be documented in the test files with detailed comments explaining the rationale
- **CONTRIBUTING.md Scope**: Keep CONTRIBUTING.md focused on project structure overview, development
  workflows, and contribution guidelines. Avoid implementation-specific details
