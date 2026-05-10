//! Module to run cli.

use std::{collections::BTreeSet, env, io::ErrorKind, process::exit};

use crate::{
    cli::{
        error::{ErrorType, Result},
        flags::{Flag, ParsedArgs},
        inout::{Style, TermInOut},
    },
    core::{
        fs::{AbsPath, LineDiff, PathType, RelPath},
        profile::{
            Profile, ProfileType,
            composite::{Composite, HashMapProfileLoader, ProfileLoader},
        },
    },
    debug,
};

mod backup;
mod clear;
mod help;
mod runner;
mod tree;
mod version;

/// Struct with data and methods to run cli.
#[derive(Debug, Clone)]
pub struct Runner {
    pub args: ParsedArgs,
    pub inout: TermInOut,
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
    const MAIN_PROF_COL: &[Style] = &[Style::Purple, Style::Bold];
    const DECORATION_COL: &[Style] = &[Style::Blue, Style::Bold];
    const PATH_MISS_COL: &[Style] = &[Style::Red, Style::Bold, Style::Underline];
    const PATH_DIFF_COL: &[Style] = &[Style::Yellow, Style::Bold, Style::Underline];
    const PATH_EQ_COL: &[Style] = &[Style::Green, Style::Bold, Style::Underline];
    const PATH_UNTRACKED_COL: &[Style] = &[Style::Yellow, Style::Bold, Style::Underline];
    const PATH_COL: &[Style] = &[Style::White, Style::Bold, Style::Underline];
    const SIGN_RM_COL: &[Style] = &[Style::Red];
    const SIGN_ADD_COL: &[Style] = &[Style::Green];
    const SIGN_SEP_COL: &[Style] = &[Style::Blue];
    const SIGN_STDOUT_COL: &[Style] = &[Style::White];
    const SIGN_SCRIPT_COL: &[Style] = &[Style::White];
    const SIGN_INPUT_COL: &[Style] = &[Style::White];

    // conf variables
    const LINE_LEN: usize = 80;

    // check there are no symlinks to the outside
    fn assert_no_escaping_symlinks(&self) -> Result<()> {
        debug!(self.inout, "Checking there are no escaping symlinks...");
        for dir in [
            self.paths("config")?,
            self.paths("backup")?,
            self.paths("run")?,
        ] {
            if !dir.exists() {
                continue;
            }
            for symlink in dir.all_files(AbsPath::FILTER_EXIST)? {
                {
                    if !symlink.check_inside(&dir) {
                        let norm_path = symlink.to_str_lossy();
                        let canon_path = symlink
                            .canonicalize()
                            .expect("path should have been canonicalizable")
                            .to_str_lossy();
                        return Err(ErrorType::OutOfBoundSymlink(norm_path, canon_path).into());
                    }
                }
            }
        }

        Ok(())
    }

    // get paths
    fn paths(&self, path: &str) -> Result<AbsPath> {
        match path {
            "home" => {
                let var = self.env("home")?;
                if PathType::from(var.as_str()) != PathType::Absolute {
                    return Err(ErrorType::InvalidEnv(
                        "AUTOSAVER_HOME".into(),
                        "Not an absolute path".into(),
                    )
                    .into());
                }
                let var = AbsPath::from(var);
                if !var.metadata().is_ok_and(|m| m.is_dir()) {
                    return Err(ErrorType::InvalidEnv(
                        "AUTOSAVER_HOME".into(),
                        "Not a path to a directory".into(),
                    )
                    .into());
                }
                Ok(var)
            }
            "root" => {
                let var = self.env("root")?;
                if PathType::from(var.as_str()) != PathType::Absolute {
                    return Err(ErrorType::InvalidEnv(
                        "AUTOSAVER_ROOT".into(),
                        "Not an absolute path".into(),
                    )
                    .into());
                }
                let var = AbsPath::from(var);
                if !var.metadata().is_ok_and(|m| m.is_dir()) {
                    return Err(ErrorType::InvalidEnv(
                        "AUTOSAVER_ROOT".into(),
                        "Not a path to a directory".into(),
                    )
                    .into());
                }
                Ok(var)
            }
            "backup" => self.paths("root").map(|p| p.joins(&["backup"])),
            "config" => self.paths("root").map(|p| p.joins(&["config"])),
            "run" => self.paths("root").map(|p| p.joins(&["run"])),
            "default" => self.paths("root").map(|p| p.joins(&[".default"])),
            _ => unreachable!("Invalid path"),
        }
    }

