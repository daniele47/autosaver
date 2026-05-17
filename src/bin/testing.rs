use std::env;

use autosaver::{fs::abs::AbsPathStr, log::LogOptions, prof::Profile};

pub fn main() -> anyhow::Result<()> {
    LogOptions::new(env::args().nth(1).as_deref().unwrap_or("debug")).init();

    // find_all()?;
    parse_profile()?;

    Ok(())
}

pub fn find_all() -> anyhow::Result<()> {
    let home = AbsPathStr::new(env::var("HOME")?)?;
    let abs1 = home.join(&".config".parse()?)?.join(&"nvim".parse()?)?;

    let mut res = vec![];

    abs1.find_all(&mut res, &mut Default::default())?;
    Ok(())
}

pub fn parse_profile() -> anyhow::Result<()> {
    let config = r#"
        /! kind composite

        awd/////
    "#;
    Profile::parse_config(config, "test")?;
    Ok(())
}
