use std::{
    collections::{HashMap, hash_map::Entry},
    env,
    path::PathBuf,
    str::FromStr,
};

use anyhow::{Context, bail};
use indexmap::IndexSet;

use crate::{
    cli::col::CliColor,
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
        let paths = Self::load_paths(home, root)?;
        let root_profile = RelPathStr::from_str("all")?;
        let profiles = Self::load_profiles(&paths[&Paths::Config], &root_profile)?;
        let curr_profile = flag_prof.as_ref().unwrap_or(&root_profile).to_owned();
        let col = CliColor::default_theme();
        Ok(Self {
            paths,
            root_profile,
            profiles,
            curr_profile,
            col,
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

    fn load_profiles(
        config_dir: &AbsPathStr,
        root_profile: &RelPathStr,
    ) -> anyhow::Result<AllProfiles> {
        let mut vt_names = IndexSet::new();
        let mut vt_profiles = vec![];
        let mut vt_entries = vec![];

        // error if config directory is missing
        if !config_dir.is_dir() {
            let config_dir = config_dir.display();
            bail!("Configuration directory is missing at {config_dir}");
        }

        // find and load all profiles config files
        config_dir.find(|ctx| {
            let ftype = ctx.entry.file_type()?;
            let conf_rel = ctx.path.to_rel(config_dir)?;

            // skip dotfiles configs
            if ctx.entry.file_name().to_string_lossy().starts_with(".") {
                let p = conf_rel.display();
                bail!(format!("Configuration file '{p}' starts with a dot"));
            }

            // virtual directory parsing
            if ftype.is_dir() {
                // insert profile
                let (index_this, _) = vt_names.insert_full(conf_rel);

                // insert parent profile
                let parent = &vt_names[index_this].path().parent().expect("no parent");
                let parent = RelPathStr::new_from_pathbuf(PathBuf::from(parent))?;
                let (index_parent, _) = vt_names.insert_full(parent);
                vt_entries.push((index_parent, index_this));
            }
            // normal profile parsing
            else if ftype.is_file()
                && let Some(pname) = conf_rel.to_string_lossy().strip_suffix(".conf")
            {
                // parse profile
                let (index_this, _) = vt_names.insert_full(RelPathStr::from_str(pname)?);
                let profile = Profile::parse_config(&ctx.path.read_file()?, pname)?;
                vt_profiles.push((index_this, profile));

                // insert parent profile
                let parent = &vt_names[index_this].path().parent().expect("no parent");
                let parent = RelPathStr::new_from_pathbuf(PathBuf::from(parent))?;
                let (index_parent, _) = vt_names.insert_full(parent);
                vt_entries.push((index_parent, index_this));
            }
            // otherwise do nothing
            else {
                return Ok(true);
            }

            Ok(true)
        })?;

        // put all profiles togheter
        let mut all_profiles = HashMap::new();
        let mut all_entries = HashMap::<RelPathStr, Vec<CompositeEntry>>::new();
        for (i, prof) in vt_profiles {
            all_profiles.insert(vt_names[i].clone(), prof);
        }
        for (i, j) in vt_entries {
            all_entries
                .entry(vt_names[i].clone())
                .or_default()
                .push(CompositeEntry::new(vt_names[j].clone()));
        }
        for (name, entries) in all_entries {
            let profile = Profile::new(None, ProfileKind::Composite(Composite::new(entries)));
            match all_profiles.entry(name) {
                Entry::Vacant(v) => {
                    v.insert(profile);
                }
                Entry::Occupied(o) => {
                    let name = o.key().display();
                    bail!("Profile named '{name}' is both a directory and a config file");
                }
            }
        }
        for pname in vt_names {
            if !all_profiles.contains_key(&pname) {
                let profile = Profile::new(None, ProfileKind::Composite(Composite::new(vec![])));
                all_profiles.insert(pname, profile);
            }
        }

        // handle root profile
        if all_profiles.contains_key(root_profile) {
            let name = root_profile.display();
            bail!("Profile name '{name}' is reserved for root profile");
        }
        if let Some(value) = all_profiles.remove(&RelPathStr::from_str("")?) {
            all_profiles.insert(root_profile.to_owned(), value);
        }

        // handle empty config dir
        if !all_profiles.contains_key(root_profile) {
            let profile = Profile::new(None, ProfileKind::Composite(Composite::new(vec![])));
            all_profiles.insert(root_profile.to_owned(), profile);
        }

        Ok(AllProfiles::new(all_profiles))
    }
}
