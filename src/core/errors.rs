//! This module has the Error type used throught the entire crate.

use std::{fmt::Display, path::PathBuf};

/// Error type for the entire crate.
#[derive(Debug)]
pub enum Error {
    /// All kind of filesystem related errors.
    IoError { io: std::io::Error, path: PathBuf },

    /// Could not remove a prefix from a path string.
    InvalidPathPrefix { path: PathBuf, prefix: PathBuf },

    /// Invalid path string when trying to convert from PathBuf.
    InvalidPathString { path: PathBuf },

    /// Profile definition includes cycles.
    ProfileCycle { name: String, cycle: Vec<String> },

    /// Failure to load a profile
    ProfileNotLoaded { name: String, reason: String },
}

/// Result type for the entire crate, using `Error` error type.
pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError { io: e, path: p } => {
                write!(f, "IO error on path '{}' : {e}", p.display())
            }
            Error::InvalidPathPrefix { path: p, prefix: r } => {
                let p = p.display();
                let r = r.display();
                write!(f, "Invalid prefix '{p}' for path '{r}'")
            }
            Error::InvalidPathString { path: p } => {
                write!(f, "Invalid path string: {}", p.display())
            }
            Error::ProfileCycle { name: n, cycle: c } => {
                let cstr = c
                    .iter()
                    .chain(c.first())
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(" → ");
                write!(f, "Profile '{n}' reaches a cycle: {cstr}")
            }
            Error::ProfileNotLoaded { name: n, reason: r } => {
                write!(f, "Profile '{n}' could not be loaded: {r}")
            }
        }
    }
}
