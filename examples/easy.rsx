use dvb::easy::{find, monitor};
use dvb::Result;

fn main() -> Result<()> {

    let found = find("Hauptbahnhof")?;
    if let Some(point) = found.points.get(0) {
        println!("found {:#?}", point.name);

        if let Ok(monitor) = monitor(&point.id) {
            println!("Departures from {}:", point.name);

            for dvb::monitor::Departure{line_name, direction, real_time, ..} in &monitor.departures {
                if let Some(arrival) = real_time {
                println!("{:>5} => {:15}: {}", line_name, direction, arrival.wait());
                }
            }
        }

    }

    // monitor()?;
    // trip()?;
    Ok(())
}