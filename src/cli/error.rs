//! Error type for the cli module.

use std::fmt::Display;

/// Simple result wrapper with this module error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for cli module.
#[derive(Debug)]
pub enum Error {
    /// Error coming from core module.
    CoreError(crate::core::error::Error),
}

impl From<crate::core::error::Error> for Error {
    fn from(value: crate::core::error::Error) -> Self {
        Self::CoreError(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CoreError(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for Error {}
