//! This module implements structs and methods to handle dotfiles modules.

use std::collections::{HashMap, hash_map::Entry};

use crate::core::{
    errors::Result,
    fs::{AbsPath, RelPath},
};

/// Policy to use for module entries.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ModulePolicy {
    /// Check always both if file doesn't exist and if file differs.
    #[default]
    Track,
    /// Only check if file doesn't exists.
    NotDiff,
    /// Ignore file entirely.
    Ignore,
}

/// Represents a single module entry, aka a path and its policy.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModuleEntry {
    path: RelPath,
    policy: ModulePolicy,
}

/// Represents a module, aka an orderer list of path with their policies.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Module {
    entries: Vec<ModuleEntry>,
}

impl ModulePolicy {
    /// Get priority for `ModulePriority`.
    ///
    /// Note: Lower values have higher precedence.
    pub fn priority(&self) -> u64 {
        match self {
            ModulePolicy::Track => 2,
            ModulePolicy::NotDiff => 1,
            ModulePolicy::Ignore => 0,
        }
    }
}

impl ModuleEntry {
    /// Create new entry.
    pub fn new(path: RelPath, policy: ModulePolicy) -> Self {
        Self { path, policy }
    }

    /// Get path.
    pub fn path(&self) -> &RelPath {
        &self.path
    }

    /// Get policy.
    pub fn policy(&self) -> &ModulePolicy {
        &self.policy
    }
}

impl Module {
    /// Create new Module.
    pub fn new(entries: Vec<ModuleEntry>) -> Self {
        Self { entries }
    }

    /// Get all entries.
    pub fn entries(&self) -> &[ModuleEntry] {
        &self.entries
    }

    // Remove duplicates taking only the one with highest policy priority.
    fn cleanup(&self) -> Result<Self> {
        let mut paths: HashMap<String, ModulePolicy> = Default::default();
        for entry in &self.entries {
            let path_str = String::try_from(entry.path.clone())?;
            match paths.entry(path_str) {
                Entry::Vacant(vacant) => {
                    vacant.insert(entry.policy);
                }
                Entry::Occupied(mut occupied) => {
                    if occupied.get().priority() < entry.policy.priority() {
                        occupied.insert(entry.policy);
                    }
                }
            }
        }
        let entries: Vec<_> = paths
            .iter()
            .map(|f| ModuleEntry::new(RelPath::from(f.0.as_str()), *f.1))
            .collect();
        Ok(Module::new(entries))
    }

    /// Merge two modules into one, and properly cleans duplicates.
    pub fn merge(&self, other: &Self) -> Result<Self> {
        let mut entries = vec![];
        entries.extend(self.entries.clone());
        entries.extend(other.entries.clone());
        Self::new(entries).cleanup()
    }

    /// Cleanup and resolves all entries to actual normal files.
    ///
    /// In particular it removes duplicated entries, it leaves only those with most policy
    /// priority, it gets all files from directories and so on.
    ///
    /// Note: base specifies the prefix to use for all entries
    pub fn resolve(&self, base: &AbsPath) -> Result<Self> {
        let mut entries = vec![];
        for raw_entry in &self.entries {
            let raw_abs_path = raw_entry.path.to_absolute(base);
            if raw_abs_path.exists() {
                let metadata = raw_abs_path.metadata().unwrap();
                let mut files = vec![];
                if metadata.is_dir() {
                    files.extend(raw_abs_path.all_files()?);
                } else if metadata.is_file() {
                    files.push(raw_abs_path);
                }
                entries.extend(
                    files
                        .iter()
                        .filter(|f| f.metadata().unwrap().is_file())
                        .map(|f| f.to_relative(base).unwrap())
                        .map(|f| ModuleEntry::new(f, raw_entry.policy)),
                );
            }
        }
        Self::new(entries).cleanup()
    }
}
