use std::collections::{HashMap, hash_map::Entry};

use anyhow::bail;

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

    pub fn resolve<T>(
        &self,
        dir: &AbsPathStr,
        all: &mut HashMap<AbsPathStr, RunnerPolicy>,
        on_each: T,
    ) -> anyhow::Result<()>
    where
        T: Fn(AbsPathStr, RunnerPolicy) -> anyhow::Result<()>,
    {
        let mut here: HashMap<AbsPathStr, RunnerPolicy> = HashMap::new();
        let mut res: Vec<AbsPathStr> = vec![];

        for entry in self.entries() {
            entry.path().to_abs(dir)?.all_files(|p| {
                if all.contains_key(&p) {
                    let p = p.display();
                    bail!(format!("Path '{p}' was already found in an other profile"));
                }
                match here.entry(p) {
                    Entry::Occupied(mut e) => {
                        if (*e.get() as u64) < entry.policy as u64 {
                            e.insert(entry.policy);
                        }
                    }
                    Entry::Vacant(e) => {
                        res.push(e.key().to_owned());
                        e.insert(entry.policy);
                    }
                }
                Ok(())
            })?;
        }

        // run on each and move paths from vec into all hashmap
        all.extend(here);
        for elem in res {
            let policy = all[&elem];
            on_each(elem, policy)?;
        }

        Ok(())
    }
}
