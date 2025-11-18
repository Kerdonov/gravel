use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Server(stdsrv::error::Error),
    MdParse(cracked_md::Error),
    CommandLineArgsParse(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Server(e) => e.fmt(f),
            Error::MdParse(e) => e.fmt(f),
            Error::CommandLineArgsParse(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<cracked_md::Error> for Error {
    fn from(value: cracked_md::Error) -> Self {
        Self::MdParse(value)
    }
}

impl From<stdsrv::error::Error> for Error {
    fn from(value: stdsrv::error::Error) -> Self {
        Self::Server(value)
    }
}
