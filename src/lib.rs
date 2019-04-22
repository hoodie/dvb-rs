//! An unofficial crate to query publicly accessible API methods for Dresden's public transport system.
//!
//! Currently the endpoints are supported:
//!
//! ## Station
//! `http://widgets.vvo-online.de/abfahrtsmonitor/Haltestelle.do`
//!

mod common;
mod time;
pub mod error;

pub mod easy;

pub mod find;
pub mod monitor;
#[doc(hidden)]
pub mod trip;

pub use crate::time::DvbTime;
pub use crate::common::Mot;
pub use crate::error::Result;