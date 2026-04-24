//! This module implements structs and methods to handle dotfiles modules.

use crate::core::fs::RelPath;

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
}
