//! # 05 - spawn_io Methods: Non-blocking I/O Control
//!
//! This example demonstrates scripty's spawn_io_*() family of methods for
//! fine-grained control over process I/O streams. These methods provide
//! non-blocking access to stdin, stdout, and stderr handles for advanced
//! interaction patterns:
//! - spawn_io_in() - stdin control only
//! - spawn_io_out() - stdout control only  
//! - spawn_io_err() - stderr control only
//! - spawn_io_in_out() - stdin + stdout control
//! - spawn_io_in_err() - stdin + stderr control
//! - spawn_io_out_err() - stdout + stderr control
//! - spawn_io_all() - complete I/O control
//!
//! Estimated time: ~10 minutes
//! Prerequisites: Understanding of threading and async I/O patterns
//! Previous examples: 01_simple_pipes.rs, 02_pipe_modes.rs, 03_read_ext.rs, 04_run_with_io.rs

#![allow(clippy::manual_flatten)]

use scripty::*;
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    println!("🎛️ spawn_io Methods: Non-blocking I/O Control");
    println!("=============================================\n");

    println!("📊 Mathematical I/O Pattern Coverage (2³ = 8 patterns):");
    println!("   Pattern | In | Out | Err | Method            | Use Case");
    println!("   --------|----|----|-----|-------------------|------------------");
    println!("   000     | -  | -  | -   | run()/output()    | Basic execution");
    println!("   100     | ✓  | -  | -   | spawn_io_in()     | Input control");
    println!("   010     | -  | ✓  | -   | spawn_io_out()    | Output capture");
    println!("   001     | -  | -  | ✓   | spawn_io_err()    | Error monitoring");
    println!("   110     | ✓  | ✓  | -   | spawn_io_in_out() | Interactive processing ⭐");
    println!("   101     | ✓  | -  | ✓   | spawn_io_in_err() | Debug scenarios ⭐");
    println!("   011     | -  | ✓  | ✓   | spawn_io_out_err()| Output separation");
    println!("   111     | ✓  | ✓  | ✓   | spawn_io_all()    | Complete control");
    println!();

    // Pattern 100: Input control only
    println!("1. spawn_io_in() - Input Control Only (Pattern 100):");
    input_only_examples()?;

    // Pattern 010: Output capture only
    println!("\n2. spawn_io_out() - Output Capture Only (Pattern 010):");
    output_only_examples()?;

    // Pattern 001: Error monitoring only
    println!("\n3. spawn_io_err() - Error Monitoring Only (Pattern 001):");
    error_only_examples()?;

    // Pattern 110: Interactive processing (Most Important!)
    println!("\n4. spawn_io_in_out() - Interactive Processing (Pattern 110) ⭐:");
    interactive_processing_examples()?;

    // Pattern 101: Debug scenarios (Critical for development!)
    println!("\n5. spawn_io_in_err() - Debug Scenarios (Pattern 101) ⭐:");
    debug_scenarios_examples()?;

    // Pattern 011: Output separation
    println!("\n6. spawn_io_out_err() - Output Separation (Pattern 011):");
    output_separation_examples()?;

    // Pattern 111: Complete control
    println!("\n7. spawn_io_all() - Complete Control (Pattern 111):");
    complete_control_examples()?;

    println!("\n🎉 All 7 spawn_io patterns mastered!");
    println!("🧮 Mathematical completeness: 2³ = 8 patterns (excluding 000)");
    println!("💡 Key advantages:");
    println!("   • Non-blocking I/O for responsive applications");
    println!("   • Fine-grained control over individual streams");
    println!("   • Perfect for interactive tools and debugging");
    println!("   • Thread-based concurrent I/O handling");

    Ok(())
}

fn input_only_examples() -> Result<()> {
    println!("   Controlling stdin while letting stdout/stderr flow normally\n");

    // Basic input feeding
    println!("⌨️ Basic input feeding:");
    println!("   Command: wc -l (feeding lines via stdin)");
    let (handle, stdin) = cmd!("wc", "-l").spawn_io_in()?;

    let input_handle = stdin.map(|mut stdin| {
        thread::spawn(move || {
            writeln!(stdin, "First line").unwrap();
            writeln!(stdin, "Second line").unwrap();
            writeln!(stdin, "Third line").unwrap();
            // stdin automatically closes when dropped
        })
    });

    handle.wait()?;
    if let Some(handle) = input_handle {
        handle.join().unwrap();
    }
    println!("   Line counting completed successfully");

    // Interactive input simulation
    println!("\n🎮 Interactive input simulation:");
    println!("   Command: sort (feeding data incrementally)");
    let (handle, stdin) = cmd!("sort").no_echo().spawn_io_in()?;

    let input_handle = stdin.map(|mut stdin| {
        thread::spawn(move || {
            let items = ["zebra", "apple", "banana", "cherry"];
            for item in items {
                writeln!(stdin, "{}", item).unwrap();
                thread::sleep(Duration::from_millis(100)); // Simulate typing delay
            }
        })
    });

    handle.wait()?;
    if let Some(handle) = input_handle {
        handle.join().unwrap();
    }
    println!("   Incremental sorting completed successfully");

    Ok(())
}

