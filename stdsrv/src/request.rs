//! An abstraction layer for parsing HTTP requests. As simple and high level as I managed.

use crate::error::{Error, ErrorKind, Result};
use crate::http_header::HttpHeaders;
use crate::log;
use crate::logger::Level;
use std::fmt::Display;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::path::PathBuf;

/// Supports just GET methods for now.
#[derive(Debug, PartialEq)]
#[allow(dead_code, clippy::upper_case_acronyms)]
pub enum HttpMethod {
    GET,
    //PUT,
    //POST,
}

impl TryFrom<&str> for HttpMethod {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        match value {
            "GET" => Ok(Self::GET),
            //"PUT" => Ok(Self::PUT),
            //"POST" => Ok(Self::POST),
            _ => Err(Error::new(ErrorKind::UnsupportedHttpMethod, value)),
        }
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Type for HTTP requests.
#[derive(Debug)]
#[allow(dead_code)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: PathBuf,
    pub version: String,
    pub headers: HttpHeaders,
}

impl Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}\n{}",
            self.method,
            self.path.display(),
            self.version,
            self.headers
        )
    }
}

impl TryFrom<&mut BufReader<&TcpStream>> for HttpRequest {
    type Error = Error;
    fn try_from(value: &mut BufReader<&TcpStream>) -> std::result::Result<Self, Self::Error> {
        let mut request_line = String::new();

        value
            .read_line(&mut request_line)
            .map_err(|_e| Error::new(ErrorKind::RequestParse, "expected request line"))?;
        let mut parts = request_line.split_whitespace();
        let method: HttpMethod = parts
            .next()
            .ok_or(Error::new(
                ErrorKind::RequestParse,
                "expected request method",
            ))?
            .try_into()?;
        let path_with_prefix: PathBuf = parts
            .next()
            .ok_or(Error::new(ErrorKind::RequestParse, "expected request path"))?
            .into();
        let path = if path_with_prefix.starts_with("/") {
            path_with_prefix.strip_prefix("/").unwrap().into()
        } else {
            path_with_prefix
        };

        let version = parts
            .next()
            .ok_or(Error::new(
                ErrorKind::RequestParse,
                "expected request version",
            ))?
            .into();

        let mut req = Self {
            method,
            path,
            version,
            headers: HttpHeaders::new(),
        };

        loop {
            let mut line = String::new();
            value.read_line(&mut line)?;
            if line == "\r\n" || line.is_empty() {
                break;
            }
            if let Some((header, val)) = line.split_once(": ") {
                req.headers.add(header, val);
            }
        }

        Ok(req)
    }
}

impl TryFrom<&str> for HttpRequest {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self> {
        let mut lines = s.split("\r\n");

        let request_line = lines
            .next()
            .ok_or(Error::new(ErrorKind::RequestParse, "expected request line"))?;
        let mut parts = request_line.split_whitespace();

        let method = parts
            .next()
            .ok_or(Error::new(
                ErrorKind::RequestParse,
                "expected request method",
            ))?
            .try_into()?;
        let path_with_prefix: PathBuf = parts
            .next()
            .ok_or(Error::new(ErrorKind::RequestParse, "expected request path"))?
            .into();
        let path = if path_with_prefix.starts_with("/") {
            path_with_prefix.strip_prefix("/").unwrap().into()
        } else {
            path_with_prefix
        };

        let version = parts
            .next()
            .ok_or(Error::new(
                ErrorKind::RequestParse,
                "expected request version",
            ))?
            .into();

        let mut headers = HttpHeaders::new();

        for line in lines {
            if let Some(v) = line.split_once(": ") {
                headers.add(v.0, v.1)
            }
        }

        let req = Self {
            method,
            path,
            version,
            headers,
        };

        log!(Level::Info, "\n{}", req);

        Ok(req)
    }
}

// ------------------------------
// TESTS
// ------------------------------

#[cfg(test)]
mod request_test {
    use super::*;

    #[test]
    fn http_parse_method_get() {
        let s = "GET / HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/8.14.1\r\nAccept: */*\r\n\r\n";
        let req: HttpRequest = s.try_into().unwrap();

        assert_eq!(req.method, HttpMethod::GET);
    }

    #[test]
    fn http_parse_path_indexhtml() {
        let s = "GET /index.html HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/8.14.1\r\nAccept: */*\r\n\r\n";
        let req: HttpRequest = s.try_into().unwrap();

        assert_eq!(req.path, PathBuf::from("index.html"));
    }

    #[test]
    fn http_parse_version() {
        let s = "GET / HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/8.14.1\r\nAccept: */*\r\n\r\n";
        let req: HttpRequest = s.try_into().unwrap();

        assert_eq!(req.version, "HTTP/1.1");
    }

    #[test]
    fn http_parse_headers_len() {
        let s = "GET / HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/8.14.1\r\nAccept: */*\r\n\r\n";
        let req: HttpRequest = s.try_into().unwrap();

        assert_eq!(req.headers.len(), 3);
    }

    #[test]
    fn http_parse_useragent_header_curl() {
        let s = "GET / HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/8.14.1\r\nAccept: */*\r\n\r\n";
        let req: HttpRequest = s.try_into().unwrap();

        assert_eq!(req.headers.get("User-Agent").unwrap(), "curl/8.14.1");
    }

    #[test]
    fn http_parse_empty_should_fail() {
        let s = "";
        let req: Result<HttpRequest> = s.try_into();

        assert!(req.is_err());
    }

    #[test]
    fn http_parse_unsupported_method_delete() {
        let s = "DELETE / HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/8.14.1\r\nAccept: */*\r\n\r\n";
        let req: Result<HttpRequest> = s.try_into();

        assert!(req.is_err());
    }

    #[test]
    fn http_method_display() {
        let method = HttpMethod::GET;
        assert_eq!(method.to_string(), "GET")
    }
}
