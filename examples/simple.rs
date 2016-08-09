extern crate dvb;

use dvb::prelude::*;

fn main() {
    let st = Station::new("slub").city("Dresden");
    println!("{:?}", st.get());

    println!("{}", Station::new("NOE").get().access("0/0/0").unwrap());
    println!("{:?}", Station::new("NOE").get_raw());
}
