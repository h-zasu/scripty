//! Command implementation and execution logic.

use crate::cmd::{error::Error, types::*};
use std::ffi::OsStr;
use std::io::{Read, Write};
use std::path::Path;

impl Cmd {
    /// Create a new command.
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        Self {
            program: program.as_ref().to_os_string(),
            args: Vec::new(),
            envs: Vec::new(),
            current_dir: None,
            suppress_echo: false,
        }
    }

    /// Add an argument.
    pub fn arg(mut self, arg: impl AsRef<OsStr>) -> Self {
        self.args.push(arg.as_ref().to_os_string());
        self
    }

    /// Add multiple arguments.
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        for arg in args {
            self.args.push(arg.as_ref().to_os_string());
        }
        self
    }

    /// Set an environment variable.
    pub fn env(mut self, key: impl AsRef<OsStr>, val: impl AsRef<OsStr>) -> Self {
        self.envs
            .push((key.as_ref().to_os_string(), val.as_ref().to_os_string()));
        self
    }

    /// Set the working directory.
    pub fn current_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.current_dir = Some(dir.as_ref().to_path_buf());
        self
    }

    /// Convert this command into a single-command pipeline.
    pub(crate) fn into_pipeline(self) -> Pipeline {
        let suppress_echo = self.suppress_echo;
        Pipeline {
            connections: vec![(self, PipeMode::Stdout)],
            input: None,
            suppress_echo,
        }
    }

    /// Set binary input data for the command.
    /// Accepts `Vec<u8>`, `&[u8]`, or other types that can be converted to `Vec<u8>`.
    pub fn input_bytes(self, input: impl AsRef<[u8]>) -> Pipeline {
        self.into_pipeline().input_bytes(input)
    }

    /// Set binary input data for the command with zero-copy optimization.
    /// Takes ownership of `Vec<u8>` to avoid copying.
    pub fn input_bytes_owned(self, bytes: Vec<u8>) -> Pipeline {
        self.into_pipeline().input_bytes_owned(bytes)
    }

    /// Set text input for the command.
    /// Optimized to convert string directly to bytes without intermediate allocation.
    pub fn input(self, input: impl AsRef<str>) -> Pipeline {
        self.into_pipeline().input(input)
    }

    /// Run without echoing the command.
    pub fn no_echo(mut self) -> Self {
        self.suppress_echo = true;
        self
    }

    /// Pipe this command's stdout to another command's stdin.
    ///
    /// This is the standard Unix pipe behavior where stdout becomes stdin.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use scripty::cmd;
    ///
    /// // Standard pipe (stdout → stdin)
    /// let output = cmd!("echo", "hello")
    ///     .pipe_out(cmd!("tr", "[:lower:]", "[:upper:]"))
    ///     .output()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pipe_out(self, next: Cmd) -> Pipeline {
        let suppress_echo = self.suppress_echo || next.suppress_echo;
        Pipeline {
            connections: vec![(self, PipeMode::Stdout), (next, PipeMode::Stdout)],
            input: None,
            suppress_echo,
        }
    }

    /// Pipe this command to another command (alias for pipe_out).
    ///
    /// This is an alias for `pipe_out()` to maintain backward compatibility.
    /// Uses the standard Unix pipe behavior where stdout becomes stdin.
    pub fn pipe(self, next: Cmd) -> Pipeline {
        self.pipe_out(next)
    }

    /// Pipe this command's stderr to another command's stdin.
    ///
    /// This pipes the error output stream to the next command's input,
    /// useful for error processing and filtering workflows.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use scripty::cmd;
    ///
    /// // Process error messages through a pipeline
    /// let error_count = cmd!("sh", "-c", "echo 'ERROR: failed' >&2")
    ///     .pipe_err(cmd!("wc", "-l"))
    ///     .output()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pipe_err(self, next: Cmd) -> Pipeline {
        let suppress_echo = self.suppress_echo || next.suppress_echo;
        Pipeline {
            connections: vec![(self, PipeMode::Stdout), (next, PipeMode::Stderr)],
            input: None,
            suppress_echo,
        }
    }

    /// Pipe this command's combined stdout and stderr to another command's stdin.
    ///
    /// This merges both output streams and pipes them to the next command,
    /// useful for unified processing of all command output.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use scripty::cmd;
    ///
    /// // Sort all output (both stdout and stderr)
    /// let sorted_output = cmd!("sh", "-c", "echo 'out'; echo 'err' >&2")
    ///     .pipe_out_err(cmd!("sort"))
    ///     .output()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn pipe_out_err(self, next: Cmd) -> Pipeline {
        let suppress_echo = self.suppress_echo || next.suppress_echo;
        Pipeline {
            connections: vec![(self, PipeMode::Stdout), (next, PipeMode::Both)],
            input: None,
            suppress_echo,
        }
    }

    /// Run the command and return the exit status.
    pub fn run(self) -> Result<(), Error> {
        self.into_pipeline().run()
    }

    /// Get binary output from the command.
    pub fn output_bytes(self) -> Result<Vec<u8>, Error> {
        self.into_pipeline().output_bytes()
    }

    /// Get text output from the command.
    pub fn output(self) -> Result<String, Error> {
        self.into_pipeline().output()
    }

    /// Stream command's stdout to a Writer.
    /// This is more memory-efficient for large outputs.
    pub fn write_to<W: Write>(self, writer: W) -> Result<(), Error> {
        self.into_pipeline().write_to(writer)
    }

    /// Stream command's stderr to a Writer.
    /// This is useful for capturing error output separately.
    pub fn write_err_to<W: Write>(self, writer: W) -> Result<(), Error> {
        self.into_pipeline().write_err_to(writer)
    }

    /// Stream command's combined stdout and stderr to a Writer.
    /// This merges both output streams into the writer.
    pub fn write_both_to<W: Write + Send + 'static>(self, writer: W) -> Result<(), Error> {
        self.into_pipeline().write_both_to(writer)
    }

    /// Run the command with both input Reader and output Writer.
    /// This is the most flexible method for streaming I/O.
    pub fn run_with_io<R: Read + Send + 'static, W: Write>(
        self,
        reader: R,
        writer: W,
    ) -> Result<(), Error> {
        self.into_pipeline().run_with_io(reader, writer)
    }

    /// Run the command with input Reader and stderr Writer.
    /// This is useful for processing data while capturing error output.
    pub fn run_with_err_io<R: Read + Send + 'static, W: Write>(
        self,
        reader: R,
        writer: W,
    ) -> Result<(), Error> {
        self.into_pipeline().run_with_err_io(reader, writer)
    }

    /// Run the command with input Reader and combined stdout+stderr Writer.
    /// This merges both output streams for comprehensive logging.
    pub fn run_with_both_io<R: Read + Send + 'static, W: Write + Send + 'static>(
        self,
        reader: R,
        writer: W,
    ) -> Result<(), Error> {
        self.into_pipeline().run_with_both_io(reader, writer)
    }

    /// Spawn the command with full I/O control.
    pub fn spawn_io_all(self) -> Result<PipelineSpawn, Error> {
        self.into_pipeline().spawn_io_all()
    }

    /// Spawn the command with stdin control.
    pub fn spawn_io_in(self) -> Result<(PipelineHandle, Option<std::process::ChildStdin>), Error> {
        self.into_pipeline().spawn_io_in()
    }

    /// Spawn the command with stdout control.
    pub fn spawn_io_out(
        self,
    ) -> Result<(PipelineHandle, Option<std::process::ChildStdout>), Error> {
        self.into_pipeline().spawn_io_out()
    }

    /// Spawn the command with stderr control.
    pub fn spawn_io_err(
        self,
    ) -> Result<(PipelineHandle, Option<std::process::ChildStderr>), Error> {
        self.into_pipeline().spawn_io_err()
    }

    /// Spawn the command with stdin and stdout control.
    /// This is the most common interactive pattern for data transformation and interactive tools.
    pub fn spawn_io_in_out(
        self,
    ) -> Result<
        (
            PipelineHandle,
            Option<std::process::ChildStdin>,
            Option<std::process::ChildStdout>,
        ),
        Error,
    > {
        self.into_pipeline().spawn_io_in_out()
    }

    /// Spawn the command with stdin and stderr control.
    /// Useful for debugging scenarios where you need to send data and monitor errors.
    pub fn spawn_io_in_err(
        self,
    ) -> Result<
        (
            PipelineHandle,
            Option<std::process::ChildStdin>,
            Option<std::process::ChildStderr>,
        ),
        Error,
    > {
        self.into_pipeline().spawn_io_in_err()
    }

    /// Spawn the command with stdout and stderr control.
    pub fn spawn_io_out_err(
        self,
    ) -> Result<
        (
            PipelineHandle,
            Option<std::process::ChildStdout>,
            Option<std::process::ChildStderr>,
        ),
        Error,
    > {
        self.into_pipeline().spawn_io_out_err()
    }

    /// Quotes an argument for display purposes only if it contains characters that affect readability.
    ///
    /// **IMPORTANT**: This function is for visual display only and should NOT be used for
    /// shell escaping or security purposes. It focuses on readability rather than shell compatibility.
    ///
    /// Display behavior:
    /// - Arguments with spaces or control characters: wrapped in single quotes with escaping
    /// - Arguments with single quotes: wrapped in double quotes with escaping
    /// - Empty arguments: displayed as empty quotes
    /// - Safe arguments: displayed as-is
    ///
    /// This is used internally for the command echo feature to show what commands are being executed.
    pub(crate) fn quote_argument(arg: &OsStr) -> String {
        let arg_str = arg.to_string_lossy();

        // If the argument is empty, return empty quotes
        if arg_str.is_empty() {
            return "\"\"".to_string();
        }

        // Check if argument needs quoting (focus on readability and security)
        let needs_quoting = arg_str.chars().any(|c| {
            matches!(
                c,
                ' ' | '\t' | '\n' | '\r' | '"' | '\'' | '\0'
                    ..='\x1F'
                        | '\x7F'
                        | '*'
                        | '?'
                        | '['
                        | ']'
                        | '{'
                        | '}'
                        | '~'
                        | '$'
                        | '`'
                        | '|'
                        | '&'
                        | ';'
                        | '('
                        | ')'
                        | '<'
                        | '>'
                        | '#'
                        | '!'
                        | '='
            )
        });

        // Escape control characters for better display
        let escape_control_chars = |s: &str| -> String {
            s.chars()
                .map(|c| match c {
                    '\t' => "\\t".to_string(),
                    '\n' => "\\n".to_string(),
                    '\r' => "\\r".to_string(),
                    '\0' => "\\0".to_string(),
                    c if c.is_control() => format!("\\x{:02x}", c as u8),
                    c => c.to_string(),
                })
                .collect()
        };

        // Handle arguments with single quotes specially
        if arg_str.contains('\'') {
            let escaped = arg_str.replace('\\', "\\\\").replace('"', "\\\"");
            let escaped = escape_control_chars(&escaped);
            return format!("\"{}\"", escaped);
        }

        // If argument needs quoting, use single quotes with control char escaping
        if needs_quoting {
            let escaped = escape_control_chars(&arg_str);
            return format!("'{}'", escaped);
        }

        // No quoting needed
        arg_str.to_string()
    }
}
