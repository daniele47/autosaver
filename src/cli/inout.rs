use std::sync::atomic::{AtomicBool, Ordering};

#[macro_export]
macro_rules! out {
    () => {{}};
    ($($arg:tt)*) => {{
        use std::io::Write;
        let _ = write!(std::io::stdout(), $($arg)*);
    }};
}
#[macro_export]
macro_rules! outnow {
    () => {{
        use std::io::Write;
        let _ = std::io::stdout().flush();
    }};
    ($($arg:tt)*) => {{
        use std::io::Write;
        let _ = write!(std::io::stdout(), $($arg)*);
        let _ = std::io::stdout().flush();
    }};
}
#[macro_export]
macro_rules! outln {
    () => {{
        use std::io::Write;
        let _ = writeln!(std::io::stdout());
    }};
    ($($arg:tt)*) => {{
        use std::io::Write;
        let _ = writeln!(std::io::stdout(), $($arg)*);
    }};
}

#[macro_export]
macro_rules! err {
    () => {{}};
    ($($arg:tt)*) => {{
        use std::io::Write;
        let _ = write!(std::io::stderr(), $($arg)*);
    }};
}
#[macro_export]
macro_rules! errnow {
    () => {{
        use std::io::Write;
        let _ = std::io::stderr().flush();
    }};
    ($($arg:tt)*) => {{
        use std::io::Write;
        let _ = write!(std::io::stderr(), $($arg)*);
        let _ = std::io::stderr().flush();
    }};
}
#[macro_export]
macro_rules! errln {
    () => {{
        use std::io::Write;
        let _ = writeln!(std::io::stderr());
    }};
    ($($arg:tt)*) => {{
        use std::io::Write;
        let _ = writeln!(std::io::stderr(), $($arg)*);
    }};
}

#[macro_export]
macro_rules! inputln {
    () => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input
    }};
}

// [DIRTY] macros using OwoColorize crate

pub static COLOR_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn init_colors() {
    if std::env::var_os("NO_COLOR").is_none() {
        COLOR_ENABLED.store(true, Ordering::Relaxed);
    }
}

#[macro_export]
macro_rules! cout {
    ($style:expr, $($arg:tt)*) => {{
        if $crate::cli::inout::COLOR_ENABLED.load(std::sync::atomic::Ordering::Relaxed){
            use owo_colors::OwoColorize;
            $crate::out!("{}", format!($($arg)*).style($style));
        }else{
            $crate::out!("{}", format!($($arg)*))
        }
    }};
}
#[macro_export]
macro_rules! coutnow {
    ($style:expr, $($arg:tt)*) => {{
        $crate::cout!($style, $($arg)*);
        $crate::outnow!();
    }};
}
#[macro_export]
macro_rules! coutln {
    ($style:expr, $($arg:tt)*) => {{
        $crate::cout!($style, $($arg)*);
        $crate::outln!();
    }};
}

#[macro_export]
macro_rules! cerr {
    ($style:expr, $($arg:tt)*) => {{
        if $crate::cli::inout::COLOR_ENABLED.load(std::sync::atomic::Ordering::Relaxed){
            use owo_colors::OwoColorize;
            $crate::err!("{}", format!($($arg)*).style($style));
        }else{
            $crate::err!("{}", format!($($arg)*))
        }
    }};
}
#[macro_export]
macro_rules! cerrnow {
    ($style:expr, $($arg:tt)*) => {{
        $crate::cerr!($style, $($arg)*);
        $crate::errnow!();
    }};
}
#[macro_export]
macro_rules! cerrln {
    ($style:expr, $($arg:tt)*) => {{
        $crate::cerr!($style, $($arg)*);
        $crate::errln!();
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use owo_colors::Style;
        $crate::cerr!(Style::new().red().bold(), "error: ");
        $crate::errln!($($arg)*);
    }};
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {{
        use owo_colors::Style;
        $crate::cerr!(Style::new().yellow().bold(), "warning: ");
        $crate::errln!($($arg)*);
    }};
}
