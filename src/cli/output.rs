//! This module contains an interface to nicely render to a frontend.

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

/// Render strings nicely to a frontend, be it a terminal or whatever.
pub trait Renderer {
    /// Generic error type.
    type Error: std::error::Error;

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
