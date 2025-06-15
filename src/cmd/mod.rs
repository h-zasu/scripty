//! Simple command execution and piping functionality.

mod command;
mod error;
mod macros;
mod pipeline;
mod types;

// Re-export public API
pub use error::Error;
pub use types::{Cmd, Pipeline, PipelineHandle, PipelineSpawn};

// Internal items for testing and io_ext
pub(crate) use types::CmdInput;
#[cfg(test)]
pub(crate) use types::PipeMode;

#[cfg(test)]
mod tests;
