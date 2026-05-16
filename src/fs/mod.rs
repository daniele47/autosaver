use std::{
    fs::{self, File},
    io::Read,
    path::Component,
};

use anyhow::{Context, Result, bail};
use tracing::{debug, instrument, trace};

use crate::fs::abs::AbsPathStr;

pub mod abs;
pub mod path;
pub mod rel;

impl AbsPathStr {
    #[instrument(err, level = "trace", skip_all, fields(self = %self.display()))]
    pub fn list_filtered<F>(&self, paths: &mut Vec<AbsPathStr>, filter: F) -> Result<()>
    where
        F: Fn(&AbsPathStr) -> bool,
    {
        fs::read_dir(self)
            .with_context(|| {
                let p = self.display();
                format!("Could not list files in directory {p}")
            })?
            .map(|e| {
                let e = e.with_context(|| format!("Failed to read entry in {}", self.display()))?;
                trace!(file = %e.path().display(), "Listed path inside directory:");
                AbsPathStr::try_from(e.path())
            })
            .filter(|e| {
                if let Ok(p) = e {
                    let keep = filter(p);
                    if !keep {
                        trace!(file = %p.path().display(), "Listed path got filtered out:");
                    }
                    keep
                } else {
                    true
                }
            })
            .try_for_each(|e| {
                paths.push(e?);
                Ok(())
            })
    }
    pub fn list_all(&self, paths: &mut Vec<AbsPathStr>) -> Result<()> {
        self.list_filtered(paths, |_| true)
    }

    #[instrument(err, level = "trace", skip_all, fields(self = %self.display()))]
    pub fn find_filtered<F>(&self, paths: &mut Vec<AbsPathStr>, filter: F) -> Result<()>
    where
        F: Fn(&AbsPathStr) -> bool,
    {
        let mut stack = Vec::<(Option<usize>, Option<usize>)>::new();
        let mut filtered_out = Vec::<AbsPathStr>::new();
        let mut buffer = Vec::<AbsPathStr>::new();
        let mut root_dir_used = false;

        loop {
            // get next stack item
            let item: &AbsPathStr;
            if !root_dir_used {
                item = self;
                root_dir_used = true;
            } else {
                if let Some((item_index, filtered_index)) = stack.pop() {
                    if let Some(ri) = item_index {
                        item = &paths[ri];
                    } else if let Some(fi) = filtered_index {
                        item = &filtered_out[fi];
                    } else {
                        unreachable!("Items must be in res or in filtered")
                    }
                } else {
                    break;
                }
            }

            // append children to vector + push chilren dirs to stack
            item.list_all(&mut buffer)?;
            for child in buffer.drain(..) {
                trace!(file = %child.display(), "Found path recursively inside directory:");
                if filter(&child) {
                    if child.is_dir() {
                        trace!(directory = %child.display(), "Directory added to stack:");
                        stack.push((Some(paths.len()), None));
                    }
                    paths.push(child);
                } else {
                    trace!(file = %child.display(), "Found path got filtered out:");
                    if child.is_dir() {
                        trace!(directory = %child.display(), "Directory added to stack:");
                        stack.push((None, Some(filtered_out.len())));
                    }
                    filtered_out.push(child);
                }
            }
        }

        Ok(())
    }
    pub fn find_all(&self, paths: &mut Vec<AbsPathStr>) -> Result<()> {
        self.find_filtered(paths, |_| true)
    }

    #[instrument(err, level = "trace", skip_all, fields(self = %self.display()))]
    pub fn all_files(&self, files: &mut Vec<AbsPathStr>) -> Result<()> {
        if self.is_file() {
            trace!(path=%self.display(), "Path is a file:");
            files.push(self.clone());
        } else if self.is_dir() {
            trace!(path=%self.display(), "Path is a directory:");
            files.push(self.clone());
            self.find_filtered(files, AbsPathStr::is_file)?;
        } else {
            trace!(path=%self.display(), "Path is neither a file nor a directory:");
        }
        Ok(())
    }

