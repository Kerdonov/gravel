//! Simple and program specific command line argument parsing solution.

// todo: refactor to <command> <subcommand> [<options>]

use slogger::{LOG_LEVEL, Level};

use crate::error::Error;
use std::net::Ipv4Addr;
use std::path::PathBuf;

pub struct ProgramArgs {
    pub outdir: PathBuf,
    pub indir: PathBuf,
    pub generate: bool,
    pub force: bool,
    pub addr: Ipv4Addr,
    pub port: u16,
    pub verbose: bool,
}

impl Default for ProgramArgs {
    fn default() -> Self {
        Self {
            indir: PathBuf::from("./web"),
            outdir: PathBuf::from("./html"),
            generate: false,
            force: false,
            addr: Ipv4Addr::UNSPECIFIED,
            port: 8080,
            verbose: false,
        }
    }
}

impl TryFrom<std::env::Args> for ProgramArgs {
    type Error = crate::error::Error;
    fn try_from(mut value: std::env::Args) -> Result<Self, Self::Error> {
        let mut a = Self::default();
        let _ = value.next(); // ignore executable path
        while let Some(v) = value.next() {
            match v.as_str() {
                "-i" => {
                    a.indir = value
                        .next()
                        .ok_or(Error::CommandLineArgsParse(
                            "Expected input directory after option `-i`".to_string(),
                        ))?
                        .into();
                }
                "-a" => {
                    let addr_string = value.next().ok_or(Error::CommandLineArgsParse(
                        "Expected listener IPv4 address after option `-a`".to_string(),
                    ))?;
                    a.addr = Ipv4Addr::parse_ascii(addr_string.as_bytes()).map_err(|_e| {
                        Error::CommandLineArgsParse(
                            "Invalid IPv4 address after option `-a`".to_string(),
                        )
                    })?;
                }
                "-p" => {
                    let port_string = value.next().ok_or(Error::CommandLineArgsParse(
                        "Expected listener port after option `-p`".to_string(),
                    ))?;
                    a.port = port_string.parse().map_err(|_e| {
                        Error::CommandLineArgsParse(
                            "Invalid 16-bit port number after option `-p`".to_string(),
                        )
                    })?;
                }
                "-g" => a.generate = true,
                "-f" => a.force = true,
                "-v" => {
                    a.verbose = true;
                    LOG_LEVEL.get_or_init(|| Level::Debug);
                }
                _ => {
                    a.outdir = v.into();
                }
            }
        }
        LOG_LEVEL.get_or_init(|| Level::Info);
        Ok(a)
    }
}
