use std::time::Instant;

use crate::{
    cli::{
        Cli,
        ctx::{CliContext, Paths},
    },
    verbose,
};

pub mod tree;

impl Cli {
    pub fn run_cmd(&self) -> anyhow::Result<()> {
        let start = Instant::now();
        let ctx = CliContext::new(&self.home, &self.root, &self.profile)?;

        // verbose output
        if self.verbose {
            verbose!("Configs parsed in {}s", start.elapsed().as_secs_f64());
            verbose!("Current profile: {}", ctx.curr_prof().display());
            verbose!("Home directory: {}", ctx.path(&Paths::Home).display());
            verbose!("Root directory: {}", ctx.path(&Paths::Root).display());
            verbose!("Backup directory: {}", ctx.path(&Paths::Backup).display());
            verbose!("Config directory: {}", ctx.path(&Paths::Config).display());
            verbose!("Run directory: {}", ctx.path(&Paths::Run).display());
        }
        let start = Instant::now();

        match self.cmd {
            super::CliCmd::List => todo!(),
            super::CliCmd::Save => todo!(),
            super::CliCmd::Restore => todo!(),
            super::CliCmd::Delete => todo!(),
            super::CliCmd::Run => todo!(),
            super::CliCmd::Tree { .. } => self.action_tree(&ctx),
            super::CliCmd::Clear => todo!(),
        }?;

        // show run time on verbose output
        if self.verbose {
            verbose!("Command run in {}s", start.elapsed().as_secs_f64());
        }

        Ok(())
    }
}
