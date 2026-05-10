//! This module implements structs and methods to handle autosaver profiles.

use crate::core::{
    error::Result,
    profile::{
        composite::{Composite, DescendContext, ProfileLoader},
        module::Module,
        runner::Runner,
    },
};

pub mod composite;
pub mod module;
pub mod runner;

/// Represents the profile type.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ProfileType {
    /// Profile storing list of profiles.
    Composite(Composite),
    /// Profile to list dotfiles.
    Module(Module),
    /// Profile to list run scripts.
    Runner(Runner),
}

/// Represents a autosaver profile.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Profile {
    name: String,
    ptype: ProfileType,
}

impl Profile {
    /// Create new profile.
    pub fn new(name: String, ptype: ProfileType) -> Self {
        Self { name, ptype }
    }

    /// Get profile name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get profile type.
    pub fn ptype(&self) -> &ProfileType {
        &self.ptype
    }

    /// Descend and act upon all nodes found.
    pub fn descend<T, S>(&self, loader: &mut T, mut on_elem: S) -> Result<()>
    where
        T: ProfileLoader,
        S: FnMut(DescendContext) -> Result<()>,
    {
        if let ProfileType::Composite(c) = self.ptype() {
            c.descend(&self.name, loader, on_elem)
        } else {
            on_elem(DescendContext::new(self, &[], &[]))
        }
    }

    /// Resolve composite profiles and get a list of all included profiles.
    pub fn resolve(&self, loader: &mut impl ProfileLoader) -> Result<Vec<Self>> {
        let mut res = vec![];
        match self.ptype() {
            ProfileType::Composite(composite) => {
                let resolved = composite.resolve(&self.name, loader)?;
                for entry in resolved.entries() {
                    res.push(loader.load(entry)?);
                }
            }
            _ => {
                res.push(self.clone());
            }
        }
        Ok(res)
    }
}
