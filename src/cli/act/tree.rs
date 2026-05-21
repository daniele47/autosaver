use owo_colors::OwoColorize;

use crate::{
    cli::{Cli, CliCmd, ctx::CliContext},
    out, outln,
    prof::{ProfileKind, TraverseOpts},
};

const TREE: [&str; 4] = ["│   ", "    ", "├── ", "└── "];

impl Cli {
    pub fn action_tree(&self) -> anyhow::Result<()> {
        match self.cmd {
            CliCmd::Tree { no_dedup, show_id } => {
                let ctx = CliContext::new(&self.home, &self.root)?;
                let trav_opts = TraverseOpts::new(no_dedup);
                let mut are_last = Vec::<bool>::new();
                ctx.profiles().traverse(&self.profile, trav_opts, |ctx| {
                    let len = ctx.path.len();

                    // render indent lines
                    let is_last = ctx.stack.last().map(|(p, _)| p) == ctx.path.last();
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
                    let prof_style = match ctx.item.kind() {
                        ProfileKind::Composite(_) => CliContext::TREE_COMPOSITE,
                        ProfileKind::Module(_) => CliContext::TREE_MODULE,
                        ProfileKind::Runner(_) => CliContext::TREE_RUNNER,
                    };
                    out!("{}", ctx.item.name().display().style(prof_style));
                    // show profile id
                    if show_id && !matches!(ctx.item.kind(), ProfileKind::Composite(_)) {
                        out!(" ({})", ctx.item.id().display());
                    }
                    // render dedup symbol
                    if !no_dedup && ctx.is_dup {
                        out!(" {}", "(*)".style(CliContext::TREE_DEDUP));
                    }
                    outln!();
                    Ok(())
                })
            }
            _ => unreachable!("Tree command should be tree"),
        }
    }
}
