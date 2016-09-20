//! Holds `Station`

use json::JsonValue;
use multimap::MultiMap;

use api::APIEndPoint;
use error::{Result,ErrorKind};

/// API Url for Stations
pub const URL: &'static str = "http://widgets.vvo-online.de/abfahrtsmonitor/Haltestelle.do";

/// Modeling the endpoint "abfahrtsmonitor/Haltestelle".
pub struct Station<'a> {
    station: &'a str,
    city: &'a str,
    lim: u32,
}


impl<'a> Station<'a> {
    /// Creates new `Station` from station name.
    /// City defaults to `"Dresden"`.
    pub fn new(name: &'a str) -> Self {
        Station {
            station: name,
            city: "Dresden",
            lim: 3
        }
    }

    /// Modifies the stations city.
    pub fn city(mut self, city: &'a str) -> Self {
        self.city = city;
        self
    }

    pub fn results(&self) -> Result<MultiMap<String,(String,String)>>{
        let data = try!(self.get());

        if let Some(&JsonValue::Array(ref ja)) = data.members().nth(1){
            ja.iter().map(|st_ja| {
                if let &JsonValue::Array(ref st_a) = st_ja {
                    if let (Some(station), Some(city), Some(id)) = (st_a[0].as_str(), st_a[1].as_str(), st_a[2].as_str()){
                        Ok((city.to_string(), (station.to_string(), id.to_string())))
                    } else {Err(ErrorKind::ApiError.into())}
                } else {Err(ErrorKind::ApiError.into())}
            })
            .collect::<Result<MultiMap<String,(String,String)>>>()
        } else {Err(ErrorKind::ApiError.into())}
    }
}

impl<'a> APIEndPoint for Station<'a> {
    fn url(&self) -> String {
        format!("{base}?ort={city}&hst={begin}&lim={lim}",
                base = URL,
                city = &self.city,
                lim = self.lim,
                begin = &self.station)
    }
}
