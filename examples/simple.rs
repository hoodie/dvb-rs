extern crate dvb;

use dvb::prelude::*;

fn main() {

    //println!("{:#?}", Station::new("SLUB").results().unwrap());

    //rintln!("Monitor");
    println!("{:#?}", Monitor::new("SLUB").by_line().unwrap());

}
