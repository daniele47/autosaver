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

        while let Some(item) = queue.pop_front() {
            let item_profile = loader.load(&item)?;
            for child in &item_profile.entries {
                if found.contains_key(child) {
                    let mut faulty_child = child;
                    while let Some(parent) = found.get(faulty_child) {
                        faulty_child = parent;
                    }
                    return Err(Error::ProfileCycle(item.clone(), faulty_child.clone()));
                }
                let child_profile = loader.load(&child)?;
                match child_profile.ptype {
                    ProfileType::Composite => {
                        queue.push_back(child.clone());
                        found.insert(child.clone(), item.clone());
                    }
                    ProfileType::Module => {
                        found.insert(child.clone(), item.clone());
                        entries.push(child.clone());
                    }
                }
            }
        }

        Ok(Self::new(self.name.clone(), entries, self.ptype))
    }
}
