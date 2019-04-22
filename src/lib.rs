//! An unofficial crate to query publicly accessible API methods for Dresden's public transport system.
//!
//! Currently the endpoints are supported:
//!
//! ## Station
//! `http://widgets.vvo-online.de/abfahrtsmonitor/Haltestelle.do`
//!

extern crate reqwest;
extern crate regex;
extern crate chrono;
extern crate num_integer;
extern crate pretty_assertions;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

mod common;
mod time;
pub mod error;

pub mod find;
pub mod monitor;
pub mod trip;

pub use time::DvbTime;
pub use common::Mot;