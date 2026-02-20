//! Route changes and disruption information from the VVO WebAPI.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{DvbResponse, common::Mot, error::Result, time::DvbTime};

const ROUTE_CHANGES_URL: &str = "https://webapi.vvo-online.de/rc";
const ROUTE_CHANGE_LINES_URL: &str = "https://webapi.vvo-online.de/rc/lines";

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ValidityPeriod {
    pub begin: Option<DvbTime>,
    pub end: Option<DvbTime>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Change {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub r#type: Option<String>,
    pub trip_request_include: Option<bool>,
    pub publish_date: Option<DvbTime>,
    #[serde(default)]
    pub line_ids: Vec<String>,
    #[serde(default)]
    pub validity_periods: Vec<ValidityPeriod>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Banner {
    pub title: Option<String>,
    pub description: Option<String>,
    pub r#type: Option<String>,
    pub modified_time: Option<DvbTime>,
    pub trip_request_include: Option<bool>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Diva {
    pub number: Option<String>,
    pub network: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Line {
    pub id: Option<String>,
    pub name: Option<String>,
    pub mot: Option<Mot>,
    pub transportation_company: Option<String>,
    #[serde(default)]
    pub divas: Vec<Diva>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RouteChanges {
    #[serde(default)]
    pub changes: Vec<Change>,
    #[serde(default)]
    pub banners: Vec<Banner>,
    #[serde(default)]
    pub lines: Vec<Line>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RouteChangeLines {
    #[serde(default)]
    pub lines: Vec<Line>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct Params<'a> {
    /// Include short-term changes.
    pub shortterm: Option<bool>,
    /// Provider filter.
    pub provider: Option<&'a str>,
    /// Response format.
    pub format: Option<&'a str>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct LinesParams<'a> {
    /// Provider filter.
    pub provider: Option<&'a str>,
    /// Response format.
    pub format: Option<&'a str>,
}

/// Fetches current route changes and disruptions from the VVO WebAPI.
///
/// # Arguments
/// * `params` - Parameters including optional short-term filter and provider.
///
/// # Returns
/// * `Result<DvbResponse<RouteChanges>>` - The parsed response containing changes, banners, and affected lines.
///
/// Endpoint: `https://webapi.vvo-online.de/rc`
pub async fn route_changes(params: Params<'_>) -> Result<DvbResponse<RouteChanges>> {
    let response = reqwest::Client::new()
        .post(ROUTE_CHANGES_URL)
        .json(&params)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

/// Fetches lines affected by route changes from the VVO WebAPI.
///
/// # Arguments
/// * `params` - Parameters including optional provider filter.
///
/// # Returns
/// * `Result<DvbResponse<RouteChangeLines>>` - The parsed response containing affected lines.
///
/// Endpoint: `https://webapi.vvo-online.de/rc/lines`
pub async fn route_change_lines(params: LinesParams<'_>) -> Result<DvbResponse<RouteChangeLines>> {
    let response = reqwest::Client::new()
        .post(ROUTE_CHANGE_LINES_URL)
        .json(&params)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
