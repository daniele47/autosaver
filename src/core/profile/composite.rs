//! This module implements structs and methods to handle autosaver composite profile.

use std::collections::{HashMap, HashSet};

use crate::core::{
    error::{Error, Result},
    profile::{Profile, ProfileType},
};

/// Struct representing a composite profile.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Composite {
    entries: Vec<String>,
}

/// Allow generic implementation of how profiles are loaded.
pub trait ProfileLoader {
    fn load(&mut self, name: &str) -> Result<Profile>;
}

/// Simple implementation of profile loader.
#[derive(Debug, Clone, Default)]
pub struct HashMapProfileLoader {
    profiles: HashMap<String, Profile>,
}

impl HashMapProfileLoader {
    /// Create new empty HashMapProfileLoader.
    pub fn new() -> Self {
        Default::default()
    }

    /// Allow mutating the profiles.
    pub fn profiles(&mut self) -> &mut HashMap<String, Profile> {
        &mut self.profiles
    }
}

impl ProfileLoader for HashMapProfileLoader {
    fn load(&mut self, name: &str) -> Result<Profile> {
        self.profiles.get(name).cloned().ok_or_else(|| {
            Error::ProfileLoadingFailure(name.into(), "Profile was not in the hashmap".into())
        })
    }
}

impl Composite {
    /// Create a new composite profile.
    pub fn new(entries: Vec<String>) -> Self {
        Self { entries }
    }

    /// Create an empty composite profile.
    pub fn empty() -> Self {
        Self::new(vec![])
    }

    /// Add an entry.
    pub fn add_entry(&mut self, entry: String) {
        self.entries.push(entry);
    }

    /// Get entries.
    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    /// Check if profile is resolved, aka all children are modules.
    pub fn is_resolved(&self, loader: &mut impl ProfileLoader) -> bool {
        for child in self.entries() {
            let child_profile = loader.load(child);
            if let Ok(cp) = child_profile {
                if !matches!(cp.ptype, ProfileType::Module(_)) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    /// Profile Resolver function.
    ///
    /// Profile has 2 implicit states:
    /// - raw: loaded as is from a config files, with possible duplication, with not leaf children
    /// - resolved: cleanup of duplicates, and with all leaf children resolved
    ///
    /// This function serves that role, in trasforming a raw profile into a resolved one.
    pub fn resolve(&self, profile: &str, loader: &mut impl ProfileLoader) -> Result<Self> {
        let mut entries = Vec::<String>::new();
        let mut visited = HashSet::<String>::new();
        let mut path = Vec::<String>::new();
        let mut stack = Vec::<(String, bool)>::new();
        stack.push((profile.to_string(), false));

        // 3 colors DFS to resolve the profile dependencies and also detect a loops
        while let Some((item_name, item_visited)) = stack.pop() {
            // grey -> black: item already visited, aka we explored all from here, and backtracked
            if item_visited {
                path.pop();
                visited.insert(item_name);
                continue;
            }

            // check if current item is already in path, aka if this is a cycle
            if let Some(pos) = path.iter().position(|x| x == &item_name) {
                let cycle = path[pos..].to_vec();
                return Err(Error::ProfileCycle(profile.to_string(), cycle));
            }

            // avoid revisiting already explored items, if graphs are complex and the same node is
            // reached multiple times from different nodes
            if visited.contains(&item_name) {
                continue;
            }

            // check if leaf profile
            let item_profile = loader.load(&item_name)?;
            if matches!(item_profile.ptype, ProfileType::Module(_)) {
                entries.push(item_name.clone());
                visited.insert(item_name);
                continue;
            }

            // add item and children to stack + add item to path
            path.push(item_name.clone());
            stack.push((item_name.clone(), true));
            if let ProfileType::Composite(composite) = item_profile.ptype {
                for child in composite.entries().iter().rev() {
                    stack.push((child.clone(), false));
                }
            }
        }

        // assert resolved profile is indeed resolved
        let res = Self::new(entries);
        assert!(res.is_resolved(loader));
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::core::profile::module::Module;

    use super::*;

    fn new_loader(extra_profiles: Vec<Profile>) -> HashMapProfileLoader {
        let mut loader = HashMapProfileLoader {
            profiles: HashMap::new(),
        };
        let empty_module = Module::empty();
        let mut profiles = vec![
            Profile::new(
                "root".to_string(),
                ProfileType::Composite(Composite::new(vec![
                    "composite1".to_string(),
                    "module1".to_string(),
                    "composite3".to_string(),
                ])),
            ),
            Profile::new(
                "composite1".to_string(),
                ProfileType::Composite(Composite::new(vec![
                    "composite2".to_string(),
                    "module2".to_string(),
                ])),
            ),
            Profile::new(
                "composite2".to_string(),
                ProfileType::Composite(Composite::new(vec!["module4".to_string()])),
            ),
            Profile::new(
                "composite3".to_string(),
                ProfileType::Composite(Composite::new(vec!["module3".to_string()])),
            ),
            Profile::new(
                "module1".to_string(),
                ProfileType::Module(empty_module.clone()),
            ),
            Profile::new(
                "module2".to_string(),
                ProfileType::Module(empty_module.clone()),
            ),
            Profile::new(
                "module3".to_string(),
                ProfileType::Module(empty_module.clone()),
            ),
            Profile::new(
                "module4".to_string(),
                ProfileType::Module(empty_module.clone()),
            ),
        ];
        profiles.extend(extra_profiles);
        for p in profiles {
            loader.profiles.insert(p.name().to_string(), p);
        }
        loader
    }

    #[test]
    fn test_resolve_success() -> Result<()> {
        let mut profile = Composite::new(vec!["composite1".to_string(), "module1".to_string()]);
        let profile_name = "root";
        let mut loader = new_loader(vec![]);

        // Check resolve works as intended
        let actual = profile.resolve(profile_name, &mut loader)?;
        let expected = Composite::new(vec![
            "module4".to_string(),
            "module2".to_string(),
            "module1".to_string(),
            "module3".to_string(),
        ]);
        assert_eq!(expected, actual);

        // Test is_resolved function
        assert!(expected.is_resolved(&mut loader));
        assert!(actual.is_resolved(&mut loader));

        Ok(())
    }

    #[test]
    fn test_resolve_failure() -> Result<()> {
        let mut profile = Composite::new(vec!["composite1".to_string(), "module1".to_string()]);
        let mut loader = new_loader(vec![Profile::new(
            "composite2".to_string(),
            ProfileType::Composite(Composite::new(vec!["composite1".to_string()])),
        )]);
        let profile_name = "root";

        // Make sure resolve fails when a loop exists
        let actual = profile.resolve(profile_name, &mut loader);
        match actual {
            Ok(_) => {}
            Err(err) => match err {
                Error::ProfileCycle(name, cycle) => {
                    assert_eq!(name.as_str(), "root");
                    assert_eq!(cycle.join(" "), "composite1 composite2");
                }
                _ => unreachable!(),
            },
        }

        Ok(())
    }
}
