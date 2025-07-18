//! unfinished

use serde::{Deserialize, Serialize};

use crate::{common::Status, error::Result, time::DvbTime};

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
    pub id: String,
    pub name: String,
    pub place: String,
    pub platform: Platform,
    pub latitude: i64,
    pub longitude: i64,
    pub position: Position,
    pub scheduled_time: Option<bool>,
    pub time: DvbTime,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Trip {
    // stops: Option<Vec<serde_json::Value>>,
    pub stops: Option<Vec<Stop>>,
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
