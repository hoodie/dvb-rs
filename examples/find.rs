use dvb::{find_stops, point::Point};

fn main() {
    let found = find_stops("walpurgis").unwrap();
    println!(
        "Found {} stops ({:?})",
        found.points.len(),
        found.expiration_time
    );

    for Point { id, city, name, .. } in &found.points {
        println!("({id}) {city} {name}");
    }
}
