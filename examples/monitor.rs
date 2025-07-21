use dvb::Result;
#[allow(unused_imports)]
use dvb::{monitor, trip, DvbTime, Mot};

fn main() -> Result<()> {
    let monitor_config = monitor::Config {
        stopid: "Dresden HBF",
        mot: Some(&[Mot::Tram]),
        limit: None,
        ..Default::default()
    };

    let departures = monitor::departure_monitor(monitor_config)?;

    if let Some(next_drei) = departures.next_line("3") {
        let time = DvbTime::in_n_minutes(180);

        let trip_config = trip::Config {
            tripid: &next_drei.id,
            // stopid: "33000028",
            stopid: "Walpurgisstraße",
            time,
            ..Default::default()
        };

        let details = trip::trip_details(&trip_config);
        println!("{details:#?}");
        println!("{next_drei:#?}");
    }

    Ok(())
}
