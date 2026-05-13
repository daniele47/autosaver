use std::{
    fmt::Display,
    path::{Component, PathBuf},
    str::FromStr,
};

use anyhow::{Result, bail};

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

// &str ---> [PathStr]
impl FromStr for PathStr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Self::new(s.into())
    }
}

// [PathStr] ---> {}
impl Display for PathStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}
