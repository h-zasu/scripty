//! # 02 - Pipe Modes: Pipeline Control
//!
//! This example demonstrates the three pipe modes available in scripty:
//! - Default stdout piping with pipe()
//! - Stderr-only piping with pipe_err()
//! - Combined stdout+stderr piping with pipe_out_err()
//!
//! Estimated time: ~3 minutes
//! Prerequisites: Complete 01_simple_pipes.rs
//! Next example: 03_io_patterns.rs

use scripty::*;

fn main() -> Result<()> {
    println!("🔀 Pipeline Control with Different Pipe Modes");
    println!("============================================\n");

    // Section 1: Basic pipe modes
    basic_pipe_modes()?;

    // Section 2: Mixed mode examples
    mixed_mode_examples()?;

    println!("\n🎉 Pipe modes tutorial completed!");
    println!("Key concepts learned:");
    println!("  • pipe() - Routes stdout to next command's stdin (default)");
    println!("  • pipe_err() - Routes stderr to next command's stdin");
    println!("  • pipe_out_err() - Routes both stdout+stderr to next command's stdin");
    println!("\n🚀 Next step:");
    println!("   • Run 'cargo run --example 03_io_patterns' for I/O operations");

    Ok(())
}

fn basic_pipe_modes() -> Result<()> {
    println!("📊 1. Basic Pipe Modes");
    println!("======================\n");

    // Example 1: Default stdout piping
    println!("🔄 Default stdout piping:");
    let output = cmd!("echo", "hello world")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .output()?;
    println!("   Input: 'hello world' → Output: '{}'", output.trim());
    println!();

    // Example 2: Stderr piping
    println!("⚠️ Stderr piping:");
    println!("   Generate error message and count its characters");
    let error_char_count = cmd!("sh", "-c", "echo 'Error: Connection failed!' >&2")
        .pipe_err(cmd!("wc", "-c"))
        .output()?;
    println!(
        "   Error message character count: {}",
        error_char_count.trim()
    );
    println!();

    // Example 3: Both stdout and stderr piping
    println!("🔀 Combined stdout+stderr piping:");
    println!("   Generate both outputs and sort them together");
    let combined_output = cmd!("sh", "-c", "echo 'stdout line'; echo 'stderr line' >&2")
        .pipe_out_err(cmd!("sort"))
        .output()?;
    println!("   Combined and sorted output:");
    for line in combined_output.lines() {
        if !line.trim().is_empty() {
            println!("     {}", line);
        }
    }
    println!();

    Ok(())
}

fn mixed_mode_examples() -> Result<()> {
    println!("🌊 2. Mixed Mode Examples");
    println!("=========================\n");

    // Example 1: stderr → stdout → stdout sequence
    println!("🔗 Error processing pipeline:");
    let char_count = cmd!("sh", "-c", "echo 'Error occurred' >&2")
        .pipe_err(cmd!("wc", "-c")) // stderr → stdout (count chars)
        .pipe(cmd!("tr", "-d", " ")) // stdout → stdout (remove spaces)
        .output()?;
    println!("   Error character count: {}", char_count.trim());
    println!();

    // Example 2: Mixed output with different processing
    println!("🎯 Mixed output processing:");
    let mixed_result = cmd!("sh", "-c", "echo 'success'; echo 'warning' >&2")
        .pipe_err(cmd!("sed", "s/^/WARN: /")) // stderr → stdout (prefix warnings)
        .pipe(cmd!("sed", "s/^/INFO: /")) // stdout → stdout (prefix info)
        .pipe_out_err(cmd!("sort")) // both → stdout (sort all)
        .output()?;
    println!("   Processing result:");
    for line in mixed_result.lines() {
        if !line.trim().is_empty() {
            println!("     {}", line);
        }
    }
    println!();

    // Example 3: Error counting
    println!("📊 Error counting example:");
    let error_count = cmd!(
        "sh",
        "-c",
        "echo 'line1'; echo 'err1' >&2; echo 'line2'; echo 'err2' >&2"
    )
    .pipe_err(cmd!("wc", "-l")) // stderr → stdout (count errors)
    .pipe(cmd!("sh", "-c", "read count; echo \"Found $count errors\""))
    .output()?;
    println!("   {}", error_count.trim());

    Ok(())
}
