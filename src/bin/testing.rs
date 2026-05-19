use std::{str::FromStr, time::Instant};

use autosaver::fs::abs::AbsPathStr;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> anyhow::Result<()> {
    init_logs();

    let abs = AbsPathStr::from_str("/home")?;
    let time = Instant::now();
    let mut count = 0;
    abs.find(
        |_| {
            count += 1;
            if count % 1024 == 0 {
                println!("{count}");
            }
            Ok(())
        },
        &mut Default::default(),
    )?;
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
