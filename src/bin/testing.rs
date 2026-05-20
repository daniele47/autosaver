use std::{env, str::FromStr};

use anyhow::Context;
use autosaver::{
    cli::ctx::CliContext,
    fs::{abs::AbsPathStr, rel::RelPathStr},
};

fn main() -> anyhow::Result<()> {
    let home = AbsPathStr::new_from_pathbuf(env::home_dir().context("err")?)?;
    let root = AbsPathStr::new_from_pathbuf(env::current_dir()?)?;
    let ctx = CliContext::new(&Some(home), &Some(root))?;

    ctx.profiles()
        .traverse(&RelPathStr::from_str("all")?, Default::default(), |ctx| {
            println!("{}", ctx.item.name().display());
            Ok(())
        })?;

    Ok(())
}
