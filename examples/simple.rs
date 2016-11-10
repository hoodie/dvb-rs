extern crate dvb;

use dvb::prelude::*;

fn main() {

    //println!("{:#?}", Station::new("SLUB").results().unwrap());

    println!("Monitor");
    println!("{:#?}", Monitor::new("Kaitzer Straße").departures_by_line().unwrap());

}
