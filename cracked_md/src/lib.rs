//! A "Markdown" parser and HTML generator. Part of a static site generator `marksmith-rs`.
//! Not following any standards, only vibes.

#![deny(unused_imports)]
#![allow(clippy::needless_pass_by_value)]

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
mod parse_trait;
pub mod parser;
pub mod to_html;

#[derive(Debug)]
pub struct MdParseError {
    file: Option<PathBuf>,
    line: Option<usize>,
    //col: Option<usize>,
    expected: String,
    got: String,
}

impl MdParseError {
    pub fn new(expected: impl ToString, got: impl ToString) -> Self {
        Self {
            file: None,
            line: None,
            //col: None,
            expected: expected.to_string(),
            got: got.to_string(),
        }
    }

    pub fn from_line(line: usize, expected: impl ToString, got: impl ToString) -> Self {
        Self {
            file: None,
            line: Some(line),
            //col: None,
            expected: expected.to_string(),
            got: got.to_string(),
        }
    }

    /*
    pub fn from_col(col: usize, expected: impl ToString, got: impl ToString) -> Self {
        Self {
            file: None,
            line: None,
            col: Some(col),
            expected: expected.to_string(),
            got: got.to_string(),
        }
    }
    */

    #[must_use]
    pub fn set_line(self, line: usize) -> Self {
        Self {
            file: self.file,
            line: Some(line),
            //col: self.col,
            expected: self.expected,
            got: self.got,
        }
    }

    #[must_use]
    pub fn set_file(self, file: PathBuf) -> Self {
        Self {
            file: Some(file),
            line: self.line,
            //col: self.col,
            expected: self.expected,
            got: self.got,
        }
    }
}

impl Display for MdParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // no error message :/
        let file = self.file.clone().unwrap_or("<unknown>".into());
        write!(
            f,
            "Parse error in '{}' on line {}: expected '{}', got '{}'",
            file.display(),
            self.line.unwrap_or(0),
            //self.col.unwrap_or(0),
            self.expected,
            self.got
        )
    }
}

impl std::error::Error for MdParseError {}

#[derive(Debug)]
pub enum Error {
    InDirIsNotDir,
    OutDirIsNotEmpty,
    OutDirIsNotDir,
    OutDirFileDeleteNotAllowed,
    OutDirDirectoryInPlaceOfFile,
    FileRead,
    DirRead,
    FileWrite,
    FileCreate,
    DirCreate,
    Parse(MdParseError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl From<MdParseError> for Error {
    fn from(value: MdParseError) -> Self {
        Error::Parse(value)
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, crate::Error>;

/// Takes two directories and a force flag as parameters, generates html files to the outdir in the
/// same directory structure as the md files in indir.
///
/// # Errors
/// Anything wrong with reading files from the directories or parsing the files.
pub fn generate(indir: &PathBuf, outdir: &PathBuf, force: bool) -> Result<()> {
    if !indir.is_dir() {
        Err(Error::InDirIsNotDir)?;
    }
    if !outdir.is_dir() {
        Err(Error::OutDirIsNotDir)?;
    }
    let files = crawl_fs(indir);

    for path in files {
        let fullpath = indir.as_path().join(&path);

        // read and parse md file
        let content = fs::read_to_string(&fullpath).map_err(|_e| Error::FileRead)?;
        let html = parse(&content)?.to_html();

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
