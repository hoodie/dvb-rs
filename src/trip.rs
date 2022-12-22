//! unfinished

use serde::{Deserialize, Serialize};

use crate::common::Status;
use crate::error::Result;
use crate::time::DvbTime;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config<'a> {
    pub tripid: &'a str,
    pub time: DvbTime,
    pub stopid: &'a str,
    pub mapdata: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Platform {
    name: String,
    r#type: String, // enum PlatformType {Platform}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum Position {
    Previous,
    Current,
    Next,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Stop {
    id: String,
    name: String,
    place: String,
    platform: Platform,
    latitude: i64,
    longitude: i64,
    position: Position,
    scheduled_time: bool,
    time: DvbTime,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Trip {
    // stops: Option<Vec<serde_json::Value>>,
    stops: Option<Vec<Stop>>,
    status: Status,
}

pub fn trip_details(config: &Config) -> Result<Trip> {
    const URL: &str = "https://webapi.vvo-online.de/dm/trip";

    let result = reqwest::blocking::Client::new()
        .post(URL)
        .json(&config)
        .send()?
        .json()?;

    Ok(result)
}
