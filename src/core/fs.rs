//! This module implements various filesystem wrappers, to guarantee safer fs operations.

use std::{
    collections::{BTreeSet, HashSet},
    env,
    fs::{self, File, Metadata},
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::core::errors::{Error, Result};

/// Struct storing an absolute path.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AbsPath {
    path: PathBuf,
}

/// Struct storing a relative path.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelPath {
    path: PathBuf,
}

impl AbsPath {
    /// Creates new AbsPath from an absolute path.
    pub fn new(path: PathBuf) -> Self {
        assert!(path.is_absolute());
        Self { path }
    }

    /// Creates a new pseudo-random AbsPath in a temporary location.
    ///
    /// This function should be used mostly for tests!
    ///
    /// Notes:
    /// - this function should mostly be used for tests, as files in `/tmp` dir in linux
    ///   are often stored directly in ram via tmpfs mount, thus it's not ideal for big files!
    /// - this doesn't guarantee the path doesn't exist, to be safe, this function should
    ///   be used in a loop and a new path should be generated until one doesn't exist.
    ///   But for simple testing purposes, this function should be good enough, just make sure
    ///   to cleanup the temporary files and directories!
    ///
    /// Implementation details: pseudo-randomicity comes from 3 simple factors:
    /// - prefix passed as a string (more of an identifier, than proper randomness)
    /// - current time in nano seconds (pretty much impossible to repeat twice)
    /// - current process pid (for some extra randomness, which does not hurt)
    pub fn new_tmp(prefix: &str) -> Self {
        let tmp_dir = env::temp_dir();
        let pid = std::process::id();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let tmp_name_str = format!("{}_{}_{}.tmp", prefix, now.as_nanos(), pid);
        let tmp_name = PathBuf::from(&tmp_name_str);
        AbsPath::from(tmp_dir.join(tmp_name))
    }

    /// Get canonicalized path.
    pub fn canonicalize(&self) -> Result<Self> {
        Ok(self
            .path
            .canonicalize()
            .map_err(|e| Error::IoError(e, self.path.clone()))?
            .into())
    }

    /// Get relative path.
    pub fn to_relative(&self, prefix: &AbsPath) -> Result<RelPath> {
        self.path
            .strip_prefix(&prefix.path)
            .map(|p| p.to_path_buf().into())
            .map_err(|_| Error::InvalidPathPrefix(self.path.clone(), prefix.path.clone()))
    }

