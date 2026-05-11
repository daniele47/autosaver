//! Error type for the cli module.

use std::{backtrace::Backtrace, fmt::Display, path::PathBuf, sync::Arc};

use crate::cli::flags::Flag;

/// Simple result wrapper with this module error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Error with backtrace for cli module.
#[derive(Debug)]
pub struct Error {
    etype: ErrorType,
    backtrace: Arc<Backtrace>,
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
    InvalidParams(String, String),

    /// Invalid command passed.
    InvalidCommand(String),

    /// No profile specified to work on.
    MissingProfile,

    /// Invalid profile specified.
    InvalidProfile(String, String),

    /// A symlink where it is not allowed to.
    NotAllowedSymlink(String),

    /// Hidden config files are not allowed.
    NotAllowedHiddenConf(String),
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
            backtrace: Arc::new(Backtrace::capture()),
        }
    }
}

impl From<crate::core::error::Error> for Error {
    fn from(value: crate::core::error::Error) -> Self {
        let original_backtrace = value.backtrace();
        Self {
            etype: ErrorType::CoreError(value),
            backtrace: original_backtrace,
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
            ErrorType::NotAllowedSymlink(symlink) => write!(f, "Not allowed symlink: '{symlink}'"),
            ErrorType::InvalidProfile(p, r) => write!(f, "Invalid profile '{p}': {r}"),
            ErrorType::InvalidParams(cmd, args) => {
                write!(f, "Command {cmd} has invalid args: {args}")
            }
            ErrorType::NotAllowedHiddenConf(path) => {
                write!(f, "Not allowed hidden config: '{path}'")
            }
        }
    }
}
