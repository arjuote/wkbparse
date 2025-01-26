//  FileName    : error.rs
//  Author      : ShuYu Wang <andelf@gmail.com>
//  Created     : Wed May 27 01:45:41 2015 by ShuYu Wang
//  Copyright   : Feather Workshop (c) 2015

use std;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Read(String),
    Write(String),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Read(_) => "error while reading",
            Error::Write(_) => "error while writing",
            Error::Other(_) => "unknown error",
        }
    }
}
