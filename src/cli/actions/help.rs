use crate::cli::{actions::Runner, error::Result, inout::InOut};

impl<I: InOut> Runner<I> {
    /// Help action to render help message.
    pub fn help(&mut self) -> Result<()> {
        self.check_flags(&["--help", "--nocolor"])?;
        let first = self.args.params().first().map(String::as_ref).unwrap_or("");
        let _ = self.args.params().get(1).map(String::as_ref).unwrap_or("");
        let _ = self.args.params().get(2).map(String::as_ref).unwrap_or("");
        let col = Self::HELP_COLOR;
        let io = &mut self.inout;
        match first {
            "list" => {
                io.writeln("Commands:", col);
                io.write("  list [PROFILE]  ", col);
                io.writeln("Show differences between home and backup files", &[]);
                io.writeln("", &[]);
                io.writeln("Flags:", col);
                io.write("  --all -a        ", col);
                io.writeln("Show files which are ignored by default", &[]);
            }
            "save" => {
                io.writeln("Commands:", col);
                io.write("  save [PROFILE]  ", col);
                io.writeln("Save changes from the home directory to the backup", &[]);
                io.writeln("", &[]);
                io.writeln("Flags:", col);
                io.write("  --assumeyes -y  ", col);
                io.writeln("Automatically answer yes to all prompts", &[]);
                io.write("  --assumeno -n   ", col);
                io.writeln("Automatically answer no to all prompts", &[]);
                io.write("  --all -a        ", col);
                io.writeln("Show files which are ignored by default", &[]);
            }
            "restore" => {
                io.writeln("Commands:", col);
                io.write("  restore [PROFILE]   ", col);
                io.writeln("Restore changes from the backup to the home directory", &[]);
                io.writeln("", &[]);
                io.writeln("Flags:", col);
                io.write("  --assumeyes -y      ", col);
                io.writeln("Automatically answer yes to all prompts", &[]);
                io.write("  --assumeno -n       ", col);
                io.writeln("Automatically answer no to all prompts", &[]);
                io.write("  --all -a            ", col);
                io.writeln("Show files which are ignored by default", &[]);
            }
            "rmhome" => {
                io.writeln("Commands:", col);
                io.write("  rmhome [PROFILE]    ", col);
                io.writeln("Delete files from home directory [DANGEROUS]", &[]);
                io.writeln("", &[]);
                io.writeln("Flags:", col);
                io.write("  --assumeyes -y      ", col);
                io.writeln("Automatically answer yes to all prompts", &[]);
                io.write("  --assumeno -n       ", col);
                io.writeln("Automatically answer no to all prompts", &[]);
            }
            "rmbackup" => {
                io.writeln("Commands:", col);
                io.write("  rmbackup [PROFILE]  ", col);
                io.writeln("Delete files from backup directory [DANGEROUS]", &[]);
                io.writeln("", &[]);
                io.writeln("Flags:", col);
                io.write("  --assumeyes -y      ", col);
                io.writeln("Automatically answer yes to all prompts", &[]);
                io.write("  --assumeno -n       ", col);
                io.writeln("Automatically answer no to all prompts", &[]);
            }
            _ => {
                io.writeln("Commands:", col);
                io.write("  AUTOSAVER_ROOT      ", col);
                io.writeln("Set the root directory for the program", &[]);
                io.write("  AUTOSAVER_HOME      ", col);
                io.writeln("Set the backup directory for the program", &[]);
                io.write("  AUTOSAVER_PROFILE   ", col);
                io.writeln("Set the profile to use if none are passed", &[]);
                io.writeln("", &[]);
                io.writeln("Commands:", col);
                io.write("  list                ", col);
                io.writeln("Show differences between home and backup files", &[]);
                io.write("  save                ", col);
                io.writeln("Save changes from the home directory to the backup", &[]);
                io.write("  restore             ", col);
                io.writeln("Restore changes from the backup to the home directory", &[]);
                io.write("  rmhome              ", col);
                io.writeln("Delete files from home directory [DANGEROUS]", &[]);
                io.write("  rmbackup            ", col);
                io.writeln("Delete files from backup directory [DANGEROUS]", &[]);
                io.writeln("", &[]);
                io.writeln("Flags:", col);
                io.write("  --help -h           ", col);
                io.writeln("Print the help message for commands and subcommands", &[]);
                io.write("  --version           ", col);
                io.writeln("Print the binary version", &[]);
                io.write("  --nocolor           ", col);
                io.writeln("Avoid all colors in the output", &[]);
            }
        }
        Ok(())
    }
}
