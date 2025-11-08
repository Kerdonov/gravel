#![deny(dead_code, unused_imports)]

use fstools::crawl_fs;
use parser::parse;
use std::{
    fmt::Display,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
use to_html::ToHtml;

pub mod ast;
pub mod parser;
pub mod to_html;

#[derive(Debug)]
pub enum Error {
    OutDirIsNotEmpty,
    OutDirFileDeleteNotAllowed,
    OutDirDirectoryInPlaceOfFile,
    FileRead,
    DirRead,
    FileWrite,
    FileCreate,
    DirCreate,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, crate::Error>;

pub fn generate(indir: &PathBuf, outdir: &PathBuf, force: bool) -> Result<()> {
    let files = crawl_fs(indir);

    for path in files {
        let fullpath = indir.as_path().join(&path);

        // read and parse md file
        let content = fs::read_to_string(&fullpath).map_err(|_e| Error::FileRead)?;
        let html = parse(&content).to_html();

        // write html data to file
        let mut newpath = outdir.to_owned();
        newpath.push(path);
        newpath.set_extension("html");

        // check if path exists
        if newpath.exists() {
            // remove if is file and if force, otherwise error
            if newpath.is_file() && force {
                fs::remove_file(&newpath).map_err(|_e| Error::OutDirFileDeleteNotAllowed)?;
            } else {
                Err(Error::OutDirDirectoryInPlaceOfFile)?;
            }
        }

        //println!("About to write file '{}'", newpath.display());

        let parent = newpath.parent().ok_or(Error::DirCreate)?;
        fs::create_dir_all(parent).map_err(|_e| Error::DirCreate)?;
        let mut newfile = File::create_new(newpath).map_err(|_e| Error::FileCreate)?;

        newfile
            .write(html.as_bytes())
            .map_err(|_e| Error::FileWrite)?;
    }

    Ok(())
}