    // easily check flags
    fn check_flags(&self, cmd: &str, flag_set: &[&str]) -> Result<()> {
        for flag in self.args.flags() {
            let flag_str = match flag {
                Flag::Letter(lflag) => format!("-{lflag}"),
                Flag::Word(wflag) => format!("--{wflag}"),
            };
            if !flag_set.contains(&flag_str.as_str()) {
                return Err(ErrorType::InvalidFlag(flag.clone(), cmd.to_string()).into());
            }
        }
        Ok(())
    }

    // utility to avoid rewriting the same code multiple times
    fn invalid_cmd_err(&self) -> Result<()> {
        Err(ErrorType::InvalidCommand(self.args.params().join(" ")).into())
    }

    // deal with environment variables
    fn load_env(&self, env: &str) -> Result<String> {
        env::var(env).map_err(|_| ErrorType::UndefinedEnv(env.to_string()).into())
    }
    fn env(&self, env: &str) -> Result<String> {
        match env {
            "profile" => self.load_env("AUTOSAVER_PROFILE"),
            "root" => self.load_env("AUTOSAVER_ROOT"),
            "home" => self.load_env("AUTOSAVER_HOME"),
            _ => unreachable!("Invalid env"),
        }
    }

    // load the profile, with the proper fallbacks
    fn load_profile(&self, param_index: usize) -> Result<String> {
        || -> Result<String> {
            match self.args.params().get(param_index) {
                Some(p) => {
                    debug!(self.inout, "Requested profile is '{p}' (cmdline)");
                    Ok(p.clone()) as Result<String>
                }
                None => match self.env("profile") {
                    Ok(env) => {
                        debug!(self.inout, "Requested profile '{env}' (AUTOSAVER_PROFILE)");
                        Ok(env)
                    }
                    Err(_) => {
                        let prof_file = self.paths("default")?;
                        if let Some(first_line) = prof_file.line_reader()?.next() {
                            let f = first_line?;
                            if !f.is_empty() {
                                debug!(self.inout, "Requested profile '{f}' (.default)");
                                return Ok(f);
                            }
                        }
                        Err(ErrorType::MissingProfile.into())
                    }
                },
            }
        }()
        .map_err(|_| ErrorType::MissingProfile.into())
    }

    // render diff between two files
    fn render_diff(&self, file1: &AbsPath, file2: &AbsPath, cut: bool) -> Result<()> {
        let diff = file1.calc_diff(file2);
        if let Err(err) = &diff
            && let crate::core::error::ErrorType::IoError(err, _) = err.error_type()
            && err.kind() == ErrorKind::InvalidData
        {
            self.inout.writeln(
                "* binary files differ but cannot be compared",
                Self::WARN_COL,
            );
            return Ok(());
        }
        let show = if cut { 9 } else { usize::MAX };
        let mut count = 0;
        let mut last_eq = false;
        let cut_line = |i: String| {
            if cut {
                let len = Self::LINE_LEN - 2;
                let res: String = i.chars().take(len).collect();
                res + if i.len() > len { " ..." } else { "" }
            } else {
                i
            }
        };
        for line in diff? {
            if count >= show {
                self.inout.writeln("... ".to_string(), Self::NO_COL);
                break;
            }
            match line {
                LineDiff::Equal(_) => {
                    if !last_eq && !cut {
                        self.inout.writeln("@", Self::SIGN_SEP_COL);
                    }
                    last_eq = true;
                }
                LineDiff::Insert(line) => {
                    self.inout.write("+ ", Self::SIGN_ADD_COL);
                    self.inout.writeln(cut_line(line), Self::NO_COL);
                    last_eq = false;
                    count += 1;
                }
                LineDiff::Delete(line) => {
                    self.inout.write("- ", Self::SIGN_RM_COL);
                    self.inout.writeln(cut_line(line), Self::NO_COL);
                    last_eq = false;
                    count += 1;
                }
            }
        }
        Ok(())
    }

    // generic prompt
    fn generic_prompt(&self, prompt: &str) {
        self.inout.write("$ ", Self::SIGN_INPUT_COL);
        self.inout.write(prompt, Self::NO_COL);
    }

