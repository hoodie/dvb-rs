//! Error definictions combining *Json*, *Hyper*, *IO* and *Api* Errors.
//! This is generated using [error-chain](https://crates.io/crates/error-chain).

use json;
use hyper;
use std::io;

error_chain!{

    types {
        Error, ErrorKind, Result;
    }

    links { }

    foreign_links {
        json::Error, Json;
        hyper::Error, Hyper;
        io::Error, Io;
    }

    errors {
        ApiError{
            description("unexpected response from service")
        }
    }
}
