use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Result, bail};
use tracing::instrument;

use crate::fs::{abs::AbsPathStr, path::PathStr};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct RelPathStr {
    pathstr: PathStr,
}

impl RelPathStr {
    pub fn new(path: PathStr) -> Result<Self> {
        // check path is relative
        if !path.path().is_relative() {
            let p = path.path().display();
            bail!("Path is not relative: {p}");
        } else {
            Ok(Self { pathstr: path })
        }
    }

    pub(super) fn path(&self) -> &Path {
        self.pathstr.path()
    }

    pub fn to_str(&self) -> Option<&str> {
        self.pathstr.to_str()
    }

    pub fn to_string_lossy(&self) -> String {
        self.pathstr.to_string_lossy()
    }

    pub fn display(&self) -> impl Display {
        self.path().display()
    }

    #[instrument(ret, err, level = "trace", skip_all, fields(self= %self.display(), suffix=%suffix.display()))]
    pub fn join(&self, suffix: Self) -> Result<Self> {
        self.path().join(suffix.path()).try_into()
    }

    #[instrument(ret, err, level = "trace", skip_all, fields(self= %self.display(), base=%base.display()))]
    pub fn to_abs(&self, base: &AbsPathStr) -> Result<AbsPathStr> {
        base.join(self)
    }

    #[instrument(ret, err, level = "trace", skip_all, fields(self= %self.display()))]
    pub fn basename(&self) -> Result<Self> {
        self.pathstr.basename()?.try_into()
    }

    #[instrument(ret, level = "trace", skip_all, fields(self= %self.display(), base=%base.display()))]
    pub fn is_inside(&self, base: &AbsPathStr) -> bool {
        if let Ok(abs) = self.to_abs(base) {
            abs.is_inside(base)
        } else {
            false
        }
    }
}

impl TryFrom<PathStr> for RelPathStr {
    type Error = anyhow::Error;

    fn try_from(value: PathStr) -> std::prelude::v1::Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl TryFrom<PathBuf> for RelPathStr {
    type Error = anyhow::Error;

    fn try_from(path: PathBuf) -> std::prelude::v1::Result<Self, Self::Error> {
        Self::new(path.try_into()?)
    }
}
impl TryFrom<String> for RelPathStr {
    type Error = anyhow::Error;

    fn try_from(path: String) -> std::prelude::v1::Result<Self, Self::Error> {
        Self::new(path.try_into()?)
    }
}
impl FromStr for RelPathStr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Self::new(PathStr::from_str(s)?)
    }
}
impl TryFrom<&Path> for RelPathStr {
    type Error = anyhow::Error;

    fn try_from(value: &Path) -> std::prelude::v1::Result<Self, Self::Error> {
        PathStr::try_from(value)?.try_into()
    }
}

impl AsRef<Path> for RelPathStr {
    fn as_ref(&self) -> &Path {
        self.path()
    }
}
