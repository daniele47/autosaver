use std::{
    collections::{HashMap, HashSet},
    env,
    path::PathBuf,
    str::FromStr,
};

use crate::{
    cli::{config::col::CliColor, prompt::Prompt},
    fs::{abs::AbsPathStr, rel::RelPathStr},
    prof::{AllProfiles, TraverseDupPolicy},
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
    pub exclude_all: HashSet<RelPathStr>,
    pub prompt: Prompt,
}

impl CliContext {
    pub fn new(
        home: &Option<PathBuf>,
        root: &Option<PathBuf>,
        flag_prof: &Option<RelPathStr>,
        exclude: &[RelPathStr],
        prompt: Prompt,
    ) -> anyhow::Result<Self> {
        let paths = load_env::load_paths_and_envvars(home, root)?;
        let root_profile = RelPathStr::from_str("all")?;
        let profiles =
            load_prof::load_profiles(&paths[&Paths::Config], &root_profile, &[&root_profile])?;
        let curr_profile;
        if let Some(flag_profs) = flag_prof {
            curr_profile = flag_profs.to_owned();
        } else if let Ok(prof) = env::var("AUTOSAVER_PROFILE") {
            curr_profile = RelPathStr::try_from(prof)?;
        } else {
            curr_profile = root_profile.clone();
        }
        let col = CliColor::parse_theme(&paths[&Paths::LocalConfigColors])?;
        let mut exclude_all = HashSet::new();
        for e in exclude {
            profiles.traverse(
                e,
                TraverseDupPolicy::Exclude,
                |_| true,
                |e| {
                    exclude_all.insert(e.name.to_owned());
                    Ok(())
                },
            )?;
        }
        Ok(Self {
            paths,
            root_profile,
            profiles,
            curr_profile,
            col,
            exclude_all,
            prompt,
        })
    }
}
