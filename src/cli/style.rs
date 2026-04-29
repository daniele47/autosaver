//! Module to provide a backend indipendent way to colorize and output strings.

use crate::core::error::Result;

/// Trait providing all functionalities
pub trait Style {
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

    /// Render the styled output to the frontend.
    fn visualize(&mut self) -> Result<()>;
}
