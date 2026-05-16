use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, bail};
use tracing::instrument;

use crate::fs::{path::PathStr, rel::RelPathStr};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AbsPathStr {
    pathstr: PathStr,
}

impl AbsPathStr {
    pub(super) fn new_from_pathstr(path: PathStr) -> anyhow::Result<Self> {
        // check path is relative
        if !path.path().is_absolute() {
            let p = path.path().display();
            bail!("Path is not absolute: {p}");
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

    #[instrument(ret, err, level = "trace", skip_all, fields(self=%self.display(), suffix=%suffix.display()))]
    pub fn join(&self, suffix: &RelPathStr) -> anyhow::Result<Self> {
        Self::new_from_pathbuf(self.path().join(suffix.path()))
    }

    #[instrument(ret, err, level = "trace", skip_all, fields(self=%self.display(), base=%base.display()))]
    pub fn to_rel(&self, base: &Self) -> anyhow::Result<RelPathStr> {
        let stripped = self.path().strip_prefix(base.path()).with_context(|| {
            let p = self.display();
            let b = base.display();
            format!("Could not get relative path for {p} with base {b}")
        })?;
        RelPathStr::new_from_pathbuf(stripped.into())
    }

    #[instrument(ret, err, level = "trace", skip_all, fields(self= %self.display()))]
    pub fn basename(&self) -> anyhow::Result<Self> {
        self.path()
            .file_name()
            .map(|f| Self::new_from_pathbuf(PathBuf::from(f)))
            .with_context(|| format!("Could not get basename of {}", self.display()))?
    }

    #[instrument(ret, err, level = "trace", skip_all, fields(self= %self.display()))]
    pub fn canonicalize(&self) -> anyhow::Result<Self> {
        self.path()
            .canonicalize()
            .map(Self::new_from_pathbuf)?
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

    fn try_from(value: PathStr) -> Result<Self, Self::Error> {
        Self::new_from_pathstr(value)
    }
}
impl TryFrom<String> for AbsPathStr {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl FromStr for AbsPathStr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.into())
    }
}
