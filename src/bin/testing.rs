use std::{
    io::{Write, stdout},
    time::Instant,
};

use autosaver::fs::abs::AbsPathStr;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> anyhow::Result<()> {
    init_logs();

    // let abs = AbsPathStr::try_from(env!("HOME").to_string() + "")?;
    let abs = AbsPathStr::try_from("/tmp".to_string())?;
    let time = Instant::now();
    let mut count = 0;
    abs.find(|ctx| {
        count += 1;
        println!("{}", ctx.path.display());
        if count % 1024 == 0 {
            // print!("\r{count}");
            stdout().flush()?;
        }
        // println!("PATH ({}): {}", ctx.depth, ctx.path.display());
        Ok(true)
    })
    .unwrap();
    println!("{count}");
    println!("Function took: {:?}", time.elapsed());

    Ok(())
}

fn init_logs() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_line_number(true))
        .with(EnvFilter::new("trace"))
        .init();
}
