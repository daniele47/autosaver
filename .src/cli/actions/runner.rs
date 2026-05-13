use std::{
    fs,
    io::{BufRead, BufReader},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    cli::{
        actions::Runner,
        error::{Error, ErrorType, Result},
        flags::Flag,
    },
    core::{
        fs::RelPath,
        profile::{ProfileType, composite::ProfileLoader, runner::RunnerPolicy},
    },
    debug,
};

impl Runner {
    /// Backup action to list/save/restore files.
    pub fn runner(&self) -> Result<()> {
        debug!(self.inout, "Running runner action...");

        // check command and flags
        if self.args.params().len() > 2 {
            return self.invalid_args_err(1);
        }
        self.check_flags(
            "run",
            &[
                "--show",
                "-s",
                "--list",
                "-l",
                "--assume-yes",
                "-y",
                "--assume-no",
                "-n",
                "--full",
                "-f",
                "--no-color",
                "--debug",
            ],
        )?;

        // get args
        let wflag_show = self.args.flags().contains(&Flag::Word("show".into()));
        let lflag_show = self.args.flags().contains(&Flag::Letter('s'));
        let flag_show = wflag_show || lflag_show;
        let wflag_list = self.args.flags().contains(&Flag::Word("list".into()));
        let lflag_list = self.args.flags().contains(&Flag::Letter('l'));
        let flag_list = wflag_list || lflag_list;
        let wflag_full = self.args.flags().contains(&Flag::Word("full".into()));
        let lflag_full = self.args.flags().contains(&Flag::Letter('f'));
        let flag_full = wflag_full || lflag_full;

        // closure to shrink output
        let cut_line = |i: String, flag_full: bool| {
            if !flag_full {
                let len = Self::LINE_LEN - 6;
                let res: String = i.chars().take(len).collect();
                res + if i.len() > len { " ..." } else { "" }
            } else {
                i
            }
        };

        // paths
        let run_dir = self.paths("run")?;

        // resolve profile into all leafs
        let profile = self.load_profile(1)?;
        let mut profile_loader = self.profile_loader()?;
        let root_profile = profile_loader.load(&profile)?;
        let profiles = root_profile.resolve(&mut profile_loader)?;

        // iterate over all leaf profiles
        self.output_main_profile(&profile);
        for profile in profiles {
            match profile.ptype() {
                ProfileType::Composite(_) => unreachable!("Composite profile impossible here"),
                ProfileType::Module(_) => {}
                ProfileType::Runner(runner) => {
                    let run_dir = run_dir.join(runner.id());
                    let runner = runner.resolve(&run_dir)?;
                    self.output_profile(profile.name());
                    debug!(self.inout, "run_dir: {run_dir:?}");

                    for entry in runner.entries() {
                        if entry.policy() == &RunnerPolicy::Skip {
                            continue;
                        }

                        // output script path
                        let path = entry.path().to_str_lossy();
                        self.inout.writeln(&path, Self::PATH_COL);
                        let abs_path = run_dir.join(&RelPath::from(path));

                        // show script if show flag is passed
                        if flag_show {
                            let show = if flag_full { usize::MAX } else { 10 };
                            for (count, line) in abs_path.line_reader()?.enumerate() {
                                if count >= show {
                                    self.inout.writeln("... ".to_string(), Self::NO_COL);
                                    break;
                                }
                                match line {
                                    Ok(l) => {
                                        self.inout.write("* ", Self::SIGN_SCRIPT_COL);
                                        self.inout.writeln(cut_line(l, flag_full), Self::NO_COL);
                                    }
                                    Err(_) => {
                                        self.inout.warning("Could not show the entire script file");
                                        break;
                                    }
                                }
                            }
                        }

                        // run script if no dryrun flag is passed
                        if !flag_list {
                            self.prompt("Do you want to run it?", |s| {
                                // make file executable
                                fs::set_permissions(
                                    PathBuf::from(abs_path.clone()),
                                    fs::Permissions::from_mode(0o755),
                                )
                                .map_err(|e| {
                                    Error::from(ErrorType::ScriptFailure(
                                        abs_path.clone().into(),
                                        format!("Could not make executable: {e}"),
                                    ))
                                })?;

                                let cmd_res = || -> Result<()> {
                                    // execute the script directly
                                    let mut child = Command::new(abs_path.to_str_lossy())
                                        .stdin(Stdio::null())
                                        .stdout(Stdio::piped())
                                        .stderr(Stdio::piped())
                                        .spawn()
                                        .map_err(|e| {
                                            let p = abs_path.clone().into();
                                            Error::from(ErrorType::ScriptFailure(p, e.to_string()))
                                        })?;

                                    let stdout = child.stdout.take().expect("stdout not piped");
                                    let stderr = child.stderr.take().expect("stderr not piped");

                                    let inout_mutex = Arc::new(Mutex::new(s.inout.clone()));

                                    // stdout thread
                                    let stdout_handle = {
                                        let inout_mutex = Arc::clone(&inout_mutex);
                                        let abspath_out = abs_path.clone();
                                        thread::spawn(move || -> Result<()> {
                                            let reader = BufReader::new(stdout);

                                            for line in reader.lines() {
                                                match line {
                                                    Ok(l) => {
                                                        let inout = inout_mutex
                                                            .lock()
                                                            .expect("Failure to lock mutex");
                                                        inout.write("> ", Self::SIGN_STDOUT_COL);
                                                        inout.writeln(
                                                            cut_line(l, flag_full),
                                                            Self::NO_COL,
                                                        );
                                                    }
                                                    Err(e) => {
                                                        return Err(ErrorType::ScriptFailure(
                                                            abspath_out.into(),
                                                            format!("Failure parsing stdout: {e}"),
                                                        )
                                                        .into());
                                                    }
                                                }
                                            }

                                            Ok(())
                                        })
                                    };

                                    // stderr thread
                                    let stderr_handle = {
                                        let inout_mutex = Arc::clone(&inout_mutex);
                                        let abspath_err = abs_path.clone();
                                        thread::spawn(move || -> Result<()> {
                                            let reader = BufReader::new(stderr);

                                            for line in reader.lines() {
                                                match line {
                                                    Ok(l) => {
                                                        let inout = inout_mutex
                                                            .lock()
                                                            .expect("Failure to lock mutex");
                                                        inout.write("! ", Self::SIGN_STDERR_COL);
                                                        inout.writeln(
                                                            cut_line(l, flag_full),
                                                            Self::NO_COL,
                                                        );
                                                    }
                                                    Err(e) => {
                                                        return Err(ErrorType::ScriptFailure(
                                                            abspath_err.into(),
                                                            format!("Failure parsing stderr: {e}"),
                                                        )
                                                        .into());
                                                    }
                                                }
                                            }

                                            Ok(())
                                        })
                                    };

                                    // wait for process
                                    let status = child.wait().map_err(|e| {
                                        let p = abs_path.clone().into();
                                        ErrorType::ScriptFailure(p, e.to_string())
                                    })?;

                                    // wait for reader threads
                                    stdout_handle.join().map_err(|e| {
                                        let p = abs_path.clone().into();
                                        let r = format!("Failure joining stdout: {e:?}");
                                        ErrorType::ScriptFailure(p, r)
                                    })??;
                                    stderr_handle.join().map_err(|e| {
                                        let p = abs_path.clone().into();
                                        let r = format!("Failure joining stderr: {e:?}");
                                        ErrorType::ScriptFailure(p, r)
                                    })??;

                                    // script failed
                                    if !status.success() {
                                        return Err(ErrorType::ScriptFailure(
                                            abs_path.clone().into(),
                                            format!("Exited with code {status}"),
                                        )
                                        .into());
                                    }

                                    Ok(())
                                }();

                                // write line separator no matter what
                                s.inout.writeln("-".repeat(Self::LINE_LEN), Self::NO_COL);
                                cmd_res
                            })?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
