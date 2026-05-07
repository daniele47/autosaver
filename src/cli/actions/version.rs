use crate::cli::{actions::Runner, error::Result};

impl Runner {
    /// Version action to render the binary version.
    pub fn version(&mut self) -> Result<()> {
        let fmt = format!("{} {}", Self::BIN_NAME, Self::CARGO_VERSION);
        if !self.args.params().is_empty() {
            return self.invalid_cmd_err();
        }
        self.check_flags("--version", &["--version", "--no-color"])?;
        self.inout.writeln(fmt, Self::DECORATION_COL);
        Ok(())
    }
}
