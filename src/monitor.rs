use reqwest;

use error::Result;
use common::{ArrivalState, Mot, Status};
use time::DvbTime;

#[derive(Serialize, Debug, Default)]
pub struct Config<'a> {
    pub stopid: &'a str,
    pub limit: Option<u32>,
    pub time: Option<&'a str>,
    pub isarrival: Option<bool>,
    pub shorttermchanges: Option<bool>,
    pub mot: Option<&'a [Mot]>
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
    pub expiration_time: String,
    pub departures: Vec<Departure>
}

pub fn departure_monitor(config: Config) -> Result<DepartureMonitor> {
// pub fn departure_monitor(config: Config) -> Result<Value> {
    const URL: &str = "https://webapi.vvo-online.de/dm";

    let result = reqwest::Client::new()
        .post(URL)
        .json(&config)
        .send()?
        .json()?;

    Ok(result)
}
