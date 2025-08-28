//! Departure monitor API types and functions.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    DvbResponse,
    common::{ArrivalState, Mot},
    error::Result,
    time::DvbTime,
};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Departure {
    pub id: String,
    pub line_name: String,
    pub direction: String,
    // platform
    pub mot: Mot,
    pub real_time: Option<DvbTime>,
    pub state: Option<ArrivalState>,
    pub route_changes: Option<Vec<String>>,
    // diva
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DepartureMonitor {
    pub name: Option<String>,
    pub place: Option<String>,
    pub departures: Option<Vec<Departure>>,
}

impl DepartureMonitor {
    pub fn next_line<'a>(&'a self, line_name: &str) -> Option<&'a Departure> {
        if let Some(ref deps) = self.departures {
            deps.iter().find(|dep| dep.line_name == line_name)
        } else {
            None
        }
    }
}

const MONITOR_URL: &str = "https://webapi.vvo-online.de/dm";

#[derive(Serialize, Debug, Default)]
pub struct Params<'a> {
    /// The stop ID to monitor.
    pub stopid: &'a str,
    /// Maximum number of departures to return.
    pub limit: Option<u32>,
    /// Time for the departure query.
    pub time: Option<&'a str>,
    /// If true, time is interpreted as arrival time.
    pub isarrival: Option<bool>,
    /// Include short-term changes.
    pub shorttermchanges: Option<bool>,
    /// Filter by mode of transport.
    pub mot: Option<&'a [Mot]>,
}

/// Fetches upcoming departures from a specified stop using the VVO WebAPI.
///
/// Endpoint: `https://webapi.vvo-online.de/dm`
pub async fn departure_monitor<'a>(params: Params<'a>) -> Result<DvbResponse<DepartureMonitor>> {
    let result = reqwest::Client::new()
        .post(MONITOR_URL)
        .json(&params)
        .send()
        .await?
        .json()
        .await?;

    Ok(result)
}
