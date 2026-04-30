//! Module to run cli.

use std::env;

use crate::{
    cli::{
        error::{Error, Result},
        flags::{Flag, ParsedArgs},
        output::Renderer,
    },
    core::{
        fs::{AbsPath, RelPath},
        profile::{Profile, composite::ProfileLoader},
    },
};

mod backup;
mod help;
mod version;

/// Struct with data and methods to run cli.
pub struct Runner<I>
where
    I: Renderer,
{
    args: ParsedArgs,
    renderer: I,
}

impl<I> Runner<I>
where
    I: Renderer<Error = Error>,
{
    /// Create new runner.
    pub fn new(args: ParsedArgs, renderer: I) -> Self {
        Self { args, renderer }
    }

    const CARGO_VERSION: &str = env!("CARGO_PKG_VERSION");
    const BIN_NAME: &str = env!("CARGO_PKG_NAME");

    fn paths(path: &str) -> Result<AbsPath> {
        match path {
            "home" => env::var("HOME")
                .map_err(|_| Error::FailureLoadingPath(path.to_string()))
                .map(AbsPath::from),
            "exe" => env::current_exe()
                .map(AbsPath::from)
                .map_err(|_| Error::FailureLoadingPath(path.to_string())),
            "root" => Ok(Self::paths("exe")?.file_parent()?),
            "backup" => Ok(Self::paths("root")?.joins(&["backup"])),
            "config" => Ok(Self::paths("root")?.joins(&["config"])),
            _ => unreachable!("Invalid path"),
        }
    }

    fn profile_loader() -> Result<impl ProfileLoader<Error = Error>> {
        struct ProfileLoaderImpl {
            config_dir: AbsPath,
        }

        impl ProfileLoaderImpl {
            fn new(config_dir: AbsPath) -> Self {
                Self { config_dir }
            }
        }

        impl ProfileLoader for ProfileLoaderImpl {
            type Error = Error;

            fn load(
                &mut self,
                name: &str,
            ) -> std::result::Result<crate::core::profile::Profile, Self::Error> {
                let profile_filename = format!("{name}.conf");
                let prof_file = self.config_dir.join(&RelPath::from(profile_filename));
                if !prof_file.metadata().is_ok_and(|m| m.is_file()) {
                    Err(crate::core::error::Error::ProfileLoadingFailure(
                        name.into(),
                        "configuration file is missing".into(),
                    ))?;
                }
                Ok(Profile::parse(name.into(), prof_file.line_reader()?)?)
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
