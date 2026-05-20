use std::{
    borrow::Cow,
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::bail;

use crate::fs::{abs::AbsPathStr, path::PathStr};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct RelPathStr {
    pathstr: PathStr,
}

impl RelPathStr {
    pub fn new_from_pathstr(path: PathStr) -> anyhow::Result<Self> {
        // check path is relative
        if !path.path().is_relative() {
            let p = path.path().display();
            bail!("Path is not relative: {p}");
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

    pub fn join(&self, suffix: Self) -> anyhow::Result<Self> {
        PathStr::new_from_pathbuf(self.path().join(suffix.path()))?.try_into()
    }

    pub fn to_abs(&self, base: &AbsPathStr) -> anyhow::Result<AbsPathStr> {
        base.join(self)
    }

    pub fn basename(&self) -> anyhow::Result<Self> {
        self.pathstr.basename()?.try_into()
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
