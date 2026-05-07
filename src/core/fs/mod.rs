//! This module implements various filesystem wrappers, to guarantee safer fs operations.

use std::{
    collections::{BTreeSet, HashSet},
    env,
    fs::{self, File, Metadata},
    io::{BufRead, BufReader, BufWriter, Lines, Read, Write},
    path::{Component, Path, PathBuf},
};

use crate::core::error::{Error, Result};

mod myers;
pub use myers::LineDiff;

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

/// Simple enum to check what type of path a string is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PathType {
    Relative,
    Absolute,
}

/// Trait to get a simple way to read line by line from a buffered file.
pub trait LineReader: Iterator<Item = Result<String>> {}

/// Trait to get a simple way to write line by line into a buffered file.
pub trait LineWriter {
    /// Write a single line to file.
    ///
    /// Note: It doesn't make guarantees about it being instantly on file.
    /// Call `flush` to make sure the written line is actually on file.
    fn write_line<S: AsRef<str>>(&mut self, line: S) -> Result<()>;

    /// Make sure what was written is actually on file.
    fn flush(&mut self) -> Result<()>;

    /// Write an entire iterator to file and flushes.
    fn write_all_lines<I, S>(&mut self, lines: I) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for line in lines {
            self.write_line(line.as_ref())?
        }
        self.flush()
    }
}

/// Simple generic implementation of LineReader that preserves the lazy evaluation capabilities.
#[derive(Debug)]
pub struct AnyLineReader<I: Iterator<Item = Result<String>>> {
    lines: I,
}

/// Simple generic implementation of LineWrite that writes to a vector of strings.
#[derive(Debug, Default)]
pub struct AnyLineWriter {
    lines: Vec<String>,
}

/// Check if a path has parent directory symbols.
pub fn check_has_parent_dirs(path: &str) -> bool {
    Path::new(path)
        .components()
        .any(|c| c == Component::ParentDir)
}

impl<I: Iterator<Item = Result<String>>> AnyLineReader<I> {
    /// Create new AnyLineReader that stores an iterator.
    pub fn new(iter: I) -> Self {
        Self { lines: iter }
    }
}

impl<I: Iterator<Item = Result<String>>> Iterator for AnyLineReader<I> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next()
    }
}

impl<I: Iterator<Item = Result<String>>> LineReader for AnyLineReader<I> {}

impl AnyLineWriter {
    /// Create new AnyLineReader that stores an iterator.
    pub fn new() -> Self {
        Default::default()
    }
}

