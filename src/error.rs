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

    }
}
