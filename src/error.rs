//! Unchained Error implementation

use reqwest;
use std::io;
use std::fmt;
use std::error::{self, Error as _};

#[derive(Debug)]
pub enum Error {
    ApiError,
    DateParse,
    Io(io::Error),
    Reqwest(reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::ApiError => "unexpected response from service",
            Error::DateParse => "can't parse date",
            Error::Reqwest(e) => e.description(),
            Error::Io(e) => e.description(),
        }
    }
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Reqwest(e) => Some(e),
            Error::Io(e) => Some(e),
            _ => None
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Reqwest(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}