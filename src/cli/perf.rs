use std::{env, time::Instant};

use crate::verbose;

pub fn perf<T>(msg: &str, run: impl FnOnce() -> T) -> T {
    if env::var_os("PERF").is_some() {
        let start = Instant::now();
        let res = run();
        verbose!("{msg} {:.6}s", start.elapsed().as_secs_f64());
        res
    } else {
        run()
    }
}
