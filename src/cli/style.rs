//! Module to provide a backend indipendent way to colorize and output strings.

use crate::core::error::Result;

#[macro_export]
macro_rules! render {
    ($($arg:expr),+ $(,)?) => {{
        $(
            let _styled: &mut dyn $crate::style::Styler = &mut $arg;
            _styled.render()?;
        )+
        Ok(())
    }};
}

#[macro_export]
macro_rules! renderln {
    ($($arg:expr),+ $(,)?) => {{
        $crate::render!($($arg),+, $crate::styler::Styler::new("\n"))?;
    }}
}

/// Trait providing all functionalities
pub trait Styler {
    /// Apply white color on text.
    fn white(&mut self) -> &mut Self;
    /// Apply red color on text.
    fn red(&mut self) -> &mut Self;
    /// Apply light green color on text.
    fn lgreen(&mut self) -> &mut Self;
    /// Applu green color on text.
    fn green(&mut self) -> &mut Self;
    /// Apply yellow color on text.
    fn yellow(&mut self) -> &mut Self;
    /// Apply blue color on text.
    fn blue(&mut self) -> &mut Self;
    /// Apply purple color on text.
    fn purple(&mut self) -> &mut Self;

    /// Apply bold on the text.
    fn bold(&mut self) -> &mut Self;
    /// Apply underline on the text.
    fn underline(&mut self) -> &mut Self;

    /// Treat the text as an error (ignores styling).
    fn error(&mut self) -> &mut Self;
    /// Treat the text as a warning (ignores styling).
    fn warning(&mut self) -> &mut Self;

    /// Render the styled output on the frontend.
    fn render(&mut self) -> Result<()>;
}

/// Implementation of Style to render the text on the terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermStyle {
    text: String,
    term_color: &'static str,
    term_decor: &'static str,
    text_type: TextType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TextType {
    Normal,
    Error,
    Warning,
}

const WHITE: &str = "\x1b[37m";
const RED: &str = "\x1b[31m";
const LGREEN: &str = "\x1b[32m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const PURPLE: &str = "\x1b[35m";
const BOLD: &str = "\x1b[1m";
const UNDERLINE: &str = "\x1b[4m";
const RESET: &str = "\x1b[m";

impl TermStyle {
    /// Create new TermStyle
    pub fn new(text: String) -> Self {
        Self {
            text,
            term_color: "",
            term_decor: "",
            text_type: TextType::Normal,
        }
    }
}

impl Styler for TermStyle {
    fn white(&mut self) -> &mut Self {
        self.term_color = WHITE;
        self
    }

    fn red(&mut self) -> &mut Self {
        self.term_color = RED;
        self
    }

    fn lgreen(&mut self) -> &mut Self {
        self.term_color = LGREEN;
        self
    }

    fn green(&mut self) -> &mut Self {
        self.term_color = GREEN;
        self
    }

    fn yellow(&mut self) -> &mut Self {
        self.term_color = YELLOW;
        self
    }

    fn blue(&mut self) -> &mut Self {
        self.term_color = BLUE;
        self
    }

    fn purple(&mut self) -> &mut Self {
        self.term_color = PURPLE;
        self
    }

    fn bold(&mut self) -> &mut Self {
        self.term_decor = BOLD;
        self
    }

    fn underline(&mut self) -> &mut Self {
        self.term_decor = UNDERLINE;
        self
    }

    fn error(&mut self) -> &mut Self {
        self.text_type = TextType::Error;
        self
    }

    fn warning(&mut self) -> &mut Self {
        self.text_type = TextType::Warning;
        self
    }

    fn render(&mut self) -> Result<()> {
        match self.text_type {
            TextType::Normal => {
                let col = self.term_color;
                let dec = self.term_decor;
                let text = self.text.as_str();
                println!("{col}{dec}{text}{RESET}",);
            }
            TextType::Error => {
                eprintln!("{RED}{BOLD}ERROR: {}{RESET}", self.text);
            }
            TextType::Warning => {
                eprintln!("{YELLOW}{BOLD}WARNING: {}{RESET}", self.text);
            }
        }
        Ok(())
    }
}

impl From<String> for TermStyle {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for TermStyle {
    fn from(value: &str) -> Self {
        Self::new(value.into())
    }
}
