//! This module implements structs and methods to handle autosaver modules.

use std::collections::{HashMap, HashSet, hash_map::Entry};

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
    pub fn policy(&self) -> ModulePolicy {
        self.policy
    }
}

impl Module {
    /// Create new Module.
    pub fn new(entries: Vec<ModuleEntry>) -> Self {
        Self { entries }
    }

    /// Create new empty Module. Useful for tests
    pub fn empty() -> Self {
        Self::new(vec![])
    }

    /// Get all entries.
    pub fn entries(&self) -> &[ModuleEntry] {
        &self.entries
    }

    fn cleanup_paths(paths: Vec<(AbsPath, AbsPath, ModulePolicy)>) -> Result<Self> {
        // Note: first abspath is the full path, second is the path prefix!
        let mut values = HashMap::<String, (AbsPath, AbsPath, ModulePolicy)>::new();
        let mut entries = vec![];
        let mut entries_unique = HashSet::<String>::new();

        // make sure files are unique BASED on canonicalized path
        for path in paths {
            let path_str = String::try_from(path.0.canonicalize()?)?;
            match values.entry(path_str) {
                Entry::Occupied(mut entry) => {
                    let old = entry.get();
                    if path.2.priority() < old.2.priority() {
                        entry.insert(path);
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(path);
                }
            }
        }

        // collect into proper result type, removing pure entry duplicates
        for (_, (path, base, policy)) in values {
            let entry = ModuleEntry::new(path.to_relative(&base)?, policy);
            if entries_unique.insert(String::try_from(entry.path().clone())?) {
                entries.push(entry);
            }
        }

        Ok(Self::new(entries))
    }

    fn resolve_module(&self, base: &AbsPath) -> Result<Vec<(AbsPath, AbsPath, ModulePolicy)>> {
        let mut paths = vec![];
        for raw_entry in &self.entries {
            let raw_abs_path = raw_entry.path.to_absolute(base);
            if raw_abs_path.exists() {
                let metadata = raw_abs_path.metadata()?;
                let mut files = vec![];

                // if path is directory, collect all files within the directory
                if metadata.is_dir() {
                    for f in raw_abs_path.all_files(AbsPath::FILTER_FILES)? {
                        files.push((f, base.clone(), raw_entry.policy));
                    }
                }
                // if path is a file, collect the file itself only
                else if metadata.is_file() {
                    files.push((raw_abs_path, base.clone(), raw_entry.policy()));
                }
                paths.extend(files);
            }
        }
        Ok(paths)
    }

    /// Resolves raw module into a list of all actual files, relative to `base` as the base directory.
    ///
    /// This guarantees the result will be sorted based on lossy path string!
    pub fn resolve(&self, base: &AbsPath) -> Result<Self> {
        let mut res = Self::cleanup_paths(self.resolve_module(base)?)?;
        res.sort();
        Ok(res)
    }

    /// Takes a module, and resolve it based on if at least one base has the file present.
    ///
    /// This guarantees the result will be sorted based on lossy path string!
    pub fn merge_bases(&self, base: &AbsPath, oth_base: &AbsPath) -> Result<Self> {
        let mut tmp = self.resolve_module(base)?;
        tmp.extend(self.resolve_module(oth_base)?);
        let mut res = Self::cleanup_paths(tmp)?;
        res.sort();
        Ok(res)
    }

    fn sort(&mut self) {
        self.entries.sort_by_cached_key(|e| e.path.to_str_lossy());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::fs::{AbsPath, RelPath};

    fn purge_path_even_on_panic(tmpdir: &AbsPath) -> impl Drop {
        struct Guard(AbsPath);
        impl Drop for Guard {
            fn drop(&mut self) {
                let _ = self.0.purge_path(true);
            }
        }
        Guard(tmpdir.clone())
    }

    #[test]
    fn test_resolve() -> Result<()> {
        // Create temp directory
        let tmp = AbsPath::new_tmp("test_resolve");
        tmp.create_dir()?;
        let _guard = purge_path_even_on_panic(&tmp);

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

        // Verify count
        assert_eq!(resolved.entries().len(), 4);

        // Verify a single entry for semplicity
        for entry in resolved.entries() {
            let path_str = entry.path().to_str_lossy();

            match path_str.as_str() {
                "file1.txt" => {
                    assert_eq!(entry.policy(), ModulePolicy::Ignore);
                }
                _ => {}
            }
        }

        // Cleanup
        tmp.purge_path(true)?;
        Ok(())
    }

    #[test]
    fn test_merge_bases() -> Result<()> {
        // Create temp directory
        let tmp = AbsPath::new_tmp("test_merge_bases");
        tmp.create_dir()?;
        let _guard = purge_path_even_on_panic(&tmp);

        // Create directories and files
        let base1 = tmp.joins(&["home"]);
        let base2 = tmp.joins(&["backup"]);
        let file1_base1 = base1.joins(&["only_in_home.txt"]);
        let file2_base1 = base1.joins(&["in_both.txt"]);
        let subdir_base1 = base1.joins(&["subdir"]);
        let file3_base1 = subdir_base1.joins(&["nested_home.txt"]);
        let file4_base1 = subdir_base1.joins(&["nested2.txt"]);
        let file1_base2 = base2.joins(&["only_in_backup.txt"]);
        let file2_base2 = base2.joins(&["in_both.txt"]);
        let subdir_base2 = base2.joins(&["subdir"]);
        let file3_base2 = subdir_base2.joins(&["nested_backup.txt"]);
        let file4_base2 = subdir_base2.joins(&["nested2.txt"]);
        base1.create_dir()?;
        base2.create_dir()?;
        subdir_base1.create_dir()?;
        subdir_base2.create_dir()?;
        file1_base1.create_file(false)?;
        file1_base2.create_file(false)?;
        file2_base1.create_file(false)?;
        file2_base2.create_file(false)?;
        file3_base1.create_file(false)?;
        file3_base2.create_file(false)?;
        file4_base1.create_file(false)?;
        file4_base2.create_file(false)?;

        // Create module that tracks paths
        let module = Module::new(vec![
            ModuleEntry::new(RelPath::from("in_neither.txt"), ModulePolicy::Track),
            ModuleEntry::new(RelPath::from("only_in_home.txt"), ModulePolicy::Ignore),
            ModuleEntry::new(RelPath::from("only_in_backup.txt"), ModulePolicy::NotDiff),
            ModuleEntry::new(RelPath::from("in_both.txt"), ModulePolicy::Track),
            ModuleEntry::new(RelPath::from("subdir"), ModulePolicy::Ignore),
        ]);

        // Merge bases
        let merged = module.merge_bases(&base1, &base2)?;
        assert_eq!(merged.entries().len(), 6);

        // Collect paths for verification
        let paths: HashMap<_, _> = merged
            .entries()
            .iter()
            .map(|e| (e.path().to_str_lossy(), e.policy()))
            .collect();

        assert_eq!(paths.get("in_neither.txt".into()), None);
        assert_eq!(
            paths.get("only_in_home.txt".into()),
            Some(&ModulePolicy::Ignore)
        );
        assert_eq!(
            paths.get("only_in_backup.txt".into()),
            Some(&ModulePolicy::NotDiff)
        );

        Ok(())
    }
}
