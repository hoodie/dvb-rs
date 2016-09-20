//! An unofficial rust crates giving you a few options to query a collection of publicly accessible API methods for Dresden's public transport system.
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
//! Monitor::new("HBF").by_line();
//! ```
//!

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

pub use station::URL as STATION_URL;
pub use monitor::URL as MONITOR_URL;
