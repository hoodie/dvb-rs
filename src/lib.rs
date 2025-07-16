//! An unofficial crate to query publicly accessible API methods for Dresden's public transport system.
//!
//! Currently the endpoints are supported:
//!
//! ## Station
//! `http://widgets.vvo-online.de/abfahrtsmonitor/Haltestelle.do`
//!

mod common;
pub mod error;
mod time;

pub mod easy;

pub mod find;
pub mod monitor;
pub mod trip;

pub use crate::common::Mot;
pub use crate::error::Result;
pub use crate::time::DvbTime;
