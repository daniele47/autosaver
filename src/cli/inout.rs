#[macro_export]
macro_rules! out {
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
    ($($arg:tt)*) => {{
        use std::io::Write;
        let _ = writeln!(std::io::stdout(), $($arg)*);
    }};
}

#[macro_export]
macro_rules! err {
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

#[macro_export]
macro_rules! verbose {
    ($($arg:tt)*) => {{
        $crate::err!("verbose: ");
        $crate::errln!($($arg)*);
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        $crate::err!("{}", "error: ".red().bold());
        $crate::errln!($($arg)*);
    }};
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        $crate::err!("{}", "warning: ".yellow().bold());
        $crate::errln!($($arg)*);
    }};
}
