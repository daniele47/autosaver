use std::collections::{HashMap, HashSet};

use anyhow::{Context, bail};

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllProfiles {
    profiles: HashMap<RelPathStr, Profile>,
}

// structs to make traverse function work properly
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraverseContext<'a> {
    pub item: &'a Profile,
    pub path: &'a [&'a RelPathStr],
    pub stack: &'a [(&'a RelPathStr, bool)],
    pub is_dup: bool,
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TraverseOpts {
    pub hide_duplicates: bool,
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
}

impl AllProfiles {
    pub fn new(profiles: HashMap<RelPathStr, Profile>) -> Self {
        Self { profiles }
    }

    pub fn get(&self, name: &RelPathStr) -> anyhow::Result<&Profile> {
        self.profiles
            .get(name)
            .with_context(|| format!("Missing profile: {}", name.display()))
    }

    pub fn traverse<S>(
        &self,
        root: &RelPathStr,
        opts: TraverseOpts,
        mut on_elem: S,
    ) -> anyhow::Result<()>
    where
        S: FnMut(TraverseContext) -> anyhow::Result<()>,
    {
        let mut visited = HashSet::<&RelPathStr>::new();
        let mut path = Vec::<&RelPathStr>::new();
        let mut stack = Vec::<(&RelPathStr, bool)>::new();
        stack.push((root, false));

        // 3 colors DFS to traverse whilst properly detecting loops
        while let Some((item_name, item_visited)) = stack.pop() {
            // item already visited, aka we explored all from here, and backtracked
            if item_visited {
                path.pop();
                continue;
            }

            // check if current item is already in path, aka if this is a cycle
            if let Some(pos) = path.iter().position(|x| x == &item_name) {
                let cycle = &path[pos..]
                    .iter()
                    .chain(path.get(pos))
                    .map(|s| s.to_string_lossy())
                    .collect::<Vec<_>>()
                    .join(" --> ");
                let name = root.display();
                bail!(format!("Profile {name} has a dependency cycle: {cycle}"));
            }

            // load profile
            let item_profile = self.get(item_name).with_context(|| {
                    let name = root.to_string_lossy();
                    let inv_par = path.last().map(|p|p.to_string_lossy()).unwrap_or(name.clone());
                    let inv_name = item_name.display();
                    format!("Profile {name} traversal found invalid profile name {inv_name} as a child of {inv_par}")
                })?;

            // act on profile
            on_elem(TraverseContext {
                item: item_profile,
                path: &path,
                stack: &stack,
                is_dup: visited.contains(&item_name),
            })?;

            // end traversal if it was a duplicate, otherwise add to visited set
            if !visited.insert(item_name) && !opts.hide_duplicates {
                continue;
            }

            // add item and children to stack + add item to path if composite
            if let ProfileKind::Composite(composite) = item_profile.kind() {
                path.push(item_name);
                stack.push((item_name, true));
                for child in composite.entries().iter().rev() {
                    stack.push((child.child(), false));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::prof::composite::CompositeEntry;

    use super::*;
    use std::{collections::HashMap, str::FromStr};

    fn setup_test_profiles() -> anyhow::Result<AllProfiles> {
        let mut profiles = HashMap::new();

        // Leaf module
        let module1 = Profile::new(
            RelPathStr::from_str("module1")?,
            RelPathStr::from_str("module1")?,
            ProfileKind::Module(Module::new(vec![])),
        );
        profiles.insert(RelPathStr::from_str("module1")?, module1);

        // profile3 depends on module1
        let profile3 = Profile::new(
            RelPathStr::from_str("profile3")?,
            RelPathStr::from_str("profile3")?,
            ProfileKind::Composite(Composite::new(vec![CompositeEntry::new(
                RelPathStr::from_str("module1")?,
            )])),
        );
        profiles.insert(RelPathStr::from_str("profile3")?, profile3);

        // profile2 is a leaf (no dependencies)
        let profile2 = Profile::new(
            RelPathStr::from_str("profile2")?,
            RelPathStr::from_str("profile2")?,
            ProfileKind::Module(Module::new(vec![])),
        );
        profiles.insert(RelPathStr::from_str("profile2")?, profile2);

        // profile1 depends on profile2 and profile3
        let profile1 = Profile::new(
            RelPathStr::from_str("profile1")?,
            RelPathStr::from_str("profile1")?,
            ProfileKind::Composite(Composite::new(vec![
                CompositeEntry::new(RelPathStr::from_str("profile3")?),
                CompositeEntry::new(RelPathStr::from_str("profile2")?),
            ])),
        );
        profiles.insert(RelPathStr::from_str("profile1")?, profile1);

        Ok(AllProfiles::new(profiles))
    }

    #[test]
    fn test_traverse_full_tree() -> anyhow::Result<()> {
        let profiles = setup_test_profiles()?;
        let pname = RelPathStr::from_str("profile1")?;

        let mut visited_order = Vec::new();

        profiles.traverse(&pname, Default::default(), |ctx| {
            visited_order.push(ctx.item.name().to_string_lossy().to_string());
            Ok(())
        })?;

        assert_eq!(
            visited_order,
            vec!["profile1", "profile3", "module1", "profile2"]
        );

        Ok(())
    }
}
