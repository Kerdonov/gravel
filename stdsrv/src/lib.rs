//! A simple web server with 0 dependencies (other than Rust's stdlib).

#![feature(never_type)]
#![allow(dead_code)]

use std::{
    io::{BufReader, BufWriter},
    net::{Ipv4Addr, TcpListener},
    path::PathBuf,
};

use error::Error;
use fileserver::FileServer;
use request::HttpRequest;
use responder::Responder;
use slogger::{Level, log};

pub mod error;
mod fileserver;
mod http_header;
mod request;
mod responder;
mod response;

/// Opens a file server on a specified address and port which serves all files in dir.
///
/// # Errors
/// Errors that come up while serving files. Look at [`Error`].
///
/// # Panics
/// Never. Added to allow compiler to check for ! type.
pub fn serve(addr: Ipv4Addr, port: u16, dir: PathBuf) -> Result<(), Error> {
    /*
    let args: ProgramArgs = std::env::args().try_into()?;

    if args.generate {
        match generate(&args.indir, &args.outdir, args.force) {
            Ok(()) => log!(
                Level::Info,
                "HTML generation from `{}` to `{}` successful",
                args.indir.display(),
                args.outdir.display()
            ),
            Err(cracked_md::Error::OutDirIsNotEmpty) => {
                log!(
                    Level::Error,
                    "HTML generation failed, run `rm -r {}` and retry",
                    args.outdir.display()
                );
                process::exit(1);
            }
            Err(e) => {
                log!(Level::Error, "HTML generation failed with error: {}", e,);
                process::exit(1);
            }
        }
    }

    */
    let listener = TcpListener::bind((addr, port))?;
    log!(Level::Info, "Listening on addr `{}:{}`", addr, port);

    // todo: refactor this
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let outdir = dir.clone();
                std::thread::spawn(move || {
                    log!(Level::Debug, "TcpStream handler spawned");
                    let mut reader = BufReader::new(&stream);
                    let mut writer = BufWriter::new(&stream);
                    let server = match FileServer::new(outdir.as_path()) {
                        Ok(s) => s,
                        Err(_e) => return,
                    };

                    while let Ok(req) = HttpRequest::try_from(&mut reader) {
                        let _ = server
                            .respond(req)
                            .add_header("Server", "stdsrv")
                            .add_header("Connection", "keep-alive")
                            .send(&mut writer)
                            .map_err(|e| log!(Level::Error, "{}", e));
                    }
                    log!(Level::Debug, "TcpStream handler exited");
                });
            }
            Err(e) => log!(Level::Warn, "Connection failed: {}", e),
        }
    }
    Ok(())
}
