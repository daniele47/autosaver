use std::{
    fmt::Display,
    path::{Component, Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, Result, bail};
use derive_getters::{Dissolve, Getters};

#[derive(Debug, Clone, PartialEq, Eq, Getters, Dissolve)]
pub struct PathStr {
    path: PathBuf,
}

impl PathStr {
    pub fn new(path: PathBuf) -> Result<Self> {
        // check path doesn't contain parent directory
        if path.components().any(|c| c == Component::ParentDir) {
            let p = path.display();
            bail!("Path string contains parent directory: {p}");
        }

        Ok(Self { path })
    }

    pub fn file_name(&self) -> Result<Self> {
        Self::new(
            self.path
                .file_name()
                .with_context(|| format!("Could not get basename of: {}", self.path.display()))?
                .into(),
        )
    }

    pub fn join(&self, suffix: Self) -> Result<Self> {
        Self::new(self.path.join(suffix))
    }
}

impl FromStr for PathStr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Self::new(s.into())
    }
}

impl TryFrom<String> for PathStr {
    type Error = anyhow::Error;

    fn try_from(value: String) -> std::prelude::v1::Result<Self, Self::Error> {
        Self::new(value.into())
    }
}

impl TryFrom<PathStr> for String {
    type Error = anyhow::Error;

    fn try_from(value: PathStr) -> std::prelude::v1::Result<Self, Self::Error> {
        value
            .path
            .to_str()
            .with_context(|| {
                format!(
                    "Could not convert to valid utf8 string: {}",
                    value.path.display()
                )
            })
            .map(str::to_string)
    }
}

impl Display for PathStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

impl AsRef<Path> for PathStr {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}
