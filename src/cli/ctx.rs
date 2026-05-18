use std::{collections::HashMap, env, str::FromStr};

use anyhow::Context;

use crate::{
    cli::Cli,
    fs::{abs::AbsPathStr, rel::RelPathStr},
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
}

impl CliContext {
    pub fn new(cli: &Cli) -> anyhow::Result<Self> {
        let mut paths = HashMap::new();

        // load home directory
        let home_dir;
        if let Some(home) = &cli.home {
            home_dir = home.clone();
        } else {
            let home = env::home_dir().context("Failure getting home directory")?;
            home_dir = AbsPathStr::new_from_pathbuf(home).context("Invalid home directory")?;
        }

        // load root directory
        let root_dir;
        if let Some(root) = &cli.root {
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

        Ok(Self { paths })
    }

    pub fn path(&self, path: Paths) -> &AbsPathStr {
        self.paths.get(&path).expect("Paths is incomplete")
    }
}