fn output_only_examples() -> Result<()> {
    println!("   Capturing stdout while letting stdin/stderr flow normally\n");

    // Output capture and processing
    println!("📊 Output capture and processing:");
    println!("   Command: seq 1 5 (capturing numeric sequence)");
    let (handle, stdout) = cmd!("seq", "1", "5").spawn_io_out()?;

    let output_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            let mut numbers = Vec::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    numbers.push(line.trim().to_string());
                }
            }
            numbers
        })
    });

    handle.wait()?;
    if let Some(handle) = output_handle {
        let numbers = handle.join().unwrap();
        println!("   Captured numbers: {}", numbers.join(", "));
    }
    println!("   Sequence generation completed successfully");

    // Real-time output monitoring
    println!("\n🔍 Real-time output monitoring:");
    println!("   Command: ping -c 3 127.0.0.1 (monitoring ping responses)");
    let (handle, stdout) = cmd!("ping", "-c", "3", "127.0.0.1")
        .no_echo()
        .spawn_io_out()?;

    let output_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            let mut response_times = Vec::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.contains("time=") {
                        // Extract time value (simplified parsing)
                        if let Some(pos) = line.find("time=") {
                            let time_part = &line[pos + 5..];
                            if let Some(end) = time_part.find(' ') {
                                response_times.push(time_part[..end].to_string());
                            }
                        }
                    }
                }
            }
            response_times
        })
    });

    handle.wait()?;
    if let Some(handle) = output_handle {
        let times = handle.join().unwrap();
        if !times.is_empty() {
            println!("   Ping response times: {} ms", times.join(", "));
        } else {
            println!("   Ping completed (times not parsed)");
        }
    }
    println!("   Network test completed successfully");

    Ok(())
}

fn error_only_examples() -> Result<()> {
    println!("   Monitoring stderr while letting stdin/stdout flow normally\n");

    // Error stream monitoring
    println!("🚨 Error stream monitoring:");
    println!("   Command: sh -c 'echo normal; echo error >&2' (capturing stderr)");
    let (handle, stderr) =
        cmd!("sh", "-c", "echo 'Normal output'; echo 'Error message' >&2").spawn_io_err()?;

    let error_handle = stderr.map(|stderr| {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            let mut error_messages = Vec::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    error_messages.push(line);
                }
            }
            error_messages
        })
    });

    handle.wait()?;
    if let Some(handle) = error_handle {
        let errors = handle.join().unwrap();
        if !errors.is_empty() {
            println!("   Captured errors: {}", errors.join("; "));
        } else {
            println!("   No errors detected");
        }
    }
    println!("   Error monitoring completed successfully");

    // Compiler warning/error detection
    println!("\n🛠️ Compiler warning detection:");
    println!("   Command: rustc --version (checking for stderr output)");
    let (handle, stderr) = cmd!("rustc", "--version").no_echo().spawn_io_err()?;

    let error_handle = stderr.map(|stderr| {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            let mut warning_count = 0;
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.contains("warning") || line.contains("error") {
                        warning_count += 1;
                    }
                }
            }
            warning_count
        })
    });

    handle.wait()?;
    if let Some(handle) = error_handle {
        let warnings = handle.join().unwrap();
        println!("   Warnings detected: {}", warnings);
    }
    println!("   Version check completed successfully");

    Ok(())
}

