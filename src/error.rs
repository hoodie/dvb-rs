//! Unchained Error implementation

use reqwest;
use std::{fmt, io};

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
        match self {
            Error::ApiError => write!(f, "unexpected response from service"),
            Error::DateParse => write!(f, "can't parse date"),
            Error::Reqwest(ref e) => write!(f, "{e}"),
            Error::Io(ref e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Reqwest(e) => Some(e),
            Error::Io(e) => Some(e),
            _ => None,
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
