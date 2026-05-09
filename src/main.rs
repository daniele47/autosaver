use std::{env, process::exit};

use autosaver::{
    cli::{
        actions::Runner,
        flags::ParsedArgs,
        inout::{IoOutOptions, TermInOut},
    },
    debug,
};

fn main() {
    // parse cmdline args
    let parsed_args = ParsedArgs::parse(env::args().skip(1).collect());

    // get a frontend renderer
    let inout = TermInOut::new(IoOutOptions::new(true, false));

    // get cli runner
    let mut runner = Runner::new(parsed_args, inout);

    // run cli
    if let Err(e) = runner.run() {
        runner.inout.error(&e);
        debug!(runner.inout, "\n{}", e.backtrace());
        exit(1);
    }
}
