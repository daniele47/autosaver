use tracing::instrument;

use crate::fs::rel::RelPathStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositePolicy {
    Ignore,
    NotDiff,
    Track,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositeEntry {
    child: RelPathStr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Composite {
    entries: Vec<CompositeEntry>,
}

impl CompositeEntry {
    #[instrument(ret, level = "trace", skip_all)]
    pub fn new(child: RelPathStr) -> Self {
        Self { child }
    }

    pub fn child(&self) -> &RelPathStr {
        &self.child
    }
}

impl Composite {
    #[instrument(level = "trace", skip_all)]
    pub fn new(entries: Vec<CompositeEntry>) -> Self {
        Self { entries }
    }

    pub fn entries(&self) -> &[CompositeEntry] {
        &self.entries
    }
}