fn interactive_processing_examples() -> Result<()> {
    println!("   ⭐ Most important pattern for interactive tools and data processing\n");

    // Interactive calculator
    println!("🧮 Interactive calculator:");
    println!("   Command: bc -l (interactive math calculations)");
    let (handle, stdin, stdout) = cmd!("bc", "-l").spawn_io_in_out()?;

    // Send mathematical expressions
    let input_handle = stdin.map(|mut stdin| {
        thread::spawn(move || {
            writeln!(stdin, "scale=2").unwrap();
            writeln!(stdin, "22/7").unwrap(); // Pi approximation
            writeln!(stdin, "sqrt(2)").unwrap(); // Square root of 2
            writeln!(stdin, "2^10").unwrap(); // 2 to the 10th power
            writeln!(stdin, "quit").unwrap();
        })
    });

    // Read calculation results
    let output_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            let mut results = Vec::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() && !trimmed.starts_with("scale") {
                        results.push(trimmed.to_string());
                    }
                }
            }
            results
        })
    });

    handle.wait()?;
    if let Some(handle) = input_handle {
        handle.join().unwrap();
    }
    if let Some(handle) = output_handle {
        let results = handle.join().unwrap();
        println!("   Calculation results: {}", results.join(", "));
    }
    println!("   Calculator session completed successfully");

    // Data transformation pipeline
    println!("\n🔄 Data transformation pipeline:");
    println!("   Command: tr a-z A-Z (uppercase conversion)");
    let (handle, stdin, stdout) = cmd!("tr", "a-z", "A-Z").no_echo().spawn_io_in_out()?;

    let input_handle = stdin.map(|mut stdin| {
        thread::spawn(move || {
            let phrases = ["hello world", "rust is awesome", "scripty rocks"];
            for phrase in phrases {
                writeln!(stdin, "{}", phrase).unwrap();
            }
        })
    });

    let output_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            let mut transformed = Vec::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    transformed.push(line.trim().to_string());
                }
            }
            transformed
        })
    });

    handle.wait()?;
    if let Some(handle) = input_handle {
        handle.join().unwrap();
    }
    if let Some(handle) = output_handle {
        let results = handle.join().unwrap();
        println!("   Transformed text: {}", results.join(" | "));
    }
    println!("   Text transformation completed successfully");

    Ok(())
}

fn debug_scenarios_examples() -> Result<()> {
    println!("   ⭐ Critical pattern for development tools and debugging\n");

    // Compilation error monitoring
    println!("🛠️ Compilation error monitoring:");
    println!("   Command: rustc - (compiling with error monitoring)");
    let invalid_rust = "fn main() {\n    println!(\"missing semicolon\")\n    let x = 5\n}";
    let (handle, stdin, stderr) = cmd!("rustc", "-").spawn_io_in_err()?;

    let input_handle = stdin.map(|mut stdin| {
        thread::spawn(move || {
            write!(stdin, "{}", invalid_rust).unwrap();
        })
    });

    let error_handle = stderr.map(|stderr| {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            let mut error_count = 0;
            let mut first_error = String::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.contains("error") && first_error.is_empty() {
                        first_error = line.trim().to_string();
                    }
                    if line.contains("error:") {
                        error_count += 1;
                    }
                }
            }
            (error_count, first_error)
        })
    });

    let _ = handle.wait();
    if let Some(handle) = input_handle {
        handle.join().unwrap();
    }
    if let Some(handle) = error_handle {
        let (count, first) = handle.join().unwrap();
        println!("   Compilation errors detected: {}", count);
        if !first.is_empty() {
            println!("   First error: {}", first);
        }
    }
    println!("   Code analysis completed (errors expected)");

    // JSON validation with error feedback
    println!("\n📋 JSON validation with error feedback:");
    println!("   Command: jq . (JSON syntax validation)");
    let malformed_json = r#"{"name": "test", "value": invalid_syntax, "count": 42}"#;
    let (handle, stdin, stderr) = cmd!("jq", ".").no_echo().spawn_io_in_err()?;

    let input_handle = stdin.map(|mut stdin| {
        thread::spawn(move || {
            write!(stdin, "{}", malformed_json).unwrap();
        })
    });

    let error_handle = stderr.map(|stderr| {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            let mut has_errors = false;
            let mut error_details = String::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.contains("parse error") || line.contains("Invalid") {
                        has_errors = true;
                        if error_details.is_empty() {
                            error_details = line.trim().to_string();
                        }
                    }
                }
            }
            (has_errors, error_details)
        })
    });

    let _ = handle.wait();
    if let Some(handle) = input_handle {
        handle.join().unwrap();
    }
    if let Some(handle) = error_handle {
        let (has_errors, details) = handle.join().unwrap();
        if has_errors {
            println!("   JSON validation failed: {}", details);
        } else {
            println!("   JSON validation completed");
        }
    }
    println!("   Validation completed (errors expected)");

    Ok(())
}

