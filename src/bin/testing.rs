use std::env;

use anyhow::Context;
use autosaver::{cli::ctx::CliContext, fs::abs::AbsPathStr};

fn main() -> anyhow::Result<()> {
    let home = AbsPathStr::new_from_pathbuf(env::home_dir().context("err")?)?;
    let root = AbsPathStr::new_from_pathbuf(env::current_dir()?)?;
    let ctx = CliContext::new(&Some(home), &Some(root), &None)?;

    ctx.profiles
        .traverse(&ctx.root_profile, Default::default(), |ctx| {
            println!("{}", ctx.item.name().display());
            Ok(())
        })?;

    Ok(())
}
