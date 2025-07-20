use std::{fmt::Debug, time::Duration};

use serde::{Deserialize, Serialize};

use crate::{DvbResponse, Mot, error::Result};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Mode {
    title: String,
    name: String,
    icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Diva {
    pub network: String,
    pub number: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Lines {
    pub lines: Vec<Line>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Direction {
    name: String,
    time_tables: Vec<TimeTable>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TimeTable {
    id: String,
    name: String,
}

pub fn lines(stop_id: &str, timeout: Option<u64>) -> Result<DvbResponse<Lines>> {
    let response: DvbResponse<Lines> = reqwest::blocking::Client::new()
        .get("https://webapi.vvo-online.de/stt/lines")
        .query(&[("format", "json"), ("stopid", stop_id)])
        .timeout(Duration::from_millis(timeout.unwrap_or(15000)))
        .send()?
        .json()?;

    Ok(response)
}
