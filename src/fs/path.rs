use std::{
    fmt::Display,
    path::{Component, Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, Result, anyhow, bail};
use tracing::instrument;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathStr {
    path: PathBuf,
}

impl PathStr {
    pub fn new(path: PathBuf) -> Result<Self> {
        // check path doesn't contain parent directory
        if path.components().any(|c| c == Component::ParentDir) {
            bail!("Path contains parent directory: {}", path.display());
        }

        Ok(Self { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn to_string_lossy(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    pub fn display(&self) -> impl Display {
        self.path().display()
    }

    #[instrument(ret, err, level = "trace", skip_all, fields(self= %self.display()))]
    pub fn basename(&self) -> Result<Self> {
        self.path
            .file_name()
            .map(|p| Self::try_from(p.as_ref()))
            .with_context(|| format!("Could not get basename of {}", self.display()))?
    }
}

// CONVERT FROM
impl TryFrom<PathBuf> for PathStr {
    type Error = anyhow::Error;

    fn try_from(path: PathBuf) -> std::prelude::v1::Result<Self, Self::Error> {
        Self::new(path)
    }
}
impl TryFrom<String> for PathStr {
    type Error = anyhow::Error;

    fn try_from(path: String) -> std::prelude::v1::Result<Self, Self::Error> {
        Self::new(path.into())
    }
}
impl FromStr for PathStr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Self::new(s.into())
    }
}
impl TryFrom<&Path> for PathStr {
    type Error = anyhow::Error;

    fn try_from(value: &Path) -> std::prelude::v1::Result<Self, Self::Error> {
        Self::try_from(PathBuf::from(value))
    }
}

// CONVERT INTO
impl From<PathStr> for PathBuf {
    fn from(value: PathStr) -> Self {
        value.path
    }
}
impl TryFrom<PathStr> for String {
    type Error = anyhow::Error;

    fn try_from(value: PathStr) -> Result<Self> {
        value
            .path
            .into_os_string()
            .into_string()
            .map_err(|os| anyhow!("Path contains invalid UTF-8: {:?}", os))
    }
}
impl AsRef<Path> for PathStr {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}
