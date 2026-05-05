//! Module to run cli.

use std::{env, io::ErrorKind, process::exit};

use crate::{
    cli::{
        error::{Error, Result},
        flags::{Flag, ParsedArgs},
        inout::{Style, TermInOut},
    },
    core::{
        fs::{AbsPath, LineDiff, PathType, RelPath},
        profile::{
            Profile,
            composite::{HashMapProfileLoader, ProfileLoader},
        },
    },
};

mod backup;
mod help;
mod runner;
mod version;

/// Struct with data and methods to run cli.
#[derive(Debug, Clone)]
pub struct Runner {
    args: ParsedArgs,
    inout: TermInOut,
}

impl Runner {
    /// Create new runner.
    pub fn new(args: ParsedArgs, inout: TermInOut) -> Self {
        Self { args, inout }
    }

    // crate metadata
    const CARGO_VERSION: &str = env!("CARGO_PKG_VERSION");
    const BIN_NAME: &str = env!("CARGO_PKG_NAME");

    // colors
    const NO_COL: &[Style] = &[];
    const WARN_COL: &[Style] = &[Style::Yellow];
    const DECORATION_COL: &[Style] = &[Style::Blue, Style::Bold];
    const PATH_MISS_COL: &[Style] = &[Style::Red, Style::Bold, Style::Underline];
    const PATH_DIFF_COL: &[Style] = &[Style::Yellow, Style::Bold, Style::Underline];
    const PATH_SCRIPT_COL: &[Style] = &[Style::White, Style::Bold, Style::Underline];
    const SIGN_RM_COL: &[Style] = &[Style::Red];
    const SIGN_ADD_COL: &[Style] = &[Style::Green];
    const SIGN_SCRIPT_COL: &[Style] = &[Style::White];

    fn paths(path: &str) -> Result<AbsPath> {
        match path {
            "home" => {
                let var = Self::env("home")?;
                if PathType::from(var.as_str()) != PathType::Absolute {
                    return Err(Error::InvalidEnv(
                        "AUTOSAVER_HOME".into(),
                        "Not an absolute path".into(),
                    ));
                }
                let var = AbsPath::from(var);
                if !var.metadata().is_ok_and(|m| m.is_dir()) {
                    return Err(Error::InvalidEnv(
                        "AUTOSAVER_HOME".into(),
                        "Not a path to a directory".into(),
                    ));
                }
                Ok(var)
            }
            "root" => {
                let var = Self::env("root")?;
                if PathType::from(var.as_str()) != PathType::Absolute {
                    return Err(Error::InvalidEnv(
                        "AUTOSAVER_ROOT".into(),
                        "Not an absolute path".into(),
                    ));
                }
                let var = AbsPath::from(var);
                if !var.metadata().is_ok_and(|m| m.is_dir()) {
                    return Err(Error::InvalidEnv(
                        "AUTOSAVER_ROOT".into(),
                        "Not a path to a directory".into(),
                    ));
                }
                Ok(var)
            }
            "backup" => Self::paths("root").map(|p| p.joins(&["backup"])),
            "config" => Self::paths("root").map(|p| p.joins(&["config"])),
            "run" => Self::paths("root").map(|p| p.joins(&["run"])),
            "default" => Self::paths("root").map(|p| p.joins(&[".default"])),
            _ => unreachable!("Invalid path"),
        }
    }

    fn check_flags(&self, cmd: &str, flag_set: &[&str]) -> Result<()> {
        for flag in self.args.flags() {
            let flag_str = match flag {
                Flag::Letter(lflag) => format!("-{lflag}"),
                Flag::Word(wflag) => format!("--{wflag}"),
            };
            if !flag_set.contains(&flag_str.as_str()) {
                return Err(Error::InvalidFlag(flag.clone(), cmd.to_string()));
            }
        }
        Ok(())
    }

    fn load_env(env: &str) -> Result<String> {
        env::var(env).map_err(|_| Error::UndefinedEnv(env.to_string()))
    }

    fn env(env: &str) -> Result<String> {
        match env {
            "profile" => Self::load_env("AUTOSAVER_PROFILE"),
            "root" => Self::load_env("AUTOSAVER_ROOT"),
            "home" => Self::load_env("AUTOSAVER_HOME"),
            _ => unreachable!("Invalid env"),
        }
    }

    fn load_profile(&mut self, param_index: usize) -> Result<String> {
        || -> Result<String> {
            match self.args.params().get(param_index) {
                Some(p) => Ok(p.clone()) as Result<String>,
                None => match Self::env("profile") {
                    Ok(env_prof) => Ok(env_prof),
                    Err(_) => {
                        let prof_file = Self::paths("default")?;
                        if let Some(first_line) = prof_file.line_reader()?.next() {
                            let first_line = first_line?;
                            if !first_line.is_empty() {
                                return Ok(first_line);
                            }
                        }
                        Err(Error::MissingProfile)
                    }
                },
            }
        }()
        .map_err(|_| Error::MissingProfile)
    }

    fn render_diff(&mut self, file1: &AbsPath, file2: &AbsPath) -> Result<()> {
        let diff = file1.calc_diff(file2);
        if let Err(err) = &diff
            && let crate::core::error::Error::IoError(err, _) = err
            && err.kind() == ErrorKind::InvalidData
        {
            self.inout.writeln(
                "* binary files differ but cannot be compared",
                Self::WARN_COL,
            );
            return Ok(());
        }
        for line in diff? {
            match line {
                LineDiff::Equal(_) => {}
                LineDiff::Insert(line) => {
                    self.inout.write("+ ", Self::SIGN_ADD_COL);
                    self.inout.writeln(line, Self::NO_COL);
                }
                LineDiff::Delete(line) => {
                    self.inout.write("- ", Self::SIGN_RM_COL);
                    self.inout.writeln(line, Self::NO_COL);
                }
            }
        }
        Ok(())
    }

    fn prompt<T: Fn(&mut Self) -> Result<()>>(&mut self, msg: &str, run: T) -> Result<()> {
        let wflag_y = self.args.flags().contains(&Flag::Word("assumeyes".into()));
        let lflag_y = self.args.flags().contains(&Flag::Letter('y'));
        let flag_y = wflag_y || lflag_y;
        let wflag_n = self.args.flags().contains(&Flag::Word("assumeno".into()));
        let lflag_n = self.args.flags().contains(&Flag::Letter('n'));
        let flag_n = wflag_n || lflag_n;

        self.inout.write(format!("{msg} [y/n/q] "), Self::NO_COL);
        if flag_n {
            self.inout.writeln("n", Self::NO_COL);
            return Ok(());
        }
        if flag_y {
            self.inout.writeln("y", Self::NO_COL);
            run(self)?;
            return Ok(());
        }
        let input = self.inout.read_line();
        if input == "q" {
            exit(0);
        }
        if input == "y" {
            run(self)?;
        }
        Ok(())
    }

    fn output_profile(&mut self, profile: &str) {
        let msg = format!("*** {profile} ***");
        self.inout.writeln(msg, Self::DECORATION_COL)
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
            "list" | "save" | "restore" | "rmhome" | "rmbackup" => self.backup(),
            "run" => self.runner(),
            _ => self.check_flags("", &[]),
        }
    }
}
