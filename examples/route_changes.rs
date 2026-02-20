//! Example: Query current route changes and disruptions

use dvb::route_changes::{Params, route_changes};

#[tokio::main]
async fn main() -> dvb::Result<()> {
    let params = Params {
        shortterm: Some(true),
        format: Some("json"),
        ..Default::default()
    };

    let response = route_changes(params).await?;

    println!("Route Changes ({}):", response.changes.len());
    for change in &response.changes {
        println!(
            "  [{}] {}",
            change.id.as_deref().unwrap_or("?"),
            change.title.as_deref().unwrap_or("(no title)"),
        );
        if !change.validity_periods.is_empty() {
            for period in &change.validity_periods {
                println!(
                    "    Valid: {} - {}",
                    period
                        .begin
                        .as_ref()
                        .map(|t| t.to_rfc3339())
                        .unwrap_or_default(),
                    period
                        .end
                        .as_ref()
                        .map(|t| t.to_rfc3339())
                        .unwrap_or_default(),
                );
            }
        }
    }

    if !response.banners.is_empty() {
        println!("\nBanners ({}):", response.banners.len());
        for banner in &response.banners {
            println!("  {}", banner.title.as_deref().unwrap_or("(no title)"));
        }
    }

    if !response.lines.is_empty() {
        println!("\nAffected Lines ({}):", response.lines.len());
        for line in &response.lines {
            println!(
                "  {} ({:?}) - {}",
                line.name.as_deref().unwrap_or("?"),
                line.mot,
                line.transportation_company.as_deref().unwrap_or("?"),
            );
        }
    }

    Ok(())
}
