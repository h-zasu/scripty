//! # 04 - run_with_io Methods: Blocking Reader-Writer I/O
//!
//! This example demonstrates scripty's run_with_io() family of methods for
//! connecting readers and writers in blocking I/O operations. These methods
//! provide direct streaming between input sources and output destinations:
//! - run_with_io() - process stdin to stdout
//! - run_with_err_io() - process stdin and capture stderr
//! - run_with_both_io() - process stdin and capture combined output
//!
//! Estimated time: ~7 minutes
//! Prerequisites: Understanding of Rust I/O traits (Read/Write)
//! Previous examples: 01_simple_pipes.rs, 02_pipe_modes.rs, 03_read_ext.rs

use scripty::*;
use std::fs::File;
use std::io::{BufReader, Cursor};

fn main() -> Result<()> {
    println!("🔄 run_with_io Methods: Blocking Reader-Writer I/O");
    println!("==================================================\n");

    // Setup sample data for demonstrations
    setup_sample_data()?;

    // 1. Basic run_with_io() - stdout processing
    println!("1. run_with_io() - Standard Output Processing:");
    basic_run_with_io_examples()?;

    // 2. run_with_err_io() - error stream capture
    println!("\n2. run_with_err_io() - Error Stream Capture:");
    error_capture_examples()?;

    // 3. run_with_both_io() - combined output handling
    println!("\n3. run_with_both_io() - Combined Output Handling:");
    combined_output_examples()?;

    // 4. Real-world use cases
    println!("\n4. Real-world Use Cases:");
    real_world_examples()?;

    // 5. Performance and streaming benefits
    println!("\n5. Performance and Streaming Benefits:");
    performance_examples()?;

    // Cleanup
    cleanup_sample_data()?;

    println!("\n🎉 run_with_io methods mastered!");
    println!("💡 Key advantages:");
    println!("   • Direct reader-to-writer streaming (no intermediate buffers)");
    println!("   • Separate handling of stdout/stderr streams");
    println!("   • Memory-efficient processing of large data");
    println!("   • Blocking operation with complete control");

    Ok(())
}

