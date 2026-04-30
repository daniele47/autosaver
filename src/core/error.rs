//! This module has the Error type used throught the entire crate.

use std::{fmt::Display, path::PathBuf};

/// Error type for the entire crate.
#[derive(Debug)]
pub enum Error {
    /// All kind of filesystem related errors.
    IoError(std::io::Error, PathBuf),

    /// Path was required to be a file, but it was not.
    NotAFile(PathBuf),

    /// Could not remove a prefix from a path string.
    InvalidPathPrefix(PathBuf, PathBuf),

    /// Invalid path string when trying to convert from PathBuf.
    InvalidPathString(PathBuf),

    /// Profile definition includes cycles.
    ProfileCycle(String, Vec<String>),

    /// Profile could not be loaded.
    ProfileLoadingFailure(String, String),

    /// Config file doesn't specify the profile type.
    MissingProfileType(String),

    /// Invalid option line in config file.
    InvalidOptionLine(String, usize, String),

    /// Invalid data line in config file.
    InvalidDataLine(String, usize, String),
}

/// Result type for the entire crate, using `Error` error type.
pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(io, path) => {
                write!(f, "IO error on path {} : {io}", path.display())
            }
            Error::NotAFile(path) => write!(f, "Not a file: {}", path.display()),
            Error::InvalidPathPrefix(path, prefix) => {
                let path = path.display();
                let prefix = prefix.display();
                write!(f, "Invalid prefix {path} for path: {prefix}")
            }
            Error::InvalidPathString(path) => {
                write!(f, "Invalid path string: {}", path.display())
            }
            Error::ProfileCycle(name, cycle) => {
                let cstr = cycle
                    .iter()
                    .chain(cycle.first())
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(" → ");
                write!(f, "Profile {name} reaches a cycle: {cstr}")
            }
            Error::ProfileLoadingFailure(name, reason) => {
                write!(f, "Profile {name} could not be loaded: {reason}")
            }
            Error::MissingProfileType(name) => {
                write!(f, "Profile {name} lacks the profile type option line")
            }
            Error::InvalidOptionLine(name, n, line) => {
                write!(f, "Invalid option line ({n}) in profile {name} : {line}")
            }
            Error::InvalidDataLine(name, n, line) => {
                write!(f, "Invalid data line ({n}) in profile {name} : {line}")
            }
        }
    }
}

impl std::error::Error for Error {}
