use crate::{
    cli::{actions::Runner, error::Result},
    debug,
};

impl Runner {
    /// Help action to render help message.
    pub fn tree(&self) -> Result<()> {
        debug!(self.inout, "Running tree action...");

        // checks
        if self.args.params().len() > 1 {
            return self.invalid_cmd_err();
        }
        self.check_flags("tree", &["--no-color", "--debug"])?;
        todo!()
    }
}
