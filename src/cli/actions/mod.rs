//! Module to run cli.

use std::env;

use crate::{
    cli::{
        error::{Error, Result},
        flags::{Flag, ParsedArgs},
        render::Renderer,
    },
    core::{
        fs::{AbsPath, RelPath},
        profile::{
            Profile,
            composite::{HashMapProfileLoader, ProfileLoader},
        },
    },
};

mod backup;
mod help;
mod version;

/// Struct with data and methods to run cli.
pub struct Runner<I: Renderer> {
    args: ParsedArgs,
    renderer: I,
}

impl<I: Renderer> Runner<I> {
    /// Create new runner.
    pub fn new(args: ParsedArgs, renderer: I) -> Self {
        Self { args, renderer }
    }

    const CARGO_VERSION: &str = env!("CARGO_PKG_VERSION");
    const BIN_NAME: &str = env!("CARGO_PKG_NAME");

    fn paths(path: &str) -> AbsPath {
        match path {
            "home" => {
                let home = env::var("HOME").expect("Missing HOME environment variable");
                let home = AbsPath::from(home);
                assert!(
                    home.metadata().is_ok_and(|m| m.is_dir()),
                    "HOME does not contain a valid directory path"
                );
                home
            }
            "root" => {
                let root = env::var("AUTOSAVER_ROOT");
                let root = root.expect("Missing AUTOSAVER_ROOT environment variable");
                let root = AbsPath::from(root);
                assert!(
                    root.metadata().is_ok_and(|m| m.is_dir()),
                    "AUTOSAVER_ROOT does not contain a valid directory path"
                );
                root
            }
            "backup" => Self::paths("root").joins(&["backup"]),
            "config" => Self::paths("root").joins(&["config"]),
            _ => unreachable!("Invalid path"),
        }
    }

    fn profile_loader() -> Result<impl ProfileLoader> {
        struct ProfileLoaderImpl {
            cached: HashMapProfileLoader,
            config_dir: AbsPath,
        }

        impl ProfileLoaderImpl {
            fn new(config_dir: AbsPath) -> Self {
                Self {
                    cached: Default::default(),
                    config_dir,
                }
            }
        }

        impl ProfileLoader for ProfileLoaderImpl {
            fn load(&mut self, name: &str) -> crate::core::error::Result<Profile> {
                let cached_profiles = self.cached.profiles();
                let cached = cached_profiles.get(name);
                if let Some(cached_prof) = cached {
                    return Ok(cached_prof.clone());
                }
                let profile_filename = format!("{name}.conf");
                let prof_file = self.config_dir.join(&RelPath::from(profile_filename));
                if !prof_file.metadata().is_ok_and(|m| m.is_file()) {
                    Err(crate::core::error::Error::ProfileLoadingFailure(
                        name.into(),
                        format!(
                            "configuration file is missing: {}",
                            prof_file.to_str_lossy()
                        ),
                    ))?;
                }
                let loaded = Profile::parse(name.into(), prof_file.line_reader()?)?;
                cached_profiles.insert(name.into(), loaded.clone());
                Ok(loaded)
            }
        }

        Ok(ProfileLoaderImpl::new(Self::paths("config")))
    }

    /// Run the cli application.
    pub fn run(&mut self) -> Result<()> {
        let flags = self.args.flags();
        let wflag_help = flags.contains(&Flag::Word("help".into()));
        let lflag_help = flags.contains(&Flag::Letter('h'));
        let flag_version = flags.contains(&Flag::Word("version".into()));
        let flag_nocolor = flags.contains(&Flag::Word("nocolor".into()));

        // handle global flags
        if flag_nocolor {
            self.renderer.options().has_colors = false;
        }
        if flag_version {
            return self.version();
        }
        if lflag_help || wflag_help {
            return self.help();
        }

        // handle commands
        let command = self.args.params().first().map(|s| s.as_str()).unwrap_or("");
        match command {
            "list" | "save" | "restore" => self.backup(),
            _ => {
                let err_msg = format!("Invalid command '{}'", command);
                Err(Error::EarlyExit(err_msg))
            }
        }
    }
}
