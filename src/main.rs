use std::str::FromStr;

use anyhow::Result;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::fs::abs::AbsPathStr;

pub mod fs;

fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer().with_timer(fmt::time::ChronoLocal::rfc_3339()))
        .with(EnvFilter::from_default_env())
        .init();

    let path = AbsPathStr::from_str("/bin/tree")?;
    let base = AbsPathStr::from_str("/bin")?;
    let rel = path.to_rel(&base)?;
    println!("{rel:?}");

    Ok(())
}
