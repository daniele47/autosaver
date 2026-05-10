use crate::{
    cli::{actions::Runner, error::Result},
    debug,
};

impl Runner {
    /// Version action to render the binary version.
    pub fn version(&self) -> Result<()> {
        debug!(self.inout, "Running version action...");
        let fmt = format!("{} {}", Self::BIN_NAME, Self::CARGO_VERSION);
        if !self.args.params().is_empty() {
            return self.invalid_params_err("--version".into(), 0);
        }
        self.check_flags("--version", &["--version", "--no-color", "--debug"])?;
        self.inout.writeln(fmt, Self::DECORATION_COL);
        Ok(())
    }
}
