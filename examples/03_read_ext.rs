//! # 03 - ReadExt: Fluent Reader-to-Command Piping
//!
//! This example showcases scripty's ReadExt trait that extends std::io::Read
//! with fluent piping capabilities. Learn how to pipe data directly from
//! readers to commands using intuitive method chaining:
//! - File-to-command piping
//! - Memory-efficient streaming
//! - Chaining with existing pipeline methods
//! - Working with different reader types
//!
//! Estimated time: ~5 minutes
//! Prerequisites: Basic familiarity with Rust I/O and pipeline examples
//! Previous examples: 01_simple_pipes.rs, 02_pipe_modes.rs

use scripty::*;
use std::fs::File;
use std::io::{BufReader, Cursor};

fn main() -> Result<()> {
    println!("🔗 ReadExt: Fluent Reader-to-Command Piping");
    println!("=============================================\n");

    // Setup: Create sample data files for demonstration
    setup_sample_files()?;

    // 1. Basic file-to-command piping
    println!("1. File-to-command piping:");
    file_piping_examples()?;

    // 2. Memory-efficient streaming
    println!("\n2. Memory-efficient streaming:");
    streaming_examples()?;

    // 3. Different reader types
    println!("\n3. Working with different reader types:");
    reader_types_examples()?;

    // 4. Chaining with pipeline methods
    println!("\n4. Advanced pipeline chaining:");
    advanced_chaining_examples()?;

    // Cleanup
    cleanup_sample_files()?;

    println!("\n🎉 ReadExt examples completed!");
    println!("💡 Key benefits:");
    println!("   • Intuitive method chaining: reader.pipe(cmd)");
    println!("   • Memory efficient streaming");
    println!("   • Seamless integration with existing pipelines");

    Ok(())
}

fn setup_sample_files() -> Result<()> {
    // Create sample log file
    let log_data = r#"2024-01-15 10:30:45 INFO Application started
2024-01-15 10:30:46 DEBUG Loading configuration
2024-01-15 10:30:47 INFO Configuration loaded successfully
2024-01-15 10:31:15 WARN Connection timeout, retrying...
2024-01-15 10:31:16 ERROR Failed to connect to database
2024-01-15 10:31:17 INFO Retrying database connection
2024-01-15 10:31:18 INFO Database connection established
2024-01-15 10:31:20 DEBUG Processing user request
2024-01-15 10:31:21 INFO Request processed successfully
2024-01-15 10:31:22 ERROR Invalid user credentials"#;

    fs::write("sample_log.txt", log_data)?;

    // Create sample numbers file
    let numbers_data = "42\n17\n8\n23\n91\n6\n34\n15\n77\n2";
    fs::write("numbers.txt", numbers_data)?;

    // Create sample CSV data
    let csv_data = "name,age,city\nAlice,25,Tokyo\nBob,30,London\nCharlie,35,Paris\nDiana,28,Berlin\nEve,22,Madrid";
    fs::write("data.csv", csv_data)?;

    Ok(())
}

fn cleanup_sample_files() -> Result<()> {
    let _ = fs::remove_file("sample_log.txt");
    let _ = fs::remove_file("numbers.txt");
    let _ = fs::remove_file("data.csv");
    Ok(())
}

fn file_piping_examples() -> Result<()> {
    println!("   Direct file-to-command piping with ReadExt\n");

    // Basic file processing
    println!("📄 Basic file processing:");
    println!("   Command: file.pipe(wc -l)");
    let file = File::open("sample_log.txt")?;
    let line_count = file.pipe(cmd!("wc", "-l")).output()?;
    println!("   Log file has {} lines", line_count.trim());

    // Error filtering
    println!("\n🔍 Error log filtering:");
    println!("   Command: file.pipe(grep ERROR)");
    let file = File::open("sample_log.txt")?;
    let errors = file.pipe(cmd!("grep", "ERROR")).output()?;
    println!("   ERROR entries found:");
    for line in errors.lines() {
        if !line.trim().is_empty() {
            println!("     {}", line.trim());
        }
    }

    // Numeric processing
    println!("\n🧮 Numeric processing:");
    println!("   Command: file.pipe(sort -n)");
    let file = File::open("numbers.txt")?;
    let sorted_numbers = file.pipe(cmd!("sort", "-n")).output()?;
    println!("   Sorted numbers: {}", sorted_numbers.replace('\n', ", "));

    Ok(())
}

