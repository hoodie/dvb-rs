extern crate dvb;

use dvb::error::Result;

fn find(query: &str) -> Result<dvb::find::Found> {
    let point = dvb::find::find_point(
        &dvb::find::Config {
            query,
            stops_only: Some(true),
            ..Default::default()
        })?;

    Ok(point)
}

fn monitor(stopid: &str) -> Result<dvb::monitor::DepartureMonitor> {
    let monitor = dvb::monitor::departure_monitor(
        dvb::monitor::Config {
            stopid,
            mot: Some(&[dvb::Mot::Tram]),
            limit: Some(10),
            ..Default::default()
        })?;

    Ok(monitor)
}

fn main() -> Result<()> {

    let found = find("Tharanter Straße")?;
    if let Some(point) = found.points.get(0) {
        println!("found {:#?}", point.name);
        if let Ok(monitor) = monitor(&point.id) {
            println!("Departures from {}:", point.name);
            for dvb::monitor::Departure{line_name, direction, real_time, ..} in &monitor.departures {
                if let Some(arrival) = real_time {
                println!("{:>5} => {:10}: {}", line_name, direction, arrival.wait());
                }
            }
        }

    }

    // monitor()?;
    // trip()?;
    Ok(())
}