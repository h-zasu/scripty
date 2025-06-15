use clap::{Parser, Subcommand};
use scripty::*;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use toml::Value;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Development task runner for scripty
#[derive(Parser)]
#[command(
    name = "xtask",
    about = "Development task runner for scripty",
    long_about = "⚠️  IMPORTANT: README.md is auto-generated from src/lib.rs\n   To update README.md: edit src/lib.rs and run 'cargo readme'\n\nBefore committing: cargo xtask ci",
    version
)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Suppress output (overrides verbose)
    #[arg(short, long, global = true)]
    quiet: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run pre-commit checks (test + clippy + fmt)
    Precommit,
    /// Run all CI tasks
    Ci,
}

fn get_project_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    let current_name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    if current_name == "xtask" {
        Ok(current_dir.parent().unwrap().to_path_buf())
    } else {
        Ok(current_dir)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set global verbosity
    let verbose = cli.verbose && !cli.quiet;
    let _quiet = cli.quiet;

    match cli.command {
        Commands::Precommit => run_precommit(verbose)?,
        Commands::Ci => run_ci(verbose)?,
    }

    Ok(())
}

fn check_tool(name: &str) -> bool {
    cmd!("which", name).output().is_ok()
}

fn ensure_tool_installed(name: &str, package: &str, _verbose: bool) -> Result<bool> {
    if !check_tool(name) {
        // In CI environment, don't prompt for installation
        if std::env::var("CI").is_ok() {
            return Err(format!("{} is not installed. Please install it manually.", name).into());
        }

        println!("⚠️  {} is not installed.", name);
        print!("Would you like to install it via cargo? [y/N] ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == "y" {
            println!("📦 Installing {}...", name);
            cmd!("cargo", "install", package).run()?;
            println!("✅ {} installed successfully!", name);
            Ok(true)
        } else {
            println!("⚠️  Skipping {} formatting (not installed)", name);
            Ok(false)
        }
    } else {
        Ok(true)
    }
}

fn run_format_toml(project_root: &std::path::Path, verbose: bool) -> Result<()> {
    if ensure_tool_installed("taplo", "taplo-cli", verbose)? {
        if !verbose {
            println!("🎨 Formatting TOML files...");
        }
        cmd!("taplo", "fmt").current_dir(project_root).run()?;
        if !verbose {
            println!("✅ TOML files formatted!");
        }
    }
    Ok(())
}

fn run_format_markdown(project_root: &std::path::Path, verbose: bool) -> Result<()> {
    if ensure_tool_installed("dprint", "dprint", verbose)? {
        if !verbose {
            println!("🎨 Formatting Markdown files...");
        }
        cmd!("dprint", "fmt", "**/*.md")
            .current_dir(project_root)
            .run()?;
        if !verbose {
            println!("✅ Markdown files formatted!");
        }
    }
    Ok(())
}

fn run_format_toml_check(project_root: &std::path::Path, verbose: bool) -> Result<()> {
    let tool_available = ensure_tool_installed("taplo", "taplo-cli", verbose)?;
    if !tool_available {
        return Err("taplo is required for CI checks but not installed".into());
    }
    if !verbose {
        println!("🎨 Checking TOML formatting...");
    }
    cmd!("taplo", "fmt", "--check")
        .current_dir(project_root)
        .run()?;
    if !verbose {
        println!("✅ TOML format check passed!");
    }
    Ok(())
}

fn run_format_markdown_check(project_root: &std::path::Path, verbose: bool) -> Result<()> {
    let tool_available = ensure_tool_installed("dprint", "dprint", verbose)?;
    if !tool_available {
        return Err("dprint is required for CI checks but not installed".into());
    }
    if !verbose {
        println!("🎨 Checking Markdown formatting...");
    }
    cmd!("dprint", "check", "**/*.md")
        .current_dir(project_root)
        .run()?;
    if !verbose {
        println!("✅ Markdown format check passed!");
    }
    Ok(())
}

fn sync_version(verbose: bool) -> Result<()> {
    let project_root = get_project_root()?;

    // Read version from Cargo.toml
    let version = read_version_from_cargo_toml(&project_root)?;
    if !verbose {
        println!("📖 Read version {} from Cargo.toml", version);
    }

    // Update lib.rs documentation
    let lib_rs_path = project_root.join("src/lib.rs");
    let lib_content = fs::read_to_string(&lib_rs_path)?;
    let updated_lib = update_version_in_documentation(&lib_content, &version);
    fs::write(&lib_rs_path, updated_lib)?;
    if !verbose {
        println!("✅ Updated src/lib.rs documentation");
    }

    if !verbose {
        println!("🎉 Version {} synced to documentation!", version);
    }

    Ok(())
}

fn read_version_from_cargo_toml(project_root: &std::path::Path) -> Result<String> {
    let cargo_toml_path = project_root.join("Cargo.toml");
    let content = fs::read_to_string(&cargo_toml_path)?;
    let parsed: Value = toml::from_str(&content)?;

    let version = parsed["package"]["version"]
        .as_str()
        .ok_or("Version not found in Cargo.toml")?;

    Ok(version.to_string())
}

