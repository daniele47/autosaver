use crate::{
    cli::{actions::Runner, error::Result, flags::Flag},
    core::profile::{ProfileType, composite::ProfileLoader, module::ModulePolicy},
};

impl Runner {
    /// Backup action to list/save/restore files.
    pub fn backup(&mut self) -> Result<()> {
        // check command and flags
        if self.args.params().len() > 2 {
            return self.invalid_cmd_err();
        }
        let cmd = self.args.params().first().map(String::as_str).unwrap_or("");
        match cmd {
            "save" | "restore" => {
                self.check_flags(
                    cmd,
                    &[
                        "--assume-yes",
                        "-y",
                        "--assume-no",
                        "-n",
                        "--all",
                        "-a",
                        "--diff",
                        "-d",
                        "--list",
                        "-l",
                        "--no-color",
                    ],
                )?;
            }
            "rmhome" | "rmbackup" => {
                self.check_flags(
                    cmd,
                    &[
                        "--assume-yes",
                        "-y",
                        "--assume-no",
                        "-n",
                        "--all",
                        "-a",
                        "--list",
                        "-l",
                        "--no-color",
                    ],
                )?;
            }
            _ => unreachable!("Invalid command"),
        }

        // get args
        let mut iter = self.args.params().iter();
        let arg_command = iter.next().map(String::as_str).unwrap_or_default();
        let act_save = arg_command == "save";
        let act_restore = arg_command == "restore";
        let act_rmhome = arg_command == "rmhome";
        let act_rmbackup = arg_command == "rmbackup";
        let profile = self.load_profile(1)?;
        let wflag_all = self.args.flags().contains(&Flag::Word("all".into()));
        let lflag_all = self.args.flags().contains(&Flag::Letter('a'));
        let flag_all = wflag_all || lflag_all;
        let wflag_diff = self.args.flags().contains(&Flag::Word("diff".into()));
        let lflag_diff = self.args.flags().contains(&Flag::Letter('d'));
        let flag_diff = wflag_diff || lflag_diff;
        let wflag_list = self.args.flags().contains(&Flag::Word("list".into()));
        let lflag_list = self.args.flags().contains(&Flag::Letter('l'));
        let flag_list = wflag_list || lflag_list;

        // paths
        let home_dir = Self::paths("home")?;
        let backup_dir = Self::paths("backup")?;

        // resolve profile into all leafs
        let mut profile_loader = Self::profile_loader()?;
        let root_profile = profile_loader.load(&profile)?;
        let profiles = root_profile.resolve(&mut profile_loader)?;

        // iterate over all leaf profiles
        self.output_main_profile(&profile);
        for profile in profiles {
            match profile.ptype() {
                ProfileType::Composite(_) => unreachable!("Composite profile impossible here"),
                ProfileType::Runner(_) => {}
                ProfileType::Module(module) => {
                    let backup_dir = &backup_dir.joins(&[profile.name()]);
                    let module = module.merge_bases(&home_dir, backup_dir)?;

                    self.output_profile(profile.name());

                    // iterate all entries of a module
                    for entry in module.entries() {
                        if entry.policy() == ModulePolicy::Ignore {
                            continue;
                        }
                        let home_file = home_dir.join(entry.path());
                        let backup_file = backup_dir.join(entry.path());
                        let is_home_file = home_file.metadata().is_ok_and(|m| m.is_file());
                        let is_backup_file = backup_file.metadata().is_ok_and(|m| m.is_file());
                        let path = entry.path().to_str_lossy();

                        // rmhome
                        if act_rmhome && is_home_file {
                            self.inout.writeln(path.to_string(), Self::PATH_COL);
                            if !flag_list {
                                self.prompt("Do you want to delete the home file?", |_| {
                                    Ok(home_file.purge_path(false)?)
                                })?;
                            }
                            continue;
                        }

                        // rmbackup
                        if act_rmbackup && is_backup_file {
                            self.inout.writeln(path.to_string(), Self::PATH_COL);
                            if !flag_list {
                                self.prompt("Do you want to delete the backup file?", |_| {
                                    Ok(backup_file.purge_path(false)?)
                                })?;
                            }
                            continue;
                        }

                        // list|save|restore
                        match (is_home_file, is_backup_file) {
                            // files differ
                            (true, true) if !home_file.content_eq(&backup_file) => {
                                if entry.policy() == ModulePolicy::NotDiff && !flag_all {
                                    continue;
                                }
                                self.inout.writeln(path.to_string(), Self::PATH_DIFF_COL);
                                if flag_diff {
                                    if act_restore {
                                        self.render_diff(&home_file, &backup_file)?;
                                    } else {
                                        self.render_diff(&backup_file, &home_file)?;
                                    }
                                }
                                if !flag_list {
                                    self.prompt("Do you want to update it?", |_| {
                                        if act_restore {
                                            Ok(backup_file.copy_file(&home_file, false)?)
                                        } else {
                                            Ok(home_file.copy_file(&backup_file, false)?)
                                        }
                                    })?;
                                }
                            }
                            // home => backup
                            (true, false) => {
                                if !act_save {
                                    continue;
                                }
                                self.inout.writeln(path.to_string(), Self::PATH_MISS_COL);
                                if !flag_list {
                                    self.prompt("Do you want to save it?", |_| {
                                        Ok(home_file.copy_file(&backup_file, false)?)
                                    })?;
                                }
                            }
                            // backup => home
                            (false, true) => {
                                if !act_restore {
                                    continue;
                                }
                                self.inout.writeln(path.to_string(), Self::PATH_MISS_COL);
                                if !flag_list {
                                    self.prompt("Do you want to restore it?", |_| {
                                        Ok(backup_file.copy_file(&home_file, false)?)
                                    })?;
                                }
                            }
                            (false, false) => unreachable!("At least one file should exist"),
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
