use owo_colors::Style;

use crate::{
    cli::{Cli, CliCmd, config::CliContext, prompt::Prompt},
    coutln,
    fs::{path::PathStr, rel::RelPathStr},
};

pub mod backup;
pub mod clear;
pub mod run;
pub mod tree;

impl Cli {
    const SYMLINK_FLAG: &str = "--allow-symlink";
    const PURGE_FLAG: &str = "--allow-purge";

    fn output_profile(profile: &RelPathStr, style: Style) {
        let profile = profile.display();
        coutln!(style, "*** {profile} ***");
    }

    fn output_path(path: impl AsRef<PathStr>, style: Style) {
        let path = path.as_ref();
        coutln!(style, "- {}", path.display());
    }

    pub fn run_cmd(&self) -> anyhow::Result<()> {
        let prompt = Prompt::new(
            self.auto_answers.as_deref().unwrap_or(""),
            self.assume_yes,
            self.assume_no,
            self.dry_run,
        )?;
        let ctx = CliContext::new(&self.home, &self.root, &self.profile, prompt)?;

        match self.cmd {
            CliCmd::List { .. }
            | CliCmd::Save { .. }
            | CliCmd::Restore { .. }
            | CliCmd::Delete { .. } => self.action_backup(&ctx),
            CliCmd::Run { .. } => self.action_run(&ctx),
            CliCmd::Tree { .. } => self.action_tree(&ctx),
            CliCmd::Clear { .. } => self.action_clear(&ctx),
        }
    }
}
