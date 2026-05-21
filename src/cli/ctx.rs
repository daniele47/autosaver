use std::{collections::HashMap, env, str::FromStr};

use anyhow::{Context, bail};
use owo_colors::Style;

use crate::{
    fs::{abs::AbsPathStr, rel::RelPathStr},
    prof::{
        AllProfiles, Profile, ProfileKind,
        composite::{Composite, CompositeEntry},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Paths {
    Home,
    Root,
    Backup,
    Config,
    Run,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliContext {
    paths: HashMap<Paths, AbsPathStr>,
    root_profile: RelPathStr,
    profiles: AllProfiles,
}

impl CliContext {
    pub(super) const TREE_COMPOSITE: Style = Style::new();
    pub(super) const TREE_RUNNER: Style = Style::new().bright_green();
    pub(super) const TREE_MODULE: Style = Style::new().bright_blue();
    pub(super) const TREE_DEDUP: Style = Style::new().yellow();

    pub fn new(home: &Option<AbsPathStr>, root: &Option<AbsPathStr>) -> anyhow::Result<Self> {
        let paths = Self::load_paths(home, root)?;
        let root_profile = RelPathStr::from_str("all")?;
        let profiles = Self::load_profiles(&paths[&Paths::Config], &root_profile)?;
        Ok(Self {
            paths,
            root_profile,
            profiles,
        })
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

        paths.insert(Paths::Home, home_dir);
        paths.insert(Paths::Root, root_dir);
        paths.insert(Paths::Backup, backup_dir);
        paths.insert(Paths::Run, run_dir);
        paths.insert(Paths::Config, config_dir);

        Ok(paths)
    }

    fn load_vt_profile(
        config_dir: &AbsPathStr,
        path: &AbsPathStr,
        name: RelPathStr,
    ) -> anyhow::Result<Profile> {
        let mut comp_entries = vec![];

        path.list(|ctx| {
            let ftype = ctx.entry.file_type()?;
            let fname = ctx.entry.file_name();
            let fname = fname.to_string_lossy();
            let conf_rel = ctx.path.to_rel(config_dir)?;
            let conf_str = conf_rel.to_string_lossy();
            let comp_entry;

            // skip dotfiles
            if fname.starts_with(".") {
                return Ok(());
            }

            // load child
            if ftype.is_dir() {
                comp_entry = CompositeEntry::new(conf_rel);
            } else if let Some(pname) = conf_str.strip_suffix(".conf") {
                comp_entry = CompositeEntry::new(RelPathStr::from_str(pname)?);
            } else {
                return Ok(());
            }

            // add child
            comp_entries.push(comp_entry);

            Ok(())
        })?;

        Ok(Profile::new(
            name.clone(),
            name,
            ProfileKind::Composite(Composite::new(comp_entries)),
        ))
    }

    fn load_profiles(
        config_dir: &AbsPathStr,
        root_profile: &RelPathStr,
    ) -> anyhow::Result<AllProfiles> {
        let mut all_profiles = HashMap::new();

        // load only empty all profile if config dir is missing
        if !config_dir.is_dir() {
            let config_dir = config_dir.display();
            bail!("Configuration directory is missing at {config_dir}");
        }

        // add all virtual profile
        let profile = Self::load_vt_profile(config_dir, config_dir, root_profile.clone())?;
        if let Some(old) = all_profiles.insert(root_profile.clone(), profile) {
            let old_name = old.name().display();
            bail!(format!("Profile {old_name} is reserved for root profile"));
        }

        // find and load all profiles config files
        config_dir.find(|ctx| {
            let ftype = ctx.entry.file_type()?;
            let fname = ctx.entry.file_name();
            let fname = fname.to_string_lossy();
            let conf_rel = ctx.path.to_rel(config_dir)?;
            let conf_str = conf_rel.to_string_lossy();
            let profile;

            // ignore dotfiles in config directory
            if fname.starts_with(".") {
                return Ok(false);
            }

            // virtual directory parsing
            if ftype.is_dir() {
                profile = Self::load_vt_profile(config_dir, &ctx.path, conf_rel)?;
            }
            // normal profile parsing
            else if let Some(pname) = conf_str.strip_suffix(".conf") {
                profile = Profile::parse_config(&ctx.path.read_file()?, pname)?;
            }
            // otherwise do nothing
            else {
                return Ok(true);
            }

            // insert profile
            if let Some(old) = all_profiles.insert(profile.name().clone(), profile) {
                let old_name = old.name().display();
                bail!(format!("Profile {old_name} is loaded multiple times"));
            }

            Ok(true)
        })?;

        Ok(AllProfiles::new(all_profiles))
    }

    pub fn path(&self, path: &Paths) -> &AbsPathStr {
        &self.paths[path]
    }

    pub fn root_profile(&self) -> &RelPathStr {
        &self.root_profile
    }

    pub fn profiles(&self) -> &AllProfiles {
        &self.profiles
    }

    pub fn curr_prof<'a>(&'a self, flag_prof: &'a Option<RelPathStr>) -> &'a RelPathStr {
        flag_prof.as_ref().unwrap_or(&self.root_profile)
    }
}
