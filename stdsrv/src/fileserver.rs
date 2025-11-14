//! A simple server implementation that just responds with the contents of the file requested in
//! the provided directory.

use std::fs;
use std::path::PathBuf;

use crate::{
    error::{Error, ErrorKind, Result},
    request::{HttpMethod, HttpRequest},
    responder::Responder,
    response::{HttpResponse, HttpStatus},
};

pub struct FileServer {
    root: PathBuf,
}

impl FileServer {
    pub fn new(root: &PathBuf) -> Result<FileServer> {
        if !root.is_dir() {
            return Err(Error::new(
                ErrorKind::DirNotFound,
                &root.display().to_string(),
            ));
        }

        Ok(Self {
            root: root.to_owned(),
        })
    }

    pub fn get_contents(&self, path: PathBuf) -> Option<Vec<u8>> {
        let mut fullpath = self.root.as_path().join(path);
        // default to index.html
        if fullpath.is_dir() {
            fullpath.push("index.html");
        }
        fs::read(fullpath).ok()
    }
}

impl Responder for FileServer {
    fn respond(&self, req: HttpRequest) -> HttpResponse {
        if req.version != "HTTP/1.1" {
            return HttpResponse::new_empty(HttpStatus::HTTPVersionNotSupported);
        }

        if req.method != HttpMethod::GET {
            return HttpResponse::new_empty(HttpStatus::MethodNotAllowed);
        }

        let content_type = match req.path.extension() {
            Some(s) => match s.as_encoded_bytes() {
                b"html" | b"htm" => "text/html",
                b"css" => "text/css",
                b"js" => "text/javascript",
                b"pdf" => "application/pdf",
                b"json" => "application/json",
                b"xml" => "application/xml",
                b"gif" => "image/gif",
                b"jpeg" | b"jpg" => "image/jpg",
                b"png" => "image/png",
                _ => "text/plain",
            },
            None => "text/html",
        };

        if let Some(content) = self.get_contents(req.path) {
            HttpResponse::new(HttpStatus::Ok, content).add_header("Content-Type", content_type)
        } else {
            HttpResponse::new_empty(HttpStatus::NotFound).add_header("Connection", "close")
        }
    }
}
