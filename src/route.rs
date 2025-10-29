use crate::{DvbResponse, error::Result, time::DvbTime};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Route {
    pub duration: Option<u32>,
    pub fare_zone_destination: Option<u32>,
    pub fare_zone_names: Option<String>,
    pub fare_zone_names_day_ticket: Option<String>,
    pub fare_zone_origin: Option<u32>,
    pub interchanges: Option<u32>,
    // pub map_data: Option<Vec</*MapData*/ String>>,
    pub mot_chain: Option<Vec<MotChain>>,
    pub net: Option<String>,
    pub number_of_fare_zones: Option<String>,
    pub number_of_fare_zones_day_ticket: Option<String>,
    pub partial_routes: Option<Vec<PartialRoute>>,
    pub price: Option<String>,
    pub price_day_ticket: Option<String>,
    pub price_level: Option<u32>,
    pub route_id: Option<u32>,
    pub tickets: Option<Vec<Ticket>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Routes {
    #[serde(default)]
    pub routes: Vec<Route>,
}

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "PascalCase")]
// pub struct MapData {
//     // Expand as needed
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MotChain {
    pub changes: Option<Vec<String>>,
    pub direction: Option<String>,
    pub diva: Option<Diva>,
    pub dl_id: Option<String>,
    pub name: Option<String>,
    pub operator_code: Option<String>,
    pub product_name: Option<String>,
    pub stateless_id: Option<String>,
    pub train_number: Option<String>,
    pub transportation_company: Option<String>,
    pub r#type: Option<crate::common::Mot>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Diva {
    pub network: Option<String>,
    pub number: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PartialRoute {
    pub duration: Option<u32>,
    pub map_data_index: Option<i32>,
    pub mot: Option<Mot>,
    pub next_departure_times: Option<Vec<DvbTime>>,
    pub partial_route_id: Option<u32>,
    pub previous_departure_times: Option<Vec<DvbTime>>,
    pub regular_stops: Option<Vec<RegularStop>>,
    pub shift: Option<String>,
    pub infos: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mot {
    #[serde(default)]
    pub changes: Vec<String>,
    pub direction: Option<String>,
    pub diva: Option<Diva>,
    pub dl_id: Option<String>,
    pub name: Option<String>,
    pub operator_code: Option<String>,
    pub product_name: Option<String>,
    pub stateless_id: Option<String>,
    pub train_number: Option<String>,
    pub transportation_company: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegularStop {
    pub arrival_time: Option<DvbTime>,
    pub cancel_reasons: Vec<String>,
    pub data_id: Option<String>,
    pub departure_time: Option<DvbTime>,
    pub dh_id: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub name: Option<String>,
    pub occupancy: Option<String>,
    #[serde(default)]
    pub park_and_rail: Vec<ParkAndRail>,
    pub place: Option<String>,
    pub platform: Option<Platform>,
    pub r#type: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Platform {
    pub name: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ticket {
    pub fare_zone_names: Option<String>,
    pub name: Option<String>,
    pub number_of_fare_zones: Option<String>,
    pub price: Option<String>,
    pub price_level: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ParkAndRail {
    pub coordinates: Option<Coordinate>,
    pub free_spaces: Option<u32>,
    pub name: Option<String>,
    #[serde(default)]
    pub parking_lots: Vec<ParkingLot>,
    pub disabled_persons_only: Option<bool>,
    pub total_spaces: Option<u32>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Coordinate {
    pub lat: Option<f64>,
    pub lng: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ParkingLot {
    pub coordinates: Coordinate,
    #[serde(default)]
    pub occupied: bool,
    #[serde(default)]
    pub disabled_persons_only: bool,
}

const ROUTE_URL: &str = "https://webapi.vvo-online.de/tr/trips";

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Params<'a> {
    /// Origin stop ID.
    pub origin: &'a str,
    /// Destination stop ID.
    pub destination: &'a str,
    /// Time for the query (DvbTime).
    pub time: DvbTime,
    /// Interpret time as arrival time.
    pub isarrivaltime: bool,
    /// Include short-term changes.
    pub shorttermchanges: bool,
    /// Response format (e.g., "json").
    pub format: &'a str, // TODO: verify existence
    /// Intermediate stop ID.
    pub via: Option<&'a str>, // TODO: verify existence
}

pub async fn route_details_json<'a>(params: &Params<'a>) -> Result<Value> {
    Ok(reqwest::Client::new()
        .get(ROUTE_URL)
        .query(&params)
        .send()
        .await?
        .json()
        .await?)
}

/// Queries possible routes between two stops using the VVO WebAPI.
///
/// # Arguments
/// * `params` - Parameters including origin, destination, time, and optional flags.
///
/// # Returns
/// * `Result<DvbResponse<Routes>>` - The parsed response containing possible routes.
///
/// Endpoint: `https://webapi.vvo-online.de/tr/trips`
pub async fn route_details<'a>(params: &Params<'a>) -> Result<DvbResponse<Routes>> {
    let routes = reqwest::Client::new()
        .get(ROUTE_URL)
        .query(&params)
        .send()
        .await?
        .json()
        .await?;

    Ok(routes)
}
