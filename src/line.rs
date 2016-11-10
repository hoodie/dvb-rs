#![allow(dead_code)]
use json::JsonValue;
use regex::Regex;

use std::time::Duration;

type Direction = String;

#[derive(Eq, PartialEq, Debug)]
pub struct Departure {
    pub line: Line,
    pub direction: Direction,
    pub eta: Duration,
}

impl Departure {
    pub fn from_json(stop: &JsonValue) -> Option<Self> {
        match stop {
            &JsonValue::Array(ref a) => {
                match (a[0].as_str(),
                       a[1].as_str(),
                       a[2].as_str().and_then(|s| s.parse::<u64>().ok())) {
                    (Some(line), Some(dir), Some(time)) => {
                        Some(// (line.to_string(),(dir.to_string(),time))
                            Departure {
                            line: Line::from(line),
                            direction: dir.to_string(),
                            eta: Duration::from_secs(time * 60),
                        })
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub enum TransportMode {
    Tram,
    Bus,
    Regionalbus,
    Cablecar,
    Ferry,
    Train,
    CityTrain,
    OnCallBus,
    Unknown
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub struct Line {
    pub identifier: String, // "61"
    pub mode: TransportMode,
}

impl<T: ToString> From<T> for Line {
    fn from(s: T) -> Self {
        Line {
            identifier: s.to_string(),
            mode: TransportMode::from(s)
        }
    }
}

lazy_static!{
    static ref ALITA: Regex     = Regex::new(r#"alita"#).unwrap();
    static ref TRAM: Regex      = Regex::new(r#"^E(1)?[1-9]"#).unwrap();
    static ref BUS: Regex       = Regex::new(r#"^E(V[1-9]|[6-9]\d)\d*"#).unwrap();
    ////static ref TRAM: Regex      = Regex::new("E").unwrap();
    static ref FERRY: Regex     = Regex::new("^F").unwrap();
    static ref TRAIN: Regex     = Regex::new(r#"^RE|^IC|^TL|^RB|^SB|^SE|^U\d"#).unwrap();
    static ref REGIOBUS: Regex  = Regex::new(r#"\D|\D/\D"#).unwrap();
    static ref CITYTRAIN: Regex = Regex::new("^S").unwrap();
}

impl<T: ToString> From<T> for TransportMode{
    fn from(s: T) -> Self {
        let s = s.to_string();
        let num = s.parse::<u32>();
        match (s, num) {

            (_, Ok(  0... 60)) => TransportMode::Tram,
            (_, Ok( 61... 99)) => TransportMode::Bus,
            (_, Ok(100...1000)) => TransportMode::Regionalbus,

            (s @ _, _)    => {
                if ALITA.is_match(&s)         { TransportMode::OnCallBus }
                else if BUS.is_match(&s)      { TransportMode::Bus }
                else if TRAM.is_match(&s)     { TransportMode::Tram }
                else if TRAIN.is_match(&s)    { TransportMode::Train }
                else if "E" == &s { TransportMode::Tram }
                else if "Seil-/Schwebebahn" == &s { TransportMode::Cablecar }
                else if "SWB" == &s { TransportMode::Cablecar }
                else if FERRY.is_match(&s) { TransportMode::Ferry }
                else if CITYTRAIN.is_match(&s) { TransportMode::CityTrain }
                else if REGIOBUS.is_match(&s) { TransportMode::Regionalbus}
                else { TransportMode::Unknown }
            }
        }
        //match String::from(s) {
        //    identifier: s.to_string(),
        //    mode: TransportMode::Tram
        //}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_identify_correct_values_as_tram() {
        assert_eq!(Line::from("3").mode,    TransportMode::Tram);
        assert_eq!(Line::from("11").mode,   TransportMode::Tram);
        assert_eq!(Line::from("59").mode,   TransportMode::Tram);
        assert_eq!(Line::from("E8").mode,   TransportMode::Tram);
        assert_eq!(Line::from("E11").mode,  TransportMode::Tram);
        assert_eq!(Line::from("E").mode,    TransportMode::Tram);
    }

    #[test]
    fn it_should_identify_correct_values_as_bus() {
        assert_eq!(Line::from("85").mode,   TransportMode::Bus);
        assert_eq!(Line::from("99").mode,   TransportMode::Bus);
        assert_eq!(Line::from("EV2").mode,  TransportMode::Bus);
        assert_eq!(Line::from("E75").mode,  TransportMode::Bus);
    }

    #[test]
    fn it_should_identify_correct_values_as_regionalbus() {
        assert_eq!(Line::from("366").mode, TransportMode::Regionalbus);
        assert_eq!(Line::from("999").mode, TransportMode::Regionalbus);
        assert_eq!(Line::from("A").mode,   TransportMode::Regionalbus);
        assert_eq!(Line::from("Z").mode,   TransportMode::Regionalbus);
        assert_eq!(Line::from("G/L").mode, TransportMode::Regionalbus);
        assert_eq!(Line::from("H/S").mode, TransportMode::Regionalbus);
    }

    #[test]
    fn it_should_identify_correct_values_as_cablecar() {
        assert_eq!(Line::from("SWB").mode, TransportMode::Cablecar)
    }

    #[test]
    fn it_should_identify_correct_values_as_ferry() {
        assert_eq!(Line::from("F7").mode,  TransportMode::Ferry);
        assert_eq!(Line::from("F14").mode, TransportMode::Ferry);
    }

    #[test]
    fn it_should_identify_correct_values_as_train() {
        assert_eq!(Line::from("ICE 1717").mode, TransportMode::Train);
        assert_eq!(Line::from("IC 1818").mode,  TransportMode::Train);
        assert_eq!(Line::from("RB 1919").mode,  TransportMode::Train);
        assert_eq!(Line::from("TLX 2020").mode, TransportMode::Train);
        assert_eq!(Line::from("SB33").mode,     TransportMode::Train); // "Sächsische Städtebahn"
        assert_eq!(Line::from("SE19").mode,     TransportMode::Train); // "Wintersport Express" o              . O
        assert_eq!(Line::from("U28").mode,      TransportMode::Train); // fares between Bad Schandau and Děčín
    }

    #[test]
    fn it_should_identify_correct_values_as_citytrain() {
        assert_eq!(Line::from("S3").mode,     TransportMode::CityTrain);
        assert_eq!(Line::from("S 2121").mode, TransportMode::CityTrain);
    }

    #[test]
    fn it_should_identify_correct_values_as_oncallbus() {
        assert_eq!(Line::from("alita").mode,    TransportMode::OnCallBus);
        assert_eq!(Line::from("alita 95").mode, TransportMode::OnCallBus);
    }

    #[test]
    fn it_should_fail_with_nil() {
        //expect(TransportMode.Departure(line: "Lorem Ipsum")).to(beNil())
    }

}
