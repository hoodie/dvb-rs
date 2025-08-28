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
pub mod route;
pub mod trip;

pub use crate::{
    common::{DvbResponse, Mot},
    error::Result,
    time::DvbTime,
};

use crate::{
    monitor::DepartureMonitor,
    point::{Found, Params, point_finder},
};

/// Search for stops by name using the VVO PointFinder API.
///
/// This is a convenience wrapper for [`point::point_finder`] with `stops_only = true`.
/// For more advanced queries, use [`point::point_finder`] directly.
///
/// Endpoint: `https://webapi.vvo-online.de/pointfinder`
pub async fn find_stops(query: &str) -> Result<DvbResponse<Found>> {
    point_finder(&Params {
        query,
        stops_only: true,
        ..Default::default()
    })
    .await
}

/// Search for nearby and assigned stops using the VVO PointFinder API.
///
/// This is a convenience wrapper for [`point::point_finder`] with `stops_only = false` and `assigedstops = true`.
/// For more advanced queries, use [`point::point_finder`] directly.
///
/// Endpoint: `https://webapi.vvo-online.de/pointfinder`
pub async fn find_nearby_stops(query: &str) -> Result<DvbResponse<Found>> {
    point_finder(&Params {
        query,
        stops_only: false,
        assigedstops: true,
        ..Default::default()
    })
    .await
}

/// Search for points of interest (POIs) using the VVO PointFinder API.
///
/// This is a convenience wrapper for [`point::point_finder`] with `stops_only = false`.
/// For more advanced queries, use [`point::point_finder`] directly.
///
/// Endpoint: `https://webapi.vvo-online.de/pointfinder`
pub async fn find_pois(query: &str) -> Result<DvbResponse<Found>> {
    point_finder(&Params {
        query,
        stops_only: false,
        ..Default::default()
    })
    .await
}

/// Get upcoming departures for a stop using the VVO Departure Monitor API.
///
/// This is a convenience wrapper for [`monitor::departure_monitor`] with default parameters.
/// For more advanced queries (e.g., filtering by mode of transport, custom limits, etc.), use [`monitor::departure_monitor`] directly.
///
/// Endpoint: `https://webapi.vvo-online.de/dm`
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
