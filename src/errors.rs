//! This module has the Error type used throught the entire crate.

use std::path::StripPrefixError;

/// Error type for the entire crate.
#[derive(Debug)]
pub enum Error {
    // external errors
    IoError(std::io::Error),

    // internal errors
    NonCanonicalPath,
    InvalidPathPrefix,
}

/// Result type for the entire crate, using `Error` error type.
pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<StripPrefixError> for Error {
    fn from(_: StripPrefixError) -> Self {
        Self::InvalidPathPrefix
    }
}
