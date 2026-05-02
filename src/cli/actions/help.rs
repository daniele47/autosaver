use crate::cli::{actions::Runner, error::Result, inout::InOut};

impl<I: InOut> Runner<I> {
    /// Help action to render help message.
    pub fn help(&mut self) -> Result<()> {
        self.check_flags(&["--help", "--nocolor"])?;
        let first = self.args.params().first().map(String::as_ref).unwrap_or("");
        let second = self.args.params().get(1).map(String::as_ref).unwrap_or("");
        let third = self.args.params().get(2).map(String::as_ref).unwrap_or("");
        let col = Self::HELP_COLOR;
        match first {
            _ => {
                self.inout.writeln("Commands:", col);
                self.inout.write("  list        ", col);
                self.inout.writeln(
                    "Show differences between home and backup files for a profile",
                    &[],
                );
                self.inout.write("  save        ", col);
                self.inout
                    .writeln("Save changes from the home directory to the backup", &[]);
                self.inout.write("  restore     ", col);
                self.inout
                    .writeln("Restore changes from the backup to the home directory", &[]);
                self.inout.write("  delete      ", col);
                self.inout
                    .writeln("Delete files from both home and backup directories", &[]);
                self.inout.writeln("", &[]);
                self.inout.writeln("Flags:", col);
                self.inout.write("  --help -h   ", col);
                self.inout
                    .writeln("Print the help message for commands and subcommands", &[]);
                self.inout.write("  --version   ", col);
                self.inout.writeln("Print the binary version", &[]);
                self.inout.write("  --nocolor   ", col);
                self.inout.writeln("Avoid all colors in the output", &[]);
            }
        }
        Ok(())
    }
}
