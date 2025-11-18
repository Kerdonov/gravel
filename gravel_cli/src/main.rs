#![feature(addr_parse_ascii, never_type)]

use args::ProgramArgs;
use cracked_md::generate;
use error::Error;
//use slogger::{LOG_LEVEL, Level};
use stdsrv::serve;

mod args;
mod error;

fn main() -> Result<!, Error> {
    let args = ProgramArgs::try_from(std::env::args())?;

    generate(&args.indir, &args.outdir, args.force)?;
    serve(args.addr, args.port, args.outdir)?;
}
