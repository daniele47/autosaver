use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::cli::Cli;

pub mod cli;
pub mod fs;
pub mod prof;

fn main() -> Result<()> {
    // enable logging
    tracing_subscriber::registry()
        .with(fmt::layer().with_timer(fmt::time::ChronoLocal::default()))
        .with(EnvFilter::from_default_env())
        .init();

    // launch cli
    Cli::parse().run()
}
