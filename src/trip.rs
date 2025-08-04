use serde::{Deserialize, Serialize};

use crate::{DvbResponse, error::Result, time::DvbTime};

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Params<'a> {
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
    #[serde(default)]
    pub stops: Vec<Stop>,
}

const TRIP_URL: &str = "https://webapi.vvo-online.de/dm/trip";

pub async fn trip_details<'a>(params: &Params<'a>) -> Result<DvbResponse<Trip>> {
    let result = reqwest::Client::new()
        .post(TRIP_URL)
        .json(&params)
        .send()
        .await?
        .json()
        .await?;

    Ok(result)
}
