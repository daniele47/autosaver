use crate::{
    cli::{
        actions::Runner,
        error::{Error, Result},
        flags::Flag,
        inout::{InOut, Style},
    },
    core::profile::{ProfileType, composite::ProfileLoader, module::ModulePolicy},
};

impl<I: InOut> Runner<I> {
    /// Backup action to list/save/restore files.
    pub fn backup(&mut self) -> Result<()> {
        // check flags
        match self.args.params().first().map(String::as_str).unwrap_or("") {
            "list" => {
                self.check_flags(&["--nocolor"])?;
            }
            "save" | "restore" => {
                self.check_flags(&[
                    "--assumeyes",
                    "-y",
                    "--assumeno",
                    "-n",
                    "--all",
                    "-a",
                    "--nocolor",
                ])?;
            }
            "rmhome" | "rmbackup" => {
                self.check_flags(&["--assumeyes", "-y", "--assumeno", "-n", "--nocolor"])?;
            }
            _ => unreachable!("Invalid command"),
        }

        // get args
        let mut iter = self.args.params().iter();
        let arg_command = iter.next().map(String::as_str).unwrap_or_default();
        let act_list = arg_command == "list";
        let act_save = arg_command == "save";
        let act_restore = arg_command == "restore";
        let act_rmhome = arg_command == "rmhome";
        let act_rmbackup = arg_command == "rmbackup";
        let env_profile = Self::env("profile").unwrap_or_default();
        let mut arg_profile = iter.next().map(String::as_str).unwrap_or_default();
        let wflag_y = self.args.flags().contains(&Flag::Word("assumeyes".into()));
        let lflag_y = self.args.flags().contains(&Flag::Letter('y'));
        let flag_y = wflag_y || lflag_y;
        let wflag_n = self.args.flags().contains(&Flag::Word("assumeno".into()));
        let lflag_n = self.args.flags().contains(&Flag::Letter('n'));
        let flag_n = wflag_n || lflag_n;
        let wflag_all = self.args.flags().contains(&Flag::Word("all".into()));
        let lflag_all = self.args.flags().contains(&Flag::Letter('a'));
        let flag_all = wflag_all || lflag_all;

        if arg_profile.is_empty() {
            if env_profile.is_empty() {
                return Err(Error::GenericError("No profile specified".into()));
            }
            arg_profile = &env_profile;
        }

        // paths
        let home_dir = Self::paths("home")?;
        let backup_dir = Self::paths("backup")?;

        // resolve profile into all leafs
        let mut profile_loader = Self::profile_loader()?;
        let root_profile = profile_loader.load(arg_profile)?;
        let profiles = root_profile.resolve(&mut profile_loader)?;

        // iterate over all leaf profiles
        for profile in profiles {
            match profile.ptype() {
                ProfileType::Composite(_) => unreachable!("Composite profile impossible here"),
                ProfileType::Module(module) => {
                    let backup_dir = &backup_dir.joins(&[profile.name()]);
                    let module = module.merge_bases(&home_dir, backup_dir)?;

                    let str = format!("*** {} ***", profile.name());
                    self.inout.writeln(str, &[Style::Blue]);

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
                            self.inout.write("- ", &[]);
                            self.inout.writeln(path.to_string(), &[Style::Red]);
                            self.inout
                                .write("Do you want to delete the home file? [y/n] ", &[]);
                            if !flag_n && (flag_y || self.inout.read_line() == "y") {
                                home_file.purge_path(false)?;
                            }
                            if flag_n || flag_y {
                                self.inout.writeln("", &[]);
                            }
                        }

                        // rmbackup
                        if act_rmbackup && is_backup_file {
                            self.inout.write("- ", &[]);
                            self.inout.writeln(path.to_string(), &[Style::Red]);
                            self.inout
                                .write("Do you want to delete the backup file? [y/n] ", &[]);
                            if !flag_n && (flag_y || self.inout.read_line() == "y") {
                                backup_file.purge_path(false)?;
                            }
                            if flag_n || flag_y {
                                self.inout.writeln("", &[]);
                            }
                        }

                        // list|save|restore
                        match (is_home_file, is_backup_file) {
                            // files differ
                            (true, true) if !home_file.content_eq(&backup_file) => {
                                if entry.policy() == ModulePolicy::NotDiff && !flag_all {
                                    continue;
                                }
                                if !act_list && !act_save && !act_restore {
                                    continue;
                                }
                                self.inout.write("- ", &[]);
                                self.inout.writeln(path.to_string(), &[Style::Yellow]);
                                if act_save {
                                    self.inout.write("Do you want to update it? [y/n] ", &[]);
                                    if !flag_n && (flag_y || self.inout.read_line() == "y") {
                                        home_file.copy_file(&backup_file, false)?;
                                    }
                                    if flag_n || flag_y {
                                        self.inout.writeln("", &[]);
                                    }
                                } else if act_restore {
                                    self.inout.write("Do you want to update it? [y/n] ", &[]);
                                    if flag_y || self.inout.read_line() == "y" {
                                        backup_file.copy_file(&home_file, false)?;
                                    }
                                    if flag_n || flag_y {
                                        self.inout.writeln("", &[]);
                                    }
                                }
                            }
                            // home => backup
                            (true, false) => {
                                if !act_list && !act_save {
                                    continue;
                                }
                                self.inout.write("- ", &[]);
                                self.inout.writeln(path.to_string(), &[Style::Red]);
                                if act_save {
                                    self.inout.write("Do you want to save it? [y/n] ", &[]);
                                    if !flag_n && (flag_y || self.inout.read_line() == "y") {
                                        home_file.copy_file(&backup_file, false)?;
                                    }
                                    if flag_n || flag_y {
                                        self.inout.writeln("", &[]);
                                    }
                                }
                            }
                            // backup => home
                            (false, true) => {
                                if !act_list && !act_restore {
                                    continue;
                                }
                                self.inout.write("- ", &[]);
                                self.inout.writeln(path.to_string(), &[Style::Red]);
                                if act_restore {
                                    self.inout.write("Do you want to restore it? [y/n] ", &[]);
                                    if flag_y || self.inout.read_line() == "y" {
                                        backup_file.copy_file(&home_file, false)?;
                                    }
                                    if flag_n || flag_y {
                                        self.inout.writeln("", &[]);
                                    }
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
