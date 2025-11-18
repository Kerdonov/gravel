//! An abstraction layer for building and sending HTTP responses.

use crate::error::Result;
use crate::http_header::HttpHeaders;
use slogger::{Level, log};
use std::{fmt::Display, io::Write};

/// Macro for generating Http status codes (AI generated).
macro_rules! http_statuses {
    ($($name:ident => ($code:expr, $reason:expr)),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[allow(unused_attributes, dead_code)]
        pub enum HttpStatus {
            $($name),+
        }

        impl HttpStatus {
            pub fn code(&self) -> u16 {
                match self {
                    $(HttpStatus::$name => $code,)+
                }
            }

            pub fn reason(&self) -> &'static str {
                match self {
                    $(HttpStatus::$name => $reason,)+
                }
            }
        }
    };
}

http_statuses!(
    Ok => (200, "OK"),

    NotFound => (404, "Not Found"),
    MethodNotAllowed => (405, "Method Not Allowed"),

    ImATeapot => (418, "I'm a teapot"),

    InternalServerError => (500, "Internal Server Error"),
    HTTPVersionNotSupported => (505, "HTTP Version Not Supported"),
);

impl Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.code(), self.reason())
    }
}

/// HTTP response structure
#[derive(Debug)]
pub struct HttpResponse {
    version: String,
    status: HttpStatus,
    headers: HttpHeaders,
    body: Option<Vec<u8>>,
}

/*
impl Size for HttpResponse {
    fn size(&self) -> usize {
        self.as_bytes().len()
    }
}
*/

impl HttpResponse {
    pub fn new_empty(status: HttpStatus) -> Self {
        let mut headers = HttpHeaders::new();
        headers.add("Content-Length", "0");
        Self {
            version: "HTTP/1.1".into(),
            status,
            headers,
            body: None,
        }
    }

    pub fn new(status: HttpStatus, body: Vec<u8>) -> Self {
        let mut headers = HttpHeaders::new();
        headers.add("Content-Length", &body.len().to_string());
        Self {
            version: "HTTP/1.1".into(),
            status,
            headers,
            body: Some(body),
        }
    }

    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        self.headers.add(key, value);
        self
    }

    fn add_header_inner(&mut self, key: &str, value: &str) {
        self.headers.add(key, value);
    }

    /// sending images "cuts off"
    pub fn send(&mut self, stream: &mut impl Write) -> Result<()> {
        let body_len = match &self.body {
            Some(v) => format!("{}", v.len()),
            None => "0".to_string(),
        };
        self.add_header_inner("Content-Length", &body_len);
        stream.write_all(self.start_bytes())?;
        if let Some(b) = &self.body {
            stream.write_all(b)?;
        }
        //stream.write_all(b"\r\n")?;
        stream.flush()?;
        //sleep(Duration::from_millis(100));
        /*
        stream.shutdown(std::net::Shutdown::Write)?;

        // hack?
        let _ = std::io::Read::read(stream, &mut [0u8; 1]);
        */

        // todo better verbose tracking
        log!(Level::Info, "{} {}", self.version, self.status);
        log!(Level::Debug, "\n{}", &self);
        Ok(())
    }

    fn start_bytes(&self) -> &[u8] {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(self.version.as_bytes());
        bytes.extend_from_slice(b" ");
        bytes.extend_from_slice(self.status.to_string().as_bytes());
        bytes.extend_from_slice(b"\r\n");
        bytes.extend_from_slice(self.headers.to_string().as_bytes());
        bytes.extend_from_slice(b"\r\n");
        bytes.leak()
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}\r\n{}\r\n",
            self.version, self.status, self.headers,
        )?;
        if let Some(s) = &self.body {
            write!(
                f,
                "{}\r\n",
                String::from_utf8(s.clone()).unwrap_or("<binary data>".to_string())
            )?;
        }
        Ok(())
    }
}

// --------------------
// TESTS
// --------------------
#[cfg(test)]
mod response_test {
    use super::*;

    #[test]
    fn http_status_macro_display() {
        let stat = HttpStatus::ImATeapot;

        assert_eq!(stat.to_string(), "418 I'm a teapot");
    }

    #[test]
    fn http_response_new_empty() {
        let resp = HttpResponse::new_empty(HttpStatus::ImATeapot);

        assert_eq!(
            resp.to_string(),
            "HTTP/1.1 418 I'm a teapot\r\nContent-Length: 0\r\n\r\n"
        );
    }

    #[test]
    fn http_response_new_with_body() {
        let resp = HttpResponse::new(HttpStatus::ImATeapot, b"teapot".into());

        assert_eq!(
            resp.to_string(),
            "HTTP/1.1 418 I'm a teapot\r\nContent-Length: 6\r\n\r\nteapot\r\n"
        );
    }
}
