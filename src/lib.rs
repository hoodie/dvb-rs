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

extern crate hyper;
extern crate json;
extern crate regex;
extern crate multimap;

#[macro_use] extern crate error_chain;
#[macro_use] extern crate lazy_static;

pub mod error;
mod util;
mod api;
mod station;
mod monitor;
mod line;

pub mod prelude;

pub use station::Station;
pub use monitor::Monitor;

pub use station::URL as STATION_URL;
pub use monitor::URL as MONITOR_URL;
