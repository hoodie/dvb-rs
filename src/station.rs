
use api::APIEndPoint;

pub struct Station<'a> {
    pub station: &'a str,
    pub city: &'a str,
}

impl<'a> Station<'a> {
    /// Creates new `Station` from station name.
    /// City defaults to `"Dresden"`.
    pub fn new(name: &'a str) -> Self {
        Station {
            station: name,
            city: "Dresden",
        }

    }

    /// Modifies the stations city.
    pub fn city(mut self, city: &'a str) -> Self {
        self.city = city;
        self
    }
}

impl<'a> APIEndPoint for Station<'a> {
    fn url(&self) -> String {
        let base_url = "http://widgets.vvo-online.de/abfahrtsmonitor/Haltestelle.do";
        format!("{base}?ort={city}&vz=0&hst={begin}",
                base = base_url,
                city = &self.city,
                begin = &self.station)
    }
}