fn streaming_examples() -> Result<()> {
    println!("   Memory-efficient streaming with large data processing\n");

    // Buffered reading for efficiency
    println!("📊 Buffered file processing:");
    println!("   Using BufReader for optimal performance");
    let file = File::open("sample_log.txt")?;
    let buf_reader = BufReader::new(file);
    let word_count = buf_reader.pipe(cmd!("wc", "-w")).output()?;
    println!("   Total words in log: {}", word_count.trim());

    // Statistics calculation
    println!("\n📈 Statistical analysis:");
    println!("   Command: file.pipe(awk '{{sum += $1}} END {{print sum, NR, sum/NR}}')");
    let file = File::open("numbers.txt")?;
    let buf_reader = BufReader::new(file);
    let stats = buf_reader
        .pipe(cmd!("awk", "{sum += $1; count++} END {printf \"Sum: %d, Count: %d, Average: %.2f\\n\", sum, count, sum/count}"))
        .output()?;
    println!("   Statistics: {}", stats.trim());

    // Large data simulation with streaming
    println!("\n⚡ Streaming efficiency demonstration:");
    println!("   Processing data without loading everything into memory");
    let large_data = (1..=1000)
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    let large_data_bytes = large_data.into_bytes();
    let cursor = Cursor::new(large_data_bytes);
    let result = cursor
        .pipe(cmd!("tail", "-5"))
        .pipe(cmd!("head", "-3"))
        .output()?;
    println!(
        "   Last 5, then first 3 numbers: {}",
        result.replace('\n', ", ")
    );

    Ok(())
}

fn reader_types_examples() -> Result<()> {
    println!("   Working with different types of readers\n");

    // Cursor with byte data
    println!("🗂️ In-memory data processing:");
    let data = b"zebra\napple\nbanana\ncherry\ndate";
    let cursor = Cursor::new(data);
    let sorted = cursor.pipe(cmd!("sort")).output()?;
    println!("   Sorted fruits: {}", sorted.replace('\n', ", "));

    // String data processing
    println!("\n📝 Text data processing:");
    let text_data = "Hello World\nRust is awesome\nPiping is powerful";
    let cursor = Cursor::new(text_data.as_bytes());
    let word_count = cursor.pipe(cmd!("wc", "-w")).output()?;
    println!("   Total words: {}", word_count.trim());

    // CSV processing
    println!("\n📊 CSV data analysis:");
    println!("   Processing CSV file with header extraction");
    let file = File::open("data.csv")?;
    let header = file.pipe(cmd!("head", "-1")).output()?;
    println!("   CSV header: {}", header.trim());

    let file = File::open("data.csv")?;
    let record_count = file
        .pipe(cmd!("tail", "-n", "+2")) // Skip header
        .pipe(cmd!("wc", "-l"))
        .output()?;
    println!("   Data records: {}", record_count.trim());

    Ok(())
}

fn advanced_chaining_examples() -> Result<()> {
    println!("   Advanced pipeline chaining with ReadExt\n");

    // Complex log analysis
    println!("🔍 Complex log analysis pipeline:");
    println!("   Command: file.pipe(grep -v DEBUG).pipe(cut -d' ' -f4).pipe(sort).pipe(uniq -c)");
    let file = File::open("sample_log.txt")?;
    let log_levels = file
        .pipe(cmd!("grep", "-v", "DEBUG")) // Exclude DEBUG entries
        .pipe(cmd!("cut", "-d", " ", "-f", "4")) // Extract log level column
        .pipe(cmd!("sort"))
        .pipe(cmd!("uniq", "-c"))
        .output()?;

    println!("   Log level frequency (excluding DEBUG):");
    for line in log_levels.lines() {
        if !line.trim().is_empty() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                println!("     {}: {} occurrences", parts[1], parts[0]);
            }
        }
    }

    // Data transformation pipeline
    println!("\n🔄 Multi-stage data transformation:");
    println!("   Command: file.pipe(tail -n +2).pipe(cut -d, -f2).pipe(sort -n).pipe(awk '...')");
    let file = File::open("data.csv")?;
    let age_stats = file
        .pipe(cmd!("tail", "-n", "+2"))  // Skip CSV header
        .pipe(cmd!("cut", "-d", ",", "-f", "2"))  // Extract age column  
        .pipe(cmd!("sort", "-n"))  // Sort numerically
        .pipe(cmd!("awk", "{sum+=$1; count++} END {printf \"Min: %d, Max: %d, Avg: %.1f\\n\", $1, max, sum/count} {if(NR==1) min=$1; max=$1} {if($1>max) max=$1}"))
        .output()?;
    println!("   Age statistics: {}", age_stats.trim());

    // Real-time processing simulation
    println!("\n⏱️ Stream processing with immediate output:");
    println!("   Processing numbers and showing running total");
    let file = File::open("numbers.txt")?;
    file.pipe(cmd!("awk", "{sum += $1; print \"Running total:\", sum}"))
        .run()?;

    Ok(())
}
