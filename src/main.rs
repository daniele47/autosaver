use autosaver::{
    cli::{Cli, EarlyQuit},
    error,
};
use clap::Parser;

fn main() {
    // parse cmdline
    let cli = Cli::parse();
    // run application
    let run_res = cli.run_cmd();

    // handle error
    let code = match run_res {
        Ok(_) => 0,
        Err(err) if err.downcast_ref::<EarlyQuit>().is_some() => 0,
        Err(err) => {
            error!("{err:?}");
            1
        }
    };

    std::process::exit(code)
}
