use std::{fmt::Display, path::PathBuf, str::FromStr};

use anyhow::{Result, bail};

use crate::fs::path::PathStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelPathStr {
    path: PathStr,
}

impl RelPathStr {
    pub fn new(path: PathStr) -> Result<Self> {
        // check path is relative
        if path.as_ref().is_relative() {
            let p = path.as_ref().display();
            bail!("Path is not relative: {p}");
        } else {
            Ok(Self { path })
        }
    }
}

impl TryFrom<PathStr> for RelPathStr {
    type Error = anyhow::Error;

    fn try_from(value: PathStr) -> std::prelude::v1::Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl TryFrom<String> for RelPathStr {
    type Error = anyhow::Error;

    fn try_from(path: String) -> std::prelude::v1::Result<Self, Self::Error> {
        Self::new(path.try_into()?)
    }
}
impl TryFrom<PathBuf> for RelPathStr {
    type Error = anyhow::Error;

    fn try_from(path: PathBuf) -> std::prelude::v1::Result<Self, Self::Error> {
        Self::new(path.try_into()?)
    }
}

impl TryFrom<RelPathStr> for PathStr {
    type Error = anyhow::Error;

    fn try_from(value: RelPathStr) -> std::prelude::v1::Result<Self, Self::Error> {
        todo!()
    }
}
impl TryFrom<RelPathStr> for String {
    type Error = anyhow::Error;

    fn try_from(value: RelPathStr) -> std::prelude::v1::Result<Self, Self::Error> {
        todo!()
    }
}
impl From<RelPathStr> for PathBuf {
    fn from(value: PathBuf) -> Self {
        todo!()
    }
}

// impl FromStr for RelPathStr {
//     type Err = anyhow::Error;
//
//     fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
//         Self::new(PathStr::from_str(s)?)
//     }
// }
// impl AsRef<Path> for RelPathStr {
//     fn as_ref(&self) -> &Path {
//         &self.path
//     }
// }
