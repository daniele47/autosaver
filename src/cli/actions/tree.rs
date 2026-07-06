use crate::{
    cli::{Cli, CliCmd, config::CliContext},
    cout, coutln,
    prof::{ProfileKind, TraverseDupPolicy},
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
                let trav_dups = if *no_dedup {
                    TraverseDupPolicy::Include
                } else {
                    TraverseDupPolicy::Shallow
                };
                let mut are_last = Vec::<bool>::new();
                ctx.profiles.traverse_opts(
                    &ctx.curr_profile,
                    trav_dups,
                    |e| !ignore.contains(e.child()),
                    |trav_ctx| {
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
                            cout!(ctx.col.default, "{line_start}");
                        }
                        if len > 0 {
                            let line_last = if is_last { TREE[3] } else { TREE[2] };
                            cout!(ctx.col.default, "{line_last}");
                        }
                        // render profile name
                        let prof_style = match trav_ctx.item.kind() {
                            ProfileKind::Composite(_) => ctx.col.tree_composite,
                            ProfileKind::Module(_) => ctx.col.tree_module,
                            ProfileKind::Runner(_) => ctx.col.tree_runner,
                        };
                        cout!(prof_style, "{}", trav_ctx.name.display());
                        // show profile id
                        if *show_id && let Some(id) = trav_ctx.item.id() {
                            cout!(ctx.col.default, " ({})", id.display());
                        }
                        // render dedup symbol
                        if !no_dedup && trav_ctx.is_dup {
                            cout!(ctx.col.tree_dedup, "(*)");
                        }
                        coutln!(ctx.col.default, "");
                        Ok(())
                    },
                )
            }
            _ => unreachable!("Mismatching command"),
        }
    }
}
