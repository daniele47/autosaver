use crate::{
    cli::{actions::Runner, error::Result},
    debug,
};

impl Runner {
    /// Help action to render help message.
    pub fn tree(&self) -> Result<()> {
        debug!(self.inout, "Running tree action...");

        self.check_flags("tree", &["--no-color"])?;
        todo!()
    }
}
