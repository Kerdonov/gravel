//! Simple and program specific command line argument parsing solution.

// todo: refactor to <command> <subcommand> [<options>]

use slogger::{LOG_LEVEL, Level, log};

use crate::error::Error;
use std::env::Args;
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};

pub enum Command {
    Generate { force: bool },
    Serve { addr: Ipv4Addr, port: u16 },
    Init,
}

impl Default for Command {
    fn default() -> Self {
        Self::Generate { force: true }
    }
}

impl TryFrom<Args> for Command {
    type Error = Error;
    fn try_from(mut value: Args) -> Result<Self, Self::Error> {
        let mut comm = Command::default();
        let _ = value.next(); // ignore executable
        let command = value.next();

        // `gravel serve` command
        if let Some("serve") = command.as_deref() {
            let mut addr = Ipv4Addr::UNSPECIFIED;
            let mut port = 8080;
            while let Some(a) = value.next() {
                match a.as_str() {
                    "-a" => {
                        let address_str = value.next().ok_or(Error::CommandLineArgsParse(
                            "Missing argument after `-a`. Expected IPv4 address.".to_string(),
                        ))?;
                        addr = Ipv4Addr::parse_ascii(address_str.as_bytes()).map_err(|_e| {
                            Error::CommandLineArgsParse("Parsing IP address failed".to_string())
                        })?;
                    }
                    "-p" => {
                        let port_str = value.next().ok_or(Error::CommandLineArgsParse(
                            "Missing argument after `-p`. Expected TCP port number.".to_string(),
                        ))?;
                        port = port_str.parse().map_err(|_e| {
                            Error::CommandLineArgsParse("Parsing TCP port failed".to_string())
                        })?;
                    }
                    &_ => Err(Error::CommandLineArgsParse(format!(
                        "Unknown argument: `{a}`"
                    )))?,
                }
            }
            comm = Command::Serve { addr, port };
        }
        // `gravel init` command
        else if let Some("init") = command.as_deref() {
            if let Some(a) = value.next() {
                Err(Error::CommandLineArgsParse(format!(
                    "Unexpected argument: `{a}`"
                )))?;
            }
            comm = Command::Init;
        }
        // `gravel` command
        else if let Some(a) = value.next() {
            Err(Error::CommandLineArgsParse(format!(
                "Unexpected argument: `{a}`"
            )))?;
        }

        Ok(comm)
    }
}

#[allow(unused)]
pub struct ProgramConfig {
    pub outdir: PathBuf,
    pub indir: PathBuf,
    pub command: Command,
    pub verbose: bool,
}

impl Default for ProgramConfig {
    fn default() -> Self {
        Self {
            indir: PathBuf::from("./pebbles"),
            outdir: PathBuf::from("./site"),
            command: Command::default(),
            verbose: true,
        }
    }
}

impl ProgramConfig {
    pub fn new<P: AsRef<Path>>(_toml_file: P, args: Args) -> Result<Self, Error> {
        let conf = Self {
            command: args.try_into()?,
            ..Default::default()
        };
        LOG_LEVEL.get_or_init(|| Level::Debug);
        log!(Level::Warn, "TOML parsing not implemented, skipping");
        Ok(conf)
    }
}

/*
impl TryFrom<std::env::Args> for ProgramConfig {
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
*/
