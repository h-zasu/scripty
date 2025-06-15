//! # 00 - Basic: Getting Started with scripty
//!
//! This example demonstrates the basic usage of scripty:
//! - Creating commands with cmd! macro
//! - Running commands with different methods
//! - Capturing output
//! - Handling errors
//! - Setting environment variables and working directory
//!
//! Prerequisites: None
//! Next example: 01_simple_pipes.rs

use scripty::*;

fn main() -> Result<()> {
    println!("🚀 Basic scripty Usage");
    println!("=====================\n");

    // 1. Simple command execution
    println!("1. Simple command execution:");
    simple_execution()?;

    // 2. Capturing command output
    println!("\n2. Capturing command output:");
    capture_output()?;

    // 3. Working with command arguments
    println!("\n3. Working with command arguments:");
    command_arguments()?;

    // 4. Environment variables and working directory
    println!("\n4. Environment and directory:");
    environment_and_directory()?;

    // 5. Error handling
    println!("\n5. Error handling:");
    error_handling();

    println!("\n🎉 Basic examples completed!");
    println!("🚀 Next step: Run 'cargo run --example 01_simple_pipes' for pipeline operations");

    Ok(())
}

fn simple_execution() -> Result<()> {
    println!("   Running a simple echo command");

    // Basic command execution with run()
    cmd!("echo", "Hello from scripty!").run()?;

    // Using the builder pattern
    cmd!("echo")
        .arg("Builder pattern example")
        .run()?;

    Ok(())
}

fn capture_output() -> Result<()> {
    println!("   Capturing command output as string");

    // Capture output as String
    let output = cmd!("echo", "Captured output").output()?;
    println!("   Captured: {:?}", output.trim());

    // Capture output from a more complex command
    let date = cmd!("date", "+%Y-%m-%d").output()?;
    println!("   Current date: {}", date.trim());

    // Check if output contains specific text
    let contents = cmd!("echo", "scripty is awesome").output()?;
    if contents.contains("awesome") {
        println!("   ✓ Found 'awesome' in output!");
    }

    Ok(())
}

fn command_arguments() -> Result<()> {
    println!("   Building commands with multiple arguments");

    // Multiple arguments at once
    cmd!("echo", "arg1", "arg2", "arg3").run()?;

    // Adding arguments dynamically
    let mut command = cmd!("echo");
    for i in 1..=3 {
        command = command.arg(format!("item{}", i));
    }
    command.run()?;

    // Using args() for multiple arguments
    cmd!("echo")
        .args(["multiple", "args", "at", "once"])
        .run()?;

    Ok(())
}

fn environment_and_directory() -> Result<()> {
    println!("   Setting environment variables and working directory");

    // Set environment variable
    let output = cmd!("sh", "-c", "echo $MY_VAR")
        .env("MY_VAR", "Hello from env!")
        .output()?;
    println!("   Environment variable: {}", output.trim());

    // Set multiple environment variables
    cmd!("sh", "-c", "echo Name: $NAME, Version: $VERSION")
        .env("NAME", "scripty")
        .env("VERSION", "0.3.3")
        .run()?;

    // Change working directory
    let pwd = cmd!("pwd")
        .current_dir("/tmp")
        .output()?;
    println!("   Working directory: {}", pwd.trim());

    Ok(())
}

fn error_handling() {
    println!("   Demonstrating error handling");

    // Handle command not found
    match cmd!("nonexistent-command").run() {
        Ok(_) => println!("   Unexpected success!"),
        Err(e) => println!("   ✓ Expected error: {}", e),
    }

    // Handle command failure (exit code != 0)
    match cmd!("false").run() {
        Ok(_) => println!("   Unexpected success!"),
        Err(e) => println!("   ✓ Expected failure: {}", e),
    }

    // Using output() doesn't fail on non-zero exit
    match cmd!("sh", "-c", "echo 'Error!' >&2; exit 1").output() {
        Ok(output) => println!("   ✓ Captured output despite exit 1: {:?}", output.trim()),
        Err(e) => println!("   Error: {}", e),
    }
}
