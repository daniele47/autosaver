use std::{
    fmt::Display,
    path::{Component, Path, PathBuf},
    str::FromStr,
};

use anyhow::bail;
use internment::Intern;
use tracing::trace;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PathStr {
    path: Intern<PathBuf>,
}

impl PathStr {
    pub(super) fn new_from_pathbuf(path: PathBuf) -> anyhow::Result<Self> {
        // check path contains invalid components
        if !Intern::<PathBuf>::is_interned(&path) {
            for component in path.components() {
                if component == Component::ParentDir {
                    bail!("Path contains parent directory: {}", path.display());
                } else if component == Component::CurDir {
                    bail!("Path contains current directory: {}", path.display());
                }
            }
            let interned = Intern::<PathBuf>::num_objects_interned() + 1;
            trace!(path=%path.display(),interned=%interned, "Interned new path:");
        } else {
            trace!(path=%path.display(), "Path was already interned:");
        }
        Ok(Self {
            path: Intern::new(path),
        })
    }

    pub(super) fn path(&self) -> &Path {
        &self.path
    }

    pub fn new(path: String) -> anyhow::Result<Self> {
        Self::new_from_pathbuf(path.into())
    }

    pub fn to_str(&self) -> Option<&str> {
        self.path().to_str()
    }

    pub fn to_string_lossy(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    pub fn display(&self) -> impl Display {
        self.path().display()
    }
}

impl TryFrom<String> for PathStr {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
impl FromStr for PathStr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.into())
    }
}
