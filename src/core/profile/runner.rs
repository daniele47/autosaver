//! This module implements structs and modules to handle Runner profile.

use crate::core::fs::{AbsPath, RelPath};

/// Policy for runner entries.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum RunnerPolicy {
    /// Run files with this policy.
    #[default]
    Run,
    /// Do not run files with this policy.
    Skip,
}

/// Represent a single entry of a runner profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnerEntry {
    path: RelPath,
    policy: RunnerPolicy,
}

/// Represents the runner profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Runner {
    entries: Vec<RunnerEntry>,
}

impl RunnerPolicy {
    fn priority(&self) -> u64 {
        // Note: Lower values have higher precedence.
        match self {
            RunnerPolicy::Run => 1,
            RunnerPolicy::Skip => 0,
        }
    }
}

impl RunnerEntry {
    /// Create new runner entry.
    pub fn new(path: RelPath, policy: RunnerPolicy) -> Self {
        Self { path, policy }
    }

    /// Get path.
    pub fn path(&self) -> &RelPath {
        &self.path
    }

    /// Get policy.
    pub fn policy(&self) -> &RunnerPolicy {
        &self.policy
    }
}

impl Runner {
    /// Create new Runner.
    pub fn new(entries: Vec<RunnerEntry>) -> Self {
        Self { entries }
    }

    /// Create new empty Runner. Useful for tests.
    pub fn empty() -> Self {
        Self::new(vec![])
    }

    /// Get entries.
    pub fn entries(&self) -> &[RunnerEntry] {
        &self.entries
    }

    /// Add a new entry.
    pub fn add_entry(&mut self, entry: RunnerEntry) {
        self.entries.push(entry);
    }

    /// Resolve a raw runner profile into one with a list of all files.
    ///
    /// Note: this is guaranteed to be orderer in the following way:
    /// - in the exact same way files appeared in the config file
    /// - directories are resolved to all files inside, orderered alphabetically
    pub fn resolve(&self, run_dir: &AbsPath) -> Self {
        todo!()
    }
}
