//! Error definictions combining *Json*, *Hyper*, *IO* and *Api* Errors.

use json;
use hyper;
use std::io;

error_chain!{

    types {
        Error, ErrorKind, ChainErr, Result;
    }

    links {
    }

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
