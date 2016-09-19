extern crate dvb;

use dvb::Monitor;

fn main() {

    println!("Monitor");
    println!("{:#?}", Monitor::new("HBF").stops().unwrap());

}
