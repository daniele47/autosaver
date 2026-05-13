use std::{fmt::Display, str::FromStr};

use derive_getters::{Dissolve, Getters};

use crate::fs::path::PathStr;

#[derive(Debug, Clone, PartialEq, Eq, Getters, Dissolve)]
pub struct RelPathStr {
    path: PathStr,
}

impl RelPathStr {
    pub fn new(path: PathStr) -> Result<Self> {
        // check path doesn't contain parent directory
        if path.components().any(|c| c == Component::ParentDir) {
            bail!("Path string contains parent directory: {}", path.display());
        }

        Ok(Self { path })
    }
}

// &str ---> [RelPathStr]
impl FromStr for RelPathStr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Self::new(PathStr::from_str(s)?)
    }
}

// [RelPathStr] ---> {}
impl Display for RelPathStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}
