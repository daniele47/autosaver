use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, bail};

use crate::{
    cli::config::Paths,
    fs::{abs::AbsPathStr, rel::RelPathStr},
};

pub fn load_paths_and_envvars(
    home: &Option<PathBuf>,
    root: &Option<PathBuf>,
) -> anyhow::Result<HashMap<Paths, AbsPathStr>> {
    let mut paths = HashMap::new();
    let marker = Path::new(".autosaver").to_path_buf();

    // load root directory
    let root_dir;
    if let Some(root) = root {
        root_dir = root.canonicalize().with_context(|| {
            format!("Root path could not be canonicalized: '{}'", root.display())
        })?;
    } else {
        root_dir = env::current_dir().context("Failure getting root directory")?;
    }
    // search marker, and if present upward, make its dir the root dir
    let root_dir = root_dir
        .ancestors()
        .find(|a| a.join(&marker).is_dir())
        .with_context(|| {
            let p = root_dir.display();
            format!("Could not find '.autosaver' marker in any ancestors of {p}")
        })?;
    let root_dir = AbsPathStr::new_from_pathbuf(root_dir.to_path_buf())?;

    // other dirs
    let backup_dir = root_dir.join(&RelPathStr::from_str("backup")?)?;
    let config_dir = root_dir.join(&RelPathStr::from_str("config")?)?;
    let run_dir = root_dir.join(&RelPathStr::from_str("run")?)?;
    let localconfig_dir = root_dir.join(&RelPathStr::new_from_pathbuf(marker)?)?;
    let localconfigenv_file = localconfig_dir.join(&RelPathStr::from_str("env")?)?;
    let localconfigcolors_file = localconfig_dir.join(&RelPathStr::from_str("colors")?)?;

    // load default env vars
    load_envvars(&localconfigenv_file)?;

    // load home directory (NEEDS TO BE DONE AFTER LOADING ENV VARS!!!)
    let home_dir;
    if let Some(home) = home {
        home_dir = home.canonicalize().with_context(|| {
            format!("Home path could not be canonicalized: '{}'", home.display())
        })?;
    } else if let Ok(home) = env::var("AUTOSAVER_HOME") {
        let home = PathBuf::from(home);
        home_dir = home.canonicalize().with_context(|| {
            format!("Home path could not be canonicalized: '{}'", home.display())
        })?;
    } else {
        home_dir = env::home_dir().context("Failure getting home directory")?;
    }
    let home_dir = AbsPathStr::new_from_pathbuf(home_dir)?;

    paths.insert(Paths::Home, home_dir);
    paths.insert(Paths::Root, root_dir);
    paths.insert(Paths::Backup, backup_dir);
    paths.insert(Paths::Run, run_dir);
    paths.insert(Paths::Config, config_dir);
    paths.insert(Paths::LocalConfig, localconfig_dir);
    paths.insert(Paths::LocalConfigEnv, localconfigenv_file);
    paths.insert(Paths::LocalConfigColors, localconfigcolors_file);

    Ok(paths)
}

fn load_envvars(env_file: &AbsPathStr) -> anyhow::Result<()> {
    if !env_file.is_file() {
        return Ok(());
    }
    const ALLOWED_ENVVARS: &[&str] = &["AUTOSAVER_HOME", "AUTOSAVER_PROFILE", "EDITOR"];

    for (i, line) in env_file.read_file()?.lines().enumerate() {
        let i = i + 1;
        let line = line.trim();
        if line.starts_with("#") {
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let (k, v) = line.split_once("=").with_context(|| {
            let p = env_file.display();
            format!("Line {i} of env config file ({p}) does not contain a `=` to separate key from value")
        })?;
        let k = k.trim();
        let v = v.trim();
        if !ALLOWED_ENVVARS.contains(&k) {
            let p = env_file.display();
            bail!(format!(
                "Line {i} of env config file ({p}) contains not relevant env var '{k}'"
            ));
        }
        unsafe {
            env::set_var(k, v);
        }
    }
    Ok(())
}
