use dvb::{DvbTime, Mot, Result, easy::find, monitor, trip};

fn main() -> Result<()> {
    let query1 = std::env::args().nth(1).unwrap_or("HauptBahnhof".into());
    let query2 = std::env::args().nth(2).unwrap_or("WalpurgisStraße".into());

    let start = find(&query1)?;
    let Some(start) = start.points.get(0) else {
        eprintln!("No stop found for '{query1}'");
        return Ok(());
    };

    let destination = find(&query2)?;
    let Some(destination) = destination.points.get(0) else {
        eprintln!("No stop found for '{query2}'");
        return Ok(());
    };

    let monitor_config = monitor::Config {
        stopid: &start.id,
        mot: Some(&[Mot::Tram]),
        limit: None,
        ..Default::default()
    };

    let departures = monitor::departure_monitor(monitor_config)?;

    if let Some(next_drei) = departures.next_line("3") {
        // println!("Next 3: {next_drei:#?}");
        let time = DvbTime::in_n_minutes(180);

        let trip_config = trip::Config {
            tripid: &next_drei.id,
            // stopid: "33000028",
            stopid: &destination.id,
            time,
            ..Default::default()
        };

        let trip_details = trip::trip_details(&trip_config)?;
        println!(
            "Next 3: real_time = {:?}, direction = {}",
            next_drei.real_time, next_drei.direction
        );
        println!("Stops:");
        for stop in &trip_details.stops.unwrap() {
            println!("  - {} ({} {:?})", stop.name, stop.id, stop.time);
        }
    }

    Ok(())
}
