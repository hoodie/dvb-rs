use dvb::{find_nearby_stops, point::Point};

fn main() {
    let found = find_nearby_stops("walpurgis").unwrap();
    println!(
        "Found {} stops ({})",
        found.points.len(),
        found.expiration_time
    );
    for Point {
        id,
        city,
        name,
        coords: (long, lat),
        r#type: typ,
    } in &found.points
    {
        println!("({id}/{typ:?}) {city} {name} ({long}, {lat})");
    }
}
