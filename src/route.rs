//! Route planning and route details for Dresden public transport.

use crate::{DvbResponse, error::Result, time::DvbTime};
use serde::{Deserialize, Serialize};
use slug::slugify;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Diva {
    pub network: Option<String>,
    pub number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Platform {
    pub name: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Ticket {
    pub fare_zone_names: Option<String>,
    pub name: Option<String>,
    pub number_of_fare_zones: Option<String>,
    pub price: Option<String>,
    pub price_level: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Coordinate {
    pub lat: Option<f64>,
    pub lng: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ParkingLot {
    pub coordinates: Coordinate,
    #[serde(default)]
    pub occupied: bool,
    #[serde(default)]
    pub disabled_persons_only: bool,
}

const ROUTE_URL: &str = "https://webapi.vvo-online.de/tr/trips";

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn make_partial_route(line: &str, direction: &str, dep_time: Option<DvbTime>) -> PartialRoute {
        PartialRoute {
            duration: Some(10),
            map_data_index: Some(1),
            mot: Some(Mot {
                changes: vec![],
                direction: Some(direction.to_string()),
                diva: None,
                dl_id: None,
                name: Some(line.to_string()),
                operator_code: None,
                product_name: None,
                stateless_id: None,
                train_number: None,
                transportation_company: None,
                r#type: None,
            }),
            next_departure_times: dep_time.map(|t| vec![t]),
            partial_route_id: Some(1),
            previous_departure_times: None,
            regular_stops: None,
            shift: None,
            infos: None,
        }
    }

    #[test]
    fn test_route_with_partial_route_map_conversion_and_serialization() {
        use std::str::FromStr;
        let pr1 = make_partial_route(
            "3",
            "Wilder Mann",
            Some(DvbTime::from_str("/Date(1717236000000+0200)/").unwrap()),
        );
        let pr2 = make_partial_route(
            "3",
            "Wilder Mann",
            Some(DvbTime::from_str("/Date(1717236900000+0200)/").unwrap()),
        );
        let pr3 = make_partial_route(
            "7",
            "Pennrich",
            Some(DvbTime::from_str("/Date(1717235100000+0200)/").unwrap()),
        );

        let route = Route {
            duration: Some(25),
            fare_zone_destination: Some(1),
            fare_zone_names: Some("Zone1".to_string()),
            fare_zone_names_day_ticket: Some("Zone1Day".to_string()),
            fare_zone_origin: Some(2),
            interchanges: Some(0),
            mot_chain: None,
            net: Some("VVO".to_string()),
            number_of_fare_zones: Some("2".to_string()),
            number_of_fare_zones_day_ticket: Some("2".to_string()),
            partial_routes: Some(vec![pr1, pr2, pr3]),
            price: Some("2.50".to_string()),
            price_day_ticket: Some("6.00".to_string()),
            price_level: Some(1),
            route_id: Some(42),
            tickets: None,
        };

        let mapped: MappedRoute = route.into();

        // Check keys
        assert!(mapped.partial_routes.contains_key("3-wilder-mann-1"));
        assert!(mapped.partial_routes.contains_key("3-wilder-mann-2"));
        assert!(mapped.partial_routes.contains_key("7-pennrich"));

        // Check correct mapping
        assert_eq!(
            mapped.partial_routes["3-wilder-mann-1"]
                .mot
                .as_ref()
                .unwrap()
                .name
                .as_deref(),
            Some("3")
        );
        assert_eq!(
            mapped.partial_routes["3-wilder-mann-2"]
                .mot
                .as_ref()
                .unwrap()
                .direction
                .as_deref(),
            Some("Wilder Mann")
        );
        assert_eq!(
            mapped.partial_routes["7-pennrich"]
                .mot
                .as_ref()
                .unwrap()
                .name
                .as_deref(),
            Some("7")
        );

        // Serialization
        let json = serde_json::to_string_pretty(&mapped).unwrap();
        assert!(json.contains("3-wilder-mann-1"));
        assert!(json.contains("7-pennrich"));
    }
}

// New struct for mapped partial routes
#[derive(Serialize, Deserialize, Debug)]
pub struct MappedRoute {
    pub duration: Option<u32>,
    pub fare_zone_destination: Option<u32>,
    pub fare_zone_names: Option<String>,
    pub fare_zone_names_day_ticket: Option<String>,
    pub fare_zone_origin: Option<u32>,
    pub interchanges: Option<u32>,
    pub mot_chain: Option<Vec<MotChain>>,
    pub net: Option<String>,
    pub number_of_fare_zones: Option<String>,
    pub number_of_fare_zones_day_ticket: Option<String>,
    pub price: Option<String>,
    pub price_day_ticket: Option<String>,
    pub price_level: Option<u32>,
    pub route_id: Option<u32>,
    pub tickets: Option<Vec<Ticket>>,
    pub partial_routes: HashMap<String, PartialRoute>,
}

// From<Route> for RouteWithPartialRouteMap
impl From<Route> for MappedRoute {
    fn from(route: Route) -> Self {
        let mut partial_routes_map: HashMap<String, PartialRoute> = HashMap::new();

        if let Some(partial_routes) = route.partial_routes {
            // Group by (line name, direction)
            let mut grouped: HashMap<(String, String), Vec<PartialRoute>> = HashMap::new();

            for pr in partial_routes {
                let name = pr
                    .mot
                    .as_ref()
                    .and_then(|mot| mot.name.clone())
                    .unwrap_or_default();
                let direction = pr
                    .mot
                    .as_ref()
                    .and_then(|mot| mot.direction.clone())
                    .unwrap_or_default();
                grouped.entry((name, direction)).or_default().push(pr);
            }

            // For each group, sort by departure time and insert with index
            for ((name, direction), mut prs) in grouped {
                prs.sort_by_key(|pr| {
                    pr.next_departure_times
                        .as_ref()
                        .and_then(|times| times.first().cloned())
                });

                let prs_len = prs.len();
                for (idx, pr) in prs.into_iter().enumerate() {
                    let mut key = format!("{}-{}", name, direction);
                    key = slugify(&key);
                    if prs_len > 1 {
                        key = format!("{}-{}", key, idx + 1);
                    }
                    partial_routes_map.insert(key, pr);
                }
            }
        }

        MappedRoute {
            duration: route.duration,
            fare_zone_destination: route.fare_zone_destination,
            fare_zone_names: route.fare_zone_names,
            fare_zone_names_day_ticket: route.fare_zone_names_day_ticket,
            fare_zone_origin: route.fare_zone_origin,
            interchanges: route.interchanges,
            mot_chain: route.mot_chain,
            net: route.net,
            number_of_fare_zones: route.number_of_fare_zones,
            number_of_fare_zones_day_ticket: route.number_of_fare_zones_day_ticket,
            price: route.price,
            price_day_ticket: route.price_day_ticket,
            price_level: route.price_level,
            route_id: route.route_id,
            tickets: route.tickets,
            partial_routes: partial_routes_map,
        }
    }
}

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

/// Queries possible routes between two stops using the VVO WebAPI.
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
