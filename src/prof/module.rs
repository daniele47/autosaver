use std::collections::BTreeSet;

use crate::fs::rel::RelPathStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModulePolicy {
    Exclude,
    NotDiff,
    Include,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleEntry {
    pub path: RelPathStr,
    pub policy: ModulePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub entries: Vec<ModuleEntry>,
    pub cleanup: BTreeSet<RelPathStr>,
}
