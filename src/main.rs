use anyhow::Result;
use tracing::{trace, *};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub mod fs;

fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer().with_timer(fmt::time::ChronoLocal::rfc_3339()))
        .with(EnvFilter::new("debug"))
        .init();

    trace!("Test trace");
    warn!("Wanr");
    info!("Info");
    debug!("debug");
    error!("Err");

    Ok(())
}
