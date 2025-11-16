use std::fmt::Display;

#[derive(Debug)]
#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}]\x1b[0m",
            match self {
                Self::Error => "\x1b[1;31m[ERROR",
                Self::Warn => "\x1b[1;33m[WARN",
                Self::Info => "\x1b[0;32m[INFO",
                Self::Debug => "\x1b[0;36m[DEBUG",
            }
        )
    }
}

/// A logging macro. Takes a [`Level`] and a formatted string.
///
/// [`Level`]: ./logger/enum.Level.html
#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {{
        if $level != Level::Debug || crate::args::VERBOSE.get().unwrap().to_owned() {
            println!(
                "{} {}:{}:{}: {}",
                $level,
                std::module_path!(),
                std::file!(),
                std::line!(),
                format!($($arg)*)
            );
        }
    }};
}
