//! Error definictions combining *Json*, *Hyper*, *IO* and *Api* Errors.
//! This is generated using [error-chain](https://crates.io/crates/error-chain).

use reqwest;
use std::io;

error_chain!{

    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links { }

    foreign_links {
        Reqwest(reqwest::Error);
        Io(io::Error);
    }

    errors {
        ApiError {
            description("unexpected response from service")
        }
        DateParse {
            description("can't parse date")
        }
    }
}
