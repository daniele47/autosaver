use std::{
    fs,
    io::{BufRead, BufReader},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::{Command, Stdio},
    thread,
};

use crate::{
    cli::{
        actions::Runner,
        error::{Error, Result},
        flags::Flag,
    },
    core::{
        fs::RelPath,
        profile::{ProfileType, composite::ProfileLoader, runner::RunnerPolicy},
    },
};

impl Runner {
    /// Backup action to list/save/restore files.
    pub fn runner(&mut self) -> Result<()> {
        // check flags
        self.check_flags(
            "run",
            &[
                "--show",
                "-s",
                "--dryrun",
                "--assumeyes",
                "-y",
                "--assumeno",
                "-n",
                "--nocolor",
            ],
        )?;

        // get args
        let wflag_show = self.args.flags().contains(&Flag::Word("show".into()));
        let lflag_show = self.args.flags().contains(&Flag::Letter('s'));
        let flag_show = wflag_show || lflag_show;
        let flag_dryrun = self.args.flags().contains(&Flag::Word("dryrun".into()));

        // paths
        let run_dir = Self::paths("run")?;

        // resolve profile into all leafs
        let mut profile_loader = Self::profile_loader()?;
        let root_profile = profile_loader.load(&self.load_profile(1)?)?;
        let profiles = root_profile.resolve(&mut profile_loader)?;

        // iterate over all leaf profiles
        for profile in profiles {
            match profile.ptype() {
                ProfileType::Composite(_) => unreachable!("Composite profile impossible here"),
                ProfileType::Module(_) => {}
                ProfileType::Runner(runner) => {
                    let runner = runner.resolve(&run_dir)?;
                    self.output_profile(profile.name());

                    for entry in runner.entries() {
                        if entry.policy() == &RunnerPolicy::Skip {
                            continue;
                        }

                        // output script path
                        let path = entry.path().to_str_lossy();
                        self.inout.writeln(&path, Self::SCRIPT_COLOR);
                        let abs_path = run_dir.join(&RelPath::from(path));

                        // show script if show flag is passed
                        if flag_show {
                            for line in abs_path.line_reader()? {
                                match line {
                                    Ok(l) => {
                                        self.inout.write("* ", Self::SIGN_SCRIPT_COLOR);
                                        self.inout.writeln(l, &[]);
                                    }
                                    Err(_) => {
                                        self.inout.warning("Could not show the entire script file");
                                        break;
                                    }
                                }
                            }
                        }

                        // run script if no dryrun flag is passed
                        if !flag_dryrun {
                            self.prompt("Do you want to run it?", |s| {
                                // make file executable
                                fs::set_permissions(
                                    PathBuf::from(abs_path.clone()),
                                    fs::Permissions::from_mode(0o755),
                                )
                                .map_err(|e| {
                                    Error::ScriptFailure(
                                        abs_path.clone().into(),
                                        format!("Could not make executable: {e}"),
                                    )
                                })?;

                                // execute the script
                                let mut child = Command::new(abs_path.to_str_lossy())
                                    .stdin(Stdio::null())
                                    .stdout(Stdio::piped())
                                    .stderr(Stdio::piped())
                                    .spawn()
                                    .map_err(|e| {
                                        let p = abs_path.clone().into();
                                        Error::ScriptFailure(p, e.to_string())
                                    })?;

                                // take handles to stdout/stderr as needed
                                let stdout = child.stdout.take().ok_or_else(|| {
                                    Error::ScriptFailure(
                                        abs_path.clone().into(),
                                        "Unable to capture stdout".into(),
                                    )
                                })?;
                                let stderr = child.stderr.take().ok_or_else(|| {
                                    Error::ScriptFailure(
                                        abs_path.clone().into(),
                                        "Unable to capture stderr".into(),
                                    )
                                })?;

                                // spawn threads to handle stdout/stderr
                                let mut inout = s.inout.clone();
                                let abs_path_clone = abs_path.clone();
                                let stdout_handle = thread::spawn(move || -> Result<()> {
                                    let reader = BufReader::new(stdout);
                                    for line in reader.lines() {
                                        match line {
                                            Ok(line) => {
                                                inout.write("> ", Self::SIGN_STDOUT_COLOR);
                                                inout.writeln(line, &[]);
                                            }
                                            Err(e) => {
                                                return Err(Error::ScriptFailure(
                                                    abs_path_clone.into(),
                                                    format!("Failure in reading stdout line: {e}"),
                                                ));
                                            }
                                        }
                                    }
                                    Ok(())
                                });
                                let mut inout = s.inout.clone();
                                let abs_path_clone = abs_path.clone();
                                let stderr_handle = thread::spawn(move || -> Result<()> {
                                    let reader = BufReader::new(stderr);
                                    for line in reader.lines() {
                                        match line {
                                            Ok(line) => {
                                                inout.write("> ", Self::SIGN_STDERR_COLOR);
                                                inout.writeln(line, &[]);
                                            }
                                            Err(e) => {
                                                return Err(Error::ScriptFailure(
                                                    abs_path_clone.into(),
                                                    format!("Failure in reading stderr line: {e}"),
                                                ));
                                            }
                                        }
                                    }
                                    Ok(())
                                });

                                // close stdout/stderr handles
                                stdout_handle.join().map_err(|e| {
                                    Error::ScriptFailure(
                                        abs_path.clone().into(),
                                        format!("Failure handling stdout: {e:?}"),
                                    )
                                })??;
                                stderr_handle.join().map_err(|e| {
                                    Error::ScriptFailure(
                                        abs_path.clone().into(),
                                        format!("Failure handling stderr: {e:?}"),
                                    )
                                })??;

                                // wait for script to end execution
                                child
                                    .wait()
                                    .map_err(|e| {
                                        let p = abs_path.clone().into();
                                        Error::ScriptFailure(p, e.to_string())
                                    })
                                    .and_then(|code| {
                                        if !code.success() {
                                            return Err(Error::ScriptFailure(
                                                abs_path.clone().into(),
                                                format!("Exited with code {code}"),
                                            ));
                                        }
                                        Ok(())
                                    })
                            })?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
