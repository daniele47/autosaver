use crate::{
    cli::{actions::Runner, error::Result, flags::Flag},
    core::{
        fs::RelPath,
        profile::{ProfileType, composite::ProfileLoader},
    },
    debug,
};

const TREE: [&str; 4] = ["│   ", "    ", "├── ", "└── "];
const TREE_ASCII: [&str; 4] = ["|   ", "    ", "+-- ", "`-- "];

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
            return self.invalid_args_err(1);
        }
        self.check_flags(
            "tree",
            &[
                "--short-names",
                "-n",
                "--show-types",
                "-t",
                "--ascii",
                "-a",
                "--no-color",
                "--debug",
            ],
        )?;

        // flags
        let wflag_short_names = self
            .args
            .flags()
            .contains(&Flag::Word("short_names".into()));
        let lflag_short_names = self.args.flags().contains(&Flag::Letter('n'));
        let flag_short_names = wflag_short_names || lflag_short_names;
        let wflag_show_types = self.args.flags().contains(&Flag::Word("show_types".into()));
        let lflag_show_types = self.args.flags().contains(&Flag::Letter('t'));
        let flag_show_types = wflag_show_types || lflag_show_types;
        let wflag_ascii = self.args.flags().contains(&Flag::Word("ascii".into()));
        let lflag_ascii = self.args.flags().contains(&Flag::Letter('a'));
        let flag_ascii = wflag_ascii || lflag_ascii;

        // load profile
        let profile = self.load_profile(1)?;
        let mut loader = self.profile_loader()?;
        let root_profile = loader.load(&profile)?;
        self.output_main_profile(&profile);

        // descent into profiles
        let mut are_last = Vec::<bool>::new();
        let chars = if flag_ascii { TREE_ASCII } else { TREE };
        root_profile.descend(true, &mut loader, |ctx| {
            let p = ctx.path;
            let is_last = ctx.stack.last().map(|(p, _)| p) == ctx.path.last();
            let len = ctx.path.len();
            let line = if is_last { chars[3] } else { chars[2] };
            insert_at(&mut are_last, ctx.path.len(), is_last);
            for item in are_last.iter().take(len).skip(1) {
                let line = if *item { chars[1] } else { chars[0] };
                self.inout.write(line, Self::NO_COL);
            }
            self.inout.write(line.repeat(1.min(p.len())), Self::NO_COL);
            let item_col = match ctx.item.ptype() {
                ProfileType::Composite(_) => Self::TREE_COMPOSITE_COL,
                ProfileType::Module(_) => Self::TREE_MODULE_COL,
                ProfileType::Runner(_) => Self::TREE_RUNNER_COL,
            };
            let mut item_name = ctx.item.name().to_string();
            if flag_short_names {
                item_name = RelPath::from(item_name).basename().to_str_lossy();
            }
            if flag_show_types {
                match ctx.item.ptype() {
                    ProfileType::Composite(_) => item_name.insert_str(0, "[C] "),
                    ProfileType::Module(_) => item_name.insert_str(0, "[M] "),
                    ProfileType::Runner(_) => item_name.insert_str(0, "[R] "),
                };
            }
            self.inout.writeln(item_name, item_col);
            Ok(())
        })?;

        Ok(())
    }
}
