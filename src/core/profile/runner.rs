//! This module implements structs and modules to handle Runner profile.

use std::collections::{HashMap, hash_map::Entry};

use crate::core::{
    error::Result,
    fs::{AbsPath, RelPath},
};

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
    pub fn resolve(&self, run_dir: &AbsPath) -> Result<Self> {
        let mut found_canon_paths = HashMap::<AbsPath, RunnerPolicy>::new();
        let mut found_ord_paths = Vec::<(RelPath, RunnerPolicy)>::new();

        for entry in self.entries() {
            // accumulate all files from each entry
            let entry_path = run_dir.join(entry.path());
            let mut files = vec![];
            if entry_path.metadata().is_ok_and(|f| f.is_file()) {
                files.push(entry_path);
            } else if entry_path.metadata().is_ok_and(|f| f.is_dir()) {
                // BTreeSet are always sorted automagically
                files.extend(entry_path.all_files(AbsPath::FILTER_FILES)?);
            }

            // operate on every single file individually
            for file in files {
                let canon = file.canonicalize()?;
                match found_canon_paths.entry(canon) {
                    Entry::Occupied(mut e) => {
                        if entry.policy.priority() < e.get().priority() {
                            e.insert(entry.policy);
                            found_ord_paths.push((file.to_relative(run_dir)?, entry.policy));
                        }
                    }
                    Entry::Vacant(e) => {
                        e.insert(entry.policy);
                        found_ord_paths.push((file.to_relative(run_dir)?, entry.policy));
                    }
                }
            }
        }

        // remove duplicates with wrong policy
        let mut entries = vec![];
        for found_ord_path in found_ord_paths {
            let canon = run_dir.join(&found_ord_path.0).canonicalize()?;
            if found_canon_paths.get(&canon) == Some(&found_ord_path.1) {
                entries.push(RunnerEntry::new(found_ord_path.0, found_ord_path.1));
            }
        }
        Ok(Self::new(entries))
    }
}
