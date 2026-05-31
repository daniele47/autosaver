use autosaver::{
    cli::{Cli, error::EarlyQuit, perf::perf},
    errnow, error, outnow,
};
use clap::Parser;

fn main() {
    let run_res = perf("- Executed program   -->", || {
        // parse cmdline
        let cli = perf("  - Parsed cli flags -->", Cli::parse);

        // run application
        perf("  - Executed command -->", || cli.run_cmd())
    });

    // assure output streams are flushed
    outnow!();
    errnow!();

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
