//! # 01 - Simple Pipes: Basic Pipeline Operations
//!
//! This example demonstrates scripty's core strength: pipeline operations.
//! Learn how to chain commands together for powerful data processing:
//! - Two-command pipelines
//! - Multi-stage command chains
//! - Input data processing through pipelines
//! - Performance considerations
//!
//! Estimated time: ~5 minutes
//! Prerequisites: Basic familiarity with Unix command-line tools
//! Next example: 02_pipe_modes.rs

use scripty::*;

fn main() -> Result<()> {
    println!("🔗 Pipeline Fundamentals - scripty's Core Strength");
    println!("=================================================\n");

    // 1. Basic two-command pipes
    println!("1. Basic pipelines:");
    basic_pipes()?;

    // 2. Multiple command chains
    println!("\n2. Multi-stage command chains:");
    multiple_pipes()?;

    // 3. Input data processing
    println!("\n3. Data processing pipelines:");
    input_processing()?;

    // 4. Performance and memory efficiency
    println!("\n4. Performance advantages:");
    performance_demo()?;

    println!("\n🎉 Pipeline fundamentals completed!");
    println!("🚀 Next step:");
    println!("   • Run 'cargo run --example 02_pipe_modes' for stderr/stdout control");

    Ok(())
}

fn performance_demo() -> Result<()> {
    println!("   Understanding scripty's performance advantages\n");

    // Memory efficiency demonstration
    println!("🚀 Memory efficiency:");
    println!("   scripty uses native pipes - data streams between commands");
    println!("   without loading everything into memory at once.\n");

    // Generate sample data to demonstrate streaming
    let sample_data = (1..=100)
        .map(|i| format!("line {}", i))
        .collect::<Vec<_>>()
        .join("\n");

    println!("📈 Streaming pipeline example:");
    println!("   Processing 100 lines through multiple commands");
    println!("   Command: generate_data | head -5 | wc -l");

    let result = cmd!("head", "-5")
        .pipe(cmd!("wc", "-l"))
        .input(&sample_data)
        .output()?;

    println!("   Result: {} lines processed", result.trim());
    println!("   💡 The entire dataset never loads into memory simultaneously!");

    // Demonstrate pipeline vs individual commands
    println!("\n⚡ Pipeline vs individual commands:");
    println!("   Pipeline:    echo 'test data' | wc -c | tr -d ' '");
    let pipeline_result = cmd!("echo", "test data")
        .pipe(cmd!("wc", "-c"))
        .pipe(cmd!("tr", "-d", " "))
        .output()?;

    println!("   Individual:  Requires intermediate storage");
    let step1 = cmd!("echo", "test data").output()?;
    let step2 = cmd!("wc", "-c").input(&step1).output()?;
    let individual_result = cmd!("tr", "-d", " ").input(&step2).output()?;

    println!(
        "     Both approaches result: {} characters",
        pipeline_result.trim()
    );
    println!("     (Individual result: {})", individual_result.trim());
    println!("   But pipelines are more memory efficient! ⚡");

    Ok(())
}

fn basic_pipes() -> Result<()> {
    println!("   Two commands connected via pipe()\n");

    // Convert text to uppercase
    println!("📝 Text transformation:");
    println!("   Command: echo 'hello world' | tr '[:lower:]' '[:upper:]'");
    cmd!("echo", "hello world")
        .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
        .run()?;

    // Reverse string
    println!("\n🔄 String reversal:");
    println!("   Command: echo 'scripty rocks' | rev");
    cmd!("echo", "scripty rocks").pipe(cmd!("rev")).run()?;

    // Count words and capture output
    println!("\n🔢 Word counting with output capture:");
    println!("   Command: echo 'Hello beautiful scripty world' | wc -w");
    let word_count = cmd!("echo", "Hello beautiful scripty world")
        .pipe(cmd!("wc", "-w"))
        .output()?;
    println!("   Result: {} words", word_count.trim());

    // Character counting
    println!("\n📊 Character analysis:");
    let char_analysis = cmd!("echo", "scripty").pipe(cmd!("wc", "-c")).output()?;
    println!(
        "   'scripty' has {} characters (including newline)",
        char_analysis.trim()
    );

    Ok(())
}

fn multiple_pipes() -> Result<()> {
    println!("   Chaining multiple commands for complex data processing\n");

    // Three-stage transformation
    println!("🔗 Three-stage text transformation:");
    println!("   Command: echo 'Hello World' | tr '[:upper:]' '[:lower:]' | rev");
    println!("   Process: Uppercase → Lowercase → Reverse");
    cmd!("echo", "Hello World")
        .pipe(cmd!("tr", "[:upper:]", "[:lower:]")) // to lowercase
        .pipe(cmd!("rev")) // reverse
        .run()?;

    // Data processing pipeline
    println!("\n🔄 Multi-stage data processing:");
    println!("   Command: echo 'zebra,apple,cherry,banana' | tr ',' '\\n' | sort | tr '\\n' ','");
    println!("   Process: CSV → Lines → Sort → CSV");
    let result = cmd!("echo", "zebra,apple,cherry,banana")
        .pipe(cmd!("tr", ",", "\n")) // comma to newline
        .pipe(cmd!("sort")) // sort lines
        .pipe(cmd!("tr", "\n", ",")) // newline to comma
        .output()?;
    println!("   Sorted result: {}", result.trim());

    // Text analysis pipeline
    println!("\n📊 Text analysis pipeline:");
    println!("   Analyzing word frequency in a sentence");
    let analysis = cmd!(
        "echo",
        "the quick brown fox jumps over the lazy dog the end"
    )
    .pipe(cmd!("tr", " ", "\n")) // words to lines
    .pipe(cmd!("sort")) // sort words
    .pipe(cmd!("uniq", "-c")) // count occurrences
    .pipe(cmd!("sort", "-nr")) // sort by frequency
    .output()?;
    println!("   Word frequency (most common first):");
    for line in analysis.lines() {
        if !line.trim().is_empty() {
            println!("     {}", line.trim());
        }
    }

    Ok(())
}

fn input_processing() -> Result<()> {
    println!("   Processing custom input data through pipelines\n");

    // Sample data processing
    let fruit_data = "orange\napple\nbanana\napple\ncherry\nbanana\ndate\napple";
    println!("📄 Sample dataset:");
    println!("   {}", fruit_data.replace('\n', ", "));

    // Remove duplicates and sort
    println!("\n🔍 Deduplication and sorting:");
    println!("   Command: sort | uniq");
    let unique_sorted = cmd!("sort").pipe(cmd!("uniq")).input(fruit_data).output()?;
    println!("   Unique items (sorted):");
    for line in unique_sorted.lines() {
        if !line.trim().is_empty() {
            println!("     • {}", line.trim());
        }
    }

    // Count total and unique items
    let total_count = cmd!("wc", "-l").input(fruit_data).output()?;
    let unique_count = cmd!("sort")
        .pipe(cmd!("uniq"))
        .pipe(cmd!("wc", "-l"))
        .input(fruit_data)
        .output()?;

    println!("\n📊 Statistics:");
    println!("   Total items: {}", total_count.trim());
    println!("   Unique items: {}", unique_count.trim());

    // Find most common item
    println!("\n🏆 Frequency analysis:");
    println!("   Command: sort | uniq -c | sort -nr | head -1");
    let most_common = cmd!("sort")
        .pipe(cmd!("uniq", "-c"))
        .pipe(cmd!("sort", "-nr"))
        .pipe(cmd!("head", "-1"))
        .input(fruit_data)
        .output()?;
    println!("   Most common: {}", most_common.trim());

    Ok(())
}