    #[instrument(err, level = "trace", skip_all, fields(self = %self.display()))]
    pub fn delete_path(&self) -> Result<()> {
        if !self.path().exists() {
            debug!(path = %self.display(), "Path does not exist, nothing to delete:");
            return Ok(());
        }

        // purge file
        let canon = self.canonicalize()?;
        if canon.is_file() {
            fs::remove_file(&canon).with_context(|| {
                let p = canon.display();
                format!("Could not delete file: {p}")
            })?;
            debug!(file = %self.display(), "File successfully deleted: ");
        }
        // purge empty directory
        else if canon.is_dir() {
            fs::remove_dir(&canon).with_context(|| {
                let p = canon.display();
                format!("Could not delete directory: {p}")
            })?;
            debug!(directory = %self.display(), "Directory successfully deleted");
        }
        // fail if not either file nor directory
        else {
            let p = canon.display();
            bail!("Could not delete path: {p}");
        }

        // delete empty directories
        let mut parent = canon.path().parent();
        while let Some(p) = parent {
            if fs::remove_dir(p).is_err() {
                break;
            }
            debug!(directory = %p.display(), "Deleted empty parent directory");
            parent = p.parent();
        }

        Ok(())
    }

    #[instrument(err, level = "trace", skip_all, fields(self = %self.display()))]
    pub fn create_file(&self) -> Result<()> {
        if self.is_file() {
            debug!(file = %self.display(), "File already exists, left untouched:");
            return Ok(());
        }

        // valid file can be created
        if !matches!(
            self.path().components().next_back(),
            Some(Component::Normal(_))
        ) {
            let p = self.display();
            bail!("Path cannot be created as a file: {p}")
        }

        // create parent dirs
        if let Some(parent) = self.path().parent() {
            fs::create_dir_all(parent).with_context(|| {
                let p = parent.display();
                format!("Failed to create directory: {p}")
            })?;
            debug!(directory = %parent.display(), "Parent directory successfully created:");
        } else {
            let p = self.display();
            bail!("Could not create parent directories: {p}");
        }

        // create file
        File::create(self).with_context(|| {
            let p = self.display();
            format!("Failed to create file: {p}")
        })?;
        debug!(file = %self.display(), "File successfully created:");

        Ok(())
    }

    #[instrument(err, level = "trace", skip_all, fields(self = %self.display()))]
    pub fn read_file(&self) -> Result<String> {
        if !self.is_file() {
            let p = self.display();
            bail!("Cannot read a path that is not a file: {p}");
        }
        fs::read_to_string(self)
            .with_context(|| {
                let p = self.display();
                format!("Could not read file: {p}")
            })
            .inspect(|_| debug!(file = %self.display(), "File successfully read into string:"))
    }

    #[instrument(err, level = "trace", , skip_all, fields(self = %self.display(), dst=%dst.display()))]
    pub fn copy_file(&self, dst: &Self) -> Result<()> {
        dst.create_file()?;
        fs::copy(self, dst).with_context(|| {
            let p = self.display();
            let t = dst.display();
            format!("Failed to copy from {p} to {t}")
        })?;
        debug!(src_path = %self.display(), dst_path = %dst.display(), "Source file successfully copied into destination file:");
        Ok(())
    }

    #[instrument(ret, level = "trace", , skip_all, fields(self = %self.display(), other = %other.display()))]
    pub fn files_eq(&self, other: &Self) -> bool {
        || -> Result<()> {
            let sm = self.path().metadata()?;
            let om = other.path().metadata()?;

            // check both paths are files
            if !sm.is_file() || !om.is_file() {
                bail!("Not files");
            }

            // check file len for faster checks
            if sm.len() != om.len() {
                bail!("Length differs");
            }

            // chunked byte comparison (works for both text and binary)
            let mut file1 = File::open(self)?;
            let mut file2 = File::open(other)?;

            let mut buf1 = [0; 4096];
            let mut buf2 = [0; 4096];

            loop {
                let n1 = file1.read(&mut buf1)?;
                let n2 = file2.read(&mut buf2)?;

                if n1 != n2 || buf1[..n1] != buf2[..n2] {
                    bail!("Chunk differs");
                }
                if n1 == 0 {
                    return Ok(());
                }
            }
        }()
        .is_ok()
    }
}
