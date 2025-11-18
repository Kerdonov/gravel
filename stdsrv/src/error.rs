//! Custom Error type and Result enum and their most standard trait implementations.

use std::fmt::{Debug, Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ErrorKind {
    StreamReadFailed,
    CommandLineArgsParse,
    UnsupportedHttpMethod,
    UnsupportedHttpVersion,
    RequestParse,
    FileNotFound,
    DirNotFound,
    Io,
    TcpBind,
    NotImplemented,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Error {
    kind: ErrorKind,
    msg: String,
}

impl Error {
    #[must_use]
    pub fn new(kind: ErrorKind, msg: &str) -> Self {
        Self {
            kind,
            msg: msg.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.msg)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self {
            kind: ErrorKind::Io,
            msg: value.to_string(),
        }
    }
}
