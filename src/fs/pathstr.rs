use std::path::{Component, Path, PathBuf};

use anyhow::{Result, bail};
use derive_getters::Getters;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathKind {
    Absolute,
    Relative,
}

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct PathStr {
    path: PathBuf,
    kind: PathKind,
}

impl PathStr {
    pub fn new(pathstr: impl AsRef<Path>) -> Result<Self> {
        let pathstr = pathstr.as_ref();
        let kind;

        // check path doesn't contain parent directory
        if pathstr.components().any(|c| c == Component::ParentDir) {
            let p = pathstr.display();
            bail!("Path string contains parent directory: {p}");
        }

        // get kind
        if pathstr.is_absolute() {
            kind = PathKind::Absolute;
        } else if pathstr.is_relative() {
            kind = PathKind::Relative;
        } else {
            unreachable!("Path string must be absolute or relative!")
        }

        Ok(Self {
            path: pathstr.to_path_buf(),
            kind,
        })
    }

    pub fn new_typed(pathstr: impl AsRef<Path>, kind: PathKind) -> Result<Self> {
        let res = Self::new(pathstr)?;

        if res.kind != kind {
            let p = res.path.display();
            bail!("Path string {p} is not of kind {kind:?}")
        }

        Ok(res)
    }
}
