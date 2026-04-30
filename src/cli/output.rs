//! This module contains an interface to nicely render to a frontend.

use crate::cli::error::Error;

/// All possible styles for the strings.
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

    /// Format as an error.
    Error,
    /// Format as warning.
    Warning,
}

/// Options for the `Renderer`.
#[derive(Debug, Clone, Default)]
pub struct RendererOptions {
    has_colors: bool,
}

/// Struct that implements `Renderer` to write to the terminal.
#[derive(Debug, Clone, Default)]
pub struct TermRenderer {
    options: RendererOptions,
}

/// Render strings nicely to a frontend, be it a terminal or whatever.
pub trait Renderer {
    /// Generic error type.
    type Error: std::error::Error;

    /// Allow setting options for the `Renderer`.
    fn set_options(&mut self, options: RendererOptions);

    /// Write a nicely formatted string to the frontend.
    fn write(
        &mut self,
        str: impl Into<String>,
        styles: &[Style],
    ) -> std::result::Result<(), Self::Error>;

    /// Write with an ending newline.
    fn writeln(
        &mut self,
        str: impl Into<String>,
        styles: &[Style],
    ) -> std::result::Result<(), Self::Error> {
        self.write(str.into() + "\n", styles)
    }
}

impl RendererOptions {
    /// Create new option struct.
    pub fn new(has_colors: bool) -> Self {
        Self { has_colors }
    }
}

impl TermRenderer {
    /// Create new struct.
    pub fn new(options: RendererOptions) -> Self {
        Self { options }
    }
}

impl Renderer for TermRenderer {
    type Error = Error;

    fn set_options(&mut self, options: RendererOptions) {
        (*self).options = options;
    }

    fn write(
        &mut self,
        str: impl Into<String>,
        styles: &[Style],
    ) -> std::result::Result<(), Self::Error> {
        todo!()
    }
}
