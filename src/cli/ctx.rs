use std::{collections::HashMap, env, str::FromStr};

use anyhow::Context;
use tracing::trace;

use crate::{
    fs::{abs::AbsPathStr, rel::RelPathStr},
    prof::AllProfiles,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Paths {
    Home,
    Root,
    Backup,
    Config,
    Run,
}
pub struct CliContext {
    paths: HashMap<Paths, AbsPathStr>,
    profiles: AllProfiles,
}

impl CliContext {
    pub fn new(home: &Option<AbsPathStr>, root: &Option<AbsPathStr>) -> anyhow::Result<Self> {
        let paths = Self::load_paths(home, root)?;
        let profiles = Self::load_profiles(&paths[&Paths::Config])?;
        Ok(Self { paths, profiles })
    }

    fn load_paths(
        home: &Option<AbsPathStr>,
        root: &Option<AbsPathStr>,
    ) -> anyhow::Result<HashMap<Paths, AbsPathStr>> {
        let mut paths = HashMap::new();

        // load home directory
        let home_dir;
        if let Some(home) = home {
            home_dir = home.clone();
        } else {
            let home = env::home_dir().context("Failure getting home directory")?;
            home_dir = AbsPathStr::new_from_pathbuf(home).context("Invalid home directory")?;
        }

        // load root directory
        let root_dir;
        if let Some(root) = root {
            root_dir = root.clone();
        } else {
            let root = env::current_dir().context("Failure getting root directory")?;
            root_dir = AbsPathStr::new_from_pathbuf(root).context("Invalid root directory")?;
        }

        // other dirs
        let backup_dir = root_dir.join(&RelPathStr::from_str("backup")?)?;
        let config_dir = root_dir.join(&RelPathStr::from_str("config")?)?;
        let run_dir = root_dir.join(&RelPathStr::from_str("run")?)?;

        trace!(home_dir=%home_dir.display(),"Home directory:");
        trace!(root_dir=%root_dir.display(),"Root directory:");
        trace!(backup_dir=%backup_dir.display(),"Backup directory:");
        trace!(run_dir=%run_dir.display(),"Run directory:");
        trace!(config_dir=%config_dir.display(),"Config directory:");
        paths.insert(Paths::Home, home_dir);
        paths.insert(Paths::Root, root_dir);
        paths.insert(Paths::Backup, backup_dir);
        paths.insert(Paths::Run, run_dir);
        paths.insert(Paths::Config, config_dir);

        Ok(paths)
    }

    fn load_profiles(config_dir: &AbsPathStr) -> anyhow::Result<AllProfiles> {
        let all_profiles = AllProfiles::new();

        dbg!(all_profiles, config_dir);
        todo!("implement profile loader function");

        // // load nothing if there are no profiles
        // if !config_dir.is_dir() {
        //     return Ok(all_profiles);
        // }
        //
        // // find and load all profiles config files
        // config_dir.find(|ctx| {
        //     println!("IMPLEMENT PROFILE LOADER FUNCTION! {ctx:?}");
        //     Ok(())
        // })?;
        //
        // Ok(all_profiles)
    }

    pub fn path(&self, path: &Paths) -> &AbsPathStr {
        &self.paths[path]
    }

    pub fn profiles(&self) -> &AllProfiles {
        &self.profiles
    }
}
