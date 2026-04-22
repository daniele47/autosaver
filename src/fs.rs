use std::path::PathBuf;

#[derive(Debug)]
pub struct AbsPath {
    path: PathBuf,
}

#[derive(Debug)]
pub struct RelPath {
    path: PathBuf,
}

impl From<PathBuf> for AbsPath {
    fn from(value: PathBuf) -> Self {
        Self { path: value }
    }
}

impl From<AbsPath> for PathBuf {
    fn from(value: AbsPath) -> Self {
        value.path
    }
}

impl From<PathBuf> for RelPath {
    fn from(value: PathBuf) -> Self {
        Self { path: value }
    }
}

impl From<RelPath> for PathBuf {
    fn from(value: RelPath) -> Self {
        value.path
    }
}
