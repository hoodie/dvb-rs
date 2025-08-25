#![allow(unused)]

use dvb::{
    DvbTime, find_stops,
    route::{Params, Route, Routes, route_details},
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
        .unwrap_or_else(|| "Albertplatz".to_string());
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
    let routes: &Routes = &response;

    println!(
        "Gefunden: {} Verbindung(en) von '{}' nach '{}':",
        routes.routes.len(),
        origin.name,
        destination.name
    );

    for (i, route) in routes.routes.iter().enumerate() {
        println!("Verbindung {}:", i + 1);
        // println!("{route:#?}"); break;

        if let Some(duration) = route.duration {
            println!("  Dauer: {} min", duration);
        }
        if let Some(interchanges) = route.interchanges {
            println!("  Umstiege: {}", interchanges);
        }
        if let Some(price) = &route.price {
            println!("  Preis: {} EUR", price);
        }
        if let Some(partials) = &route.partial_routes {
            for (j, partial) in partials.iter().enumerate() {
                println!("    Abschnitt {}:", j + 1);
                if let Some(mot) = &partial.mot {
                    if let Some(name) = &mot.name {
                        println!("      Linie {name}");
                    }
                    println!(
                        "      Verkehrsmittel: {}",
                        mot.product_name.as_deref().unwrap_or("Unbekannt")
                    );
                    println!(
                        "      Richtung: {}",
                        mot.direction.as_deref().unwrap_or("-")
                    );
                }
                if let Some(stops) = &partial.regular_stops {
                    for stop in stops {
                        println!(
                            "        {time}: {name}",
                            name = stop.name.as_deref().unwrap_or("Unbekannt"),
                            time = stop
                                .arrival_time
                                .as_ref()
                                .map(DvbTime::to_time)
                                .unwrap_or_default()
                        );
                    }
                }
            }
        }
        println!();
    }

    Ok(())
}
