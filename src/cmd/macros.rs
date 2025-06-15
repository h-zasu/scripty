//! Macros for convenient command creation.

/// Macro to create a new command.
#[macro_export]
macro_rules! cmd {
    ($program:expr) => {
        $crate::Cmd::new($program)
    };
    ($program:expr, $($arg:expr),* $(,)?) => {
        $crate::Cmd::new($program)$(.arg($arg))*
    };
}
