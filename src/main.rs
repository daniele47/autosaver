use std::{env, process::exit};

use autosaver::cli::{
    actions::Runner,
    flags::ParsedArgs,
    inout::{IoOutOptions, TermInOut},
};

fn main() {
    // parse cmdline args
    let parsed_args = ParsedArgs::parse(env::args().skip(1).collect());

    // get a frontend renderer
    let mut inout = TermInOut::new(IoOutOptions::new(true));

    // get cli runner
    let mut runner = Runner::new(parsed_args, inout.clone());

    // run cli
    if let Err(e) = runner.run() {
        inout.error(&e);
        exit(1);
    }
}
