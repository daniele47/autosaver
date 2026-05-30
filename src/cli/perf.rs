use std::{env, time::Instant};

use crate::performance;

pub fn perf<T>(msg: &str, run: impl FnOnce() -> T) -> T {
    if env::var_os("PERF").is_some() {
        let start = Instant::now();
        let res = run();
        performance!("{msg} {:.6}s", start.elapsed().as_secs_f64());
        res
    } else {
        run()
    }
}
