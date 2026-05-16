use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, Result, bail};
use tracing::instrument;

use crate::fs::{path::PathStr, rel::RelPathStr};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AbsPathStr {
    pathstr: PathStr,
}

impl AbsPathStr {
    pub fn new(path: PathStr) -> Result<Self> {
        // check path is relative
        if !path.path().is_absolute() {
            let p = path.path().display();
            bail!("Path is not absolute: {p}");
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

    #[instrument(ret, err, level = "trace", skip_all, fields(self=%self.display(), suffix=%suffix.display()))]
    pub fn join(&self, suffix: &RelPathStr) -> Result<Self> {
        self.path().join(suffix.path()).try_into()
    }

    #[instrument(ret, err, level = "trace", skip_all, fields(self=%self.display(), base=%base.display()))]
    pub fn to_rel(&self, base: &Self) -> Result<RelPathStr> {
        let stripped = self.path().strip_prefix(base.path()).with_context(|| {
            let p = self.display();
            let b = base.display();
            format!("Could not get relative path for {p} with base {b}")
        })?;
        RelPathStr::try_from(stripped)
    }

    #[instrument(ret, err, level = "trace", skip_all, fields(self= %self.display()))]
    pub fn basename(&self) -> Result<Self> {
        self.pathstr.basename()?.try_into()
    }

    #[instrument(ret, err, level = "trace", skip_all, fields(self= %self.display()))]
    pub fn canonicalize(&self) -> Result<Self> {
        self.path()
            .canonicalize()
            .map(Self::try_from)?
            .with_context(|| format!("Failed to canonicalize {}", self.display()))
    }

    #[instrument(ret, level = "trace", skip_all, fields(self= %self.display()))]
    pub fn is_file(&self) -> bool {
        self.path().is_file()
    }

    #[instrument(ret, level = "trace", skip_all, fields(self= %self.display()))]
    pub fn is_dir(&self) -> bool {
        self.path().is_dir()
    }

    #[instrument(ret, level = "trace", skip_all, fields(self= %self.display(),base=%base.display()))]
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

impl AsRef<Path> for AbsPathStr {
    fn as_ref(&self) -> &Path {
        self.path()
    }
}
