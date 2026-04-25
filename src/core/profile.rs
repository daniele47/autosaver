//! This module implements structs and methods to handle dotfiles profiles.

use std::collections::{HashMap, VecDeque};

use crate::core::errors::{Error, Result};

/// Represents the profile type.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProfileType {
    /// Profile storing list of profiles.
    Composite,
    /// Special leaf profile with no children.
    Module,
}

/// Represents a dotfiles profile.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Profile {
    name: String,
    entries: Vec<String>,
    ptype: ProfileType,
}

/// Allow generic implementation of how profiles are loaded.
pub trait ProfileLoader {
    fn load(&mut self, name: &str) -> Result<Profile>;
}

impl Profile {
    /// Create new profile.
    pub fn new(name: String, entries: Vec<String>, ptype: ProfileType) -> Self {
        assert_eq!(entries.is_empty(), ptype == ProfileType::Module);
        Self {
            name,
            entries,
            ptype,
        }
    }

    /// Get profile name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get profile entries.
    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    /// Get profile type.
    pub fn ptype(&self) -> &ProfileType {
        &self.ptype
    }

    /// Check if profile is resolved, aka all children are modules.
    pub fn is_resolved(&self, loader: &mut impl ProfileLoader) -> bool {
        for child in &self.entries {
            let child_profile = loader.load(child);
            if let Ok(cp) = child_profile {
                if cp.ptype != ProfileType::Module {
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
    pub fn resolve(&mut self, loader: &mut impl ProfileLoader) -> Result<Self> {
        let mut entries = Vec::<String>::new();
        let mut found = HashMap::<String, String>::new(); // elem -> parent
        let mut queue = VecDeque::<String>::new();
        queue.push_back(self.name.clone());

        // TODO: write comment ONCE you modify this to use DFS
        while let Some(item) = queue.pop_front() {
            let item_profile = loader.load(&item)?;
            for child in &item_profile.entries {
                // loop detected, proceeds to calculate extra infos about the loop itself
                if found.contains_key(child) {
                    let mut faulty_child = child;
                    while let Some(parent) = found.get(faulty_child) {
                        if parent == &self.name {
                            break;
                        }
                        faulty_child = parent;
                    }
                    return Err(Error::ProfileCycle {
                        name: self.name.clone(),
                        child: faulty_child.clone(),
                    });
                }

                // add child to various variables
                let child_profile = loader.load(child)?;
                match child_profile.ptype {
                    ProfileType::Composite => {
                        queue.push_back(child.clone());
                    }
                    ProfileType::Module => {
                        entries.push(child.clone());
                    }
                }
                assert!(found.insert(child.clone(), item.clone()).is_none());
            }
        }

        // assert resolved profile is indeed resolved
        let res = Self::new(self.name.clone(), entries, self.ptype);
        assert!(res.is_resolved(loader));
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestProfileLoader {
        profiles: HashMap<String, Profile>,
    }

    impl TestProfileLoader {
        fn new(extra_profiles: Vec<Profile>) -> Self {
            let mut loader = Self {
                profiles: HashMap::new(),
            };
            let mut profiles = vec![
                Profile::new(
                    "root".to_string(),
                    vec![
                        "composite1".to_string(),
                        "module1".to_string(),
                        "composite3".to_string(),
                    ],
                    ProfileType::Composite,
                ),
                Profile::new(
                    "composite1".to_string(),
                    vec!["composite2".to_string(), "module2".to_string()],
                    ProfileType::Composite,
                ),
                Profile::new(
                    "composite2".to_string(),
                    vec!["module4".to_string()],
                    ProfileType::Composite,
                ),
                Profile::new(
                    "composite3".to_string(),
                    vec!["module3".to_string()],
                    ProfileType::Composite,
                ),
                Profile::new("module1".to_string(), vec![], ProfileType::Module),
                Profile::new("module2".to_string(), vec![], ProfileType::Module),
                Profile::new("module3".to_string(), vec![], ProfileType::Module),
                Profile::new("module4".to_string(), vec![], ProfileType::Module),
            ];
            profiles.extend(extra_profiles);
            for p in profiles {
                loader.profiles.insert(p.name().to_string(), p);
            }
            loader
        }
    }

    impl ProfileLoader for TestProfileLoader {
        fn load(&mut self, name: &str) -> Result<Profile> {
            self.profiles
                .get(name)
                .cloned()
                .ok_or(Error::ProfileNotLoaded {
                    name: name.into(),
                    reason: "Test code failure".into(),
                })
        }
    }

    #[test]
    fn test_resolve_success() -> Result<()> {
        let mut profile = Profile::new(
            "root".to_string(),
            vec!["composite1".to_string(), "module1".to_string()],
            ProfileType::Composite,
        );
        let mut loader = TestProfileLoader::new(vec![]);

        // Check resolve works as intended
        let actual = profile.resolve(&mut loader)?;
        let expected = Profile::new(
            "root".to_string(),
            vec![
                "module1".to_string(),
                "module2".to_string(),
                "module3".to_string(),
                "module4".to_string(),
            ],
            ProfileType::Composite,
        );
        assert_eq!(expected, actual);

        // Test is_resolved function
        assert!(expected.is_resolved(&mut loader));
        assert!(actual.is_resolved(&mut loader));

        Ok(())
    }

    #[test]
    fn test_resolve_failure() -> Result<()> {
        let mut profile = Profile::new(
            "root".to_string(),
            vec!["composite1".to_string(), "module1".to_string()],
            ProfileType::Composite,
        );
        let mut loader = TestProfileLoader::new(vec![Profile::new(
            "composite2".to_string(),
            vec!["composite1".to_string()],
            ProfileType::Composite,
        )]);

        // Make sure resolve fails when a loop exists
        let actual = profile.resolve(&mut loader);
        match actual {
            Ok(_) => {}
            Err(err) => match err {
                Error::ProfileCycle { name, child } => {
                    assert_eq!(name.as_str(), "root");
                    assert_eq!(child.as_str(), "composite1");
                }
                _ => unreachable!(),
            },
        }

        Ok(())
    }
}
