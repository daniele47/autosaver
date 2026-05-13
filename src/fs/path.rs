use std::{
    fmt::Display,
    path::{Component, Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, Result, bail};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathStr {
    path: PathBuf,
}

impl PathStr {
    pub fn new(path: PathBuf) -> Result<Self> {
        // check path doesn't contain parent directory
        if path.components().any(|c| c == Component::ParentDir) {
            bail!("Path string contains parent directory: {}", path.display());
        }

        Ok(Self { path })
    }
}

impl TryFrom<String> for PathStr {
    type Error = anyhow::Error;

    fn try_from(path: String) -> std::prelude::v1::Result<Self, Self::Error> {
        Self::new(path.into())
    }
}
impl TryFrom<PathBuf> for PathStr {
    type Error = anyhow::Error;

    fn try_from(path: PathBuf) -> std::prelude::v1::Result<Self, Self::Error> {
        Self::new(path)
    }
}
impl FromStr for PathStr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Self::new(s.into())
    }
}

impl TryFrom<PathStr> for String {
    type Error = anyhow::Error;

    fn try_from(value: PathStr) -> std::prelude::v1::Result<Self, Self::Error> {
        value
            .path
            .to_str()
            .with_context(|| {
                let p = value.path.display();
                format!("Could not convert to string: {p}")
            })
            .map(str::to_string)
    }
}
impl From<PathStr> for PathBuf {
    fn from(value: PathStr) -> Self {
        value.path
    }
}
impl AsRef<Path> for PathStr {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}
impl Display for PathStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}
