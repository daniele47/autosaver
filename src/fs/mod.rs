use std::{
    fs::{self, File},
    path::Component,
};

use anyhow::{Context, Result, bail};
use tracing::instrument;

use crate::fs::abs::AbsPathStr;

pub mod abs;
pub mod path;
pub mod rel;

impl AbsPathStr {
    #[instrument(ret, err, level = "trace")]
    pub fn list_all(&self) -> Result<Vec<AbsPathStr>> {
        fs::read_dir(self.path())
            .with_context(|| {
                let p = self.to_string_lossy();
                format!("Could not list files in directory {p}")
            })?
            .map(|entry| {
                let entry = entry.with_context(|| {
                    format!("Failed to read entry in {}", self.to_string_lossy())
                })?;
                AbsPathStr::try_from(entry.path())
            })
            .collect()
    }

    #[instrument(ret, err, level = "trace")]
    pub fn find_all(&self) -> Result<Vec<AbsPathStr>> {
        let mut stack = Vec::<usize>::new();
        let mut res = Vec::<AbsPathStr>::new();
        let mut root_dir_used = false;

        loop {
            // get next stack item
            let item: &AbsPathStr;
            if !root_dir_used {
                item = self;
                root_dir_used = true;
            } else {
                if let Some(item_index) = stack.pop() {
                    item = &res[item_index];
                } else {
                    break;
                }
            }

            // append children to vector + push chilren dirs to stack
            for child in item.list_all()? {
                if child.is_dir() {
                    stack.push(res.len());
                }
                res.push(child);
            }
        }

        Ok(res)
    }

    #[instrument(ret, err, level = "trace")]
    pub fn delete_path(&self) -> Result<()> {
        if !self.path().exists() {
            return Ok(());
        }

        // purge file
        let canon = self.canonicalize()?;
        if canon.is_file() {
            fs::remove_file(canon.path()).with_context(|| {
                let p = canon.to_string_lossy();
                format!("Could not delete file: {p}")
            })?;
        }
        // purge empty directory
        else if canon.is_dir() {
            fs::remove_dir(canon.path()).with_context(|| {
                let p = canon.to_string_lossy();
                format!("Could not delete directory: {p}")
            })?;
        }
        // fail if not either file nor directory
        else {
            let p = canon.to_string_lossy();
            bail!("Could not delete path: {p}");
        }

        // delete empty directories
        let mut parent = canon.path().parent();
        while let Some(p) = parent {
            if fs::remove_dir(p).is_err() {
                break;
            }
            parent = p.parent();
        }

        Ok(())
    }

    #[instrument(ret, err, level = "trace")]
    pub fn create_file(&self) -> Result<()> {
        if self.is_file() {
            return Ok(());
        }

        // valid file can be created
        if !matches!(self.path().components().last(), Some(Component::Normal(_))) {
            let p = self.to_string_lossy();
            bail!("Path cannot be created as a file: {p}")
        }

        // create parent dirs
        if let Some(parent) = self.path().parent() {
            fs::create_dir_all(parent).with_context(|| {
                let p = parent.to_string_lossy();
                format!("Failed to create directory: {p}")
            })?;
        } else {
            let p = self.to_string_lossy();
            bail!("Could not create parent directories: {p}");
        }

        // create file
        File::create(self.path()).with_context(|| {
            let p = self.to_string_lossy();
            format!("Failed to create file: {p}")
        })?;

        Ok(())
    }
}
