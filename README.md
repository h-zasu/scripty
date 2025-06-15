# scripty

## scripty

[![Crates.io](https://img.shields.io/crates/v/scripty.svg)](https://crates.io/crates/scripty)
[![Documentation](https://docs.rs/scripty/badge.svg)](https://docs.rs/scripty)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.87.0%2B-blue.svg)](https://github.com/rust-lang/rust)

**Scripty** - A simple and intuitive library that makes running shell commands and file operations
easy and visible.

### Why scripty?

When you need to write system administration scripts, build tools, or automation in Rust, you often
find yourself wrestling with `std::process::Command` and `std::fs`. scripty provides a clean,
shell-script-like interface while keeping all the benefits of Rust's type safety and error handling.

#### Key Features

- **🎨 Colorful output**: See exactly what commands are being executed
- **🔗 Easy piping**: Chain commands together naturally with stdout, stderr, or both
- **📁 File operations**: Wrapper around `std::fs` with automatic logging
- **🔧 Builder pattern**: Fluent API for command construction
- **⚡ Minimal dependencies**: Only uses `anstyle` for colors
- **🛡️ Type safe**: All the safety of Rust with the convenience of shell scripts
- **🚰 Streaming I/O**: Efficient handling of large data with readers and writers
- **🔌 Reader Extensions**: Fluent piping from any `Read` implementation to commands
- **✍️ Write Methods**: Direct output streaming to writers with stdout/stderr control

### Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
scripty = "0.3.3"
```

### Platform Support

Currently supported platforms:

- **Linux** ✅ Full support with native pipe optimization
- **macOS** ✅ Full support with native pipe optimization

Scripty is designed for Unix-like systems and uses Unix shell commands and utilities.

### Requirements

- **Rust 1.87.0 or later** - Uses native `std::io::pipe` for optimal pipeline performance

### Security Considerations

**⚠️ Warning**: This library executes system commands with the privileges of the current process.

- **Never pass untrusted user input directly to commands**
- Validate and sanitize all inputs before use
- Be aware of command injection risks when constructing command arguments dynamically
- Consider using allowlists for command names and arguments when dealing with user input

Example of unsafe usage:

```rust
// DON'T DO THIS with untrusted input!
let user_input = get_user_input();
// cmd!("sh", "-c", user_input).run()?; // Dangerous!
```

### Basic Usage

#### Command Execution

```rust
use scripty::*;

// Simple command execution
cmd!("echo", "Hello, World!").run()?;

// Get command output
let output = cmd!("date").output()?;
println!("Current date: {}", output.trim());

// Command with multiple arguments
cmd!("ls", "-la", "/tmp").run()?;

// Using the builder pattern
cmd!("grep", "error")
    .arg("logfile.txt")
    .current_dir("/var/log")
    .env("LANG", "C")
    .run()?;
```

#### Reader-to-Command Piping

Pipe data directly from any `Read` implementation to commands using the `ReadExt` trait:

```rust
use scripty::*;
use std::fs::File;
use std::io::{BufReader, Cursor};

// Pipe file contents directly to commands
let file = File::open("data.txt")?;
let result = file.pipe(cmd!("grep", "pattern"))
    .pipe(cmd!("sort"))
    .pipe(cmd!("uniq"))
    .output()?;

// Memory-efficient processing with BufReader
let large_file = File::open("huge_dataset.txt")?;
let reader = BufReader::new(large_file);
reader.pipe(cmd!("awk", "{sum += $1} END {print sum}"))
    .run()?;

// In-memory data processing
let data = Cursor::new(b"zebra\napple\ncherry\n");
let sorted = data.pipe(cmd!("sort")).output()?;
```

#### Command Piping

Chain commands together just like in shell scripts using native `std::io::pipe` for enhanced
performance and memory efficiency!

```rust
use scripty::*;

// Simple pipe (stdout)
cmd!("echo", "hello world")
    .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
    .run()?;

// Pipe stderr
cmd!("some-command")
    .pipe_err(cmd!("grep", "ERROR"))
    .run()?;

// Pipe both stdout and stderr
cmd!("some-command")
    .pipe_out_err(cmd!("sort"))
    .run()?;

// Multiple pipes using efficient native pipes
cmd!("cat", "/etc/passwd")
    .pipe(cmd!("grep", "bash"))
    .pipe(cmd!("wc", "-l"))
    .run()?;

// Get piped output with efficient streaming
let result = cmd!("ps", "aux")
    .pipe(cmd!("grep", "rust"))
    .pipe(cmd!("wc", "-l"))
    .output()?;
println!("Rust processes: {}", result.trim());
```

##### Pipeline Performance Features

- **Memory efficient**: Uses streaming instead of buffering all data
- **Better performance**: Native pipes reduce process overhead
- **Platform independent**: No shell dependency for multi-command pipes
- **Native implementation**: Uses `std::io::pipe` for optimal performance

```rust
use scripty::*;

// Large data processing with efficient streaming
let large_data = "..."; // Megabytes of data
let result = cmd!("grep", "pattern")
    .pipe(cmd!("sort"))
    .pipe(cmd!("uniq", "-c"))
    .input(large_data)
    .output()?; // Processes without loading all data into memory
```

#### Core API Reference

##### The `cmd!` Macro

The heart of scripty is the `cmd!` macro for creating commands:

```rust
use scripty::*;

// Basic command
cmd!("ls").run()?;

// Command with arguments
cmd!("ls", "-la").run()?;

// Multiple arguments
cmd!("echo", "Hello", "World").run()?;
```

##### Command Builder Methods

Commands support a fluent builder pattern:

```rust
use scripty::*;

cmd!("grep", "error")
    .arg("logfile.txt")                    // Add single argument
    .args(["--color", "always"])           // Add multiple arguments
    .current_dir("/var/log")               // Set working directory
    .env("LANG", "C")                      // Set environment variable
    .no_echo()                             // Suppress command echoing
    .run()?;
```

##### Execution Methods

Different ways to execute commands:

```rust
use scripty::*;

// Execute and check exit status
cmd!("echo", "hello").run()?;

// Capture text output
let output = cmd!("date").output()?;
println!("Current date: {}", output.trim());

// Capture binary output
let bytes = cmd!("cat", "binary-file").output_bytes()?;
```

##### Output Streaming with Write Methods

Stream command output directly to writers with precise control over stdout/stderr:

```rust
use scripty::*;
use std::fs::File;

// Stream stdout to a file
let output_file = File::create("output.txt")?;
cmd!("ls", "-la").write_to(output_file)?;

// Stream stderr to an error log
let error_file = File::create("errors.log")?;
cmd!("risky-command").write_err_to(error_file)?;

// Stream both stdout and stderr to the same destination
let combined_file = File::create("full.log")?;
cmd!("verbose-app").write_both_to(combined_file)?;

// Use with any Writer (Vec, File, Cursor, etc.)
let mut buffer = Vec::new();
cmd!("echo", "test").write_to(&mut buffer)?;
```

##### Input Methods

Provide input to commands in various ways:

```rust
use scripty::*;
use std::io::Cursor;

// Text input
let result = cmd!("sort")
    .input("banana\napple\ncherry\n")
    .output()?;
println!("Sorted fruits: {}", result.trim());

// Binary input
let bytes = cmd!("cat")
    .input_bytes(b"binary data")
    .output_bytes()?;

// Stream from reader using ReadExt
use std::fs::File;
let file = File::open("data.txt")?;
file.pipe(cmd!("sort")).run()?;

// Buffered reading for large files
use std::io::BufReader;
let large_file = File::open("large.txt")?;
let reader = BufReader::new(large_file);
reader.pipe(cmd!("grep", "pattern")).run()?;
```

##### Advanced I/O Control with spawn_io_* Methods

For complex I/O scenarios, use the spawn methods for non-blocking control:

```rust
use scripty::*;
use std::io::{BufRead, BufReader, Write};
use std::thread;

// Full I/O control with spawn_io_all
let spawn = cmd!("sort").spawn_io_all()?;

// Handle input in separate thread
if let Some(mut stdin) = spawn.stdin {
    thread::spawn(move || {
        writeln!(stdin, "zebra").unwrap();
        writeln!(stdin, "apple").unwrap();
        writeln!(stdin, "banana").unwrap();
    });
}

// Read output
if let Some(stdout) = spawn.stdout {
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        println!("Line: {}", line?);
    }
}

spawn.handle.wait()?;
```

###### spawn_io_* Method Variants

Choose the right spawn method for your needs:

```rust
use scripty::*;
use std::io::Write;

// spawn_io_in - Control stdin only (common for sending data)
let (handle, stdin) = cmd!("grep", "pattern").spawn_io_in()?;
if let Some(mut stdin) = stdin {
    writeln!(stdin, "test line with pattern")?;
    writeln!(stdin, "another line")?;
    drop(stdin); // Close to signal EOF
}
handle.wait()?;

// spawn_io_out - Control stdout only (common for reading output)
let (handle, stdout) = cmd!("ls", "-la").spawn_io_out()?;
if let Some(stdout) = stdout {
    use std::io::Read;
    let mut output = String::new();
    stdout.take(100).read_to_string(&mut output)?;
    println!("First 100 bytes: {}", output);
}
handle.wait()?;

// spawn_io_in_out - Control both stdin and stdout (interactive commands)
let (handle, stdin, stdout) = cmd!("sort", "-n").spawn_io_in_out()?;
// Send numbers to sort
if let Some(mut stdin) = stdin {
    writeln!(stdin, "42")?;
    writeln!(stdin, "7")?;
    writeln!(stdin, "100")?;
    drop(stdin);
}
// Read sorted output
if let Some(stdout) = stdout {
    use std::io::BufRead;
    let reader = std::io::BufReader::new(stdout);
    for line in reader.lines() {
        println!("Sorted: {}", line?);
    }
}
handle.wait()?;

// spawn_io_err - Control stderr only (error monitoring)
let (handle, stderr) = cmd!("command-with-errors").spawn_io_err()?;
if let Some(mut stderr) = stderr {
    use std::io::Read;
    let mut errors = String::new();
    stderr.read_to_string(&mut errors)?;
    eprintln!("Errors: {}", errors);
}
handle.wait()?;
```

##### Simple Reader-to-Writer Operations

For straightforward input-to-output scenarios:

```rust
use scripty::*;
use std::fs::File;
use std::io::Cursor;

// Process file data through command (stdout)
let input_file = File::open("data.txt")?;
let output_file = File::create("sorted.txt")?;
cmd!("sort").run_with_io(input_file, output_file)?;

// Capture error output while processing
let source_code = Cursor::new("fn main() { invalid syntax }");
let mut error_log = Vec::new();
let _ = cmd!("rustc", "-").run_with_err_io(source_code, &mut error_log);
println!("Compilation errors: {}", String::from_utf8_lossy(&error_log));

// Capture both stdout and stderr for comprehensive logging
let input_data = Cursor::new("test data\nmore data");
let log_file = File::create("process.log")?;
cmd!("complex-tool").run_with_both_io(input_data, log_file)?;
```

##### File System Operations

All file operations are automatically logged:

```rust
use scripty::*;

// Basic file operations
fs::write("config.txt", "debug=true\nport=8080")?;
let content = fs::read_to_string("config.txt")?;
println!("Config: {}", content);

// Directory operations
fs::create_dir_all("project/src")?;
fs::copy("config.txt", "project/config.txt")?;

// Directory traversal
for entry in fs::read_dir("project")? {
    let entry = entry?;
    println!("Path: {}", entry.path().display());
}

// Cleanup
fs::remove_file("config.txt")?;
fs::remove_dir_all("project")?;
```

##### Error Handling

Use standard Rust error handling patterns:

```rust
use scripty::*;

// Handle command failures gracefully
match cmd!("nonexistent-command").run() {
    Ok(_) => println!("Command succeeded"),
    Err(e) => println!("Command failed: {}", e),
}

// Check command availability
if cmd!("which", "git").no_echo().run().is_ok() {
    println!("Git is available");
    cmd!("git", "--version").run()?;
}

// Use the ? operator for early returns
fn deploy_app() -> Result<()> {
    cmd!("cargo", "build", "--release").run()?;
    cmd!("docker", "build", "-t", "myapp", ".").run()?;
    cmd!("docker", "push", "myapp").run()?;
    println!("Deployment complete!");
    Ok(())
}
```

##### Global Configuration

Control scripty's behavior with environment variables:

- `NO_ECHO`: Set to any value to suppress command echoing globally

```bash
NO_ECHO=1 cargo run  # Run without command echoing
```

Or use the `.no_echo()` method on individual commands.

### Examples

This crate includes focused examples showcasing scripty's core strengths: **pipeline operations**
and **I/O handling**:

Examples are numbered for optimal learning progression:

1. **`01_simple_pipes.rs`** - Basic pipeline operations and command chaining
2. **`02_pipe_modes.rs`** - Complete pipeline control with stdout/stderr piping
3. **`03_read_ext.rs`** - Fluent reader-to-command piping with ReadExt trait
4. **`04_run_with_io.rs`** - Blocking reader-writer I/O with run_with_*() methods
5. **`05_spawn_io.rs`** - Non-blocking I/O control with spawn_io_*() methods

Run examples in order for the best learning experience:

```bash
cargo run --example 01_simple_pipes    # 1. Pipeline fundamentals
cargo run --example 02_pipe_modes      # 2. Advanced piping control
cargo run --example 03_read_ext        # 3. Reader-to-command piping
cargo run --example 04_run_with_io     # 4. Blocking reader-writer I/O
cargo run --example 05_spawn_io        # 5. Non-blocking I/O control
```

**Learning Path:** Start with `01_simple_pipes.rs` and progress through each numbered example in
sequence to build your expertise with scripty's pipeline and I/O capabilities.

#### Real-World Example: cargo-xtask + clap + scripty

This project's `xtask/` demonstrates scripty with cargo-xtask and clap:

```bash
cargo xtask ci              # Full CI pipeline
cargo xtask precommit       # Pre-commit checks (includes version sync)
```

See `xtask/src/main.rs` for the complete implementation combining all three tools.

#### Advanced Pipeline Performance & Best Practices

##### Performance Optimization

scripty's native pipeline implementation provides significant performance benefits:

```rust
use scripty::*;

// ✅ Efficient: Native pipes with streaming
let result = cmd!("cat", "large_file.txt")
    .pipe(cmd!("grep", "pattern"))
    .pipe(cmd!("sort"))
    .pipe(cmd!("uniq", "-c"))
    .output()?; // Processes without loading all data into memory

// ✅ Memory efficient: Stream large data directly
use std::fs::File;
let large_file = File::open("multi_gb_file.txt")?;
large_file.pipe(cmd!("grep", "ERROR"))
    .pipe(cmd!("wc", "-l"))
    .output()?; // Handles gigabytes efficiently
```

##### Pipeline Best Practices

**Memory Management:**

```rust
use scripty::*;

// ✅ Good: Stream processing for large data
cmd!("find", "/var/log", "-name", "*.log")
    .pipe(cmd!("xargs", "grep", "ERROR"))
    .pipe(cmd!("sort"))
    .output()?;

// ❌ Avoid: Loading large outputs into memory first
// let large_output = cmd!("find", "/", "-type", "f").output()?; // Don't do this
// Instead, use streaming with pipes directly
cmd!("find", "/", "-type", "f")
    .pipe(cmd!("grep", "pattern"))
    .output()?;
```

**Error-Prone Pipelines:**

```rust
use scripty::*;

// ✅ Good: Graceful error handling in pipelines
match cmd!("risky-command")
    .pipe(cmd!("sort"))
    .no_echo()
    .output()
{
    Ok(result) => println!("Success: {}", result.trim()),
    Err(_) => {
        // Fallback strategy
        println!("Using fallback approach");
        cmd!("safe-alternative").run()?;
    }
}
```

**Complex Data Processing:**

```rust
use scripty::*;

// ✅ Efficient multi-stage processing
let processed = cmd!("cat", "data.json")
    .pipe(cmd!("jq", ".items[]"))           // Extract items
    .pipe(cmd!("grep", "active"))           // Filter active
    .pipe(cmd!("jq", "-r", ".name"))        // Extract names
    .pipe(cmd!("sort"))                     // Sort results
    .output()?;
```

##### Troubleshooting Common Issues

**Large Data Processing:**

```rust
use scripty::*;
// Problem: Memory usage with large files
// Solution: Use streaming with BufReader and ReadExt
use std::io::BufReader;
use std::fs::File;

let large_file = File::open("huge_dataset.txt")?;
let reader = BufReader::new(large_file);

reader.pipe(cmd!("awk", "{sum += $1} END {print sum}"))
    .output()?; // Processes efficiently regardless of file size
```

**Pipeline Debugging:**

```rust
use scripty::*;

// Enable command echoing for debugging (set before running your program)
// export SCRIPTY_DEBUG=1

// Commands are echoed by default unless .no_echo() is used
cmd!("complex-command")
    .pipe(cmd!("grep", "pattern"))
    .pipe(cmd!("sort"))
    .run()?; // Commands will be shown as they execute
```

**Error Isolation:**

```rust
use scripty::*;

// Test each stage of a complex pipeline individually
let stage1 = cmd!("stage1-command").output()?;
println!("Stage 1 output: {}", stage1);

let stage2 = cmd!("stage2-command").input(&stage1).output()?;
println!("Stage 2 output: {}", stage2);

// Then combine when each stage works correctly
```

### Platform Support

- **Linux** ✅ Full support with native pipe optimization
- **macOS** ✅ Full support with native pipe optimization
- **Windows** ❌ Not supported (Unix-like systems only)

### Contributing

We welcome contributions! Please see our [GitHub repository](https://github.com/h-zasu/scripty) for
more information.

### License

This project is licensed under the MIT License.

License: MIT
