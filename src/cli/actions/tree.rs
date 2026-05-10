use crate::{
    cli::{actions::Runner, error::Result},
    core::profile::{ProfileType, composite::ProfileLoader},
    debug,
};

const TREE: [&str; 4] = ["│   ", "    ", "├──", "└──"];

fn insert_at(vec: &mut Vec<bool>, index: usize, value: bool) {
    if index < vec.len() {
        vec[index] = value;
    } else {
        vec.resize(index + 1, value);
    }
}

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
        let mut are_last = Vec::<bool>::new();
        root_profile.descend(true, &mut loader, |ctx| {
            let p = ctx.path;
            let is_last = ctx.stack.last().map(|(p, _)| p) == ctx.path.last();
            let len = ctx.path.len();
            let line = if is_last { TREE[3] } else { TREE[2] };
            insert_at(&mut are_last, ctx.path.len(), is_last);
            for item in are_last.iter().take(len).skip(1) {
                let line = if *item { TREE[1] } else { TREE[0] };
                self.inout.write(line, Self::NO_COL);
            }
            self.inout.write(line.repeat(1.min(p.len())), Self::NO_COL);
            let item_col = match ctx.item.ptype() {
                ProfileType::Composite(_) => Self::TREE_COMPOSITE_COL,
                ProfileType::Module(_) => Self::TREE_MODULE_COL,
                ProfileType::Runner(_) => Self::TREE_RUNNER_COL,
            };
            self.inout.writeln(ctx.item.name(), item_col);
            Ok(())
        })?;

        Ok(())
    }
}
