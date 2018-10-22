use reqwest;
use error::Result;
use common::{ArrivalState, Mot, Status};

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
    id: String,
    line_name: String,
    direction: String,
    // platform
    mot: Mot,
    real_time: Option<String>,
    state: Option<ArrivalState>,
    route_changes: Option<Vec<String>>,
    // diva
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DepartureMonitor {
    name: String,
    status: Status,
    place: String,
    expiration_time: String,
    departures: Vec<Departure>
}

pub fn departure_monitor(config: Config) -> Result<DepartureMonitor> {
    const URL: &str = "https://webapi.vvo-online.de/dm";

    let result = reqwest::Client::new()
        .post(URL)
        .json(&config)
        .send()?
        .json()?;

    Ok(result)

}
