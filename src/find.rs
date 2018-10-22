use reqwest;
use error::Result;

use common::Status;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config<'a> {
    pub query:  &'a str,
    pub limit: Option<u32>,
    pub stops_only: Option<bool>,
    pub assigedstops: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Point {
    point_status: String,
    status: Status,
    points: Vec<String>,
    expiration_time: String,
}

pub fn find_point(config: &Config) -> Result<Point> {
    const URL: &str = "https://webapi.vvo-online.de/tr/pointfinder";

    let result = reqwest::Client::new().post(URL).json(&config).send()?.json()?;

    Ok(result)

}
