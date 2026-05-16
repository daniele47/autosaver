use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, bail};
use tracing::instrument;

use crate::fs::{abs::AbsPathStr, path::PathStr};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct RelPathStr {
    pathstr: PathStr,
}

impl RelPathStr {
    pub(super) fn new_from_pathstr(path: PathStr) -> anyhow::Result<Self> {
        // check path is relative
        if !path.path().is_relative() {
            let p = path.path().display();
            bail!("Path is not relative: {p}");
        } else {
            Ok(Self { pathstr: path })
        }
    }

    pub(super) fn new_from_pathbuf(path: PathBuf) -> anyhow::Result<Self> {
        Self::new_from_pathstr(PathStr::new_from_pathbuf(path)?)
    }

    pub(super) fn path(&self) -> &Path {
        self.pathstr.path()
    }

    pub fn new(path: String) -> anyhow::Result<Self> {
        Self::new_from_pathbuf(path.into())
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
    pub fn join(&self, suffix: Self) -> anyhow::Result<Self> {
        PathStr::new_from_pathbuf(self.path().join(suffix.path()))?.try_into()
    }

    #[instrument(ret, err, level = "trace", skip_all, fields(self= %self.display(), base=%base.display()))]
    pub fn to_abs(&self, base: &AbsPathStr) -> anyhow::Result<AbsPathStr> {
        base.join(self)
    }

    #[instrument(ret, err, level = "trace", skip_all, fields(self= %self.display()))]
    pub fn basename(&self) -> anyhow::Result<Self> {
        self.path()
            .file_name()
            .map(|f| Self::new_from_pathbuf(PathBuf::from(f)))
            .with_context(|| format!("Could not get basename of {}", self.display()))?
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

    fn try_from(value: PathStr) -> Result<Self, Self::Error> {
        Self::new_from_pathstr(value)
    }
}
impl TryFrom<String> for RelPathStr {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl FromStr for RelPathStr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.into())
    }
}
