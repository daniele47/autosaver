use std::collections::{HashMap, HashSet};

use anyhow::{Context, bail};

use crate::{
    fs::rel::RelPathStr,
    prof::{
        composite::{Composite, CompositeEntry},
        module::Module,
        runner::Runner,
    },
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
    pub id: Option<RelPathStr>,
    pub kind: ProfileKind,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllProfiles {
    pub profiles: HashMap<RelPathStr, Profile>,
}

// structs to make traverse function work properly
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraverseContext<'a> {
    pub name: &'a RelPathStr,
    pub item: &'a Profile,
    pub path: &'a [&'a RelPathStr],
    pub stack: &'a [(&'a RelPathStr, bool)],
    pub is_dup: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraverseDupPolicy {
    Include,
    Shallow,
    Exclude,
}

impl Profile {
    pub fn id_or<'a>(&'a self, name: &'a RelPathStr) -> &'a RelPathStr {
        self.id.as_ref().unwrap_or(name)
    }
}

impl AllProfiles {
    pub fn get(&self, name: &RelPathStr) -> anyhow::Result<&Profile> {
        self.profiles
            .get(name)
            .with_context(|| format!("Missing profile: '{}'", name.display()))
    }

    pub fn traverse(
        &self,
        root: &RelPathStr,
        on_elem: impl FnMut(TraverseContext) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        self.traverse_opts(root, TraverseDupPolicy::Exclude, |_| true, on_elem)
    }

    pub fn traverse_opts(
        &self,
        root: &RelPathStr,
        dups_policy: TraverseDupPolicy,
        ignore_elem: impl Fn(&CompositeEntry) -> bool,
        mut on_elem: impl FnMut(TraverseContext) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
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
                    .map(|s| format!("'{}'", s.display()))
                    .collect::<Vec<_>>()
                    .join(" --> ");
                let name = root.display();
                bail!(format!("Profile '{name}' has a dependency cycle: {cycle}"));
            }

            // load profile
            let item_profile = if let Some(last) = path.last() {
                self.get(item_name).with_context(|| {
                    let name = root.to_string_lossy();
                    let inv_par = last.display();
                    let inv_name = item_name.display();
                    format!("Profile '{name}' traversal found invalid profile name '{inv_name}' as a child of '{inv_par}'")
                })
            } else {
                self.get(item_name)
            }?;

            // act depending on duplicated policy
            let is_dup = !visited.insert(item_name);
            if !is_dup || dups_policy != TraverseDupPolicy::Exclude {
                on_elem(TraverseContext {
                    name: item_name,
                    item: item_profile,
                    path: &path,
                    stack: &stack,
                    is_dup,
                })?;
            }
            if is_dup && dups_policy != TraverseDupPolicy::Include {
                continue;
            }

