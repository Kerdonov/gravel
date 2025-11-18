use std::{fmt::Display, sync::OnceLock};

pub static LOG_LEVEL: OnceLock<Level> = OnceLock::new();

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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
#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {{
        if &$level <= $crate::LOG_LEVEL.get().unwrap_or(&$crate::Level::Info) {
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

// todo: implement clean verbose/short logging
