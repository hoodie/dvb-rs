//! Holds `Monitor`

use json::JsonValue;
use multimap::MultiMap;

use api::APIEndPoint;
use error::{Result,ErrorKind};

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


    /// Gives you lists tuples `(direction:String, time-to-departure:u32)` by line name
    pub fn by_line(&self) -> Result<MultiMap<String, (String, u32)>> {
        let mmap = try!(self.get())
            .members()
            .map(|stop| {
                match stop {
                    &JsonValue::Array(ref a) => {
                        match (a[0].as_str(), a[1].as_str(), a[2].as_str().and_then(|s| s.parse::<u32>().ok()))
                        {
                            (Some(line), Some(dir), Some(time)) => Some((line.to_string(),(dir.to_string(),time))),
                            _ => None
                        }
                    }
                    _ => None
                }
            }).collect();
        match mmap{
            Some(mmap) => Ok(mmap),
            None => Err(ErrorKind::ApiError.into())
        }
    }
}

impl<'a> APIEndPoint for Monitor<'a> {
    fn url(&self) -> String {
        format!("{base}?ort={ort}&vz={vz}&hst={stop}&lim={lim}",
                base = URL,
                ort = &self.city,
                lim = self.lim,
                vz = self.vz,
                stop = &self.name)
    }
}