fn output_separation_examples() -> Result<()> {
    println!("   Separate handling of stdout and stderr streams\n");

    // Dual stream capture
    println!("📊 Dual stream capture:");
    println!("   Command: sh -c 'echo success; echo warning >&2' (separating outputs)");
    let (handle, stdout, stderr) = cmd!(
        "sh",
        "-c",
        "echo 'Operation successful'; echo 'Warning: deprecated API' >&2"
    )
    .spawn_io_out_err()?;

    let stdout_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            let mut messages = Vec::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    messages.push(line.trim().to_string());
                }
            }
            messages
        })
    });

    let stderr_handle = stderr.map(|stderr| {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            let mut warnings = Vec::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    warnings.push(line.trim().to_string());
                }
            }
            warnings
        })
    });

    handle.wait()?;

    if let Some(handle) = stdout_handle {
        let messages = handle.join().unwrap();
        println!("   Success messages: {}", messages.join("; "));
    }

    if let Some(handle) = stderr_handle {
        let warnings = handle.join().unwrap();
        println!("   Warning messages: {}", warnings.join("; "));
    }

    println!("   Dual capture completed successfully");

    // Build process monitoring
    println!("\n🔨 Build process monitoring:");
    println!("   Command: sh script (separating build output from warnings)");
    let (handle, stdout, stderr) = cmd!("sh", "-c", "echo 'Build started'; echo 'Compiling module 1'; echo 'Warning: unused variable' >&2; echo 'Build completed'").no_echo().spawn_io_out_err()?;

    let stdout_handle = stdout.map(|stdout| {
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            let mut build_steps = Vec::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    build_steps.push(line.trim().to_string());
                }
            }
            build_steps
        })
    });

    let stderr_handle = stderr.map(|stderr| {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            let mut warning_count = 0;
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.to_lowercase().contains("warning") {
                        warning_count += 1;
                    }
                }
            }
            warning_count
        })
    });

    handle.wait()?;

    if let Some(handle) = stdout_handle {
        let steps = handle.join().unwrap();
        println!("   Build steps: {} completed", steps.len());
    }

    if let Some(handle) = stderr_handle {
        let warnings = handle.join().unwrap();
        println!("   Build warnings: {}", warnings);
    }

    println!("   Build monitoring completed successfully");

    Ok(())
}

fn complete_control_examples() -> Result<()> {
    println!("   Complete control over all I/O streams\n");

    // Full process interaction
    println!("🎛️ Full process interaction:");
    println!("   Command: grep item (complete I/O control for filtering)");

    let spawn = cmd!("grep", "item").no_echo().spawn_io_all()?;

    // Input data in background
    let input_handle = spawn.stdin.map(|mut stdin| {
        thread::spawn(move || {
            let items = [
                "item1\n",
                "not_matching\n",
                "item2\n",
                "ignore_this\n",
                "special_item\n",
            ];
            for item in items {
                write!(stdin, "{}", item).unwrap();
                thread::sleep(Duration::from_millis(50));
            }
        })
    });

    // Capture filtered output
    let output_handle = spawn.stdout.map(|stdout| {
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            let mut filtered_items = Vec::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    filtered_items.push(line.trim().to_string());
                }
            }
            filtered_items
        })
    });

    // Monitor any errors
    let error_handle = spawn.stderr.map(|stderr| {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            let mut error_messages = Vec::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    error_messages.push(line.trim().to_string());
                }
            }
            error_messages
        })
    });

    // Wait for completion
    spawn.handle.wait()?;

    if let Some(handle) = input_handle {
        handle.join().unwrap();
    }

    if let Some(handle) = output_handle {
        let filtered = handle.join().unwrap();
        println!("   Filtered items: {}", filtered.join(", "));
    }

    if let Some(handle) = error_handle {
        let errors = handle.join().unwrap();
        if !errors.is_empty() {
            println!("   Errors: {}", errors.join("; "));
        } else {
            println!("   No errors detected");
        }
    }

    println!("   Complete control example completed successfully");

    // Complex data processing pipeline
    println!("\n⚙️ Complex data processing pipeline:");
    println!("   Command: awk script (advanced data analysis with full control)");

    let spawn = cmd!("awk", "{ if (NF >= 2) { sum += $2; count++ } else { print \"Invalid record: \" $0 > \"/dev/stderr\" } } END { if (count > 0) print \"Average:\", sum/count; else print \"No valid records\" }").no_echo().spawn_io_all()?;

    // Send test data
    let input_handle = spawn.stdin.map(|mut stdin| {
        thread::spawn(move || {
            let data = [
                "record1 100\n",
                "invalid_line\n",
                "record2 200\n",
                "record3 150\n",
                "malformed\n",
            ];
            for line in data {
                write!(stdin, "{}", line).unwrap();
            }
        })
    });

    // Capture analysis results
    let output_handle = spawn.stdout.map(|stdout| {
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            let mut results = Vec::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    results.push(line.trim().to_string());
                }
            }
            results
        })
    });

    // Capture validation errors
    let error_handle = spawn.stderr.map(|stderr| {
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            let mut validation_errors = Vec::new();
            for line in reader.lines() {
                if let Ok(line) = line {
                    validation_errors.push(line.trim().to_string());
                }
            }
            validation_errors
        })
    });

    spawn.handle.wait()?;

    if let Some(handle) = input_handle {
        handle.join().unwrap();
    }

    if let Some(handle) = output_handle {
        let results = handle.join().unwrap();
        println!("   Analysis results: {}", results.join("; "));
    }

    if let Some(handle) = error_handle {
        let errors = handle.join().unwrap();
        if !errors.is_empty() {
            println!("   Validation errors: {} detected", errors.len());
        }
    }

    println!("   Advanced processing completed successfully");

    Ok(())
}
