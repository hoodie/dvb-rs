//! Time parsing and formatting utilities for Dresden transport API.
//! Provides DvbTime for the custom `/Date(...)` format.

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

#[derive(Clone, JsonSchema)]
pub struct DvbTime(DateTime<FixedOffset>);

impl fmt::Debug for DvbTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " (/Date){}", &self.0)
    }
}

impl DvbTime {
    /// Returns a human-readable string representing the wait time in minutes from now until this time.
    ///
    /// # Example
    /// ```rust
    /// # use dvb::DvbTime;
    /// let future = DvbTime::in_n_minutes(10);
    /// let wait = future.wait();
    /// assert!(wait.ends_with("min"));
    /// ```
    pub fn wait(&self) -> String {
        let now = Local::now();
        let dt: DateTime<FixedOffset> = now.with_timezone(now.offset());
        let min = self.0.sub(dt).num_minutes();

        format!("{min}min")
    }

    /// Returns the current local time as a `DvbTime`.
    pub fn now() -> Self {
        DvbTime::from(Local::now())
    }

    /// Returns a `DvbTime` representing `mins` minutes from now.
    ///
    /// # Example
    /// ```rust
    /// # use dvb::DvbTime;
    /// let t = DvbTime::in_n_minutes(5);
    /// assert!(t.to_datetime() > DvbTime::now().to_datetime());
    /// ```
    pub fn in_n_minutes(mins: i64) -> Self {
        DvbTime::from(Local::now() + chrono::Duration::minutes(mins))
    }

    /// Returns the underlying `DateTime<FixedOffset>`.
    pub fn to_datetime(&self) -> DateTime<FixedOffset> {
        self.0
    }

    /// Formats the time as an RFC3339 string.
    ///
    /// # Example
    /// ```rust
    /// # use dvb::DvbTime;
    /// let rfc = DvbTime::now().to_rfc3339();
    /// assert!(rfc.contains('T'));
    /// ```
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }

    /// Returns the time portion as a string (HH:MM:SS).
    ///
    /// # Example
    /// ```rust
    /// # use dvb::DvbTime;
    /// let time_str = DvbTime::now().to_time();
    /// assert!(time_str.contains(':'));
    /// ```
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

/// Serializes a DvbTime as an ISO8601/RFC3339 string.
///
/// Use with `#[serde(serialize_with = "serialize_as_iso8601")]` to explicitly
/// serialize as ISO8601, or enable the `iso8601-serialization` feature to make
/// this the default behavior for all DvbTime instances.
#[cfg(feature = "iso8601-serialization")]
fn serialize_as_iso8601<S: Serializer>(dt: &DvbTime, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&dt.to_rfc3339())
}

/// Serializes DvbTime.
///
/// By default, serializes to the DVB `/Date(...)` format.
/// With the `iso8601-serialization` feature enabled, serializes to ISO8601/RFC3339 format.
///
/// Note: Deserialization always expects the DVB `/Date(...)` format regardless of this feature.
impl Serialize for DvbTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[cfg(feature = "iso8601-serialization")]
        {
            serialize_as_iso8601(self, serializer)
        }
        #[cfg(not(feature = "iso8601-serialization"))]
        {
            serializer.serialize_str(&self.to_string())
        }
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

    #[test]
    fn deserialize_dvb_format() {
        let json = r#""/Date(1609459200000+0100)/""#;
        let dt: DvbTime = serde_json::from_str(json).unwrap();
        assert!(dt.to_datetime().timestamp() > 0);
    }

    #[test]
    #[cfg(not(feature = "iso8601-serialization"))]
    fn serialize_dvb_format_default() {
        let dt = DvbTime::from_str("/Date(1609459200000+0100)/").unwrap();
        let json = serde_json::to_string(&dt).unwrap();
        assert!(json.contains("/Date("));
        assert!(json.contains(")/"));
    }

    #[test]
    #[cfg(feature = "iso8601-serialization")]
    fn serialize_iso8601_format_with_feature() {
        let dt = DvbTime::from_str("/Date(1609459200000+0100)/").unwrap();
        let json = serde_json::to_string(&dt).unwrap();
        assert!(json.contains("T"));
        assert!(json.contains("Z") || json.contains("+") || json.contains("-"));
        assert!(!json.contains("/Date("));
    }
}
