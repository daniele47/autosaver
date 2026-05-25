use std::time::Instant;

use crate::{
    cli::{
        Cli, CliCmd,
        ctx::{CliContext, Paths},
    },
    verbose,
};

pub mod run;
pub mod tree;

impl Cli {
    pub fn run_cmd(&self) -> anyhow::Result<()> {
        let start = Instant::now();
        let ctx = CliContext::new(&self.home, &self.root, &self.profile)?;

        // verbose output
        if self.verbose {
            verbose!("Configs parsed in {}s", start.elapsed().as_secs_f64());
            verbose!("Current profile: {}", ctx.curr_profile.display());
            verbose!("Home directory: {}", ctx.paths[&Paths::Home].display());
            verbose!("Root directory: {}", ctx.paths[&Paths::Root].display());
            verbose!("Backup directory: {}", ctx.paths[&Paths::Backup].display());
            verbose!("Config directory: {}", ctx.paths[&Paths::Config].display());
            verbose!("Run directory: {}", ctx.paths[&Paths::Run].display());
        }
        let start = Instant::now();

        match self.cmd {
            CliCmd::List => todo!(),
            CliCmd::Save => todo!(),
            CliCmd::Restore => todo!(),
            CliCmd::Delete => todo!(),
            CliCmd::Run { .. } => self.action_run(&ctx),
            CliCmd::Tree { .. } => self.action_tree(&ctx),
            CliCmd::Clear => todo!(),
        }?;

        // show run time on verbose output
        if self.verbose {
            verbose!("Command run in {}s", start.elapsed().as_secs_f64());
        }

        Ok(())
    }
}
