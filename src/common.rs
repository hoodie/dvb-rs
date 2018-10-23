

#[derive(Serialize, Deserialize, Debug)]
pub enum ArrivalState {
    Delayed,
    InTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Mot {
    Tram,
    CityBus,
    IntercityBus,
    SuburbanRailway,
    Train,
    Cableway,
    Ferry,
    HailedSharedTaxi,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StatusCode {
    Ok,
    ValidationError,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Status {
    code: StatusCode,
    message: Option<String>,
}