impl LineWriter for AnyLineWriter {
    fn write_line<S: AsRef<str>>(&mut self, line: S) -> Result<()> {
        self.lines.push(line.as_ref().to_string());
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl From<&str> for PathType {
    fn from(path: &str) -> Self {
        let pathbuf = PathBuf::from(path);
        if pathbuf.is_absolute() {
            PathType::Absolute
        } else if pathbuf.is_relative() {
            PathType::Relative
        } else {
            unreachable!("File MUST be either absolute or relative")
        }
    }
}

impl From<String> for PathType {
    fn from(path: String) -> Self {
        path.as_str().into()
    }
}

impl AbsPath {
    pub const FILTER_ALL: fn(&AbsPath) -> bool = Self::filter_all;
    pub const FILTER_FILES: fn(&AbsPath) -> bool = Self::filter_files;
    pub const FILTER_DIRS: fn(&AbsPath) -> bool = Self::filter_directories;
    pub const FILTER_SYMLINKS: fn(&AbsPath) -> bool = Self::filter_symlinks;

    /// Creates new AbsPath from an absolute path.
    pub fn new(path: PathBuf) -> Self {
        assert!(path.is_absolute(), "Path is not absolute");
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
    pub fn metadata(&self) -> Result<Metadata> {
        self.path
            .metadata()
            .map_err(|e| Error::IoError(e, self.path.clone()))
    }

    /// Check if path exists.
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Get parent directory. Note: this is safe ONLY if the path is a file.
    pub fn file_parent(&self) -> Result<AbsPath> {
        if !self.metadata()?.is_file() {
            return Err(Error::NotAFile(self.path.clone()));
        }
        Ok(self
            .path
            .parent()
            .map(|p| AbsPath::from(p.to_owned()))
            .expect("files MUST have a parent directory!"))
    }

    /// Create directory with all missing parents.
    ///
    /// Notes: there could be some directory left created on failure!
    pub fn create_dir(&self) -> Result<()> {
        // delete what already exists in path, if it is not a directory
        if self.exists() && !self.metadata()?.is_dir() {
            self.purge_path(false)?;
        }

        // create directory
        fs::create_dir_all(&self.path).map_err(|e| Error::IoError(e, self.path.clone()))
    }

    /// Create file, with all missing parents.
    ///
    /// Notes:
    /// - there could be some directory left created on failure!
    /// - `allow_recursive_deletion` allows deleting not empty dirs if in path
    pub fn create_file(&self, allow_recursive_deletion: bool) -> Result<()> {
        // delete whatever exists in the path, if it is not a file
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

        // create file
        File::create(&self.path).map_err(|e| Error::IoError(e, self.path.clone()))?;
        Ok(())
    }

    /// Delete empty directory and its ancestors until it finds the first not empty dir!
    pub fn delete_dirs(&self) -> Result<()> {
        // early exit if path is already not a directory
        if !self.exists() {
            return Ok(());
        }

        // keep deleting empty dirs until a not empty directory is found
        let mut curr = self.canonicalize()?;
        loop {
            if fs::remove_dir(&curr.path).is_err() {
                break;
            }
            let parent = curr.path.parent();
            match parent {
                Some(p) => curr = p.to_path_buf().into(),
                None => break,
            }
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

    /// Check if two files content is equal.
    pub fn content_eq(&self, other: &AbsPath) -> bool {
        if let Ok(sm) = self.metadata()
            && let Ok(om) = other.metadata()
        {
            // check both paths are files
            if !sm.is_file() || !om.is_file() {
                return false;
            }

            // check file len for faster checks
            if sm.len() != om.len() {
                return false;
            }

            // chunked byte comparison (works for both text and binary)
            let mut file1 = match std::fs::File::open(&self.path) {
                Ok(f) => f,
                Err(_) => return false,
            };
            let mut file2 = match std::fs::File::open(&other.path) {
                Ok(f) => f,
                Err(_) => return false,
            };

            let mut buf1 = [0; 8192];
            let mut buf2 = [0; 8192];

            loop {
                let n1 = match file1.read(&mut buf1) {
                    Ok(n) => n,
                    Err(_) => return false,
                };
                let n2 = match file2.read(&mut buf2) {
                    Ok(n) => n,
                    Err(_) => return false,
                };

                if n1 != n2 || buf1[..n1] != buf2[..n2] {
                    return false;
                }
                if n1 == 0 {
                    return true;
                }
            }
        }
        false
    }

    /// List all files in a directory.
    ///
    /// Notes: this will get ALL files, even directories, symlinks, all rust can get.
    pub fn list_files<F: Fn(&AbsPath) -> bool>(&self, filter: F) -> Result<BTreeSet<AbsPath>> {
        Ok(fs::read_dir(&self.path)
            .map_err(|e| Error::IoError(e, self.path.clone()))?
            .filter_map(|entry| entry.ok())
            .map(|entry| AbsPath::new(entry.path()))
            .filter(filter)
            .collect())
    }

    fn filter_all(_: &AbsPath) -> bool {
        true
    }
    fn filter_files(path: &AbsPath) -> bool {
        path.metadata().is_ok_and(|m| m.is_file())
    }
    fn filter_directories(path: &AbsPath) -> bool {
        path.metadata().is_ok_and(|m| m.is_dir())
    }
    fn filter_symlinks(path: &AbsPath) -> bool {
        println!("{:?}", path.path.symlink_metadata());
        path.path.symlink_metadata().is_ok_and(|m| m.is_symlink())
    }

    /// Check path canonicalized is inside an other dir.
    pub fn check_inside(&self, dir: &AbsPath) -> bool {
        if let Ok(self_canon) = self.canonicalize()
            && let Ok(dir_canon) = dir.canonicalize()
        {
            self_canon.path.starts_with(&dir_canon.path)
        } else {
            false
        }
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
    pub fn all_files<F: Fn(&AbsPath) -> bool>(&self, filter: F) -> Result<BTreeSet<AbsPath>> {
        let mut files = BTreeSet::new();
        let mut norm_files = HashSet::new();
        let mut stack = Vec::new();
        stack.push(self.clone());

        // Depth first search of all files, using a hashset of already found canonicalized paths, to
        // avoid endless recursion if there are symlinks creating endless loops
        while let Some(item) = stack.pop() {
            for dir_file in item.list_files(AbsPath::FILTER_ALL)? {
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

        Ok(files.into_iter().filter(filter).collect())
    }

    /// Buffered line by line read of files
    ///
    /// Note: since this uses a buffered reader, read could costantly fail. It is thus necessary
    /// to handle the potential error on every each line read! The error is of type std::io::Error!
    pub fn line_reader(&self) -> Result<impl LineReader> {
        // implement line reader
        struct LineReaderImpl {
            path: AbsPath,
            inner: Lines<BufReader<File>>,
        }
        impl Iterator for LineReaderImpl {
            type Item = Result<String>;

            fn next(&mut self) -> Option<Self::Item> {
                self.inner
                    .next()
                    .map(|line| line.map_err(|e| Error::IoError(e, self.path.clone().into())))
            }
        }
        impl LineReader for LineReaderImpl {}

        // open file are get a line reader from such file
        let file = File::open(&self.path).map_err(|e| Error::IoError(e, self.path.clone()))?;
        Ok(LineReaderImpl {
            inner: BufReader::new(file).lines(),
            path: self.path.clone().into(),
        })
    }

    /// Buffered line by line write to file
    ///
    /// Returns a writer that implements `Write` and `BufWrite`, allowing efficient
    /// line-by-line writing. The writer will be automatically flushed when dropped.
    pub fn line_writer(&self) -> Result<impl LineWriter> {
        // implement line writer
        struct LineWriterImpl<W: Write> {
            inner: BufWriter<W>,
            path: AbsPath,
        }
        impl<W: Write> LineWriter for LineWriterImpl<W> {
            fn write_line<S: AsRef<str>>(&mut self, line: S) -> Result<()> {
                writeln!(self.inner, "{}", line.as_ref())
                    .map_err(|e| Error::IoError(e, self.path.clone().into()))?;
                Ok(())
            }
            fn flush(&mut self) -> Result<()> {
                self.inner
                    .flush()
                    .map_err(|e| Error::IoError(e, self.path.clone().into()))?;
                Ok(())
            }
        }

        // open file and get a line writer for such file
        let file = File::create(&self.path).map_err(|e| Error::IoError(e, self.path.clone()))?;
        Ok(LineWriterImpl {
            inner: BufWriter::new(file),
            path: self.path.clone().into(),
        })
    }
}

impl RelPath {
    /// Creates new RelPath from relative path.
    pub fn new(path: PathBuf) -> Self {
        assert!(path.is_relative(), "Path is not relative");
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

    /// Check if relative path appended to `dir` is actually still inside `dir`.
    pub fn check_inside(&self, dir: &AbsPath) -> bool {
        self.to_absolute(dir).check_inside(dir)
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

impl From<String> for AbsPath {
    fn from(s: String) -> Self {
        Self::new(PathBuf::from(s))
    }
}

impl From<String> for RelPath {
    fn from(s: String) -> Self {
        Self::new(PathBuf::from(s))
    }
}

impl TryFrom<AbsPath> for String {
    type Error = Error;

    fn try_from(value: AbsPath) -> std::result::Result<Self, Self::Error> {
        value
            .path
            .to_str()
            .ok_or_else(|| Error::InvalidPathString(value.path.clone()))
            .map(String::from)
    }
}

impl TryFrom<RelPath> for String {
    type Error = Error;

    fn try_from(value: RelPath) -> std::result::Result<Self, Self::Error> {
        value
            .path
            .to_str()
            .ok_or_else(|| Error::InvalidPathString(value.path.clone()))
            .map(String::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn purge_path_even_on_panic(tmpdir: &AbsPath) -> impl Drop {
        struct Guard(AbsPath);
        impl Drop for Guard {
            fn drop(&mut self) {
                let _ = self.0.purge_path(true);
            }
        }
        Guard(tmpdir.clone())
    }

    fn setup_test_directory() -> Result<AbsPath> {
        let tmp_dir = AbsPath::new_tmp("setup_test_directory");

        tmp_dir.purge_path(true)?;
        tmp_dir.create_dir()?;

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

        subdir1.create_dir()?;
        subsubdir1.create_dir()?;
        subdir2.create_dir()?;
        empty_dir.create_dir()?;
        file6.create_file(false)?;
        file1.create_file(false)?;
        file2.create_file(false)?;
        file3.create_file(false)?;
        file4.create_file(false)?;
        file5.create_file(false)?;

        Ok(tmp_dir)
    }

    #[test]
    fn test_new_tmp() {
        let tmp1 = AbsPath::new_tmp("test_new_tmp1");
        let tmp2 = AbsPath::new_tmp("test_new_tmp2");

        // Should be different paths
        assert_ne!(tmp1, tmp2);
        assert!(tmp1.path.is_absolute());
        assert!(tmp2.path.is_absolute());

        // Should be in temp directory
        assert!(tmp1.path.starts_with(env::temp_dir()));
        assert!(tmp2.path.starts_with(env::temp_dir()));
    }

    #[test]
    fn test_create_dir() -> Result<()> {
        let root = AbsPath::new_tmp("test_create_dir");
        root.purge_path(true)?;
        let _guard = purge_path_even_on_panic(&root);

        // Create nested directory
        let nested = root.joins(&["a", "b"]);
        assert!(!nested.exists());
        nested.create_dir()?;
        assert!(nested.exists());
        assert!(nested.metadata()?.is_dir());

        Ok(())
    }

    #[test]
    fn test_create_file() -> Result<()> {
        let root = AbsPath::new_tmp("test_create_file");
        root.purge_path(true)?;
        root.create_dir()?;
        let _guard = purge_path_even_on_panic(&root);

        // Create file in existing directory
        let file = root.joins(&["test.txt"]);
        assert!(!file.exists());
        file.create_file(false)?;
        assert!(file.exists());
        assert!(file.metadata()?.is_file());

        // Create file with nested directories
        let nested = root.joins(&["nested", "dir", "file.txt"]);
        assert!(!nested.exists());
        nested.create_file(false)?;
        assert!(nested.exists());
        assert!(nested.metadata()?.is_file());

        // Creating existing file should be idempotent
        nested.create_file(false)?;
        assert!(nested.exists());

        Ok(())
    }

    #[test]
    fn test_list_files() -> Result<()> {
        let root = setup_test_directory()?;
        let _guard = purge_path_even_on_panic(&root);

        let files = root.list_files(AbsPath::FILTER_ALL)?;
        let file_names: HashSet<_> = files
            .iter()
            .filter_map(|f| f.path.file_name().and_then(|name| name.to_str()))
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

        Ok(())
    }

    #[test]
    fn test_all_files() -> Result<()> {
        let root = setup_test_directory()?;
        let _guard = purge_path_even_on_panic(&root);

        let all_paths = root.all_files(AbsPath::FILTER_ALL)?;
        let path_names: HashSet<_> = all_paths
            .iter()
            .filter_map(|f| f.path.file_name().and_then(|name| name.to_str()))
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

        Ok(())
    }

    #[test]
    fn test_delete_dirs() -> Result<()> {
        let root = AbsPath::new_tmp("test_delete_dirs");
        let nested = root.joins(&["a", "b", "c"]);
        nested.create_dir()?;
        let _guard = purge_path_even_on_panic(&root);

        // The nested directory should be gone
        assert!(nested.exists());
        nested.delete_dirs()?;
        assert!(!nested.exists());

        Ok(())
    }

    #[test]
    fn test_purge_path() -> Result<()> {
        let root = setup_test_directory()?;
        let file = root.joins(&["file1.txt"]);
        let _guard = purge_path_even_on_panic(&root);

        // Try purging simple file
        assert!(file.exists());
        file.purge_path(false)?;
        assert!(!file.exists());
        assert!(root.exists());

        // Try to purge non-empty directory without recursive flag (should fail)
        let result = root.purge_path(false);
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_read_write_lines() -> Result<()> {
        let tmp = AbsPath::new_tmp("test_read_write_lines");
        tmp.create_dir()?;
        let _guard = purge_path_even_on_panic(&tmp);

        let test_file = tmp.joins(&["test.txt"]);
        let lines_in = vec!["first line", "second line", "third line"];

        // Write lines
        let mut writer = test_file.line_writer()?;
        writer.write_all_lines(lines_in.iter())?;

        // Read lines back
        let reader = test_file.line_reader()?;
        let lines_out: Vec<String> = reader.map(|line| line).collect::<Result<_>>()?;

        assert_eq!(lines_in, lines_out);

        Ok(())
    }

    #[test]
    fn test_content_eq() -> Result<()> {
        let tmp = AbsPath::new_tmp("test_read_write_lines");
        tmp.create_dir()?;
        let _guard = purge_path_even_on_panic(&tmp);

        let tmpfile1 = tmp.joins(&["file1.txt"]);
        let tmpfile2 = tmp.joins(&["file2.txt"]);
        tmpfile1.create_file(false)?;
        tmpfile2.create_file(false)?;

        // check files are equal
        tmpfile1.line_writer()?.write_all_lines(["Test Ciao"])?;
        tmpfile2.line_writer()?.write_all_lines(["Test Ciao"])?;
        assert!(tmpfile1.content_eq(&tmpfile2));

        // check files differ
        tmpfile1.line_writer()?.write_all_lines(["Test Ciao!!"])?;
        tmpfile2.line_writer()?.write_all_lines(["diff string"])?;
        assert!(!tmpfile1.content_eq(&tmpfile2));

        Ok(())
    }
}
