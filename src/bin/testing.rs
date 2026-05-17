use std::env;

use autosaver::{fs::abs::AbsPathStr, log::LogOptions};

fn main() -> anyhow::Result<()> {
    LogOptions::new(env::args().nth(1).as_deref().unwrap_or("debug")).init();

    let home = AbsPathStr::new(env::var("HOME")?)?;
    let abs1 = home.join(&".config".parse()?)?.join(&"nvim".parse()?)?;

    let mut res = vec![];

    abs1.find_all(&mut res, &mut Default::default())?;

    Ok(())
}