    // prompt user before running an action
    fn prompt<T: Fn(&Self) -> Result<()>>(&self, msg: &str, run: T) -> Result<()> {
        let wflag_y = self.args.flags().contains(&Flag::Word("assume-yes".into()));
        let lflag_y = self.args.flags().contains(&Flag::Letter('y'));
        let flag_y = wflag_y || lflag_y;
        let wflag_n = self.args.flags().contains(&Flag::Word("assume-no".into()));
        let lflag_n = self.args.flags().contains(&Flag::Letter('n'));
        let flag_n = wflag_n || lflag_n;

        self.generic_prompt(format!("{msg} [y/n/q] ").as_str());
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

    // nice profile output
    fn output_profile(&self, profile: &str) {
        let msg = format!("*** {profile} ***");
        self.inout.writeln(msg, Self::DECORATION_COL)
    }
    fn output_main_profile(&self, profile: &str) {
        let msg = format!("****** {profile} ******");
        self.inout.writeln(msg, Self::MAIN_PROF_COL)
    }

    // get a struct that implements profile loader
    fn profile_loader(&self) -> Result<impl ProfileLoader + 'static> {
        struct ProfileLoaderImpl {
            cached: HashMapProfileLoader,
            config_dir: AbsPath,
            inout: TermInOut,
        }

        impl ProfileLoaderImpl {
            fn new(config_dir: AbsPath, inout: TermInOut) -> Self {
                debug!(inout, "Creating profile loader...");
                Self {
                    cached: Default::default(),
                    config_dir,
                    inout,
                }
            }
        }

        impl ProfileLoader for ProfileLoaderImpl {
            fn load(&mut self, name: &str) -> crate::core::error::Result<Profile> {
                let cached_profiles = self.cached.profiles();
                let cached = cached_profiles.get(name);
                if let Some(cached_prof) = cached {
                    // debug!(self.inout, "Loading already cached profile: '{name}'");
                    return Ok(cached_prof.clone());
                }
                let prof_file = self.config_dir.join(&RelPath::from(format!("{name}.conf")));
                let prof_dir = self.config_dir.join(&RelPath::from(name));

                // if <profile>.conf file exist, consider <profile> the profile name
                if prof_file.metadata().is_ok_and(|m| m.is_file()) {
                    debug!(self.inout, "Loading conf file profile: '{name}'");
                    let loaded = Profile::parse(name.into(), prof_file.line_reader()?)?;
                    cached_profiles.insert(name.into(), loaded.clone());
                    Ok(loaded)
                }
                // if <profile>/ directory exist, consider <profile> the profile name
                // and create a fake composite type, treating this dir as if it included all files
                else if prof_dir.metadata().is_ok_and(|m| m.is_dir()) {
                    debug!(self.inout, "Loading virtual dir profile: '{name}'");
                    let mut entries = BTreeSet::new();
                    for child in prof_dir.list_files(AbsPath::FILTER_EXIST)? {
                        let rel_child_str = child.to_relative(&self.config_dir)?.to_str_lossy();
                        if child.metadata().is_ok_and(|m| m.is_file()) {
                            if let Some(profile_name) = rel_child_str.strip_suffix(".conf") {
                                entries.insert(profile_name.to_string());
                            }
                        } else if child.metadata().is_ok_and(|m| m.is_dir()) {
                            entries.insert(rel_child_str.to_string());
                        }
                    }
                    let composite = Composite::new(entries.into_iter().collect());
                    let loaded = Profile::new(name.into(), ProfileType::Composite(composite));
                    cached_profiles.insert(name.into(), loaded.clone());
                    Ok(loaded)
                }
                // <profile> does not exist
                else {
                    Err(crate::core::error::ErrorType::ProfileLoadingFailure(
                        name.into(),
                        "configuration file or directory is missing".into(),
                    )
                    .into())
                }
            }
        }
        let config_dir = self.paths("config")?;
        Ok(ProfileLoaderImpl::new(config_dir, self.inout.clone()))
    }

    pub fn init(&mut self) {
        // get flags
        let flags = self.args.flags();
        let flag_nocolor = flags.contains(&Flag::Word("no-color".into()));
        let flag_debug = flags.contains(&Flag::Word("debug".into()));

        // handle global flags
        self.inout.options().has_colors = !flag_nocolor;
        self.inout.options().show_debug = flag_debug;

        debug!(self.inout, "Options initialized!");
    }

    /// Run the cli application.
    pub fn run(&self) -> Result<()> {
        // get flags
        let flags = self.args.flags();
        let wflag_help = flags.contains(&Flag::Word("help".into()));
        let lflag_help = flags.contains(&Flag::Letter('h'));
        let flag_version = flags.contains(&Flag::Word("version".into()));

        // print some initial debug
        debug!(self.inout, "inout: {:?}", &self.inout);
        debug!(self.inout, "args: {:?}", &self.args);

        if flag_version {
            return self.version();
        }
        if lflag_help || wflag_help {
            return self.help();
        }

        // run symlink checks
        if !self.args.params().is_empty() {
            self.assert_no_escaping_symlinks()?;
        }

        // handle commands
        let command = self.args.params().first().map(|s| s.as_str()).unwrap_or("");
        match command {
            "list" | "save" | "restore" | "rmhome" | "rmbackup" => self.backup(),
            "run" => self.runner(),
            "clear" => self.clear(),
            "tree" => self.tree(),
            "" => self.check_flags("", &[]),
            _ => self.invalid_cmd_err(),
        }
    }
}
