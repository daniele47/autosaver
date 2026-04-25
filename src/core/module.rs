//! This module implements structs and methods to handle dotfiles modules.

use std::collections::HashSet;

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
    fn priority(&self) -> u64 {
        // Note: Lower values have higher precedence.
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

    fn resolve_with_seen(&self, base: &AbsPath, seen: &mut HashSet<AbsPath>) -> Result<Self> {
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
        Ok(Self::new(entries))
    }

    /// Resolves raw module into a list of all actual files, relative to `base` as the base directory.
    pub fn resolve(&self, base: &AbsPath) -> Result<Self> {
        self.resolve_with_seen(base, &mut Default::default())
    }

    /// Sort by path
    pub fn sort(&mut self) {
        self.entries.sort_by_cached_key(|e| e.path.to_str_lossy());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::fs::{AbsPath, RelPath};

    #[test]
    fn test_resolve() -> Result<()> {
        // Create temp directory
        let tmp = AbsPath::new_tmp("test_resolve");
        tmp.create_dir()?;

        // Create test structure
        let dir1 = tmp.joins(&["dir1"]);
        let dir2 = tmp.joins(&["dir2"]);
        let file1 = tmp.joins(&["file1.txt"]);
        let file2 = dir1.joins(&["file2.txt"]);
        let file3 = dir1.joins(&["file3.txt"]);
        let subdir = dir1.joins(&["subdir"]);
        let file4 = subdir.joins(&["file4.txt"]);

        dir1.create_dir()?;
        dir2.create_dir()?;
        subdir.create_dir()?;
        file1.create_file(false)?;
        file2.create_file(false)?;
        file3.create_file(false)?;
        file4.create_file(false)?;

        // Create module with overlapping entries
        let module = Module::new(vec![
            ModuleEntry::new(RelPath::from("dir1//"), ModulePolicy::Track),
            ModuleEntry::new(RelPath::from("dir1"), ModulePolicy::NotDiff),
            ModuleEntry::new(
                RelPath::from("dir1").joins(&["file3.txt"]),
                ModulePolicy::Track,
            ),
            ModuleEntry::new(
                RelPath::from("dir1").joins(&["subdir"]),
                ModulePolicy::Track,
            ),
            ModuleEntry::new(RelPath::from("file1.txt"), ModulePolicy::Ignore),
        ]);

        // Resolve
        let resolved = module.resolve(&tmp)?;
        for entry in resolved.entries() {
            println!("{entry:?}");
        }

        // Verify count
        assert_eq!(resolved.entries().len(), 4);

        // Verify a single entry for semplicity
        for entry in resolved.entries() {
            let path_str = entry.path().to_str_lossy();

            match path_str.as_str() {
                "file1.txt" => {
                    assert_eq!(*entry.policy(), ModulePolicy::Ignore);
                }
                _ => {}
            }
        }

        // Cleanup
        tmp.purge_path(true)?;

        Ok(())
    }
}
