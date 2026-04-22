//! This module has the Error type used throught the entire crate.

/// Error type for the entire crate.
#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    NonCanonicalPath,
}

/// Result type for the entire crate, using `Error` error type.
pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
