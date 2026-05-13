use std::{env, process::exit};

use autosaver::{
    cli::{actions::Runner, flags::ParsedArgs},
    debug,
};

fn main() {
    // parse cmdline args
    let parsed_args = ParsedArgs::parse(env::args().skip(1).collect());

    // get cli runner
    let runner = Runner::new(parsed_args);

    // run cli
    if let Err(e) = runner.run() {
        runner.inout.error(&e);
        debug!(runner.inout, "\n{}", e.backtrace());
        exit(1);
    }
}
