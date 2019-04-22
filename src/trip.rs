use serde_json;
use serde::{Serialize, Deserialize};

use crate::error::Result;
use crate::common::Status;

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

pub fn trip_details(config: &Config) -> Result<Trip> {
    const URL: &str = "https://webapi.vvo-online.de/dm/trip";

    let result = reqwest::Client::new()
        .post(URL)
        .json(&config)
        .send()?
        .json()?;

    Ok(result)
}