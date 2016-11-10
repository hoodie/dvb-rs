//! Holds `Monitor`

use multimap::MultiMap;

use api::APIEndPoint;
use error::{Result,ErrorKind};
use line::{Line,Departure};

/// API Url for Monitor
pub const URL: &'static str = "http://widgets.vvo-online.de/abfahrtsmonitor/Abfahrten.do";

/// Modeling the endpoint "abfahrtsmonitor/Abfahrten".
pub struct Monitor<'a> {
    name: &'a str,
    city: &'a str,
    lim: u32,
    vz: u32,
}

impl<'a> Monitor<'a> {
    /// Creates new `Monitor` from name.
    /// City defaults to `""`.
    pub fn new(name: &'a str) -> Self {
        Monitor {
            name: name,
            city: "",
            lim: 0,
            vz: 0,
        }
    }

    /// Modifies the monitors city.
    pub fn city(mut self, city: &'a str) -> Self {
        self.city = city;
        self
    }


    /// Gives you a list b
    pub fn departures(&self) -> Result<Vec<Departure>> {
        try!(self.get())
            .members()
            .map( Departure::from_json)
            .collect::<Option<_>>()
            .ok_or(ErrorKind::ApiError.into())
    }

    /// Gives you lists tuples `(direction:String, time-to-departure:u32)` by line name
    pub fn departures_by_line(&self) -> Result<MultiMap<Line,Departure>> {
        Ok(try!(self.departures()).into_iter().map(|d| (d.line.clone(), d)).collect())
    }
}

impl<'a> APIEndPoint for Monitor<'a> {
    fn url(&self) -> String {
        format!("{base}?ort={ort}&vz={vz}&hst={stop}&lim={lim}",
                base = URL,
                ort  = &self.city,
                lim  = self.lim,
                vz   = self.vz,
                stop = &self.name
                )
    }
}
