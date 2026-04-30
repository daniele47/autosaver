//! Module to run cli.

use crate::cli::{error::Result, flags::ParsedArgs};

/// Struct with data and methods to run cli.
pub struct Runner {
    args: ParsedArgs,
}

impl Runner {
    /// Create new runner.
    pub fn new(args: ParsedArgs) -> Self {
        Self { args }
    }

    /// Run the cli application.
    pub fn run(&mut self) -> Result<()> {
        Ok(())
    }
}
