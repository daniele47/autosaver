//! This module implements various filesystem wrappers, to garantee safer fs operations.
//!
//! ```rust
//! todo!("add a simple example showcasing this module functionalities!!!");
//! todo!("add tests for this module");
//! ```

use std::{fs::FileType, path::PathBuf};

use crate::errors::{Error, Result};

/// Struct storing an absolute path.
///
/// Note: it doesn't make garantees the path exists, nor that it is not in a symlinked location.
/// Thus those needs to be validated using `validate` method or manually somehow.
#[derive(Debug)]
pub struct AbsPath {
    path: PathBuf,
}

/// Struct storing a relative path.
#[derive(Debug)]
pub struct RelPath {
    path: PathBuf,
}

impl AbsPath {
    /// Creates new AbsPath from an absolute path.
    pub fn new(path: PathBuf) -> Self {
        assert!(path.is_absolute());
        Self { path: path }
    }

    /// Validate path is valid.
    pub fn validate(self) -> Result<Self> {
        assert!(self.path.is_absolute());
        let norm_path = self.path.canonicalize()?;
        if norm_path != self.path {
            return Err(Error::NonCanonicalPath);
        }
        Ok(self)
    }

    /// Get FileType.
    pub fn file_type(&self) -> Result<FileType> {
        Ok(self.path.metadata()?.file_type())
    }
}

impl RelPath {
    /// Creates new RelPath from relative path.
    pub fn new(path: PathBuf) -> Self {
        assert!(path.is_relative());
        Self { path }
    }

    /// Add a prefix to turn relative path into absolute path.
    pub fn to_absolute(&self, base: &AbsPath) -> AbsPath {
        let abs = base.path.join(&self.path);
        AbsPath::new(abs)
    }
}

impl From<PathBuf> for AbsPath {
    fn from(value: PathBuf) -> Self {
        Self::new(value)
    }
}

impl From<PathBuf> for RelPath {
    fn from(value: PathBuf) -> Self {
        Self::new(value)
    }
}

impl From<AbsPath> for PathBuf {
    fn from(value: AbsPath) -> Self {
        value.path
    }
}

impl From<RelPath> for PathBuf {
    fn from(value: RelPath) -> Self {
        value.path
    }
}
