use dvb::{
    DvbTime, Mot, Result, find_stops, monitor,
    trip::{self, Stop},
};

#[tokio::main]
async fn main() -> Result<()> {
    let origin_query = std::env::args().nth(1).unwrap_or("HauptBahnhof".into());
    let destination_query = std::env::args().nth(2).unwrap_or("WalpurgisStraße".into());

    let origin = find_stops(&origin_query).await?;
    let Some(origin) = origin.points.first() else {
        eprintln!("No stop found for '{origin_query}'");
        return Ok(());
    };

    let destination = find_stops(&destination_query).await?;
    let Some(destination) = destination.points.first() else {
        eprintln!("No stop found for '{destination_query}'");
        return Ok(());
    };

    let monitor_params = monitor::Params {
        stopid: &origin.id,
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
