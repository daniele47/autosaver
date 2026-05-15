use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result, bail};

use crate::{
    fs::rel::RelPathStr,
    prof::{composite::Composite, module::Module, runner::Runner},
};

pub mod composite;
pub mod module;
pub mod parser;
pub mod runner;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileKind {
    Composite(Composite),
    Module(Module),
    Runner(Runner),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    name: RelPathStr,
    id: RelPathStr,
    kind: ProfileKind,
}
pub type AllProfiles = HashMap<RelPathStr, Profile>;

// structs to make traverse function work properly
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraverseContext<'a> {
    pub item: &'a Profile,
    pub path: &'a [&'a RelPathStr],
    pub stack: &'a [(&'a RelPathStr, bool)],
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraverseParams<'a> {
    pub allow_duplicates: bool,
    pub all_profiles: &'a AllProfiles,
}

impl Profile {
    pub fn new(name: RelPathStr, id: RelPathStr, kind: ProfileKind) -> Self {
        Self { name, id, kind }
    }

    pub fn name(&self) -> &RelPathStr {
        &self.name
    }

    pub fn id(&self) -> &RelPathStr {
        &self.id
    }

    pub fn kind(&self) -> &ProfileKind {
        &self.kind
    }

    pub fn traverse<S>(&self, params: TraverseParams, mut on_elem: S) -> Result<()>
    where
        S: FnMut(TraverseContext) -> Result<()>,
    {
        if let ProfileKind::Composite(_) = self.kind() {
            let mut visited = HashSet::<&RelPathStr>::new();
            let mut path = Vec::<&RelPathStr>::new();
            let mut stack = Vec::<(&RelPathStr, bool)>::new();
            stack.push((self.name(), false));

            // 3 colors DFS to traverse whilst properly detecting loops
            while let Some((item_name, item_visited)) = stack.pop() {
                // grey -> black: item already visited, aka we explored all from here, and backtracked
                if item_visited {
                    path.pop();
                    visited.insert(item_name);
                    continue;
                }

                // check if current item is already in path, aka if this is a cycle
                if let Some(pos) = path.iter().position(|x| x == &item_name) {
                    let cycle = &path[pos..]
                        .iter()
                        .chain(path.get(pos))
                        .map(|s| s.to_string_lossy())
                        .collect::<Vec<_>>()
                        .join(" → ");
                    let name = self.name().to_string_lossy();
                    bail!(format!("Profile {name} has a dependency cycle: {cycle}"));
                }

                // avoid revisiting already explored items, if graphs are complex and the same node is
                // reached multiple times from different nodes
                if !params.allow_duplicates && visited.contains(&item_name) {
                    continue;
                }

                // check if leaf profile
                let item_profile = params.all_profiles.get(item_name).with_context(|| {
                    let name = self.name().to_string_lossy();
                    let inv_name = item_name.to_string_lossy();
                    format!("Profile {name} traversal found invalid profile name {inv_name}")
                })?;
                on_elem(TraverseContext {
                    item: &item_profile,
                    path: &path,
                    stack: &stack,
                })?;
                if !matches!(item_profile.kind(), ProfileKind::Composite(_)) {
                    visited.insert(item_name);
                    continue;
                }

                // add item and children to stack + add item to path
                path.push(item_name);
                stack.push((item_name, true));
                if let ProfileKind::Composite(composite) = item_profile.kind() {
                    for child in composite.entries().iter().rev() {
                        stack.push((child.child(), false));
                    }
                }
            }

            Ok(())
        } else {
            on_elem(TraverseContext {
                item: self,
                path: &[],
                stack: &[],
            })
        }
    }
}
