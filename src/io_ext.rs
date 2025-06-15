//! Extension traits for standard library I/O types to enable fluent piping.

use crate::cmd::{Cmd, CmdInput, Pipeline};
use std::io::Read;

/// Extension trait for `std::io::Read` to enable fluent piping to commands.
///
/// This trait allows any `Read`-implementing type to be piped directly to commands,
/// enabling intuitive method chaining for I/O operations.
///
/// # Examples
///
/// ```no_run
/// use scripty::*;
/// use std::fs::File;
///
/// // Pipe file contents through a command pipeline
/// let file = File::open("data.txt")?;
/// let result = file.pipe(cmd!("grep", "pattern"))
///     .pipe(cmd!("sort"))
///     .pipe(cmd!("uniq"))
///     .output()?;
///
/// // Process large files efficiently with streaming
/// use std::io::BufReader;
/// let large_file = File::open("huge_dataset.txt")?;
/// let reader = BufReader::new(large_file);
/// reader.pipe(cmd!("awk", "{sum += $1} END {print sum}"))
///     .run()?;
///
/// // Chain with existing pipeline methods
/// use std::io::Cursor;
/// let data = Cursor::new(b"line1\nline2\nline3\n");
/// data.pipe(cmd!("sort"))
///     .pipe(cmd!("wc", "-l"))
///     .output()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub trait ReadExt: Read {
    /// Pipe this reader's data to a command's stdin.
    ///
    /// This method creates a pipeline where the reader's data becomes the input
    /// for the specified command. The resulting `Pipeline` can be further chained
    /// with additional commands or executed with methods like `run()`, `output()`,
    /// or `write_to()`.
    ///
    /// # Type Requirements
    ///
    /// The reader must be `Send + 'static` to support the pipeline's threading model.
    /// This is automatically satisfied by most standard library types like `File`,
    /// `BufReader`, `Cursor`, etc.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use scripty::*;
    /// use std::fs::File;
    ///
    /// // Basic piping
    /// let file = File::open("input.txt")?;
    /// file.pipe(cmd!("sort")).run()?;
    ///
    /// // Get output
    /// let file = File::open("numbers.txt")?;
    /// let sum = file.pipe(cmd!("awk", "{sum += $1} END {print sum}"))
    ///     .output()?;
    ///
    /// // Chain multiple commands
    /// let file = File::open("log.txt")?;
    /// let errors = file.pipe(cmd!("grep", "ERROR"))
    ///     .pipe(cmd!("wc", "-l"))
    ///     .output()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn pipe(self, cmd: Cmd) -> Pipeline
    where
        Self: Sized + Send + 'static,
    {
        let mut pipeline = cmd.into_pipeline();
        pipeline.input = Some(CmdInput::Reader(Box::new(self)));
        pipeline
    }
}

// Implement ReadExt for all types that implement Read
impl<R: Read> ReadExt for R {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd;
    use std::io::Cursor;

    #[test]
    fn test_read_ext_basic_pipe() -> Result<(), Box<dyn std::error::Error>> {
        let data = Cursor::new(b"hello\nworld\n");
        let result = data.pipe(cmd!("wc", "-l")).output()?;
        assert_eq!(result.trim(), "2");
        Ok(())
    }

    #[test]
    fn test_read_ext_chained_pipe() -> Result<(), Box<dyn std::error::Error>> {
        let data = Cursor::new(b"banana\napple\ncherry\n");
        let result = data.pipe(cmd!("sort")).pipe(cmd!("head", "-1")).output()?;
        assert_eq!(result.trim(), "apple");
        Ok(())
    }

    #[test]
    fn test_read_ext_with_empty_input() -> Result<(), Box<dyn std::error::Error>> {
        let data = Cursor::new(b"");
        let result = data.pipe(cmd!("wc", "-l")).output()?;
        assert_eq!(result.trim(), "0");
        Ok(())
    }

    #[test]
    fn test_read_ext_binary_data() -> Result<(), Box<dyn std::error::Error>> {
        let binary_data = vec![0u8, 1, 2, 3, 4, 5];
        let data = Cursor::new(binary_data);
        let result = data.pipe(cmd!("wc", "-c")).output()?;
        assert_eq!(result.trim(), "6");
        Ok(())
    }
}
