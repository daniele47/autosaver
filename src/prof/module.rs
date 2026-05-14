use tracing::instrument;

use crate::fs::rel::RelPathStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModulePolicy {
    Ignore,
    NotDiff,
    Track,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleEntry {
    path: RelPathStr,
    policy: ModulePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    entries: Vec<ModuleEntry>,
}

impl ModuleEntry {
    #[instrument(ret, level = "trace")]
    pub fn new(path: RelPathStr, policy: ModulePolicy) -> Self {
        Self { path, policy }
    }

    pub fn path(&self) -> &RelPathStr {
        &self.path
    }

    pub fn policy(&self) -> &ModulePolicy {
        &self.policy
    }
}

impl Module {
    #[instrument(ret, level = "trace")]
    pub fn new(entries: Vec<ModuleEntry>) -> Self {
        Self { entries }
    }

    pub fn entries(&self) -> &[ModuleEntry] {
        &self.entries
    }
}
