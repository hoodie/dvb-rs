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

pub use crate::{
    common::{DvbResponse, Mot},
    error::Result,
    time::DvbTime,
};

use crate::{
    monitor::DepartureMonitor,
    point::{Config, Found, point_finder},
};

pub async fn find_stops(query: &str) -> Result<DvbResponse<Found>> {
    point_finder(&Config {
        query,
        stops_only: true,
        ..Default::default()
    })
    .await
}

pub async fn find_nearby_stops(query: &str) -> Result<DvbResponse<Found>> {
    point_finder(&Config {
        query,
        stops_only: false,
        assigedstops: true,
        ..Default::default()
    })
    .await
}

pub async fn find_pois(query: &str) -> Result<DvbResponse<Found>> {
    point_finder(&Config {
        query,
        stops_only: false,
        ..Default::default()
    })
    .await
}

pub async fn monitor_departures(stopid: &str) -> Result<DvbResponse<DepartureMonitor>> {
    let monitor = monitor::departure_monitor(monitor::Params {
        stopid,
        mot: None,
        limit: Some(15),
        ..Default::default()
    })
    .await?;

    Ok(monitor)
}
