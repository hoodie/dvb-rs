use serde::{Deserialize, Serialize};

use crate::common::{ArrivalState, Mot, Status};
use crate::error::Result;
use crate::time::DvbTime;

#[derive(Serialize, Debug, Default)]
pub struct Config<'a> {
    pub stopid: &'a str,
    pub limit: Option<u32>,
    pub time: Option<&'a str>,
    pub isarrival: Option<bool>,
    pub shorttermchanges: Option<bool>,
    pub mot: Option<&'a [Mot]>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DepartureMonitor {
    pub name: String,
    pub status: Status,
    pub place: String,
    pub expiration_time: Option<String>,
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

pub fn departure_monitor(config: Config) -> Result<DepartureMonitor> {
    // pub fn departure_monitor(config: Config) -> Result<serde_json::Value> {
    const URL: &str = "https://webapi.vvo-online.de/dm";

    let result = reqwest::blocking::Client::new()
        .post(URL)
        .json(&config)
        .send()?
        .json()?;

    Ok(result)
}
