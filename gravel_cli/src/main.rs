#![feature(addr_parse_ascii, never_type)]

use std::process;

use config::{Command, ProgramConfig};
use cracked_md::generate;
use error::Error;
use slogger::{Level, log};
use stdsrv::serve;

mod config;
mod error;

fn run() -> Result<(), Error> {
    let conf = ProgramConfig::new("gravel.toml", std::env::args())?;

    match conf.command {
        Command::Init => todo!("project init not implemented"),
        Command::Serve { addr, port } => serve(addr, port, conf.outdir)?,
        Command::Generate { force } => generate(&conf.indir, &conf.outdir, force)?,
    }
    Ok(())
}

fn main() {
    let _ = run().map_err(|e| {
        log!(Level::Error, "{}", e);
        process::exit(1);
    });
}
