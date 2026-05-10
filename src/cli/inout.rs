//! This module contains an interface to nicely interact with a cli frontend.

use std::{
    fmt::Display,
    io::{Write, stdout},
};

#[macro_export]
macro_rules! debug {
    ($ctx:expr, $($arg:tt)*) => {
        $ctx.debug(format!("DEBUG [{}:{}]: {}", file!(), line!(), format_args!($($arg)*)));
    };
}

// colors
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const PURPLE: &str = "\x1b[35m";
const WHITE: &str = "\x1b[37m";
const BOLD: &str = "\x1b[1m";
const UNDERLINE: &str = "\x1b[4m";

/// All possible styles for the strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Style {
    /// Color white.
    White,
    /// Color yellow.
    Yellow,
    /// Color red.
    Red,
    /// Color green.
    Green,
    /// Color blue.
    Blue,
    /// Color purple.
    Purple,

    /// Bold intensity.
    Bold,
    /// Apply underline.
    Underline,
}

/// Options for the `InOut`.
#[derive(Debug, Clone)]
pub struct InOutOptions {
    pub has_colors: bool,
    pub show_debug: bool,
}

/// Struct that implements `InOut` to write to the terminal.
#[derive(Debug, Clone, Default)]
pub struct TermInOut {
    options: InOutOptions,
}

impl InOutOptions {
    /// Create new option struct.
    pub fn new(has_colors: bool, show_debug: bool) -> Self {
        Self {
            has_colors,
            show_debug,
        }
    }
}

impl TermInOut {
    /// Create new struct.
    pub fn new(options: InOutOptions) -> Self {
        Self { options }
    }

    fn actual_write(&self, text: impl Display, add_newline: bool, mut output: impl Write) {
        let _ = if add_newline {
            writeln!(output, "{text}")
        } else {
            write!(output, "{text}")
        };
    }
    fn actual_error(&self, text: impl Display) {
        self.actual_write(text, true, std::io::stderr());
    }

    /// Write to the terminal, with choosen styles.
    pub fn write(&self, str: impl Display, styles: &[Style]) {
        let colors = styles
            .iter()
            .map(|f| match f {
                Style::White => WHITE,
                Style::Yellow => YELLOW,
                Style::Red => RED,
                Style::Green => GREEN,
                Style::Blue => BLUE,
                Style::Purple => PURPLE,
                Style::Bold => BOLD,
                Style::Underline => UNDERLINE,
            })
            .collect::<Vec<_>>()
            .join("");
        let text = match self.options.has_colors {
            true => format!("{colors}{str}{RESET}"),
            false => format!("{str}"),
        };
        self.actual_write(text, false, std::io::stdout());
    }

    /// Write a line to the terminal, with the choosen styles.
    pub fn writeln(&self, str: impl Display, styles: &[Style]) {
        self.write(format!("{str}\n"), styles);
    }

    /// Write to debug, only if debug is enabled.
    pub fn debug(&self, str: impl Display) {
        if self.options.show_debug {
            self.actual_error(str);
        }
    }

    /// Write an error.
    pub fn error(&self, str: impl Display) {
        let text = match self.options.has_colors {
            true => format!("{RED}{BOLD}ERROR: {str}{RESET}"),
            false => format!("ERROR: {str}"),
        };
        self.actual_error(text);
    }

    /// Write a warning.
    pub fn warning(&self, str: impl Display) {
        let text = match self.options.has_colors {
            true => format!("{YELLOW}{BOLD}WARNING: {str}{RESET}"),
            false => format!("WARNING: {str}"),
        };
        self.actual_error(text);
    }

    /// Read a line in input
    pub fn read_line(&self) -> String {
        let mut input = String::new();

        // flush stdout
        stdout().flush().expect("flush failed");

        // read from stdin
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        // trims
        input.lines().next().map(String::from).unwrap_or_default()
    }
}

impl Default for InOutOptions {
    fn default() -> Self {
        Self {
            has_colors: true,
            show_debug: false,
        }
    }
}
