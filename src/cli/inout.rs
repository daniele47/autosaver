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
const LGREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const PURPLE: &str = "\x1b[35m";
const GREEN: &str = "\x1b[32m";
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
    /// Color light green.
    LGreen,
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
}

impl TermInOut {
    pub fn options(&mut self) -> &mut InOutOptions {
        &mut self.options
    }

    pub fn write(&self, str: impl Display, styles: &[Style]) {
        let colors = styles
            .iter()
            .map(|f| match f {
                Style::White => WHITE,
                Style::Yellow => YELLOW,
                Style::Red => RED,
                Style::LGreen => LGREEN,
                Style::Green => GREEN,
                Style::Blue => BLUE,
                Style::Purple => PURPLE,
                Style::Bold => BOLD,
                Style::Underline => UNDERLINE,
            })
            .collect::<Vec<_>>()
            .join("");
        match self.options.has_colors {
            true => print!("{colors}{str}{RESET}"),
            false => print!("{str}"),
        };
    }

    pub fn writeln(&self, str: impl Display, styles: &[Style]) {
        self.write(str, styles);
        self.write("\n", &[]);
    }

    pub fn debug(&self, str: impl Display) {
        if self.options.show_debug {
            eprintln!("{str}");
        }
    }

    pub fn error(&self, str: impl Display) {
        match self.options.has_colors {
            true => eprintln!("{RED}{BOLD}ERROR: {str}{RESET}"),
            false => eprintln!("ERROR: {str}"),
        };
    }

    pub fn warning(&self, str: impl Display) {
        match self.options.has_colors {
            true => eprintln!("{YELLOW}{BOLD}WARNING: {str}{RESET}"),
            false => eprintln!("WARNING: {str}"),
        };
    }

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
