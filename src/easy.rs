//! Easy versions of the APIp

use crate::{ monitor, find, error::Result };

pub fn find(query: &str) -> Result<find::Found> {
    let point = find::find_point(
        &find::Config {
            query,
            stops_only: Some(true),
            ..Default::default()
        })?;

    Ok(point)
}

pub fn monitor(stopid: &str) -> Result<monitor::DepartureMonitor> {
    let monitor = monitor::departure_monitor(
        monitor::Config {
            stopid,
            mot: None,
            limit: Some(15),
            ..Default::default()
        })?;

    Ok(monitor)
}
