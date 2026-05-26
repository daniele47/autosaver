use indexmap::{IndexMap, map::Entry};

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
    stdin: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Runner {
    entries: Vec<RunnerEntry>,
}

impl RunnerEntry {
    pub fn new(path: RelPathStr, policy: RunnerPolicy, stdin: bool) -> Self {
        Self {
            path,
            policy,
            stdin,
        }
    }

    pub fn path(&self) -> &RelPathStr {
        &self.path
    }

    pub fn policy(&self) -> &RunnerPolicy {
        &self.policy
    }

    pub fn stdin(&self) -> bool {
        self.stdin
    }
}

impl Runner {
    pub fn new(entries: Vec<RunnerEntry>) -> Self {
        Self { entries }
    }

    pub fn entries(&self) -> &[RunnerEntry] {
        &self.entries
    }

    pub fn resolve(&self, dir: &AbsPathStr) -> anyhow::Result<IndexMap<AbsPathStr, &RunnerEntry>> {
        let mut elems: IndexMap<AbsPathStr, &RunnerEntry> = IndexMap::new();

        for entry in self.entries() {
            let all_files_ord = entry.path().to_abs(dir)?.all_files_ord()?;
            all_files_ord.into_iter().try_for_each(|p| {
                match elems.entry(p) {
                    Entry::Occupied(mut e) => {
                        if (entry.policy as u64) < (*e.get().policy() as u64) {
                            e.insert(entry);
                        }
                    }
                    Entry::Vacant(e) => {
                        e.insert(entry);
                    }
                }
                anyhow::Ok(())
            })?;
        }

        Ok(elems)
    }
}
