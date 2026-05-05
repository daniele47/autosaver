use crate::cli::{actions::Runner, error::Result};

impl Runner {
    /// Version action to render the binary version.
    pub fn version(&mut self) -> Result<()> {
        let fmt = format!("{} {}", Self::BIN_NAME, Self::CARGO_VERSION);
        self.check_flags("--version", &["--version", "--nocolor"])?;
        self.inout.writeln(fmt, Self::HELP_COL);
        Ok(())
    }
}
