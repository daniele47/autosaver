//! Module to run cli.

use std::env;

use crate::{
    cli::{
        error::{Error, Result},
        flags::{Flag, ParsedArgs},
        inout::InOut,
    },
    core::{
        fs::{AbsPath, PathType, RelPath},
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
pub struct Runner<I: InOut> {
    args: ParsedArgs,
    inout: I,
}

impl<I: InOut> Runner<I> {
    /// Create new runner.
    pub fn new(args: ParsedArgs, inout: I) -> Self {
        Self { args, inout }
    }

    const CARGO_VERSION: &str = env!("CARGO_PKG_VERSION");
    const BIN_NAME: &str = env!("CARGO_PKG_NAME");

    fn paths(path: &str) -> Result<AbsPath> {
        match path {
            "home" => {
                let var = env::var("HOME")
                    .map_err(|_| Error::GenericError("Missing HOME environment variable".into()))?;
                if PathType::from(var.as_str()) != PathType::Absolute {
                    return Err(Error::GenericError(
                        "HOME variable is not an absolute path".into(),
                    ));
                }
                let var = AbsPath::from(var);
                if !var.metadata().is_ok_and(|m| m.is_dir()) {
                    return Err(Error::GenericError(
                        "HOME variable doesn't contain a valid directory path".into(),
                    ));
                }
                Ok(var)
            }
            "root" => {
                let var = env::var("AUTOSAVER_ROOT").map_err(|_| {
                    Error::GenericError("Missing AUTOSAVER_ROOT environment variable".into())
                })?;
                if PathType::from(var.as_str()) != PathType::Absolute {
                    return Err(Error::GenericError(
                        "AUTOSAVER_ROOT variable is not an absolute path".into(),
                    ));
                }
                let var = AbsPath::from(var);
                if !var.metadata().is_ok_and(|m| m.is_dir()) {
                    return Err(Error::GenericError(
                        "AUTOSAVER_ROOT variable doesn't contain a valid directory path".into(),
                    ));
                }
                Ok(var)
            }
            "backup" => Self::paths("root").map(|p| p.joins(&["backup"])),
            "config" => Self::paths("root").map(|p| p.joins(&["config"])),
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

        Ok(ProfileLoaderImpl::new(Self::paths("config")?))
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
            self.inout.options().has_colors = false;
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
            _ => self.help(),
        }
    }
}
