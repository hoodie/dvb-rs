use reqwest;
use serde_json;
use error::Result;

use common::Status;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config<'a> {
    pub tripid:  &'a str,
    pub time: &'a str,
    pub stopid:  &'a str,
    pub mapdata: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Trip {
    stops: Option<Vec<serde_json::Value>>,
    status: Status,
}

pub fn trip_dtails(config: &Config) -> Result<Trip> {
    const URL: &str = "https://webapi.vvo-online.de/dm/trip";

    let result = reqwest::Client::new()
        .post(URL)
        .json(&config)
        .send()?
        .json()?;

    Ok(result)
}