    /// Get path as a lossy string
    pub fn to_str_lossy(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    /// Append to path.
    pub fn join(&self, suffix: &RelPath) -> AbsPath {
        self.path.join(&suffix.path).into()
    }

    /// Appent multiple times to path at once.
    pub fn joins(&self, suffixes: &[&str]) -> Self {
        suffixes
            .iter()
            .fold(self.clone(), |p, s| p.join(&RelPath::from(*s)))
    }

    /// Get File Metadata.
    ///
    /// Note: it follows symlinks! Use `symlink_metadata` to not follow symlinks.
    pub fn metadata(&self) -> Result<Metadata> {
        self.path
            .metadata()
            .map_err(|e| Error::IoError(e, self.path.clone()))
    }

    /// Check if path exists.
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Create directory with all missing parents.
    ///
    /// Notes: there could be some directory left created on failure!
    pub fn create_dir(&self) -> Result<()> {
        if self.exists() && !self.metadata()?.is_dir() {
            self.purge_path(false)?;
        }
        fs::create_dir_all(&self.path).map_err(|e| Error::IoError(e, self.path.clone()))
    }

    /// Create file, with all missing parents.
    ///
    /// Notes:
    /// - there could be some directory left created on failure!
    /// - `allow_recursive_deletion` allows deleting not empty dirs if in path
    pub fn create_file(&self, allow_recursive_deletion: bool) -> Result<()> {
        if self.exists() {
            if self.metadata()?.is_file() {
                return Ok(());
            }
            self.purge_path(allow_recursive_deletion)?;
        }
        // note: this parent call is not fully safe, as path could not be normalized beforehand
        // not much i can do differently though ¯\_(ツ)_/¯
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|e| Error::IoError(e, self.path.clone()))?;
        }
        File::create(&self.path).map_err(|e| Error::IoError(e, self.path.clone()))?;
        Ok(())
    }

    /// Delete empty directory and its ancestors until it finds the first not empty dir!
    pub fn delete_dirs(&self) -> Result<()> {
        if !self.exists() {
            return Ok(());
        }

        // keep deleting empty dirs
        let mut curr = self.canonicalize()?;
        loop {
            if fs::remove_dir(&curr.path).is_err() {
                break;
            }
            let parent = curr.path.parent();
            if parent.is_none() {
                break;
            }
            curr = parent.unwrap().to_path_buf().into();
        }
        Ok(())
    }

    /// Purge path, whatever file type it is.
    ///
    /// DANGER: Use `allow_recursive_deletion` with caution, as it can easily purge entire
    /// directories!!! Make sure to use with extreme caution always!
    pub fn purge_path(&self, allow_recursive_deletion: bool) -> Result<()> {
        if !self.exists() {
            return Ok(());
        }

        // delete whatever is in the path
        let metadata = self.metadata()?;
        if metadata.is_dir() {
            if allow_recursive_deletion {
                fs::remove_dir_all(&self.path).map_err(|e| Error::IoError(e, self.path.clone()))?;
            } else {
                fs::remove_dir(&self.path).map_err(|e| Error::IoError(e, self.path.clone()))?;
            }
        } else {
            fs::remove_file(&self.path).map_err(|e| Error::IoError(e, self.path.clone()))?;
        }

        // clear empty parent dirs
        if let Some(parent) = self.path.parent() {
            let abs_parent = AbsPath::new(parent.to_path_buf());
            abs_parent.delete_dirs()?;
        }

        Ok(())
    }

    /// Copy file into destination.
    ///
    /// Note: `allow_recursive_deletion` purges even not empty dirs if in dst path
    pub fn copy_file(&self, dst: &AbsPath, allow_recursive_deletion: bool) -> Result<()> {
        dst.create_file(allow_recursive_deletion)?;
        fs::copy(&self.path, &dst.path).map_err(|e| Error::IoError(e, self.path.clone()))?;
        Ok(())
    }

    /// List all files in a directory.
    ///
    /// Notes: this will get ALL files, even directories, symlinks, all rust can get.
    pub fn list_files(&self) -> Result<BTreeSet<AbsPath>> {
        Ok(fs::read_dir(&self.path)
            .map_err(|e| Error::IoError(e, self.path.clone()))?
            .filter_map(|entry| entry.ok())
            .map(|entry| AbsPath::new(entry.path()))
            .collect())
    }

    /// List all files recursively inside a directory.
    ///
    /// Note: this will get ALL files, even directories, symlinks, all rust can get.
    /// Manual filtering is required when using this function!
    ///
    /// Implementation details: this function uses DFS, using a vector as a stack of directories
    /// found but yet to be explored, and a hashset of all paths explored until now, canonicalized.
    /// The hash set allows to easily check if a new directory was already explored, and if so,
    /// avoid exploring it again. This easily resolves all symlink loops that could be created.
    pub fn all_files(&self) -> Result<BTreeSet<AbsPath>> {
        let mut files = BTreeSet::new();
        let mut norm_files = HashSet::new();
        let mut stack = Vec::new();
        stack.push(self.clone());

        while let Some(item) = stack.pop() {
            let dir_files = item.list_files()?;
            for dir_file in &dir_files {
                let canon = dir_file.canonicalize()?;
                if norm_files.contains(&canon) {
                    continue;
                }
                if dir_file.metadata()?.is_dir() {
                    stack.push(dir_file.clone());
                }
                norm_files.insert(canon);
                files.insert(dir_file.clone());
            }
        }

        Ok(files)
    }

    /// Buffered line by line read of files
    ///
    /// Note: since this uses a buffered reader, read could costantly fail. It is thus necessary
    /// to handle the potential error on every each line read! The error is of type std::io::Error!
    pub fn read_lines(&self) -> Result<impl Iterator<Item = Result<String>>> {
        let file = File::open(&self.path).map_err(|e| Error::IoError(e, self.path.clone()))?;
        let reader = BufReader::new(file);
        Ok(reader
            .lines()
            .map(move |line| line.map_err(|e| Error::IoError(e, self.path.clone()))))
    }
}

