//!  An unofficial rust crates giving you a few options to query a collection of publicly accessible API methods for Dresden's public transport system.
//!
//! # Example
//!
//! ```rust
//! use dvb::prelude::*;
//! println!("{:#?}", Station::new("Slub").get().unwrap());
//! ```

extern crate hyper;
extern crate json;
#[macro_use]
extern crate error_chain;


mod error;
mod util;
mod api;
mod station;
pub mod prelude;

pub use station::Station;
