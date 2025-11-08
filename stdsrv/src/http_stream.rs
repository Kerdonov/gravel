use crate::error::ErrorKind;
use crate::log;
use crate::logger::Level;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

use crate::{error::Error, request::HttpRequest};

pub struct HttpStream {
    tcp_listener: TcpListener,
}

impl HttpStream {
    pub fn new(addr: &str) -> Self {
        let tcp_listener = TcpListener::bind(addr)
            .unwrap_or_else(|e| panic!("Failed to bind on address `{}`: {}", addr, e));
        log!(Level::Info, "Listening on `{}`", addr);

        Self { tcp_listener }
    }
}

impl Iterator for HttpStream {
    type Item = (HttpRequest, TcpStream);

    fn next(&mut self) -> Option<Self::Item> {
        // safe to unwrap, because Incoming never returns None
        let mut stream = self.tcp_listener.incoming().next().unwrap().ok()?;

        let mut buf = [0; 1024];
        let _read_bytes = stream
            .read(&mut buf)
            .or(Err(Error::new(
                ErrorKind::StreamReadFailed,
                "Reading from TCP stream failed",
            )))
            .ok()?;
        Some((
            String::from_utf8_lossy(&buf[..]).trim().try_into().ok()?,
            stream,
        ))
    }
}
