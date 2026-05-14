use std::str::FromStr;

use anyhow::Result;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::fs::abs::AbsPathStr;

pub mod fs;

fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer().with_timer(fmt::time::ChronoLocal::rfc_3339()))
        .with(EnvFilter::new("debug"))
        .init();

    let _ = AbsPathStr::from_str("/bin/tree")?;

    Ok(())
}
