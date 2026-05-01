use crate::{
    cli::{
        actions::Runner,
        error::Result,
        flags::Flag,
        render::{Renderer, Style},
    },
    core::profile::{ProfileType, composite::ProfileLoader, module::ModulePolicy},
};

impl<I: Renderer> Runner<I> {
    /// Backup action to list/save/restore files.
    pub fn backup(&mut self) -> Result<()> {
        // get args
        let mut iter = self.args.params().iter();
        let arg_command = iter.next().map(String::as_str).unwrap_or_default();
        let act_list = arg_command == "list";
        let act_save = arg_command == "save";
        let act_restore = arg_command == "restore";
        let arg_profile = iter.next().map(String::as_str).unwrap_or_default();
        let wflag_y = self.args.flags().contains(&Flag::Word("assumeyes".into()));
        let lflag_y = self.args.flags().contains(&Flag::Letter('y'));
        let flag_y = wflag_y || lflag_y;
        let wflag_n = self.args.flags().contains(&Flag::Word("assumeno".into()));
        let lflag_n = self.args.flags().contains(&Flag::Letter('n'));
        let flag_n = wflag_n || lflag_n;
        let wflag_diff = self.args.flags().contains(&Flag::Word("diff".into()));
        let lflag_diff = self.args.flags().contains(&Flag::Letter('d'));
        let flag_diff = wflag_diff || lflag_diff;
        let wflag_all = self.args.flags().contains(&Flag::Word("all".into()));
        let lflag_all = self.args.flags().contains(&Flag::Letter('a'));
        let flag_all = wflag_all || lflag_all;

        // paths
        let home_dir = Self::paths("home");
        let backup_dir = Self::paths("backup");

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
                    let module = module.merge_bases(&home_dir, &backup_dir)?;

                    let str = format!("*** {} ***\n", profile.name());
                    self.renderer.writeln(str, &[Style::Purple, Style::Bold]);

                    // iterate all entries of a module
                    for entry in module.entries() {
                        if entry.policy() == ModulePolicy::Ignore {
                            continue;
                        }
                        let home_file = home_dir.join(entry.path());
                        let backup_file = home_dir.join(entry.path());
                        let is_home_file = home_file.metadata().is_ok_and(|m| m.is_file());
                        let is_backup_file = backup_file.metadata().is_ok_and(|m| m.is_file());
                        let path = home_file.to_str_lossy();
                        match (is_home_file, is_backup_file) {
                            // files differ
                            (true, true) if !home_file.content_eq(&backup_file) => {
                                if entry.policy() == ModulePolicy::NotDiff && !flag_all {
                                    continue;
                                }
                                self.renderer.write("- ", &[]);
                                self.renderer.writeln(format!("{path}"), &[Style::Yellow]);
                            }
                            // home => backup
                            (true, false) => {
                                if !act_list && !act_save {
                                    continue;
                                }
                                self.renderer.write("- ", &[]);
                                self.renderer.writeln(format!("{path}"), &[Style::Red]);
                            }
                            // backup => home
                            (false, true) => {
                                if !act_list && !act_restore {
                                    continue;
                                }
                                self.renderer.write("- ", &[]);
                                self.renderer.writeln(format!("{path}"), &[Style::Red]);
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
