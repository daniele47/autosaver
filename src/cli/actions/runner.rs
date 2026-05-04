use std::process::Command;

use crate::{
    cli::{
        actions::Runner,
        error::{Error, Result},
        flags::Flag,
        inout::InOut,
    },
    core::{
        fs::RelPath,
        profile::{ProfileType, composite::ProfileLoader, runner::RunnerPolicy},
    },
};

impl<I: InOut> Runner<I> {
    /// Backup action to list/save/restore files.
    pub fn runner(&mut self) -> Result<()> {
        // check flags
        self.check_flags("run", &[
            "--show",
            "-s",
            "--dryrun",
            "--assumeyes",
            "-y",
            "--assumeno",
            "-n",
            "--nocolor",
        ])?;

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
                                        let msg = format!("* {l}");
                                        self.inout.writeln(msg, &[]);
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
                            self.prompt("Do you want to run it?", || {
                                Command::new(abs_path.to_str_lossy())
                                    .status()
                                    .map_err(|e| {
                                        let p = abs_path.clone().into();
                                        Error::ScriptFailure(p, e.to_string())
                                    })
                                    .and_then(|status_code| {
                                        if status_code.success() {
                                            Ok(())
                                        } else {
                                            let msg = format!("Exited with code {status_code}");
                                            Err(Error::ScriptFailure(abs_path.clone().into(), msg))
                                        }
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