impl RelPath {
    /// Creates new RelPath from relative path.
    pub fn new(path: PathBuf) -> Self {
        assert!(path.is_relative());
        Self { path }
    }

    /// Add a prefix to turn relative path into absolute path.
    pub fn to_absolute(&self, base: &AbsPath) -> AbsPath {
        base.path.join(&self.path).into()
    }

    /// Get path as a lossy string
    pub fn to_str_lossy(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    /// Append to path.
    pub fn join(&self, suffix: &RelPath) -> Self {
        self.path.join(&suffix.path).into()
    }

    /// Appent multiple times to path at once.
    pub fn joins(&self, suffixes: &[&str]) -> Self {
        suffixes
            .iter()
            .fold(self.clone(), |p, s| p.join(&RelPath::from(*s)))
    }
}

impl From<PathBuf> for AbsPath {
    fn from(value: PathBuf) -> Self {
        Self::new(value)
    }
}

impl From<PathBuf> for RelPath {
    fn from(value: PathBuf) -> Self {
        Self::new(value)
    }
}

impl From<AbsPath> for PathBuf {
    fn from(value: AbsPath) -> Self {
        value.path
    }
}

impl From<RelPath> for PathBuf {
    fn from(value: RelPath) -> Self {
        value.path
    }
}

impl From<&str> for AbsPath {
    fn from(s: &str) -> Self {
        Self::new(PathBuf::from(s))
    }
}

impl From<&str> for RelPath {
    fn from(s: &str) -> Self {
        Self::new(PathBuf::from(s))
    }
}

impl TryFrom<AbsPath> for String {
    type Error = Error;

    fn try_from(value: AbsPath) -> std::result::Result<Self, Self::Error> {
        value
            .path
            .to_str()
            .ok_or(Error::InvalidPathString(value.path.clone()))
            .map(String::from)
    }
}

impl TryFrom<RelPath> for String {
    type Error = Error;

