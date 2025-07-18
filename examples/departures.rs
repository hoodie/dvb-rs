//! Simple example: Query next arrivals at "HauptBahnhof" using dvb-rs

use dvb::{Result, find_stops, monitor_departures};

fn main() -> Result<()> {
    let query = std::env::args().nth(1).unwrap_or("HauptBahnhof".into());

    let found = find_stops(&query)?;
    let Some(point) = found.points.first() else {
        eprintln!("No stop found for '{query}'");
        return Ok(());
    };

    println!("Departures for stop: {} ({})", point.name, point.id);

    // Query departures for the found stop
    let monitor = monitor_departures(&point.id)?;
    let departures = match &monitor.departures {
        Some(deps) => deps,
        None => {
            println!("No departures found.");
            return Ok(());
        }
    };

    for dep in departures {
        if let Some(arrival) = &dep.real_time {
            println!(
                "{:>5} to {:20} in {}",
                dep.line_name,
                dep.direction,
                arrival.wait()
            );
        }
    }

    Ok(())
}
