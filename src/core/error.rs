//! This module has the Error type used throught the entire crate.

use std::{backtrace::Backtrace, fmt::Display, path::PathBuf, sync::Arc};

/// Result type for the entire crate, using `Error` error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Error with backtrace for core module.
#[derive(Debug)]
pub struct Error {
    etype: ErrorType,
    backtrace: Arc<Backtrace>,
}

/// Error type for the entire crate.
#[derive(Debug)]
pub enum ErrorType {
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
    InvalidOptionLine(String, usize, String, String),

    /// Invalid data line in config file.
    InvalidDataLine(String, usize, String, String),
}

impl Error {
    /// Get Error type.
    pub fn error_type(&self) -> &ErrorType {
        &self.etype
    }

    /// Get Error backtrace.
    pub fn backtrace(&self) -> Arc<Backtrace> {
        self.backtrace.clone()
    }
}

impl From<ErrorType> for Error {
    fn from(etype: ErrorType) -> Self {
        Self {
            etype,
            backtrace: Arc::new(Backtrace::force_capture()),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.etype)
    }
}

impl std::error::Error for Error {}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::IoError(io, path) => {
                write!(f, "IO error on path '{}' : {io}", path.display())
            }
            ErrorType::NotAFile(path) => write!(f, "Not a file: {}", path.display()),
            ErrorType::InvalidPathPrefix(path, prefix) => {
                let path = path.display();
                let prefix = prefix.display();
                write!(f, "Invalid prefix '{path}' for path: {prefix}")
            }
            ErrorType::InvalidPathString(path) => {
                write!(f, "Invalid path string: {}", path.display())
            }
            ErrorType::ProfileCycle(name, cycle) => {
                let cstr = cycle
                    .iter()
                    .chain(cycle.first())
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(" → ");
                write!(f, "Profile '{name}' reaches a cycle: {cstr}")
            }
            ErrorType::ProfileLoadingFailure(name, reason) => {
                write!(f, "Profile '{name}' could not be loaded: {reason}")
            }
            ErrorType::MissingProfileType(name) => {
                write!(f, "Profile '{name}' lacks the profile type option line")
            }
            ErrorType::InvalidOptionLine(name, n, l, r) => {
                let r = if r.is_empty() { r } else { &format!(" ({r})") };
                write!(f, "Invalid option line {n} in profile '{name}' : {l}{r}")
            }
            ErrorType::InvalidDataLine(name, n, l, r) => {
                let r = if r.is_empty() { r } else { &format!(" ({r})") };
                write!(f, "Invalid data line {n} in profile '{name}' : {l}{r}")
            }
        }
    }
}
