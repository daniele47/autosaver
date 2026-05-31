use owo_colors::OwoColorize;

use crate::{
    cli::{Cli, CliCmd, ctx::CliContext},
    out, outln,
    prof::{ProfileKind, TraverseDupPolicy, TraverseOpts},
};

const TREE: [&str; 4] = ["│   ", "    ", "├── ", "└── "];

impl Cli {
    pub fn action_tree(&self, ctx: &CliContext) -> anyhow::Result<()> {
        match &self.cmd {
            CliCmd::Tree {
                no_dedup,
                show_id,
                ignore,
            } => {
                let trav_opts = if *no_dedup {
                    TraverseDupPolicy::Include
                } else {
                    TraverseDupPolicy::Shallow
                };
                let trav_opts = TraverseOpts::new(trav_opts);
                let mut are_last = Vec::<bool>::new();
                ctx.profiles
                    .traverse(&ctx.curr_profile, trav_opts, |trav_ctx| {
                        if ignore.contains(trav_ctx.name)
                            || trav_ctx.path.iter().any(|p| ignore.contains(p))
                        {
                            return Ok(());
                        }
                        let len = trav_ctx.path.len();

                        // render indent lines
                        let is_last = trav_ctx.stack.last().map(|(p, _)| p) == trav_ctx.path.last();
                        if let Some(is_last_nth) = are_last.get_mut(len) {
                            *is_last_nth = is_last;
                        } else {
                            are_last.push(is_last);
                        }
                        for item in are_last.iter().take(len).skip(1) {
                            let line_start = if *item { TREE[1] } else { TREE[0] };
                            out!("{line_start}");
                        }
                        if len > 0 {
                            let line_last = if is_last { TREE[3] } else { TREE[2] };
                            out!("{line_last}");
                        }
                        // render profile name
                        let prof_style = match trav_ctx.item.kind() {
                            ProfileKind::Composite(_) => ctx.col.tree_composite,
                            ProfileKind::Module(_) => ctx.col.tree_module,
                            ProfileKind::Runner(_) => ctx.col.tree_runner,
                        };
                        out!("{}", trav_ctx.name.display().style(prof_style));
                        // show profile id
                        if *show_id && let Some(id) = trav_ctx.item.id() {
                            out!(" ({})", id.display());
                        }
                        // render dedup symbol
                        if !no_dedup && trav_ctx.is_dup {
                            out!(" {}", "(*)".style(ctx.col.tree_dedup));
                        }
                        outln!();
                        Ok(())
                    })
            }
            _ => unreachable!("Mismatching command"),
        }
    }
}
