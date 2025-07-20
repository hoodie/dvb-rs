use dvb::{
    find_stops,
    lines::{Diva, Line},
};

fn main() -> dvb::Result<()> {
    let query1 = std::env::args().nth(1).unwrap_or("HauptBahnhof".into());
    let found = find_stops(&query1)?;
    let start = found.points.first().unwrap();

    for Line {
        name,
        mot,
        changes,
        diva: Diva { network, .. },
        directions,
        ..
    } in &dvb::lines::lines(&start.id, None)?.lines
    {
        println!(
            "Line: {network}-{name} {mot:?} {} directions, {} changes",
            directions.len(),
            changes.len()
        );
    }
    Ok(())
}
