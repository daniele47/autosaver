//! Module to run cli.

use crate::cli::{
    error::{Error, Result},
    flags::{Flag, ParsedArgs},
    output::Renderer,
};

mod version;

/// Struct with data and methods to run cli.
pub struct Runner<I>
where
    I: Renderer,
{
    args: ParsedArgs,
    renderer: I,
}

impl<I> Runner<I>
where
    I: Renderer<Error = Error>,
{
    /// Create new runner.
    pub fn new(args: ParsedArgs, renderer: I) -> Self {
        Self { args, renderer }
    }

    /// Run the cli application.
    pub fn run(&mut self) -> Result<()> {
        let flags = self.args.flags();
        let flag_help = flags.contains(&Flag::Word("help".into()));
        let flag_help = flag_help || flags.contains(&Flag::Letter('h'));
        let flag_version = flags.contains(&Flag::Word("version".into()));
        let flag_version = flag_version || flags.contains(&Flag::Letter('v'));
        let flag_nocolor = flags.contains(&Flag::Word("nocolor".into()));

        if flag_nocolor {
            self.renderer.options().has_colors = false;
        }
        if flag_version {
            return self.version();
        }
        if flag_help {
            todo!()
        }

        if let Some(cmd) = self.args.params().first() {
            match cmd.as_str() {
                "list" | "save" | "restore" => todo!(),
                _ => todo!(),
            }
        }

        Ok(())
    }
}
