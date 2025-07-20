use dvb::{find_nearby_stops, point::Point};

#[tokio::main]
async fn main() {
    let found = find_nearby_stops("walpurgis").await.unwrap();
    println!(
        "Found {} stops ({:?})",
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
