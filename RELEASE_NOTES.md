# Release Notes for v0.3.1

## 🚀 Overview

Scripty v0.3.1 is ready for its first public release on crates.io! This release brings a mature,
well-tested library for shell command execution and file operations in Rust.

## ✨ Key Features

### Command Execution

- **Intuitive `cmd!` macro** for creating commands
- **Colorful output** showing exactly what commands are being executed
- **Builder pattern** for flexible command configuration
- **Environment variable** and working directory support

### Pipeline Operations

- **Native pipe support** using Rust 1.87.0+'s `std::io::pipe`
- **Memory-efficient streaming** for large data processing
- **Flexible piping modes**: stdout, stderr, or both
- **Chain multiple commands** naturally

### I/O Handling

- **ReadExt trait** for fluent reader-to-command piping
- **Write methods** for streaming output to writers
- **Blocking and non-blocking** I/O operations
- **Complete I/O control** with spawn methods

### File System Operations

- **Automatic logging** of all file operations
- **Wrapper around `std::fs`** with enhanced visibility
- **Consistent error handling**

## 📊 Quality Metrics

- ✅ **107 unit tests** + 26 doc tests (all passing)
- ✅ **Zero clippy warnings**
- ✅ **Comprehensive documentation** with examples
- ✅ **Minimal dependencies** (only `anstyle` for colors)
- ✅ **6 learning examples** demonstrating key features

## 🎯 Target Audience

Scripty is perfect for:

- System administration scripts
- Build tools and automation
- Command-line utilities
- Data processing pipelines
- DevOps tooling

## 🔧 Requirements

- Rust 1.87.0 or later
- Unix-like systems (Linux, macOS)

## 📦 Installation

```toml
[dependencies]
scripty = "0.3.1"
```

## 🔗 Links

- [Documentation](https://docs.rs/scripty)
- [Repository](https://github.com/h-zasu/scripty)
- [Changelog](https://github.com/h-zasu/scripty/blob/main/CHANGELOG.md)

## 🙏 Acknowledgments

This is the first public release of scripty. We're excited to share it with the Rust community and
look forward to your feedback!