fn setup_sample_data() -> Result<()> {
    // Sample text data for sorting/processing
    let sample_text = "zebra\napple\nbanana\ncherry\ndate\nfig\ngrape\nkiwi\nlemon\nmango";
    fs::write("fruits.txt", sample_text)?;

    // Sample code with syntax errors for compiler testing
    let invalid_rust = r#"fn main() {
    println!("Hello world"  // Missing semicolon and closing parenthesis
    let x = 5
    println!("x = {}", x);
"#;
    fs::write("invalid.rs", invalid_rust)?;

    // Sample log data for processing
    let log_data = r#"2024-01-15 10:30:45 [INFO] Application started successfully
2024-01-15 10:30:46 [DEBUG] Loading configuration from config.toml
2024-01-15 10:30:47 [INFO] Configuration loaded: 42 settings
2024-01-15 10:31:15 [WARN] Connection timeout detected, retrying...
2024-01-15 10:31:16 [ERROR] Database connection failed: timeout exceeded
2024-01-15 10:31:17 [INFO] Fallback database activated
2024-01-15 10:31:18 [INFO] Service restored, processing requests
2024-01-15 10:31:20 [DEBUG] Processing user request #1001
2024-01-15 10:31:21 [INFO] Request #1001 completed successfully"#;
    fs::write("app.log", log_data)?;

    Ok(())
}

fn cleanup_sample_data() -> Result<()> {
    let _ = fs::remove_file("fruits.txt");
    let _ = fs::remove_file("invalid.rs");
    let _ = fs::remove_file("app.log");
    let _ = fs::remove_file("sorted_fruits.txt");
    let _ = fs::remove_file("filtered_logs.txt");
    let _ = fs::remove_file("error_report.txt");
    Ok(())
}

fn basic_run_with_io_examples() -> Result<()> {
    println!("   Processing data through commands with stdout capture\n");

    // File-to-file sorting
    println!("📁 File-to-file data processing:");
    println!("   Command: sort < fruits.txt > sorted_fruits.txt");
    let input_file = File::open("fruits.txt")?;
    let output_file = File::create("sorted_fruits.txt")?;

    cmd!("sort").run_with_io(input_file, output_file)?;

    let sorted_content = fs::read_to_string("sorted_fruits.txt")?;
    println!("   Sorted fruits: {}", sorted_content.replace('\n', ", "));

    // Memory-to-memory processing
    println!("\n🧠 Memory-to-memory processing:");
    println!("   Command: tr '[:lower:]' '[:upper:]'");
    let input_text = "hello world from scripty";
    let input_reader = Cursor::new(input_text.as_bytes());
    let mut output_buffer = Vec::new();

    cmd!("tr", "[:lower:]", "[:upper:]")
        .no_echo()
        .run_with_io(input_reader, &mut output_buffer)?;

    let result = String::from_utf8(output_buffer)?;
    println!("   Original: {}", input_text);
    println!("   Uppercase: {}", result.trim());

    // Streaming large data
    println!("\n⚡ Streaming large data processing:");
    println!("   Command: wc -l (counting lines in streaming fashion)");
    let large_data = (1..=1000)
        .map(|i| format!("Line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let large_data_bytes = large_data.into_bytes();
    let input_reader = Cursor::new(large_data_bytes);
    let mut line_count_buffer = Vec::new();

    cmd!("wc", "-l")
        .no_echo()
        .run_with_io(input_reader, &mut line_count_buffer)?;

    let line_count = String::from_utf8(line_count_buffer)?;
    println!("   Processed {} lines efficiently", line_count.trim());

    Ok(())
}

fn error_capture_examples() -> Result<()> {
    println!("   Capturing error streams while processing input\n");

    // Compiler error capture
    println!("🛠️ Compiler error capture:");
    println!("   Command: rustc - (compiling invalid Rust code)");
    let invalid_code = fs::read_to_string("invalid.rs")?;
    let invalid_code_bytes = invalid_code.into_bytes();
    let input_reader = Cursor::new(invalid_code_bytes);
    let mut error_buffer = Vec::new();

    let _ = cmd!("rustc", "--error-format=short", "-")
        .no_echo()
        .run_with_err_io(input_reader, &mut error_buffer);

    let error_output = String::from_utf8(error_buffer)?;
    println!(
        "   Compilation errors captured ({} bytes)",
        error_output.len()
    );

    // Show first few lines of errors
    let error_lines: Vec<&str> = error_output.lines().take(3).collect();
    for (i, line) in error_lines.iter().enumerate() {
        if !line.trim().is_empty() {
            println!("     {}. {}", i + 1, line.trim());
        }
    }

    // JSON validation with error capture
    println!("\n📋 JSON validation with error capture:");
    println!("   Command: jq . (validating malformed JSON)");
    let invalid_json = r#"{"name": "test", "value": invalid_syntax_here}"#;
    let input_reader = Cursor::new(invalid_json.as_bytes());
    let mut json_error_buffer = Vec::new();

    let _ = cmd!("jq", ".")
        .no_echo()
        .run_with_err_io(input_reader, &mut json_error_buffer);

    let json_error = String::from_utf8(json_error_buffer)?;
    if !json_error.is_empty() {
        println!(
            "   JSON validation error captured: {} bytes",
            json_error.len()
        );
        println!(
            "   Error preview: {}",
            json_error.lines().next().unwrap_or("").trim()
        );
    }

    // Shell command with error output
    println!("\n🐚 Shell command error monitoring:");
    println!("   Command: sh -c 'echo data; nonexistent_command'");
    let input_data = "processing this data\n";
    let input_reader = Cursor::new(input_data.as_bytes());
    let mut shell_error_buffer = Vec::new();

    let _ = cmd!("sh", "-c", "cat; nonexistent_command_xyz")
        .no_echo()
        .run_with_err_io(input_reader, &mut shell_error_buffer);

    let shell_error = String::from_utf8(shell_error_buffer)?;
    if !shell_error.is_empty() {
        println!("   Shell error captured: {}", shell_error.trim());
    }

    Ok(())
}

fn combined_output_examples() -> Result<()> {
    println!("   Capturing both stdout and stderr in combined stream\n");

    // Combined logging capture
    println!("📊 Combined logging capture:");
    println!("   Command: sh -c 'echo INFO: processing; echo ERROR: failed >&2'");
    let input_data = "user_data_to_process\nmore_data_here";
    let input_data_bytes = input_data.as_bytes().to_vec();
    let input_reader = Cursor::new(input_data_bytes);
    let combined_buffer = Vec::new();
    let cursor = Cursor::new(combined_buffer);

    cmd!(
        "sh",
        "-c",
        "cat; echo 'INFO: Processing completed'; echo 'WARN: Some warnings detected' >&2"
    )
    .no_echo()
    .run_with_both_io(input_reader, cursor)?;

    println!("   Combined output captured successfully");
    println!("   (Output written to cursor - check buffer for actual content)");

    // Build process monitoring
    println!("\n🔨 Build process monitoring:");
    println!("   Command: sh -c 'echo Building...; echo Warning: deprecated API >&2'");
    let build_script = "#!/bin/bash\necho 'Build starting'\necho 'All tests passed'";
    let input_reader = Cursor::new(build_script.as_bytes());
    let build_output_file = File::create("build_log.txt")?;

    cmd!(
        "sh",
        "-c",
        "cat; echo 'Build completed successfully'; echo 'WARNING: 2 deprecation warnings' >&2"
    )
    .no_echo()
    .run_with_both_io(input_reader, build_output_file)?;

    let build_log = fs::read_to_string("build_log.txt")?;
    println!("   Build log saved ({} bytes)", build_log.len());
    fs::remove_file("build_log.txt").ok();

    // Data processing with progress/error reporting
    println!("\n📈 Data processing with progress reporting:");
    println!("   Command: awk script that reports progress and errors");
    let csv_data = "id,value\n1,100\n2,200\n3,invalid\n4,400\n5,500";
    let csv_data_bytes = csv_data.as_bytes().to_vec();
    let input_reader = Cursor::new(csv_data_bytes);
    let processing_log = Vec::new();
    let cursor = Cursor::new(processing_log);

    cmd!(
        "awk",
        r#"
        BEGIN { FS="," }
        NR==1 { print "Processing CSV data..."; next }
        {
            if ($2 ~ /^[0-9]+$/) {
                print "Processed record " NR-1 ": " $2
            } else {
                print "ERROR: Invalid data in record " NR-1 ": " $2 > "/dev/stderr"
            }
        }
        END { print "Processing complete." }
    "#
    )
    .no_echo()
    .run_with_both_io(input_reader, cursor)?;

    println!("   Processing completed successfully");
    println!("   (Combined stdout+stderr written to cursor)");

    Ok(())
}

fn real_world_examples() -> Result<()> {
    println!("   Practical applications in real-world scenarios\n");

    // Log file analysis and filtering
    println!("📋 Log file analysis and filtering:");
    println!("   Command: grep -v DEBUG | cut -d' ' -f4- | tee filtered_logs.txt");
    let log_file = File::open("app.log")?;
    let filtered_file = File::create("filtered_logs.txt")?;

    cmd!("sh", "-c", "grep -v DEBUG | tee filtered_logs.txt")
        .run_with_io(log_file, filtered_file)?;

    let filtered_content = fs::read_to_string("filtered_logs.txt")?;
    let filtered_lines = filtered_content.lines().count();
    println!(
        "   Filtered {} lines (excluding DEBUG entries)",
        filtered_lines
    );

    // Data transformation pipeline with error reporting
    println!("\n🔄 Data transformation with validation:");
    println!("   Command: Data validation and transformation");
    let sample_data = "apple,5\nbanana,10\ninvalid_entry\ncherry,15\ndate,abc\nfig,20";
    let input_reader = Cursor::new(sample_data.as_bytes());
    let error_report_file = File::create("error_report.txt")?;

    cmd!(
        "awk",
        r#"
        BEGIN { FS="," }
        {
            if (NF != 2) {
                print "ERROR: Invalid format in line " NR ": " $0 > "/dev/stderr"
                next
            }
            if ($2 !~ /^[0-9]+$/) {
                print "ERROR: Non-numeric value in line " NR ": " $2 > "/dev/stderr"
                next
            }
            total += $2
            count++
            print "Valid entry: " $1 " = " $2
        }
        END {
            if (count > 0) {
                print "Summary: " count " valid entries, total = " total
            }
        }
    "#
    )
    .no_echo()
    .run_with_err_io(input_reader, error_report_file)?;

    let error_report = fs::read_to_string("error_report.txt")?;
    if !error_report.is_empty() {
        println!("   Validation errors detected:");
        for line in error_report.lines() {
            println!("     {}", line);
        }
    }

    // File format conversion
    println!("\n🔄 File format conversion:");
    println!("   Command: Converting CSV to JSON format");
    let csv_input = "name,age,city\nAlice,25,Tokyo\nBob,30,London\nCharlie,35,Paris";
    let input_reader = Cursor::new(csv_input.as_bytes());
    let mut json_output = Vec::new();

    cmd!(
        "awk",
        r#"
        BEGIN { FS=","; print "[" }
        NR==1 { 
            for(i=1; i<=NF; i++) headers[i] = $i
            next 
        }
        {
            if (NR > 2) print ","
            printf "  {"
            for(i=1; i<=NF; i++) {
                if (i > 1) printf ", "
                printf "\"%s\": \"%s\"", headers[i], $i
            }
            printf "}"
        }
        END { print "\n]" }
    "#
    )
    .no_echo()
    .run_with_io(input_reader, &mut json_output)?;

    let json_result = String::from_utf8(json_output)?;
    println!("   CSV converted to JSON:");
    println!("{}", json_result);

    Ok(())
}

fn performance_examples() -> Result<()> {
    println!("   Understanding performance benefits and streaming efficiency\n");

    // Large data streaming demonstration
    println!("⚡ Large data streaming:");
    println!("   Processing 10,000 lines without loading into memory");

    // Generate large dataset
    let large_dataset = (1..=10000)
        .map(|i| format!("record_{:05},value_{}", i, i * 2))
        .collect::<Vec<_>>()
        .join("\n");

    let large_dataset_bytes = large_dataset.into_bytes();
    let input_reader = Cursor::new(large_dataset_bytes);
    let mut summary_output = Vec::new();

    cmd!(
        "awk",
        r#"
        BEGIN { FS="," }
        { 
            total_records++
            if ($2 ~ /value_[0-9]+/) valid_records++
        }
        END { 
            printf "Processed %d records, %d valid (%.1f%%)\n", 
                   total_records, valid_records, (valid_records/total_records)*100
        }
    "#
    )
    .no_echo()
    .run_with_io(input_reader, &mut summary_output)?;

    let summary = String::from_utf8(summary_output)?;
    println!("   Result: {}", summary.trim());

    // Memory efficiency comparison
    println!("\n💾 Memory efficiency demonstration:");
    println!("   run_with_io() vs loading data into memory first");

    // Efficient approach using run_with_io
    let test_data = "line1\nline2\nline3\nline4\nline5\n".repeat(1000);
    let test_data_bytes = test_data.into_bytes();
    let input_reader = Cursor::new(test_data_bytes);
    let mut efficient_result = Vec::new();

    cmd!("wc", "-l")
        .no_echo()
        .run_with_io(input_reader, &mut efficient_result)?;

    let line_count = String::from_utf8(efficient_result)?;
    println!(
        "   Efficient streaming result: {} lines processed",
        line_count.trim()
    );
    println!("   Memory usage: Only buffers needed for streaming (minimal)");

    // Buffered reading for very large files
    println!("\n📚 Buffered reading optimization:");
    println!("   Using BufReader for optimal I/O performance");

    // Create a moderately large file for demonstration
    let large_content = "data line\n".repeat(5000);
    fs::write("large_temp.txt", large_content)?;

    let large_file = File::open("large_temp.txt")?;
    let buf_reader = BufReader::new(large_file);
    let mut analysis_result = Vec::new();

    cmd!(
        "awk",
        "{ char_count += length($0) } END { print \"Total characters:\", char_count }"
    )
    .no_echo()
    .run_with_io(buf_reader, &mut analysis_result)?;

    let analysis = String::from_utf8(analysis_result)?;
    println!("   Analysis result: {}", analysis.trim());

    fs::remove_file("large_temp.txt").ok();

    Ok(())
}