            // add item and children to stack + add item to path if composite
            if let ProfileKind::Composite(composite) = &item_profile.kind {
                path.push(item_name);
                stack.push((item_name, true));
                for child in composite.entries.iter().filter(|i| ignore_elem(i)).rev() {
                    stack.push((&child.child, false));
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

    // DEPENDENCY GRAPH:
    //
    //      composite1
    //     /          \
    // composite2  composite3
    //     \          /
    //      composite4
    //     /          \
    //  runner1     module1
    fn setup_test_profiles() -> anyhow::Result<AllProfiles> {
        let mut profiles = HashMap::new();

        let module1 = Profile {
            id: None,
            kind: ProfileKind::Module(Module {
                entries: vec![],
                cleanup: vec![],
            }),
        };
        let runner1 = Profile {
            id: None,
            kind: ProfileKind::Runner(Runner { entries: vec![] }),
        };
        let composite1 = Profile {
            id: None,
            kind: ProfileKind::Composite(Composite {
                entries: vec![
                    CompositeEntry {
                        child: RelPathStr::from_str("composite2")?,
                    },
                    CompositeEntry {
                        child: RelPathStr::from_str("composite3")?,
                    },
                ],
            }),
        };
        let composite2 = Profile {
            id: None,
            kind: ProfileKind::Composite(Composite {
                entries: vec![CompositeEntry {
                    child: RelPathStr::from_str("composite4")?,
                }],
            }),
        };
        let composite3 = Profile {
            id: None,
            kind: ProfileKind::Composite(Composite {
                entries: vec![CompositeEntry {
                    child: RelPathStr::from_str("composite4")?,
                }],
            }),
        };
        let composite4 = Profile {
            id: None,
            kind: ProfileKind::Composite(Composite {
                entries: vec![
                    CompositeEntry {
                        child: RelPathStr::from_str("module1")?,
                    },
                    CompositeEntry {
                        child: RelPathStr::from_str("runner1")?,
                    },
                ],
            }),
        };

        profiles.insert(RelPathStr::from_str("module1")?, module1);
        profiles.insert(RelPathStr::from_str("runner1")?, runner1);
        profiles.insert(RelPathStr::from_str("composite1")?, composite1);
        profiles.insert(RelPathStr::from_str("composite2")?, composite2);
        profiles.insert(RelPathStr::from_str("composite3")?, composite3);
        profiles.insert(RelPathStr::from_str("composite4")?, composite4);

        Ok(AllProfiles { profiles })
    }

    // DEPENDENCY GRAPH:
    //
    //      composite1 <------+
    //     /          \       |
    // composite2  composite3 |
    //     \          /       |
    //      composite4 -------+
    //     /          \
    //  runner1     module1
    fn setup_test_profiles_cycle() -> anyhow::Result<AllProfiles> {
        let mut all_profiles = setup_test_profiles()?;

        let composite4 = all_profiles
            .profiles
            .get_mut(&RelPathStr::from_str("composite4")?)
            .unwrap();

        if let ProfileKind::Composite(c) = &mut composite4.kind {
            c.entries.push(CompositeEntry {
                child: RelPathStr::from_str("composite1")?,
            });
        } else {
            unreachable!()
        }

        Ok(all_profiles)
    }

    fn traverse(
        dup_policy: TraverseDupPolicy,
        ignore: impl Fn(&CompositeEntry) -> bool,
    ) -> anyhow::Result<Vec<String>> {
        let pname = RelPathStr::from_str("composite1")?;
        let mut visited_order = Vec::new();
        let all_profiles = setup_test_profiles()?;

        all_profiles.traverse_opts(&pname, dup_policy, ignore, |ctx| {
            visited_order.push(ctx.name.to_string_lossy().to_string());
            Ok(())
        })?;

        Ok(visited_order)
    }

    #[test]
    fn traverse_include() -> anyhow::Result<()> {
        let expected = &[
            "composite1",
            "composite2",
            "composite4",
            "module1",
            "runner1",
            "composite3",
            "composite4",
            "module1",
            "runner1",
        ];
        let actual = &traverse(TraverseDupPolicy::Include, |_| true)?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn traverse_shallow() -> anyhow::Result<()> {
        let expected = &[
            "composite1",
            "composite2",
            "composite4",
            "module1",
            "runner1",
            "composite3",
            "composite4",
        ];
        let actual = &traverse(TraverseDupPolicy::Shallow, |_| true)?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn traverse_exclude() -> anyhow::Result<()> {
        let expected = &[
            "composite1",
            "composite2",
            "composite4",
            "module1",
            "runner1",
            "composite3",
        ];
        let actual = &traverse(TraverseDupPolicy::Exclude, |_| true)?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn traverse_ignore() -> anyhow::Result<()> {
        let expected = &["composite1", "composite2", "composite3"];
        let actual = &traverse(TraverseDupPolicy::Exclude, |c| {
            c.child != "composite4".parse().unwrap()
        })?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn traverse_cycle_err() -> anyhow::Result<()> {
        let profiles_with_cycle = setup_test_profiles_cycle()?;
        let err = profiles_with_cycle
            .traverse(&"composite1".parse()?, |_| Ok(()))
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            "Profile 'composite1' has a dependency cycle: 'composite1' --> 'composite2' --> 'composite4' --> 'composite1'"
        );

        Ok(())
    }
}
