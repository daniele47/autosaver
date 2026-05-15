use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, Result, bail};

use crate::fs::{path::PathStr, rel::RelPathStr};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AbsPathStr {
    pathstr: PathStr,
}

impl AbsPathStr {
    pub fn new(path: PathStr) -> Result<Self> {
        // check path is relative
        if !path.as_ref().is_absolute() {
            let p = path.as_ref().display();
            bail!("Path is not absolute: {p}");
        } else {
            Ok(Self { pathstr: path })
        }
    }

    pub fn path(&self) -> &Path {
        self.pathstr.as_ref()
    }

    pub fn to_string_lossy(&self) -> String {
        self.pathstr.to_string_lossy()
    }

    pub fn join(&self, suffix: &RelPathStr) -> Result<Self> {
        self.path().join(suffix.path()).try_into()
    }

    pub fn to_rel(&self, base: &Self) -> Result<RelPathStr> {
        let stripped = self.path().strip_prefix(base.path()).with_context(|| {
            let p = self.to_string_lossy();
            let b = base.to_string_lossy();
            format!("Could not get relative path for {p} with base {b}")
        })?;
        RelPathStr::try_from(stripped)
    }

    pub fn basename(&self) -> Result<Self> {
        self.pathstr.basename()?.try_into()
    }

    pub fn canonicalize(&self) -> Result<Self> {
        self.path()
            .canonicalize()
            .map(Self::try_from)?
            .with_context(|| format!("Failed to canonicalize {}", self.to_string_lossy()))
    }

    pub fn is_file(&self) -> bool {
        self.path().is_file()
    }

    pub fn is_dir(&self) -> bool {
        self.path().is_dir()
    }

    pub fn is_inside(&self, base: &Self) -> bool {
        if let Ok(self_canon) = self.canonicalize()
            && let Ok(dir_canon) = base.canonicalize()
        {
            self_canon.path().starts_with(dir_canon.path())
        } else {
            false
        }
    }
}

// CONVERT INTO
impl TryFrom<PathStr> for AbsPathStr {
    type Error = anyhow::Error;

    fn try_from(value: PathStr) -> std::prelude::v1::Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl TryFrom<PathBuf> for AbsPathStr {
    type Error = anyhow::Error;

    fn try_from(path: PathBuf) -> std::prelude::v1::Result<Self, Self::Error> {
        Self::new(path.try_into()?)
    }
}
impl TryFrom<String> for AbsPathStr {
    type Error = anyhow::Error;

    fn try_from(path: String) -> std::prelude::v1::Result<Self, Self::Error> {
        Self::new(path.try_into()?)
    }
}
impl FromStr for AbsPathStr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Self::new(PathStr::from_str(s)?)
    }
}
impl TryFrom<&Path> for AbsPathStr {
    type Error = anyhow::Error;

    fn try_from(value: &Path) -> std::prelude::v1::Result<Self, Self::Error> {
        PathStr::try_from(value)?.try_into()
    }
}

// CONVERT FROM
impl From<AbsPathStr> for PathStr {
    fn from(value: AbsPathStr) -> Self {
        value.pathstr
    }
}
impl TryFrom<AbsPathStr> for String {
    type Error = anyhow::Error;

    fn try_from(value: AbsPathStr) -> std::prelude::v1::Result<Self, Self::Error> {
        value.pathstr.try_into()
    }
}
impl From<AbsPathStr> for PathBuf {
    fn from(value: AbsPathStr) -> Self {
        value.pathstr.into()
    }
}
impl AsRef<Path> for AbsPathStr {
    fn as_ref(&self) -> &Path {
        self.path()
    }
}
