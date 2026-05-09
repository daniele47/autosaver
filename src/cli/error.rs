//! Error type for the cli module.

use std::{backtrace::Backtrace, fmt::Display, path::PathBuf};

use crate::cli::flags::Flag;

/// Simple result wrapper with this module error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Error with backtrace for cli module.
#[derive(Debug)]
pub struct Error {
    etype: ErrorType,
    backtrace: Box<Backtrace>,
}

/// Error type for cli module.
#[derive(Debug)]
pub enum ErrorType {
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

    /// Invalid command passed.
    InvalidCommand(String),

    /// No profile specified to work on.
    MissingProfile,

    /// A symlink where it is not allowed to.
    OutOfBoundSymlink(String, String),
}

impl Error {
    /// Get Error backtrace.
    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }
}

impl From<ErrorType> for Error {
    fn from(etype: ErrorType) -> Self {
        Self {
            etype,
            backtrace: Box::new(Backtrace::force_capture()),
        }
    }
}

impl From<crate::core::error::Error> for Error {
    fn from(value: crate::core::error::Error) -> Self {
        ErrorType::CoreError(value).into()
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
            ErrorType::CoreError(error) => write!(f, "{error}"),
            ErrorType::UndefinedEnv(env) => {
                write!(f, "Undefined or empty environment variable '{env}'")
            }
            ErrorType::InvalidEnv(env, reason) => {
                write!(f, "Invalid environment variable '{env}': {reason}")
            }
            ErrorType::ScriptFailure(p, reason) => {
                write!(f, "Script '{}' failed: {reason}", p.display())
            }
            ErrorType::InvalidFlag(flag, cmd) => {
                write!(f, "Invalid flag '{flag}' for command '{cmd}'")
            }
            ErrorType::InvalidCommand(cmd) => write!(f, "Invalid command: {cmd}"),
            ErrorType::MissingProfile => write!(f, "No profile was specified"),
            ErrorType::OutOfBoundSymlink(file, target) => write!(
                f,
                "Symlink point where it is not allowed to: {file} -> {target}"
            ),
        }
    }
}
