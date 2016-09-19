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
extern crate multimap;


pub mod error;
mod util;
mod api;
mod station;
mod monitor;

pub mod prelude;

pub use station::Station;
pub use monitor::Monitor;
