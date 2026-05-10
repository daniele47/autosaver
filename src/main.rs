use std::{env, process::exit};

use autosaver::{
    cli::{actions::Runner, flags::ParsedArgs, inout::TermInOut},
    debug,
};

fn main() {
    // parse cmdline args
    let parsed_args = ParsedArgs::parse(env::args().skip(1).collect());

    // get a frontend renderer
    let inout = TermInOut::default();

    // get cli runner
    let mut runner = Runner::new(parsed_args, inout);

    // initalize runner
    runner.init();

    // run cli
    if let Err(e) = runner.run() {
        runner.inout.error(&e);
        debug!(runner.inout, "\n{}", e.backtrace());
        exit(1);
    }
}
