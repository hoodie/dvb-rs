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

pub mod lines;
pub mod monitor;
pub mod poi;
pub mod point;
pub mod trip;

pub use crate::{common::Mot, error::Result, time::DvbTime};

pub fn find_stops(query: &str) -> Result<point::Found> {
    point::point_finder(&point::Config {
        query,
        stops_only: true,
        ..Default::default()
    })
}

pub fn find_nearby_stops(query: &str) -> Result<point::Found> {
    point::point_finder(&point::Config {
        query,
        stops_only: false,
        assigedstops: true,
        ..Default::default()
    })
}

pub fn find_pois(query: &str) -> Result<point::Found> {
    point::point_finder(&point::Config {
        query,
        stops_only: false,
        ..Default::default()
    })
}

pub fn monitor_departures(stopid: &str) -> Result<monitor::DepartureMonitor> {
    let monitor = monitor::departure_monitor(monitor::Config {
        stopid,
        mot: None,
        limit: Some(15),
        ..Default::default()
    })?;

    Ok(monitor)
}
