use std::{
    borrow::Cow,
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, bail};

use crate::fs::{path::PathStr, rel::RelPathStr};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AbsPathStr {
    pathstr: PathStr,
}

impl AbsPathStr {
    pub fn new_from_pathstr(path: PathStr) -> anyhow::Result<Self> {
        // check path is relative
        if !path.path().is_absolute() {
            let p = path.path().display();
            bail!("Path is not absolute: {p}");
        } else {
            Ok(Self { pathstr: path })
        }
    }

    pub fn new_from_pathbuf(path: PathBuf) -> anyhow::Result<Self> {
        Self::new_from_pathstr(PathStr::new_from_pathbuf(path)?)
    }

    pub fn path(&self) -> &Path {
        self.pathstr.path()
    }

    pub fn new(path: String) -> anyhow::Result<Self> {
        Self::new_from_pathbuf(path.into())
    }

    pub fn to_str(&self) -> Option<&str> {
        self.pathstr.to_str()
    }

    pub fn to_string_lossy<'a>(&'a self) -> Cow<'a, str> {
        self.pathstr.to_string_lossy()
    }

    pub fn display(&self) -> impl Display {
        self.path().display()
    }

    pub fn join(&self, suffix: &RelPathStr) -> anyhow::Result<Self> {
        Self::new_from_pathbuf(self.path().join(suffix.path()))
    }

    pub fn to_rel(&self, base: &Self) -> anyhow::Result<RelPathStr> {
        let stripped = self.path().strip_prefix(base.path()).with_context(|| {
            let p = self.display();
            let b = base.display();
            format!("Could not get relative path for {p} with base {b}")
        })?;
        RelPathStr::new_from_pathbuf(stripped.into())
    }

    pub fn basename(&self) -> anyhow::Result<Self> {
        self.pathstr.basename()?.try_into()
    }

    pub fn canonicalize(&self) -> anyhow::Result<Self> {
        AbsPathStr::new_from_pathbuf(
            self.path()
                .canonicalize()
                .with_context(|| format!("Failed to canonicalize path {}", self.display()))?,
        )
    }

    pub fn is_file(&self) -> bool {
        self.path().is_file()
    }

    pub fn is_dir(&self) -> bool {
        self.path().is_dir()
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

impl From<AbsPathStr> for PathStr {
    fn from(value: AbsPathStr) -> Self {
        value.pathstr
    }
}
impl AsRef<PathStr> for AbsPathStr {
    fn as_ref(&self) -> &PathStr {
        &self.pathstr
    }
}
