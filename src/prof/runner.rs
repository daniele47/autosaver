use crate::fs::rel::RelPathStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunnerPolicy {
    Exclude,
    Include,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnerEntry {
    pub path: RelPathStr,
    pub policy: RunnerPolicy,
    pub stdin: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Runner {
    pub entries: Vec<RunnerEntry>,
}
