use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Result, bail};
use derive_getters::{Dissolve, Getters};

use crate::fs::path::PathStr;

#[derive(Debug, Clone, PartialEq, Eq, Getters, Dissolve)]
pub struct AbsPathStr {
    path: PathStr,
}

impl AbsPathStr {
    pub fn new(path: PathStr) -> Result<Self> {
        // check path is relative
        if !path.as_ref().is_absolute() {
            let p = path.as_ref().display();
            bail!("Path is not absolute: {p}");
        } else {
            Ok(Self { path })
        }
    }

    pub fn to_string_lossy(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    pub fn join(&self, suffix: Self) -> Result<Self> {
        self.path.path().join(suffix.path).try_into()
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
        value.path
    }
}
impl TryFrom<AbsPathStr> for String {
    type Error = anyhow::Error;

    fn try_from(value: AbsPathStr) -> std::prelude::v1::Result<Self, Self::Error> {
        value.path.try_into()
    }
}
impl From<AbsPathStr> for PathBuf {
    fn from(value: AbsPathStr) -> Self {
        value.path.into()
    }
}
impl AsRef<Path> for AbsPathStr {
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}
