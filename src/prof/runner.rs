use std::collections::{HashMap, hash_map::Entry};

use crate::fs::{abs::AbsPathStr, rel::RelPathStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunnerPolicy {
    Skip,
    Run,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnerEntry {
    path: RelPathStr,
    policy: RunnerPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Runner {
    entries: Vec<RunnerEntry>,
}

impl RunnerEntry {
    pub fn new(path: RelPathStr, policy: RunnerPolicy) -> Self {
        Self { path, policy }
    }

    pub fn path(&self) -> &RelPathStr {
        &self.path
    }

    pub fn policy(&self) -> &RunnerPolicy {
        &self.policy
    }
}

impl Runner {
    pub fn new(entries: Vec<RunnerEntry>) -> Self {
        Self { entries }
    }

    pub fn entries(&self) -> &[RunnerEntry] {
        &self.entries
    }

    pub fn resolve<T>(&self, dir: &AbsPathStr, mut on_each: T) -> anyhow::Result<()>
    where
        T: FnMut(AbsPathStr, RunnerPolicy) -> anyhow::Result<()>,
    {
        let mut here: HashMap<AbsPathStr, RunnerPolicy> = HashMap::new();
        let mut res: Vec<AbsPathStr> = vec![];

        for entry in self.entries() {
            let all_files_ord = entry.path().to_abs(dir)?.all_files_ord()?;
            all_files_ord.into_iter().try_for_each(|p| {
                match here.entry(p) {
                    Entry::Occupied(mut e) => {
                        if (entry.policy as u64) < (*e.get() as u64) {
                            e.insert(entry.policy);
                        }
                    }
                    Entry::Vacant(e) => {
                        res.push(e.key().to_owned());
                        e.insert(entry.policy);
                    }
                }
                anyhow::Ok(())
            })?;
        }

        // run on each and move paths from vec into all hashmap
        for elem in res {
            let policy = here[&elem];
            on_each(elem, policy)?;
        }

        Ok(())
    }
}
