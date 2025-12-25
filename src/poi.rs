//! Types and utilities for querying Points of Interest (POIs).

use std::str::FromStr;

use schemars::JsonSchema;
use serde::Serialize;

/// Represents the type of a Point of Interest (POI)
#[derive(Serialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoiType {
    Stop,
    Address,
    Coords,
    Poi,
}

#[derive(Serialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct PoiId {
    pub id: String,
    pub r#type: PoiType,
}

impl FromStr for PoiId {
    type Err = ();

    fn from_str(id: &str) -> Result<PoiId, ()> {
        let mut poi_id: Vec<&str> = id.split(':').collect();

        if poi_id.len() >= 4 {
            poi_id.truncate(4);
            let joined_id = poi_id.join(":");

            match poi_id[0] {
                "streetID" => {
                    return Ok(PoiId {
                        id: joined_id,
                        r#type: PoiType::Address,
                    });
                }
                "coord" => {
                    return Ok(PoiId {
                        id: joined_id,
                        r#type: PoiType::Coords,
                    });
                }
                "poiID" => {
                    return Ok(PoiId {
                        id: joined_id,
                        r#type: PoiType::Poi,
                    });
                }
                _ => {}
            }
        }

        Ok(PoiId {
            id: id.to_string(),
            r#type: PoiType::Stop,
        })
    }
}
