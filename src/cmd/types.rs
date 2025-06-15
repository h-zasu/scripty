//! Type definitions for command execution and piping.

use std::ffi::OsString;
use std::io::Read;
use std::path::PathBuf;
use std::process::Child;

/// Input source for commands - either bytes in memory or a streaming reader.
pub(crate) enum CmdInput {
    /// Pre-loaded bytes in memory
    Bytes(Vec<u8>),
    /// Streaming reader (boxed for object safety)
    Reader(Box<dyn Read + Send>),
}

impl std::fmt::Debug for CmdInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CmdInput::Bytes(bytes) => f
                .debug_tuple("Bytes")
                .field(&format!("{} bytes", bytes.len()))
                .finish(),
            CmdInput::Reader(_) => f.debug_tuple("Reader").field(&"<reader>").finish(),
        }
    }
}

/// A simple command builder.
#[derive(Debug)]
pub struct Cmd {
    pub(crate) program: OsString,
    pub(crate) args: Vec<OsString>,
    pub(crate) envs: Vec<(OsString, OsString)>,
    pub(crate) current_dir: Option<PathBuf>,
    pub(crate) suppress_echo: bool,
}

/// Specifies which output streams should be piped between commands.
///
/// This enum is used internally to track pipe modes, but you typically don't need
/// to use it directly. Instead, use the convenient builder methods on `Cmd`:
///
/// - `pipe(cmd)` - pipes stdout (default)
/// - `pipe_err(cmd)` - pipes stderr only
/// - `pipe_out_err(cmd)` - pipes both stdout and stderr combined
///
/// # Examples
///
/// ```no_run
/// use scripty::cmd;
///
/// // Pipe stdout (default)
/// let output = cmd!("echo", "hello")
///     .pipe(cmd!("tr", "[:lower:]", "[:upper:]"))
///     .output()?;
///
/// // Pipe stderr between commands
/// let output = cmd!("command-with-errors")
///     .pipe_err(cmd!("grep", "ERROR"))
///     .output()?;
///
/// // Pipe both stdout and stderr
/// let output = cmd!("command-with-mixed-output")
///     .pipe_out_err(cmd!("sort"))
///     .output()?;
///
/// // Mixed pipe modes in one pipeline
/// let output = cmd!("sh", "-c", "echo 'out'; echo 'err' >&2")
///     .pipe_err(cmd!("process-errors"))
///     .pipe(cmd!("process-output"))
///     .output()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum PipeMode {
    /// Pipe only stdout between commands (default behavior).
    ///
    /// This is the standard Unix pipe behavior where each command's stdout
    /// becomes the next command's stdin.
    Stdout,

    /// Pipe only stderr between commands.
    ///
    /// Each command's stderr becomes the next command's stdin, while stdout
    /// is not connected between commands. Useful for error processing pipelines.
    Stderr,

    /// Pipe both stdout and stderr combined between commands.
    ///
    /// Both output streams are merged and sent to the next command's stdin.
    /// Note: The order of merged output may vary due to concurrent execution.
    Both,
}

/// Handle to a spawned pipeline for waiting and collecting results.
pub struct PipelineHandle {
    pub(crate) children: Vec<Child>,
}

/// Complete I/O access to a spawned pipeline.
pub struct PipelineSpawn {
    pub handle: PipelineHandle,
    pub stdin: Option<std::process::ChildStdin>,
    pub stdout: Option<std::process::ChildStdout>,
    pub stderr: Option<std::process::ChildStderr>,
}

/// A pipeline of commands.
#[derive(Debug)]
pub struct Pipeline {
    pub(crate) connections: Vec<(Cmd, PipeMode)>,
    pub(crate) input: Option<CmdInput>,
    pub(crate) suppress_echo: bool,
}
