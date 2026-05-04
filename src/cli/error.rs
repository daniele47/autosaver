//! Error type for the cli module.

use std::{fmt::Display, path::PathBuf};

use crate::cli::flags::Flag;

/// Simple result wrapper with this module error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for cli module.
#[derive(Debug)]
pub enum Error {
    /// Error coming from core module.
    CoreError(crate::core::error::Error),

    /// Environment variable is not defined or is empty.
    UndefinedEnv(String),

    /// Environment variable contains an invalid value.
    InvalidEnv(String, String),

    /// Script failed to run.
    ScriptFailure(PathBuf, String),

    /// Invalid flag passed.
    InvalidFlag(Flag, String),

    /// No profile specified to work on.
    MissingProfile,
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
            Error::UndefinedEnv(env) => {
                write!(f, "Undefined or empty environment variable '{env}'")
            }
            Error::InvalidEnv(env, reason) => {
                write!(f, "Invalid environment variable '{env}': {reason}")
            }
            Error::ScriptFailure(p, reason) => {
                write!(f, "Script '{}' failed: {reason}", p.display())
            }
            Error::InvalidFlag(flag, cmd) => write!(f, "Invalid flag '{flag}' for command '{cmd}'"),
            Error::MissingProfile => write!(f, "No profile was specified"),
        }
    }
}

impl std::error::Error for Error {}
