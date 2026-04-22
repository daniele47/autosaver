//! This module has the Error type used throught the entire crate.

use std::path::PathBuf;

pub enum Error {
    IoError(std::io::Error),
    NonCanonicalPath(PathBuf),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
