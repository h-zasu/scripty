use anstyle::{AnsiColor, Color};

// Basic colors
pub const BLACK: Option<Color> = Some(Color::Ansi(AnsiColor::Black));
pub const RED: Option<Color> = Some(Color::Ansi(AnsiColor::Red));
pub const GREEN: Option<Color> = Some(Color::Ansi(AnsiColor::Green));
pub const YELLOW: Option<Color> = Some(Color::Ansi(AnsiColor::Yellow));
pub const BLUE: Option<Color> = Some(Color::Ansi(AnsiColor::Blue));
pub const MAGENTA: Option<Color> = Some(Color::Ansi(AnsiColor::Magenta));
pub const CYAN: Option<Color> = Some(Color::Ansi(AnsiColor::Cyan));
pub const WHITE: Option<Color> = Some(Color::Ansi(AnsiColor::White));

// Bright colors
pub const BRIGHT_BLACK: Option<Color> = Some(Color::Ansi(AnsiColor::BrightBlack));
pub const BRIGHT_RED: Option<Color> = Some(Color::Ansi(AnsiColor::BrightRed));
pub const BRIGHT_GREEN: Option<Color> = Some(Color::Ansi(AnsiColor::BrightGreen));
pub const BRIGHT_YELLOW: Option<Color> = Some(Color::Ansi(AnsiColor::BrightYellow));
pub const BRIGHT_BLUE: Option<Color> = Some(Color::Ansi(AnsiColor::BrightBlue));
pub const BRIGHT_MAGENTA: Option<Color> = Some(Color::Ansi(AnsiColor::BrightMagenta));
pub const BRIGHT_CYAN: Option<Color> = Some(Color::Ansi(AnsiColor::BrightCyan));
pub const BRIGHT_WHITE: Option<Color> = Some(Color::Ansi(AnsiColor::BrightWhite));