    fn try_from(value: RelPath) -> std::result::Result<Self, Self::Error> {
        value
            .path
            .to_str()
            .ok_or(Error::InvalidPathString(value.path.clone()))
            .map(String::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn setup_test_directory() -> AbsPath {
        let tmp_dir = AbsPath::new_tmp("dotfiles_rust_test");

        tmp_dir.purge_path(true).unwrap();
        tmp_dir.create_dir().unwrap();

        let file1 = tmp_dir.joins(&["file1.txt"]);
        let file2 = tmp_dir.joins(&["file2.txt"]);
        let subdir1 = tmp_dir.joins(&["subdir1"]);
        let file3 = subdir1.joins(&["file3.txt"]);
        let file4 = subdir1.joins(&["file4.txt"]);
        let subsubdir1 = subdir1.joins(&["subsubdir1"]);
        let file5 = subsubdir1.joins(&["file5.txt"]);
        let subdir2 = tmp_dir.joins(&["subdir2"]);
        let file6 = subdir2.joins(&["file6.txt"]);
        let empty_dir = tmp_dir.joins(&["empty_dir"]);

        subdir1.create_dir().unwrap();
        subsubdir1.create_dir().unwrap();
        subdir2.create_dir().unwrap();
        empty_dir.create_dir().unwrap();
        file6.create_file(false).unwrap();
        file1.create_file(false).unwrap();
        file2.create_file(false).unwrap();
        file3.create_file(false).unwrap();
        file4.create_file(false).unwrap();
        file5.create_file(false).unwrap();

        tmp_dir
    }

    #[test]
    fn test_new_tmp() {
        let tmp1 = AbsPath::new_tmp("test");
        let tmp2 = AbsPath::new_tmp("test");

        // Should be different paths
        assert_ne!(tmp1, tmp2);
        assert!(tmp1.path.is_absolute());
        assert!(tmp2.path.is_absolute());

        // Should be in temp directory
        assert!(tmp1.path.starts_with(env::temp_dir()));
        assert!(tmp2.path.starts_with(env::temp_dir()));
    }

    #[test]
    fn test_create_dir() {
        let root = AbsPath::new_tmp("test_create_dir");
        root.purge_path(true).unwrap();

        // Create nested directory
        let nested = root.joins(&["a", "b"]);
        assert!(!nested.exists());
        nested.create_dir().unwrap();
        assert!(nested.exists());
        assert!(nested.metadata().unwrap().is_dir());

        root.purge_path(true).unwrap();
    }

    #[test]
    fn test_create_file() {
        let root = AbsPath::new_tmp("test_create_file");
        root.purge_path(true).unwrap();
        root.create_dir().unwrap();

        // Create file in existing directory
        let file = root.joins(&["test.txt"]);
        assert!(!file.exists());
        file.create_file(false).unwrap();
        assert!(file.exists());
        assert!(file.metadata().unwrap().is_file());

        // Create file with nested directories
        let nested = root.joins(&["nested", "dir", "file.txt"]);
        assert!(!nested.exists());
        nested.create_file(false).unwrap();
        assert!(nested.exists());
        assert!(nested.metadata().unwrap().is_file());

        // Creating existing file should be idempotent
        nested.create_file(false).unwrap();
        assert!(nested.exists());

        root.purge_path(true).unwrap();
    }

    #[test]
    fn test_list_files() {
        let root = setup_test_directory();

        let files = root.list_files().unwrap();
        let file_names: HashSet<_> = files
            .iter()
            .map(|f| f.path.file_name().unwrap().to_str().unwrap())
            .collect();

        // Should list immediate children
        assert!(file_names.contains("file1.txt"));
        assert!(file_names.contains("file2.txt"));
        assert!(file_names.contains("subdir1"));
        assert!(file_names.contains("subdir2"));
        assert!(file_names.contains("empty_dir"));

        // Should not contain nested files
        assert!(!file_names.contains("file3.txt"));
        assert!(!file_names.contains("file5.txt"));

        // Assert count of paths found
        assert_eq!(files.len(), 5);

        root.purge_path(true).unwrap();
    }

    #[test]
    fn test_all_files() {
        let root = setup_test_directory();

        let all_paths = root.all_files().unwrap();
        let path_names: HashSet<_> = all_paths
            .iter()
            .map(|p| p.path.file_name().unwrap().to_str().unwrap())
            .collect();

        // Should contain all files and directories (top level and nested)
        assert!(path_names.contains("file1.txt"));
        assert!(path_names.contains("file2.txt"));
        assert!(path_names.contains("subdir1"));
        assert!(path_names.contains("subdir2"));
        assert!(path_names.contains("empty_dir"));
        assert!(path_names.contains("file3.txt"));
        assert!(path_names.contains("file4.txt"));
        assert!(path_names.contains("subsubdir1"));
        assert!(path_names.contains("file5.txt"));
        assert!(path_names.contains("file6.txt"));

        // Assert count of paths found
        assert_eq!(all_paths.len(), 10);

        root.purge_path(true).unwrap();
    }

    #[test]
    fn test_delete_dirs() {
        let root = AbsPath::new_tmp("test_delete_dirs");
        let nested = root.joins(&["a", "b", "c"]);
        nested.create_dir().unwrap();

        // The nested directory should be gone
        assert!(nested.exists());
        nested.delete_dirs().unwrap();
        assert!(!nested.exists());

        root.purge_path(true).unwrap();
    }

    #[test]
    fn test_purge_path() {
        let root = setup_test_directory();
        let file = root.joins(&["file1.txt"]);

        // Try purging simple file
        assert!(file.exists());
        file.purge_path(false).unwrap();
        assert!(!file.exists());
        assert!(root.exists());

        // Try to purge non-empty directory without recursive flag (should fail)
        let result = root.purge_path(false);
        assert!(result.is_err());

        // Purge with recursive flag
        root.purge_path(true).unwrap();
        assert!(!root.exists());
    }
}
