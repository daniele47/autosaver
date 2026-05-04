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
        self.check_flags(&["--show", "-s", "--dryrun"])?;

        // get args
        let default_profile = String::new();
        let mut arg_profile = self.args.params().get(2).unwrap_or(&default_profile);
        let env_profile = Self::env("profile").unwrap_or_default();
        let wflag_show = self.args.flags().contains(&Flag::Word("show".into()));
        let lflag_show = self.args.flags().contains(&Flag::Letter('s'));
        let flag_show = wflag_show || lflag_show;
        let flag_dryrun = self.args.flags().contains(&Flag::Word("dryrun".into()));

        if arg_profile.is_empty() {
            if env_profile.is_empty() {
                return Err(Error::GenericError("No profile specified".into()));
            }
            arg_profile = &env_profile;
        }

        // paths
        let run_dir = Self::paths("run")?;

        // resolve profile into all leafs
        let mut profile_loader = Self::profile_loader()?;
        let root_profile = profile_loader.load(arg_profile)?;
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
                                        let msg = format!("  {l}");
                                        self.inout.writeln(msg, &[]);
                                    }
                                    Err(_) => {
                                        self.inout
                                            .warning("Could not finish showing the script file");
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
                                    .map(|_| {})
                                    .map_err(|_| Error::GenericError("Unable to run script".into()))
                            })?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
