use std::env;

use autosaver::cli::{actions::Runner, error::Result, flags::ParsedArgs};

fn main() -> Result<()> {
    let parsed_args = ParsedArgs::parse(env::args().collect());
    let mut runner = Runner::new(parsed_args);
    runner.run()
}
