use std::{
    fmt::Display,
    path::{Component, Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, Result, bail};
use derive_getters::Getters;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathKind {
    Absolute,
    Relative,
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct PathStr {
    path: PathBuf,
    kind: PathKind,
}

impl PathStr {
    // CONSTRUCTORS

    pub fn new(pathstr: impl AsRef<Path>) -> Result<Self> {
        let pathstr = pathstr.as_ref();
        let kind;

        // check path doesn't contain parent directory
        if pathstr.components().any(|c| c == Component::ParentDir) {
            let p = pathstr.display();
            bail!("Path string contains parent directory: {p}");
        }

        // get kind
        if pathstr.is_absolute() {
            kind = PathKind::Absolute;
        } else if pathstr.is_relative() {
            kind = PathKind::Relative;
        } else {
            unreachable!("Path string must be absolute or relative!")
        }

        Ok(Self {
            path: pathstr.to_path_buf(),
            kind,
        })
    }

    pub fn new_with_kind(pathstr: impl AsRef<Path>, kind: PathKind) -> Result<Self> {
        let res = Self::new(pathstr)?;

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
                .with_context(|| format!("Could not get basename of: {}", self.path.display()))?,
            PathKind::Relative,
        )
    }
}

// TRAIT IMPLEMENTATIONS

impl FromStr for PathStr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Self::new(s)
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
