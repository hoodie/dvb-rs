use dvb::{
    find_stops,
    lines::{Diva, Line},
};

#[tokio::main]
async fn main() -> dvb::Result<()> {
    let query1 = std::env::args().nth(1).unwrap_or("HauptBahnhof".into());
    let found = find_stops(&query1).await?;
    let origin = found.points.first().unwrap();

    for Line {
        name,
        mot,
        changes,
        diva: Diva { network, .. },
        directions,
        ..
    } in &dvb::lines::lines(&origin.id, None).await?.lines
    {
        println!(
            "Line: {network}-{name} {mot:?} {} directions, {} changes",
            directions.len(),
            changes.len()
        );
    }
    Ok(())
}
