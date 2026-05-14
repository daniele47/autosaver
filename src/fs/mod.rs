use std::fs;

use anyhow::{Context, Result};
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
}
