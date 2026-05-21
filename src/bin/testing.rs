use autosaver::cli::ctx::CliContext;

fn main() -> anyhow::Result<()> {
    let ctx = CliContext::new(&None, &None, &None)?;

    ctx.profiles
        .traverse(&ctx.root_profile, Default::default(), |ctx| {
            println!("{}", ctx.item.name().display());
            Ok(())
        })?;

    Ok(())
}
