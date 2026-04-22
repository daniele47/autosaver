use std::path::PathBuf;

#[derive(Debug)]
pub struct AbsPath(PathBuf);

#[derive(Debug)]
pub struct RelPath(PathBuf);

impl From<PathBuf> for AbsPath {
    fn from(value: PathBuf) -> Self {
        Self(value)
    }
}

impl From<AbsPath> for PathBuf {
    fn from(value: AbsPath) -> Self {
        value.0
    }
}

impl From<PathBuf> for RelPath {
    fn from(value: PathBuf) -> Self {
        Self(value)
    }
}

impl From<RelPath> for PathBuf {
    fn from(value: RelPath) -> Self {
        value.0
    }
}
