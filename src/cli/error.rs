//! Error type for the cli module.

/// Simple result wrapper with this module error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for cli module.
pub enum Error {
    /// Error coming from core module.
    CoreError(crate::core::error::Error),
}

impl From<crate::core::error::Error> for Error {
    fn from(value: crate::core::error::Error) -> Self {
        Self::CoreError(value)
    }
}
