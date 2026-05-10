use crate::{
    cli::{actions::Runner, error::Result},
    core::profile::composite::ProfileLoader,
    debug,
};

const TREE: [&str; 4] = ["│   ", "    ", "├──", "└──"];

impl Runner {
    /// Help action to render help message.
    pub fn tree(&self) -> Result<()> {
        debug!(self.inout, "Running tree action...");

        // checks
        if self.args.params().len() > 2 {
            return self.invalid_cmd_err();
        }
        self.check_flags("tree", &["--no-color", "--debug"])?;

        // load profile
        let profile = self.load_profile(1)?;
        let mut loader = self.profile_loader()?;
        let root_profile = loader.load(&profile)?;
        self.output_main_profile(&profile);

        // descent into profiles
        root_profile.descend(&mut loader, |ctx| {
            let p = ctx.path;
            println!(
                "{}{}{}",
                TREE[0].repeat(p.len().saturating_sub(1)),
                TREE[3].repeat(1.min(p.len())),
                ctx.item.name()
            );
            Ok(())
        })?;

        Ok(())
    }
}
