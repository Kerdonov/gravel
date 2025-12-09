//! A simple web server with 0 dependencies (other than Rust's stdlib).
//! Documentation is a work in progress, go see my webpage at [jlux.dev](https://jlux.dev).

use std::{
    io::{BufReader, BufWriter},
    net::TcpListener,
    process,
};

use args::ProgramArgs;
use cracked_md::generate;
use fileserver::FileServer;
use logger::Level;
use request::HttpRequest;
use responder::Responder;

mod args;
mod error;
mod fileserver;
mod http_header;
//mod http_stream;
mod logger;
mod request;
mod responder;
mod response;

/// Entrypoint to the program.
fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let listener = TcpListener::bind(&args.addr)?;
    log!(Level::Info, "Listening on addr `{}`", &args.addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let outdir = args.outdir.clone();
                std::thread::spawn(move || {
                    log!(Level::Debug, "TcpStream handler spawned");
                    let mut reader = BufReader::new(&stream);
                    let mut writer = BufWriter::new(&stream);
                    let server = match FileServer::new(&outdir) {
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
