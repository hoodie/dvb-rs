#![allow(dead_code)]
extern crate dvb;

use dvb::error::Result;

fn monitor() -> Result<()> {

    let monitor = dvb::monitor::departure_monitor(
        dvb::monitor::Config {
            stopid: "33000728",
            limit: None,
            ..Default::default()
        })?;

    println!("montior: {:#?}", monitor);

    Ok(())

}

fn find() -> Result<()> {
    let point = dvb::find::find_point(
        &dvb::find::Config {
            query: "slub",
            stops_only: Some(false),
            ..Default::default()
        })?;

    println!("point: {:#?}", point);
    Ok(())
}

fn main() -> Result<()> {

    // monitor()?;
    find()?;
    Ok(())
}