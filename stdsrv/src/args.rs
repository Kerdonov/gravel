//! Simple and program specific command line argument parsing solution.

use crate::error::Error;
use crate::error::ErrorKind;
use std::path::PathBuf;
use std::sync::OnceLock;

pub static VERBOSE: OnceLock<bool> = OnceLock::new();

pub struct ProgramArgs {
    pub outdir: PathBuf,
    pub indir: PathBuf,
    pub generate: bool,
    pub force: bool,
    pub addr: String,
    pub verbose: bool,
}

impl Default for ProgramArgs {
    fn default() -> Self {
        Self {
            indir: PathBuf::from("./web"),
            outdir: PathBuf::from("./html"),
            generate: false,
            force: false,
            addr: "0.0.0.0:8080".to_string(),
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
                        .ok_or(Error::new(
                            ErrorKind::CommandLineArgsParse,
                            "Expected input directory after option `-i`",
                        ))?
                        .into();
                }
                "-a" => {
                    a.addr = value.next().ok_or(Error::new(
                        ErrorKind::CommandLineArgsParse,
                        "Expected listener address after option `-a`",
                    ))?;
                }
                "-g" => a.generate = true,
                "-f" => a.force = true,
                "-v" => {
                    a.verbose = true;
                    VERBOSE.get_or_init(|| true);
                }
                _ => {
                    a.outdir = v.into();
                }
            }
        }
        VERBOSE.get_or_init(|| false);
        Ok(a)
    }
}
