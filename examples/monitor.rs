use dvb::{
    DvbTime, Mot, Result, find_stops, monitor,
    trip::{self, Stop},
};

#[tokio::main]
async fn main() -> Result<()> {
    let query1 = std::env::args().nth(1).unwrap_or("HauptBahnhof".into());
    let query2 = std::env::args().nth(2).unwrap_or("WalpurgisStraße".into());

    let start = find_stops(&query1).await?;
    let Some(start) = start.points.first() else {
        eprintln!("No stop found for '{query1}'");
        return Ok(());
    };

    let destination = find_stops(&query2).await?;
    let Some(destination) = destination.points.first() else {
        eprintln!("No stop found for '{query2}'");
        return Ok(());
    };

    let monitor_params = monitor::Params {
        stopid: &start.id,
        mot: Some(&[Mot::Tram]),
        limit: None,
        ..Default::default()
    };

    let departures = monitor::departure_monitor(monitor_params).await?;

    if let Some(next_drei) = departures.next_line("3") {
        // println!("Next 3: {next_drei:#?}");
        let time = DvbTime::in_n_minutes(180);

        let trip_params = trip::Params {
            tripid: &next_drei.id,
            // stopid: "33000028",
            stopid: &destination.id,
            time,
            ..Default::default()
        };

        let trip_details = trip::trip_details(&trip_params).await?;
        println!(
            "Next 3: real_time = {:?}, direction = {}",
            next_drei.real_time, next_drei.direction
        );
        println!("Stops:");
        for Stop { id, name, time, .. } in &trip_details.stops {
            println!("{id} {time} - {name}", time = time.to_rfc3339());
        }
    }

    Ok(())
}
