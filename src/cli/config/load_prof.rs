use std::{
    collections::{HashMap, hash_map::Entry},
    path::PathBuf,
    str::FromStr,
};

use anyhow::bail;
use indexmap::IndexSet;

use crate::{
    fs::{abs::AbsPathStr, rel::RelPathStr},
    prof::{
        AllProfiles, Profile, ProfileKind,
        composite::{Composite, CompositeEntry},
    },
};

pub fn load_profiles(
    config_dir: &AbsPathStr,
    root_profile: &RelPathStr,
) -> anyhow::Result<AllProfiles> {
    let mut vt_names = IndexSet::new();
    let mut vt_profiles = vec![];
    let mut vt_entries = vec![];

    // find and load all profiles config files
    if config_dir.is_dir() {
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
    }

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
    for (name, mut entries) in all_entries {
        entries.sort_unstable_by(|a, b| a.child().cmp(b.child()));
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
        all_profiles
            .entry(pname)
            .or_insert_with(|| Profile::new(None, ProfileKind::Composite(Composite::new(vec![]))));
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
