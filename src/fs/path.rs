use std::{
    fmt::Display,
    path::{Component, Path, PathBuf},
    str::FromStr,
};

use anyhow::bail;
use internment::Intern;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathStr {
    path: Intern<PathBuf>,
}

impl PathStr {
    pub(super) fn new_from_pathbuf(path: PathBuf) -> anyhow::Result<Self> {
        // check path doesn't contain parent directory
        if path.components().any(|c| c == Component::ParentDir) {
            bail!("Path contains parent directory: {}", path.display());
        }
        Ok(Self {
            path: Intern::new(path),
        })
    }

    pub(super) fn path(&self) -> &Path {
        &self.path
    }

    pub fn new(path: String) -> anyhow::Result<Self> {
        Self::new_from_pathbuf(path.into())
    }

    pub fn to_str(&self) -> Option<&str> {
        self.path().to_str()
    }

    pub fn to_string_lossy(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    pub fn display(&self) -> impl Display {
        self.path().display()
    }
}

impl TryFrom<String> for PathStr {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl FromStr for PathStr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.into())
    }
}
