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

#[macro_export]
macro_rules! cout {
    ($style:expr, $($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        $crate::out!("{}", format!($($arg)*).style($style));
    }};
}
#[macro_export]
macro_rules! coutnow {
    ($style:expr, $($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        $crate::outnow!("{}", format!($($arg)*).style($style));
    }};
}
#[macro_export]
macro_rules! coutln {
    ($style:expr, $($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        $crate::outln!("{}", format!($($arg)*).style($style));
    }};
}

#[macro_export]
macro_rules! cerr {
    ($style:expr, $($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        $crate::err!("{}", format!($($arg)*).style($style));
    }};
}
#[macro_export]
macro_rules! cerrnow {
    ($style:expr, $($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        $crate::errnow!("{}", format!($($arg)*).style($style));
    }};
}
#[macro_export]
macro_rules! cerrln {
    ($style:expr, $($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        $crate::errln!("{}", format!($($arg)*).style($style));
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
