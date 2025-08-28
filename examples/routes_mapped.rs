#![allow(unused)]

use dvb::{
    DvbTime, find_stops,
    route::{MappedRoute, Params, Route, Routes, route_details},
};

#[tokio::main]
async fn main() -> dvb::Result<()> {
    // Get origin and destination from command line or use defaults
    let query1 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Haupbahnhof".to_string());
    let found_origin = find_stops(&query1).await?;
    let origin = found_origin
        .points
        .first()
        .expect("Start-Haltestelle nicht gefunden");

    let query2 = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "Postplatz".to_string());
    let found_destination = find_stops(&query2).await?;
    let destination = found_destination
        .points
        .first()
        .expect("Ziel-Haltestelle nicht gefunden");

    // Use current time for the route query
    let start_time = DvbTime::from(chrono::Local::now());

    let params = Params {
        origin: &origin.id,
        destination: &destination.id,
        time: start_time,
        isarrivaltime: false,
        shorttermchanges: true,
        format: "json",
        via: None,
    };

    let response = route_details(&params).await?;
    let routes = response
        .into_inner()
        .routes
        .into_iter()
        .map(MappedRoute::from)
        .collect::<Vec<_>>();

    let routes = serde_json::to_string_pretty(&routes)?;

    println!("{}", routes);

    Ok(())
}
