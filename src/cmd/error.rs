//! Error handling for command execution.

/// Command execution error.
#[derive(Debug)]
pub struct Error {
    pub(crate) message: String,
    pub(crate) source: Option<std::io::Error>,
}

impl Error {
    /// Creates an error for a command that was not found.
    #[allow(dead_code)]
    pub(crate) fn command_not_found(command: &str) -> Self {
        Error {
            message: format!("Command not found: {}", command),
            source: None,
        }
    }

    /// Creates an error for a command that failed with an exit code.
    pub(crate) fn exit_code(code: Option<i32>) -> Self {
        Error {
            message: format!("Command failed with exit code: {:?}", code),
            source: None,
        }
    }

    /// Creates an error for an invalid or empty command.
    #[allow(dead_code)]
    pub(crate) fn invalid_command(reason: &str) -> Self {
        Error {
            message: format!("Invalid command: {}", reason),
            source: None,
        }
    }

    /// Creates an error with an IO error as the source.
    pub(crate) fn io(message: &str, source: std::io::Error) -> Self {
        Error {
            message: message.to_string(),
            source: Some(source),
        }
    }

    /// Creates an error for missing stdout.
    pub(crate) fn no_stdout() -> Self {
        Error {
            message: "No stdout available to read from".to_string(),
            source: None,
        }
    }

    /// Creates an error for missing stderr.
    #[allow(dead_code)]
    pub(crate) fn no_stderr() -> Self {
        Error {
            message: "No stderr available to read from".to_string(),
            source: None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(source) = &self.source {
            write!(f, ": {}", source)?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e as &dyn std::error::Error)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error {
            message: "Command execution failed".to_string(),
            source: Some(err),
        }
    }
}
