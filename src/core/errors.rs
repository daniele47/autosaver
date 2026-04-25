//! This module has the Error type used throught the entire crate.

use std::{fmt::Display, path::PathBuf};

/// Error type for the entire crate.
#[derive(Debug)]
pub enum Error {
    /// All kind of filesystem related errors.
    IoError(std::io::Error, PathBuf),

    /// Could not remove a prefix from a path string.
    ///
    /// First string is the actual path, second string is the prefix.
    InvalidPathPrefix(PathBuf, PathBuf),

    /// Invalid path string when trying to convert from PathBuf.
    InvalidPathString(PathBuf),

    /// Profile definition includes cycles.
    ///
    /// First string is the profile name, the second is the child where the cycle was found.
    ProfileCycle(String, String),
}

/// Result type for the entire crate, using `Error` error type.
pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(error, path) => {
                write!(f, "IO error on path '{}' : {}", path.display(), error)
            }
            Error::InvalidPathString(p) => write!(f, "Invalid path string: {}", p.display()),
            Error::ProfileCycle(p, c) => {
                write!(f, "Profile '{}' reaches a cycle from child '{}'", p, c)
            }
            Error::InvalidPathPrefix(path, prefix) => {
                let path = path.display();
                let prefix = prefix.display();
                write!(f, "Invalid prefix '{path}' for path '{prefix}'")
            }
        }
    }
}
