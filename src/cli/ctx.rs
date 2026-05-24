use std::{
    collections::{HashMap, hash_map::Entry},
    env,
    path::PathBuf,
    str::FromStr,
};

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
    pub paths: HashMap<Paths, AbsPathStr>,
    pub root_profile: RelPathStr,
    pub profiles: AllProfiles,
    pub curr_profile: RelPathStr,
}

impl CliContext {
    pub const TREE_COMPOSITE: Style = Style::new();
    pub const TREE_RUNNER: Style = Style::new().green();
    pub const TREE_MODULE: Style = Style::new().bright_blue();
    pub const TREE_DEDUP: Style = Style::new().yellow();
    pub const PROMPT_MSG: Style = Style::new().underline();
    pub const OUTPUT_PATH: Style = Style::new().bright_blue();
    pub const DIFF_DELETED: Style = Style::new().red();
    pub const DIFF_INSERTED: Style = Style::new().green();
    pub const DIFF_HEADER: Style = Style::new().cyan();
    pub const SHOW_HEADER: Style = Style::new().cyan();

    pub fn new(
        home: &Option<PathBuf>,
        root: &Option<PathBuf>,
        flag_prof: &Option<RelPathStr>,
    ) -> anyhow::Result<Self> {
        let paths = Self::load_paths(home, root)?;
        let root_profile = RelPathStr::from_str("all")?;
        let profiles = Self::load_profiles(&paths[&Paths::Config], &root_profile)?;
        let curr_profile = flag_prof.as_ref().unwrap_or(&root_profile).to_owned();
        Ok(Self {
            paths,
            root_profile,
            profiles,
            curr_profile,
        })
    }

    fn load_paths(
        home: &Option<PathBuf>,
        root: &Option<PathBuf>,
    ) -> anyhow::Result<HashMap<Paths, AbsPathStr>> {
        let mut paths = HashMap::new();

        // load home directory
        let home_dir;
        if let Some(home) = home {
            home_dir = home.canonicalize().with_context(|| {
                format!("Home path could not be canonicalized: {}", home.display())
            })?;
        } else {
            home_dir = env::home_dir().context("Failure getting home directory")?;
        }
        let home_dir = AbsPathStr::new_from_pathbuf(home_dir)?;

        // load root directory
        let root_dir;
        if let Some(root) = root {
            root_dir = root.canonicalize().with_context(|| {
                format!("Root path could not be canonicalized: {}", root.display())
            })?;
        } else {
            root_dir = env::current_dir().context("Failure getting root directory")?;
        }
        let root_dir = AbsPathStr::new_from_pathbuf(root_dir)?;

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

    fn load_vt_profile(config_dir: &AbsPathStr, path: &AbsPathStr) -> anyhow::Result<Profile> {
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

        let composite = ProfileKind::Composite(Composite::new(comp_entries));
        Ok(Profile::new(None, composite))
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
        let profile = Self::load_vt_profile(config_dir, config_dir)?;
        all_profiles.insert(root_profile.to_owned(), profile);

        // find and load all profiles config files
        config_dir.find(|ctx| {
            let ftype = ctx.entry.file_type()?;
            let fname = ctx.entry.file_name();
            let fname = fname.to_string_lossy();
            let mut conf_rel = ctx.path.to_rel(config_dir)?;
            let conf_str = conf_rel.to_string_lossy();
            let profile;

            // ignore dotfiles in config directory
            if fname.starts_with(".") {
                return Ok(false);
            }

            // virtual directory parsing
            if ftype.is_dir() {
                profile = Self::load_vt_profile(config_dir, &ctx.path)?;
            }
            // normal profile parsing
            else if let Some(pname) = conf_str.strip_suffix(".conf") {
                profile = Profile::parse_config(&ctx.path.read_file()?, pname)?;
                conf_rel = RelPathStr::from_str(pname)?;
            }
            // otherwise do nothing
            else {
                return Ok(true);
            }

            // insert profile
            match all_profiles.entry(conf_rel) {
                Entry::Vacant(entry) => {
                    entry.insert(profile);
                }
                Entry::Occupied(entry) => {
                    let old_name = entry.key().display();
                    bail!(format!("Profile {old_name} is loaded multiple times"));
                }
            }

            Ok(true)
        })?;

        Ok(AllProfiles::new(all_profiles))
    }
}
