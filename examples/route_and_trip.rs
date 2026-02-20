use dvb::{
    DvbTime, find_stops,
    route::{Params as RouteParams, route_details},
    trip::{Params as TripParams, trip_details},
};

#[tokio::main]
#[allow(dead_code)]
async fn main() -> dvb::Result<()> {
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
        .unwrap_or_else(|| "Albertplatz".to_string());
    let found_destination = find_stops(&query2).await?;
    let destination = found_destination
        .points
        .first()
        .expect("Ziel-Haltestelle nicht gefunden");

    let start_time = DvbTime::from(chrono::Local::now());

    let route_params = RouteParams {
        origin: &origin.id,
        destination: &destination.id,
        time: start_time,
        isarrivaltime: false,
        shorttermchanges: true,
        format: "json",
        via: None,
        mobility_settings: None,
        standard_settings: None,
    };

    let route_response = route_details(&route_params).await?;
    let routes = &route_response.routes;

    println!("Gefundene Routen:");
    for (i, route) in routes.iter().enumerate() {
        println!(
            "Route {}: Dauer: {:?}, Umstiege: {:?}, Preis: {:?}",
            i + 1,
            route.duration,
            route.interchanges,
            route.price
        );
    }

    if let Some(first_route) = routes.first()
        && let Some(mot_chain) = first_route.mot_chain.as_ref().and_then(|mc| mc.first())
        && let Some(tripid) = mot_chain.stateless_id.as_ref()
    {
        let trip_params = TripParams {
            tripid,
            time: route_params.time,
            stopid: route_params.destination,
            mapdata: None,
        };

        let trip_response = trip_details(&trip_params).await?;
        let trip = trip_response.into_inner();

        println!("\nHaltestellen der ersten Verbindung:");
        for stop in trip.stops {
            println!(
                "- {} ({}) um {}",
                stop.name,
                stop.place,
                stop.time.to_rfc3339()
            );
        }
    }

    Ok(())
}
