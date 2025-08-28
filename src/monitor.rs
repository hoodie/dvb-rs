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
    pub stopid: &'a str,
    pub limit: Option<u32>,
    pub time: Option<&'a str>,
    pub isarrival: Option<bool>,
    pub shorttermchanges: Option<bool>,
    pub mot: Option<&'a [Mot]>,
}

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
