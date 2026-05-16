use crate::fs::rel::RelPathStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunnerPolicy {
    Skip,
    Run,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnerEntry {
    path: RelPathStr,
    policy: RunnerPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Runner {
    entries: Vec<RunnerEntry>,
}

impl RunnerEntry {
    pub fn new(path: RelPathStr, policy: RunnerPolicy) -> Self {
        Self { path, policy }
    }

    pub fn path(&self) -> &RelPathStr {
        &self.path
    }

    pub fn policy(&self) -> &RunnerPolicy {
        &self.policy
    }
}

impl Runner {
    pub fn new(entries: Vec<RunnerEntry>) -> Self {
        Self { entries }
    }

    pub fn entries(&self) -> &[RunnerEntry] {
        &self.entries
    }
}
