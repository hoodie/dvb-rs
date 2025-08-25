use chrono::prelude::*;
use num_integer::div_mod_floor;
use regex::Regex;

use schemars::JsonSchema;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};

use std::{
    error::Error,
    fmt,
    ops::{Deref, Sub},
    str::FromStr,
    string::ToString,
};

#[derive(JsonSchema)]
pub struct DvbTime(DateTime<FixedOffset>);

impl fmt::Debug for DvbTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " (/Date){}", &self.0)
    }
}

impl DvbTime {
    pub fn wait(&self) -> String {
        let now = Local::now();
        let dt: DateTime<FixedOffset> = now.with_timezone(now.offset());
        let min = self.0.sub(dt).num_minutes();

        format!("{min}min")
    }

    pub fn now() -> Self {
        DvbTime::from(Local::now())
    }

    pub fn in_n_minutes(mins: i64) -> Self {
        DvbTime::from(Local::now() + chrono::Duration::minutes(mins))
    }

    pub fn to_datetime(&self) -> DateTime<FixedOffset> {
        self.0
    }

    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }

    pub fn to_time(&self) -> String {
        self.0.time().to_string()
    }
}

impl Default for DvbTime {
    fn default() -> Self {
        Self::now()
    }
}

impl Deref for DvbTime {
    type Target = DateTime<FixedOffset>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DateTime<Local>> for DvbTime {
    fn from(dt: DateTime<Local>) -> Self {
        let dt: DateTime<FixedOffset> = dt.with_timezone(dt.offset());
        DvbTime::from(dt)
    }
}

impl From<DateTime<FixedOffset>> for DvbTime {
    fn from(dt: DateTime<FixedOffset>) -> Self {
        DvbTime(dt)
    }
}

impl AsRef<DateTime<FixedOffset>> for DvbTime {
    fn as_ref(&self) -> &DateTime<FixedOffset> {
        &self.0
    }
}

impl FromStr for DvbTime {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^/Date\((\d*)(\+|-)(\d{2})(\d{2})\)/")?;
        if let Some(caps) = re.captures(s) {
            let raw_timestamp = &caps[1];
            let timestamp: i64 = raw_timestamp.parse()?;
            let hours: i32 = caps[3].parse()?;
            let mins: i32 = caps[4].parse()?;

            let multiplier = if raw_timestamp.ends_with("000") {
                1000
            } else {
                1
            };

            let fo = FixedOffset::east_opt(hours * 3600 + mins * 60)
                .unwrap()
                .timestamp_opt(timestamp / multiplier, 0)
                .unwrap();

            Ok(DvbTime(fo))
        } else {
            Err("nothing matched".into())
        }
    }
}

impl fmt::Display for DvbTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let offset = self.offset().fix().local_minus_utc();

        let (sign, offset) = if offset < 0 {
            ('-', -offset)
        } else {
            ('+', offset)
        };
        let (mins, _sec) = div_mod_floor(offset, 60);
        let (hour, min) = div_mod_floor(mins, 60);

        write!(
            f,
            "/Date({}000{}{:02}{:02})/",
            self.timestamp(),
            sign,
            hour,
            min
        )
    }
}

impl Serialize for DvbTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for DvbTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(DvbTimeVisitor)
    }
}

struct DvbTimeVisitor;

impl<'de> Visitor<'de> for DvbTimeVisitor {
    type Value = DvbTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a string that follows that \"/Date(...)/\" format "
        )
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match DvbTime::from_str(s) {
            Ok(dt) => Ok(dt),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_reparse() {
        let now = Local::now();
        let dvb = DvbTime::from(now).to_string();
        let parsed = dvb.parse::<DvbTime>();
        println!("now: {now}\ndvb: {dvb}\nparsed: {parsed:?}");

        let (parsed_timestamp, parsed_offset) = parsed
            .map(|dt| (DateTime::timestamp(&dt.0), dt.0.offset().local_minus_utc()))
            .unwrap();

        let original_offset = now.offset().local_minus_utc();

        assert_eq!(parsed_timestamp, now.timestamp());
        assert_eq!(parsed_offset, original_offset);
    }

    #[test]
    fn negative_offset() {
        let dvb = "/Date(155581260000-0000)/";
        let parsed = DvbTime::from_str(dvb);
        println!("{parsed:?}");
    }
}
