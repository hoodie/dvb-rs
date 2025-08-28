//! Common types and enums.

use std::{fmt::Debug, ops::Deref};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub enum ArrivalState {
    Delayed,
    InTime,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[non_exhaustive]
pub enum Mot {
    Tram,
    Bus,
    CityBus,
    IntercityBus,
    SuburbanRailway,
    Train,
    Cableway,
    Ferry,
    HailedSharedTaxi,
    PlusBus,
    Footpath,
    RapidTransit,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub enum StatusCode {
    Ok,
    ValidationError,
    ServiceError,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Status {
    code: StatusCode,
    message: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DvbResponse<T: Debug> {
    pub status: Status,
    // TODO: parse this
    pub expiration_time: Option<String>,

    #[serde(flatten)]
    content: T,
}

impl<T: Debug> AsRef<T> for DvbResponse<T> {
    fn as_ref(&self) -> &T {
        &self.content
    }
}

impl<T: Debug> Deref for DvbResponse<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl<T: Debug> DvbResponse<T> {
    pub fn into_inner(self) -> T {
        self.content
    }
}
