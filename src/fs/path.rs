use std::{
    fmt::Display,
    path::{Component, PathBuf},
    str::FromStr,
};

use anyhow::{Ok, Result, bail};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathStr {
    path: PathBuf,
}

impl PathStr {
    fn new_from_pathbuf(path: PathBuf) -> Result<Self> {
        // check path doesn't contain parent directory
        if path.components().any(|c| c == Component::ParentDir) {
            bail!("Path string contains parent directory: {}", path.display());
        }

        Ok(Self { path })
    }

    pub fn new(path: String) -> Result<Self> {
        Self::new_from_pathbuf(path.into())
    }
}

// &str ---> PathStr
impl FromStr for PathStr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Self::new_from_pathbuf(s.into())
    }
}

// PathStr ---> {}
impl Display for PathStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}
