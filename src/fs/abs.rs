use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, Result, bail};
use tracing::instrument;

use crate::fs::{path::PathStr, rel::RelPathStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbsPathStr {
    pathstr: PathStr,
}

impl AbsPathStr {
    #[instrument(ret, err, level = "trace")]
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

    #[instrument(ret, err, level = "trace")]
    pub fn join(&self, suffix: &RelPathStr) -> Result<Self> {
        self.path().join(suffix.path()).try_into()
    }

    #[instrument(ret, err, level = "trace")]
    pub fn to_rel(&self, base: &Self) -> Result<RelPathStr> {
        let stripped = self.path().strip_prefix(base.path()).with_context(|| {
            let p = self.to_string_lossy();
            let b = base.to_string_lossy();
            format!("Could not turn get relative path for {p} with base {b}")
        })?;
        RelPathStr::try_from(PathBuf::from(stripped))
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
