use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Result, bail};

use crate::fs::path::PathStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelPathStr {
    path: PathStr,
}

impl RelPathStr {
    pub fn new(path: PathStr) -> Result<Self> {
        // check path is relative
        if !path.as_ref().is_relative() {
            let p = path.as_ref().display();
            bail!("Path is not relative: {p}");
        } else {
            Ok(Self { path })
        }
    }
}

// CONVERT INTO
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

// CONVERT FROM
impl From<RelPathStr> for PathStr {
    fn from(value: RelPathStr) -> Self {
        value.path
    }
}
impl TryFrom<RelPathStr> for String {
    type Error = anyhow::Error;

    fn try_from(value: RelPathStr) -> std::prelude::v1::Result<Self, Self::Error> {
        value.path.try_into()
    }
}
impl From<RelPathStr> for PathBuf {
    fn from(value: RelPathStr) -> Self {
        value.path.into()
    }
}
impl AsRef<Path> for RelPathStr {
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}
