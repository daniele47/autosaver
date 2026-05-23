#[macro_export]
macro_rules! out {
    ($($arg:tt)*) => {{
        use std::io::Write;
        let _ = write!(std::io::stdout(), $($arg)*);
    }};
}

#[macro_export]
macro_rules! outnow {
    ($($arg:tt)*) => {{
        $crate::out($($arg)*);
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
    ($($arg:tt)*) => {{
        $crate::err($($arg)*);
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
    ($buf:expr) => {
        use std::io::Write;
        std::io::stdout().flush().unwrap();
        $buf.clear();
        std::io::stdin().read_line($buf).unwrap();
        $buf.trim().to_string()
    };
}

#[macro_export]
macro_rules! verbose {
    ($($arg:tt)*) => {{
        $crate::err!("verbose: ");
        $crate::errln!($($arg)*);
    }};
}
