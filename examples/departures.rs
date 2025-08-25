//! Simple example: Query next arrivals at "HauptBahnhof" using dvb-rs

mod route_and_trip;

use dvb::{Result, find_stops, monitor_departures};

#[tokio::main]
async fn main() -> Result<()> {
    let origin_query = std::env::args().nth(1).unwrap_or("HauptBahnhof".into());

    let origin = find_stops(&origin_query).await?;
    let Some(origin) = origin.points.first() else {
        eprintln!("No stop found for '{origin_query}'");
        return Ok(());
    };

    println!("Departures for stop: {} ({})", origin.name, origin.id);

    // Query departures for the found stop
    let monitor = monitor_departures(&origin.id).await?;
    let departures = match &monitor.departures {
        Some(deps) => deps,
        None => {
            println!("No departures found.");
            return Ok(());
        }
    };

    let dir_len = departures
        .iter()
        .map(|dep| dep.direction.len())
        .max()
        .unwrap_or(0);

    for dep in departures {
        if let Some(arrival) = &dep.real_time {
            println!(
                "{:>5} to {:dir_len$} in {:>6}",
                dep.line_name,
                dep.direction,
                arrival.wait(),
            );
        }
    }

    Ok(())
}
