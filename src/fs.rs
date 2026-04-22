//! This module implements various filesystem wrappers, to garantee safer fs operations.
//!
//! ```rust
//! todo!("add a simple example showcasing this module functionalities!!!");
//! todo!("add tests for this module");
//! ```

use std::{fs::FileType, path::PathBuf};

use crate::errors::{Error, Result};

#[derive(Debug)]
pub struct AbsPath {
    path: PathBuf,
}

#[derive(Debug)]
pub struct RelPath {
    path: PathBuf,
}

impl AbsPath {
    /// Creates new AbsPath from an absolute path.
    pub fn new(path: PathBuf) -> Result<Self> {
        assert!(path.is_absolute());
        let norm_path = path.canonicalize()?;
        if norm_path != path {
            return Err(Error::NonCanonicalPath(path));
        }
        Ok(Self { path: norm_path })
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
    pub fn to_absolute(&self, base: &AbsPath) -> Result<AbsPath> {
        let abs = base.path.join(&self.path);
        AbsPath::new(abs)
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
