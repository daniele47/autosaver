use crate::cli::{
    actions::Runner,
    error::{Error, Result},
    output::Renderer,
};

const CARGO_VERSION: &str = env!("CARGO_PKG_VERSION");
const BIN_NAME: &str = env!("CARGO_PKG_NAME");

impl<I> Runner<I>
where
    I: Renderer<Error = Error>,
{
    pub fn version(&mut self) -> Result<()> {
        let fmt = format!("{BIN_NAME} {CARGO_VERSION}");
        self.renderer.writeln(fmt, &[])
    }
}
