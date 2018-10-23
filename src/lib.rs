//! An unofficial crate to query publicly accessible API methods for Dresden's public transport system.
//!
//! Currently the endpoints are supported:
//!
//! ## Station
//! `http://widgets.vvo-online.de/abfahrtsmonitor/Haltestelle.do`
//!
//! ```rust
//! # use dvb::prelude::*;
//! Station::new("Slub").results();
//! ```
//!
//! ## Monitor
//! `http://widgets.vvo-online.de/abfahrtsmonitor/Abfahrten.do`
//!
//! ```rust
//! # use dvb::prelude::*;
//! Monitor::new("HBF").departures_by_line();
//! ```
//!

extern crate reqwest;
extern crate regex;
extern crate chrono;
extern crate num_integer;
extern crate pretty_assertions;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

#[macro_use] extern crate error_chain;

mod common;
mod time;
pub mod error;

pub mod find;
pub mod monitor;
pub mod trip;

pub use time::DvbTime;
pub use common::Mot;