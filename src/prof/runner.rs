use crate::fs::rel::RelPathStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunnerPolicy {
    Exclude,
    Include,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnerEntry {
    path: RelPathStr,
    policy: RunnerPolicy,
    stdin: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Runner {
    entries: Vec<RunnerEntry>,
}

impl RunnerEntry {
    pub fn new(path: RelPathStr, policy: RunnerPolicy, stdin: bool) -> Self {
        Self {
            path,
            policy,
            stdin,
        }
    }

    pub fn path(&self) -> &RelPathStr {
        &self.path
    }

    pub fn policy(&self) -> &RunnerPolicy {
        &self.policy
    }

    pub fn stdin(&self) -> bool {
        self.stdin
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
