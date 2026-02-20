//! Types and API for querying available lines (trams, buses, etc.) at a stop.

use std::{fmt::Debug, time::Duration};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{DvbResponse, Mot, error::Result};

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Mode {
    title: String,
    name: String,
    icon_url: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Diva {
    pub network: String,
    pub number: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Lines {
    pub lines: Vec<Line>,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Line {
    pub name: String,
    pub mot: Mot,
    pub mode: Option<Mode>,
    #[serde(default)]
    pub changes: Vec<String>,
    pub diva: Diva,
    pub directions: Vec<Direction>,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Direction {
    name: String,
    time_tables: Vec<TimeTable>,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TimeTable {
    id: String,
    name: String,
}

const LINES_URL: &str = "https://webapi.vvo-online.de/stt/lines";

/// Fetches all lines (trams, buses, etc.) departing from the specified stop ID using the VVO WebAPI.
///
/// # Arguments
/// * `stop_id` - The ID of the stop for which to fetch lines.
/// * `timeout` - Optional timeout in milliseconds for the request (defaults to 15000ms).
///
/// # Returns
/// * `Result<DvbResponse<Lines>>` - The parsed response containing available lines.
///
/// Endpoint: `https://webapi.vvo-online.de/stt/lines`
pub async fn lines(stop_id: &str, timeout: Option<u64>) -> Result<DvbResponse<Lines>> {
    let response: DvbResponse<Lines> = reqwest::Client::new()
        .post(LINES_URL)
        .json(&serde_json::json!({ "stopid": stop_id, "format": "json" }))
        .timeout(Duration::from_millis(timeout.unwrap_or(15000)))
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
