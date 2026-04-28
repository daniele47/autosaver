//! This module implements structs and methods to handle autosaver composite profile.

/// Struct representing a composite profile.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Composite {
    entries: Vec<String>,
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
}