fn update_version_in_documentation(content: &str, new_version: &str) -> String {
    // Look for the pattern: scripty = "x.y.z" in documentation comments
    let pattern = r#"scripty = ""#;
    let lines: Vec<&str> = content.lines().collect();
    let mut updated_lines = Vec::new();

    for line in lines {
        if line.contains("//!") && line.contains(pattern) {
            // This is a documentation comment line containing version info
            if let Some(start_pos) = line.find(pattern) {
                let before = &line[..start_pos + pattern.len()];
                let after_start = start_pos + pattern.len();
                if let Some(quote_pos) = line[after_start..].find('"') {
                    let after = &line[after_start + quote_pos..];
                    updated_lines.push(format!("{}{}{}", before, new_version, after));
                } else {
                    updated_lines.push(line.to_string());
                }
            } else {
                updated_lines.push(line.to_string());
            }
        } else {
            updated_lines.push(line.to_string());
        }
    }

    updated_lines.join("\n")
}

fn run_format(project_root: &std::path::Path, verbose: bool) -> Result<()> {
    if !verbose {
        println!("🎨 Formatting code...");
    }
    cmd!("cargo", "fmt").current_dir(project_root).run()?;
    if !verbose {
        println!("✅ Code formatted!");
    }
    Ok(())
}

fn run_clippy(project_root: &std::path::Path, verbose: bool) -> Result<()> {
    if !verbose {
        println!("📎 Running comprehensive clippy checks...");
    }
    cmd!(
        "cargo",
        "clippy",
        "--all-targets",
        "--all-features",
        "--",
        "-D",
        "warnings"
    )
    .current_dir(project_root)
    .run()?;
    if !verbose {
        println!("✅ Clippy checks passed!");
    }
    Ok(())
}

fn run_tests(project_root: &std::path::Path, verbose: bool) -> Result<()> {
    if !verbose {
        println!("🧪 Running tests...");
    }
    cmd!("cargo", "test").current_dir(project_root).run()?;
    if !verbose {
        println!("✅ Tests passed!");
    }
    Ok(())
}

fn run_check(project_root: &std::path::Path, verbose: bool) -> Result<()> {
    if !verbose {
        println!("🔍 Running cargo check...");
    }
    cmd!("cargo", "check", "--all-targets")
        .current_dir(project_root)
        .run()?;
    if !verbose {
        println!("✅ Check passed!");
    }
    Ok(())
}

fn run_format_check(project_root: &std::path::Path, verbose: bool) -> Result<()> {
    if !verbose {
        println!("🎨 Checking code formatting...");
    }
    cmd!("cargo", "fmt", "--check")
        .current_dir(project_root)
        .run()?;
    if !verbose {
        println!("✅ Format check passed!");
    }
    Ok(())
}

fn run_examples(project_root: &std::path::Path, verbose: bool) -> Result<()> {
    if !verbose {
        println!("📋 Running examples...");
    }

    // Get all example files
    let examples_dir = project_root.join("examples");
    let entries = fs::read_dir(&examples_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let example_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or("Invalid example file name")?;

            if !verbose {
                println!("  🔧 Running example: {}", example_name);
            }

            cmd!("cargo", "run", "--example", example_name)
                .current_dir(project_root)
                .run()?;
        }
    }

    if !verbose {
        println!("✅ All examples run successfully!");
    }
    Ok(())
}

fn generate_readme(project_root: &std::path::Path, verbose: bool) -> Result<()> {
    if !verbose {
        println!("📝 Generating README.md...");
    }
    let readme_path = project_root.join("README.md");
    let readme_file = File::create(&readme_path)?;
    let buf_writer = BufWriter::new(readme_file);
    cmd!("cargo", "readme")
        .current_dir(project_root)
        .write_to(buf_writer)?;
    if !verbose {
        println!("✅ README.md generated!");
    }
    Ok(())
}

fn run_precommit(verbose: bool) -> Result<()> {
    if !verbose {
        println!("🔍 Running pre-commit checks...");
    }
    let project_root = get_project_root()?;

    // Sync version from Cargo.toml to documentation
    if !verbose {
        println!("🔄 Syncing version...");
    }
    sync_version(verbose)?;

    // Generate README
    generate_readme(&project_root, verbose)?;

    // Run examples
    run_examples(&project_root, verbose)?;

    // Run tests
    run_tests(&project_root, verbose)?;

    // Run comprehensive clippy
    run_clippy(&project_root, verbose)?;

    // Format code
    run_format(&project_root, verbose)?;

    // Format TOML files
    run_format_toml(&project_root, verbose)?;

    // Format Markdown files
    run_format_markdown(&project_root, verbose)?;

    if !verbose {
        println!("🎉 Pre-commit checks completed successfully!");
        println!("✅ Ready to commit!");
    }

    Ok(())
}

fn run_ci(verbose: bool) -> Result<()> {
    if !verbose {
        println!("🚀 Running CI checks (validation only)...");
    }
    let project_root = get_project_root()?;

    // Check formatting
    run_format_check(&project_root, verbose)?;

    // Check TOML formatting
    run_format_toml_check(&project_root, verbose)?;

    // Check Markdown formatting
    run_format_markdown_check(&project_root, verbose)?;

    // Check compilation
    run_check(&project_root, verbose)?;

    // Run static analysis
    run_clippy(&project_root, verbose)?;

    // Run examples
    run_examples(&project_root, verbose)?;

    // Run tests
    run_tests(&project_root, verbose)?;

    if !verbose {
        println!("🎉 All CI checks completed successfully!");
        println!("🔍 Summary:");
        println!("  ✅ Format check");
        println!("  ✅ TOML format check");
        println!("  ✅ Markdown format check");
        println!("  ✅ Compilation check");
        println!("  ✅ Clippy lints");
        println!("  ✅ Examples check");
        println!("  ✅ Test suite");
    }

    Ok(())
}
