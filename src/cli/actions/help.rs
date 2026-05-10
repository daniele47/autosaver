use crate::{
    cli::{actions::Runner, error::Result},
    debug,
};

impl Runner {
    /// Help action to render help message.
    pub fn help(&self) -> Result<()> {
        debug!(self.inout, "Running help action...");

        self.check_flags("--help", &["--help", "--no-color", "-h"])?;
        let command = self.args.params().join(" ");
        let col = Self::DECORATION_COL;
        let nocol = Self::NO_COL;
        let io = &self.inout;
        match command.as_str() {
            "list" => {
                io.writeln("Commands:", col);
                io.write("  list [PROFILE]  ", col);
                io.writeln("List changes from the home directory to the backup", nocol);
                io.writeln("", nocol);
                io.writeln("Flags:", col);
                io.write("  --all -a        ", col);
                io.writeln("Show files which are ignored by default", nocol);
                io.write("  --unmodified -u ", col);
                io.writeln("Show tracked and unmodified files too", nocol);
            }
            "save" => {
                io.writeln("Commands:", col);
                io.write("  save [PROFILE]  ", col);
                io.writeln("Save changes from the home directory to the backup", nocol);
                io.writeln("", nocol);
                io.writeln("Flags:", col);
                io.write("  --all -a        ", col);
                io.writeln("Show files which are ignored by default", nocol);
                io.write("  --diff -d       ", col);
                io.writeln("Show some lines of diff between files", nocol);
                io.write("  --list -l       ", col);
                io.writeln("Only list files, do not prompt to save them", nocol);
                io.write("  --full -f       ", col);
                io.writeln("Show the entire diff between the files", nocol);
            }
            "restore" => {
                io.writeln("Commands:", col);
                io.write("  restore [PROFILE]   ", col);
                io.writeln("Restore changes from the backup to the home dir", nocol);
                io.writeln("", nocol);
                io.writeln("Flags:", col);
                io.write("  --all -a            ", col);
                io.writeln("Show files which are ignored by default", nocol);
                io.write("  --diff -d           ", col);
                io.writeln("Show some lines of diff between files", nocol);
                io.write("  --list -l           ", col);
                io.writeln("Only list files, do not prompt to restore them", nocol);
                io.write("  --full -f           ", col);
                io.writeln("Show the entire diff between the files", nocol);
            }
            "rmhome" => {
                io.writeln("Commands:", col);
                io.write("  rmhome [PROFILE]    ", col);
                io.writeln("Delete files from home directory [DANGEROUS]", nocol);
                io.writeln("", nocol);
                io.writeln("Flags:", col);
                io.write("  --list -l           ", col);
                io.writeln("Only list files, do not prompt to delete them", nocol);
            }
            "rmbackup" => {
                io.writeln("Commands:", col);
                io.write("  rmbackup [PROFILE]  ", col);
                io.writeln("Delete files from backup directory [DANGEROUS]", nocol);
                io.writeln("", nocol);
                io.writeln("Flags:", col);
                io.write("  --list -l           ", col);
                io.writeln("Only list files, do not prompt to delete them", nocol);
            }
            "run" => {
                io.writeln("Commands:", col);
                io.write("  run [PROFILE]   ", col);
                io.writeln("Run scripts from the run directory", nocol);
                io.writeln("", nocol);
                io.writeln("Flags:", col);
                io.write("  --show -s       ", col);
                io.writeln("Show the scripts before running them", nocol);
                io.write("  --list -l       ", col);
                io.writeln("Only list scripts, do not prompt to run them", nocol);
                io.write("  --full -f       ", col);
                io.writeln("Show the entire script and output", nocol);
            }
            "clear" => {
                io.writeln("Commands:", col);
                io.write("  clear [PROFILE] ", col);
                io.writeln("Clear all untracked files", nocol);
                io.writeln("", nocol);
                io.writeln("Flags:", col);
                io.write("  --list -l       ", col);
                io.writeln("Only list files, do not prompt to delete them", nocol);
                io.write("  --symlinks -s   ", col);
                io.writeln("Allow clearing symlinks too", nocol);
            }
            "tree" => {
                io.writeln("Commands:", col);
                io.write("  tree [PROFILE]      ", col);
                io.writeln("Draw a tree of how a profile resolves", nocol);
                io.writeln("", nocol);
                io.writeln("Flags:", col);
                io.write("  --short-names -n    ", col);
                io.writeln("Show profile names as their basename", nocol);
                io.write("  --show-types -t     ", col);
                io.writeln("Show the type of each profile", nocol);
                io.write("  --ascii -a          ", col);
                io.writeln("Use only ascii characters", nocol);
            }
            "" => {
                io.writeln("Environment variables:", col);
                io.write("  AUTOSAVER_ROOT      ", col);
                io.writeln("Set the root directory for the program", nocol);
                io.write("  AUTOSAVER_HOME      ", col);
                io.writeln("Set the backup directory for the program", nocol);
                io.write("  AUTOSAVER_PROFILE   ", col);
                io.writeln("Set the profile to use if none are passed", nocol);
                io.writeln("", nocol);
                io.writeln("Config files:", col);
                io.write("  .default            ", col);
                io.writeln("Specify default profile, when none are provided", nocol);
                io.writeln("", nocol);
                io.writeln("Commands:", col);
                io.write("  list                ", col);
                io.writeln("List changes from the home directory to the backup", nocol);
                io.write("  save                ", col);
                io.writeln("Save changes from the home directory to the backup", nocol);
                io.write("  restore             ", col);
                io.writeln("Restore changes from the backup to the home dir", nocol);
                io.write("  rmhome              ", col);
                io.writeln("Delete files from home directory [DANGEROUS]", nocol);
                io.write("  rmbackup            ", col);
                io.writeln("Delete files from backup directory [DANGEROUS]", nocol);
                io.write("  run                 ", col);
                io.writeln("Run scripts from the run directory", nocol);
                io.write("  clear               ", col);
                io.writeln("List and prompt to delete untracked files", nocol);
                io.write("  tree                ", col);
                io.writeln("Draw a tree of how a profile resolves", nocol);
                io.writeln("", nocol);
                io.writeln("Global Flags:", col);
                io.write("  --help -h           ", col);
                io.writeln("Print the help message for commands and subcommands", nocol);
                io.write("  --version           ", col);
                io.writeln("Print the binary version", nocol);
                io.write("  --no-color          ", col);
                io.writeln("Avoid all colors in the output", nocol);
                io.write("  --debug             ", col);
                io.writeln("Show extra output to debug issues", nocol);
                io.write("  --assume-yes -y     ", col);
                io.writeln("Automatically answer yes to all prompts", nocol);
                io.write("  --assume-no -n      ", col);
                io.writeln("Automatically answer no to all prompts", nocol);
            }
            _ => self.invalid_cmd_err()?,
        }
        Ok(())
    }
}
