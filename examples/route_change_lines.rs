//! Example: Query lines affected by route changes

use dvb::route_changes::{LinesParams, route_change_lines};

#[tokio::main]
async fn main() -> dvb::Result<()> {
    let params = LinesParams {
        format: Some("json"),
        ..Default::default()
    };

    let response = route_change_lines(params).await?;

    println!("Lines with route changes ({}):", response.lines.len());
    for line in &response.lines {
        println!(
            "  {}: {} ({:?})",
            line.id.as_deref().unwrap_or("?"),
            line.name.as_deref().unwrap_or("?"),
            line.mot,
        );
    }

    Ok(())
}
