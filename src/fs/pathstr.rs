use std::{
    fmt::Display,
    path::{Component, Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, Result, bail};
use derive_getters::{Dissolve, Getters};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathKind {
    Absolute,
    Relative,
}

#[derive(Debug, Clone, PartialEq, Eq, Getters, Dissolve)]
pub struct PathStr {
    path: PathBuf,
    kind: PathKind,
}

impl PathStr {
    // CONSTRUCTORS
    pub fn new(path: PathBuf) -> Result<Self> {
        let kind;

        // check path doesn't contain parent directory
        if path.components().any(|c| c == Component::ParentDir) {
            let p = path.display();
            bail!("Path string contains parent directory: {p}");
        }

        // get kind
        if path.is_absolute() {
            kind = PathKind::Absolute;
        } else if path.is_relative() {
            kind = PathKind::Relative;
        } else {
            unreachable!("Path string must be absolute or relative!")
        }

        Ok(Self { path, kind })
    }

    pub fn new_with_kind(path: PathBuf, kind: PathKind) -> Result<Self> {
        let res = Self::new(path)?;

        if res.kind != kind {
            let p = res.path.display();
            bail!("Path string {p} is not of kind {kind:?}")
        }

        Ok(res)
    }

    // IO FUNCTION WRAPPERS

    pub fn canonicalize(&self) -> Result<Self> {
        match self.kind {
            PathKind::Absolute => {
                let canon = self.path.canonicalize().with_context(|| {
                    format!("Cannot canonicalize path: {}", self.path.display())
                })?;
                Self::new_with_kind(canon, PathKind::Absolute)
            }
            PathKind::Relative => {
                let p = self.path.display();
                bail!("Cannot canonicalize relative path: {p}")
            }
        }
    }

    pub fn file_name(&self) -> Result<Self> {
        Self::new_with_kind(
            self.path
                .file_name()
                .with_context(|| format!("Could not get basename of: {}", self.path.display()))?
                .into(),
            PathKind::Relative,
        )
    }

    pub fn join(&self, suffix: Self) -> Result<Self> {
        Self::new(self.path.join(suffix))
    }
}

// TRAIT IMPLEMENTATIONS

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
