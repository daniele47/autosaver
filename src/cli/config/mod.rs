use std::{collections::HashMap, env, path::PathBuf, str::FromStr};

use crate::{
    cli::config::col::CliColor,
    fs::{abs::AbsPathStr, rel::RelPathStr},
    prof::AllProfiles,
};

pub mod col;
pub mod load_env;
pub mod load_prof;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Paths {
    Home,
    Root,
    Backup,
    Config,
    Run,
    LocalConfig,
    LocalConfigEnv,
    LocalConfigColors,
}
#[derive(Debug, Clone, PartialEq)]
pub struct CliContext {
    pub paths: HashMap<Paths, AbsPathStr>,
    pub root_profile: RelPathStr,
    pub profiles: AllProfiles,
    pub curr_profile: RelPathStr,
    pub col: CliColor,
}

impl CliContext {
    pub fn new(
        home: &Option<PathBuf>,
        root: &Option<PathBuf>,
        flag_prof: &Option<RelPathStr>,
    ) -> anyhow::Result<Self> {
        let paths = load_env::load_paths_and_envvars(home, root)?;
        let root_profile = RelPathStr::from_str("all")?;
        let profiles = load_prof::load_profiles(&paths[&Paths::Config], &root_profile)?;
        let curr_profile;
        if let Some(prof) = flag_prof {
            curr_profile = prof.to_owned();
        } else if let Ok(prof) = env::var("AUTOSAVER_PROFILE") {
            curr_profile = RelPathStr::try_from(prof)?;
        } else {
            curr_profile = root_profile.clone();
        }
        let col = CliColor::parse_theme(&paths[&Paths::LocalConfigColors])?;
        Ok(Self {
            paths,
            root_profile,
            profiles,
            curr_profile,
            col,
        })
    }
}
