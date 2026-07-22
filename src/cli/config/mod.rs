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
    pub custom_profile: RelPathStr,
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
        flag_profs: &[RelPathStr],
        exclude: &[RelPathStr],
        prompt: Prompt,
    ) -> anyhow::Result<Self> {
        let paths = load_env::load_paths_and_envvars(home, root)?;
        let root_profile = RelPathStr::from_str("all")?;
        let custom_profile = RelPathStr::from_str("custom")?;
        let profiles = load_prof::load_profiles(
            &paths[&Paths::Config],
            &root_profile,
            &custom_profile,
            flag_profs,
        )?;
        let curr_profile;
        if flag_profs.len() == 1
            && let Some(flag_profs) = flag_profs.first()
        {
            curr_profile = flag_profs.to_owned();
        } else if !flag_profs.is_empty() {
            curr_profile = custom_profile.to_owned();
        } else if let Ok(prof) = env::var("AUTOSAVER_PROFILE") {
            curr_profile = RelPathStr::try_from(prof)?;
        } else {
            curr_profile = root_profile.to_owned();
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
            custom_profile,
            profiles,
            curr_profile,
            col,
            exclude_all,
            prompt,
        })
    }
}
