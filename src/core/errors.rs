//! This module has the Error type used throught the entire crate.

use std::{fmt::Display, path::PathBuf};

/// Error type for the entire crate.
#[derive(Debug)]
pub enum Error {
    // fs errors
    IoError(std::io::Error, PathBuf),
}

/// Result type for the entire crate, using `Error` error type.
pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(error, path) => {
                write!(f, "IO error on path '{}' : {}", path.display(), error)
            }
        }
    }
}
