//! Output utilities for scripty

/// Check if output should be echoed based on NO_ECHO environment variable
pub(crate) fn should_echo() -> bool {
    std::env::var_os("NO_ECHO").is_none()
}

/// Print to stderr if echo is enabled
pub(crate) fn conditional_eprintln(args: std::fmt::Arguments) {
    if should_echo() {
        eprintln!("{}", args);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_should_echo_normal() {
        // Save original state
        let original = std::env::var("NO_ECHO").ok();

        // Ensure NO_ECHO is not set
        unsafe {
            std::env::remove_var("NO_ECHO");
        }

        assert!(should_echo());

        // Restore original state
        unsafe {
            match original {
                Some(val) => std::env::set_var("NO_ECHO", val),
                None => std::env::remove_var("NO_ECHO"),
            }
        }
    }

    #[test]
    #[serial]
    fn test_should_echo_with_no_echo_env() {
        // Save original state
        let original = std::env::var("NO_ECHO").ok();

        // Set NO_ECHO environment variable
        unsafe {
            std::env::set_var("NO_ECHO", "1");
        }

        assert!(!should_echo());

        // Restore original state
        unsafe {
            match original {
                Some(val) => std::env::set_var("NO_ECHO", val),
                None => std::env::remove_var("NO_ECHO"),
            }
        }
    }

    #[test]
    fn test_conditional_functions_compile() {
        // Test that the functions compile and don't panic
        conditional_eprintln(format_args!("test"));
    }
}
