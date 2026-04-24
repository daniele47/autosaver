// main.rs
use std::collections::HashMap;

use dotfiles_rust::core::{
    errors::{Error, Result},
    profile::{Profile, ProfileLoader, ProfileType},
};

#[derive(Debug)]
struct TestLoader {
    profiles: HashMap<String, Profile>,
}

impl TestLoader {
    fn new() -> Self {
        let mut loader = Self {
            profiles: HashMap::new(),
        };

        // Create test profiles
        let profiles = vec![
            Profile::new(
                "root".to_string(),
                vec!["composite1".to_string(), "module1".to_string()],
                ProfileType::Composite,
            ),
            Profile::new(
                "composite1".to_string(),
                vec!["composite2".to_string(), "module2".to_string()],
                ProfileType::Composite,
            ),
            Profile::new(
                "composite2".to_string(),
                vec!["module3".to_string()],
                ProfileType::Composite,
            ),
            Profile::new("module1".to_string(), vec![], ProfileType::Module),
            Profile::new("module2".to_string(), vec![], ProfileType::Module),
            Profile::new("module3".to_string(), vec![], ProfileType::Module),
        ];
        let profiles = vec![
            Profile::new(
                "root".to_string(),
                vec!["composite1".to_string(), "module1".to_string()],
                ProfileType::Composite,
            ),
            Profile::new(
                "composite1".to_string(),
                vec!["composite2".to_string(), "module2".to_string()],
                ProfileType::Composite,
            ),
            Profile::new(
                "composite2".to_string(),
                vec!["composite1".to_string()], // ← CYCLE! Points back to composite1
                ProfileType::Composite,
            ),
            Profile::new("module1".to_string(), vec![], ProfileType::Module),
            Profile::new("module2".to_string(), vec![], ProfileType::Module),
        ];
        let profiles = vec![
            // Root
            Profile::new(
                "root".to_string(),
                vec![
                    "work".to_string(),
                    "personal".to_string(),
                    "gaming".to_string(),
                ],
                ProfileType::Composite,
            ),
            // Work branch
            Profile::new(
                "work".to_string(),
                vec![
                    "coding".to_string(),
                    "meetings".to_string(),
                    "devops".to_string(),
                ],
                ProfileType::Composite,
            ),
            Profile::new(
                "coding".to_string(),
                vec!["rust".to_string(), "python".to_string(), "work".to_string()], // ← CYCLE: work
                ProfileType::Composite,
            ),
            Profile::new(
                "meetings".to_string(),
                vec!["standup".to_string(), "retro".to_string()],
                ProfileType::Composite,
            ),
            Profile::new(
                "devops".to_string(),
                vec!["cicd".to_string(), "coding".to_string()], // ← CYCLE: coding
                ProfileType::Composite,
            ),
            // Personal branch
            Profile::new(
                "personal".to_string(),
                vec![
                    "hobbies".to_string(),
                    "chores".to_string(),
                    "coding".to_string(),
                ], // ← CYCLE: coding (cross-branch)
                ProfileType::Composite,
            ),
            Profile::new(
                "hobbies".to_string(),
                vec!["music".to_string(), "gaming".to_string()], // ← CYCLE: gaming (another one!)
                ProfileType::Composite,
            ),
            Profile::new(
                "chores".to_string(),
                vec!["cleaning".to_string(), "shopping".to_string()],
                ProfileType::Composite,
            ),
            // Gaming branch
            Profile::new(
                "gaming".to_string(),
                vec![
                    "steam".to_string(),
                    "epic".to_string(),
                    "personal".to_string(),
                ], // ← CYCLE: personal
                ProfileType::Composite,
            ),
            // Leaf modules
            Profile::new("rust".to_string(), vec![], ProfileType::Module),
            Profile::new("python".to_string(), vec![], ProfileType::Module),
            Profile::new("standup".to_string(), vec![], ProfileType::Module),
            Profile::new("retro".to_string(), vec![], ProfileType::Module),
            Profile::new("cicd".to_string(), vec![], ProfileType::Module),
            Profile::new("music".to_string(), vec![], ProfileType::Module),
            Profile::new("cleaning".to_string(), vec![], ProfileType::Module),
            Profile::new("shopping".to_string(), vec![], ProfileType::Module),
            Profile::new("steam".to_string(), vec![], ProfileType::Module),
            Profile::new("epic".to_string(), vec![], ProfileType::Module),
        ];

        for p in profiles {
            loader.profiles.insert(p.name().to_string(), p);
        }

        loader
    }
}

impl ProfileLoader for TestLoader {
    fn load(&mut self, name: &str) -> Result<Profile> {
        Ok(self.profiles.get(name).cloned().unwrap())
    }
}

fn main() {
    let mut loader = TestLoader::new();
    let mut profile = Profile::new(
        "root".to_string(),
        vec!["composite1".to_string(), "module1".to_string()],
        ProfileType::Composite,
    );

    match profile.resolve(&mut loader) {
        Ok(resolved) => {
            println!("Resolved successfully!");
            println!("Name: {}", resolved.name());
            println!("Entries: {:?}", resolved.entries());
            println!("Type: {:?}", resolved.ptype());
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
