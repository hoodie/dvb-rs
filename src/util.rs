use json::JsonValue;
use super::error;

pub trait EasyJson {
    fn access<'a>(&'a self, path: &'a str) -> Option<&'a JsonValue>;

    /// Wrapper around `get_path()`.
    ///
    /// Splits path string
    /// and replaces `JsonValue::Null` and `JsonValue::BadValue`.
    fn get<'a>(json: &'a JsonValue, key: &str) -> Option<&'a JsonValue> {
        match Self::get_path(json,
                             &key.split('/').filter(|k| !k.is_empty()).collect::<Vec<&str>>()) {
            Some(&JsonValue::Null) => None,
            content => content,
        }
    }


    /// Returns content at `path` in the json document.
    ///
    /// don't worry, I ripped this off from myself: https://github.com/hoodie/asciii-rs/blob/master/src/util/yaml.rs
    /// literally ported this from yaml to json with regex
    ///
    fn get_path<'a>(json: &'a JsonValue, path: &[&str]) -> Option<&'a JsonValue> {
        if let Some((&key, remainder)) = path.split_first() {

            return match *json {
                JsonValue::Object(ref hash) => {
                    if remainder.is_empty() {
                        hash.get(key)
                    } else {
                        hash.get(key)
                            .and_then(|c| Self::get_path(c, remainder))
                    }
                }

                JsonValue::Array(ref vec) => {
                    if let Ok(index) = key.parse::<usize>() {
                        if remainder.is_empty() {
                            vec.get(index)
                        } else {
                            vec.get(index).and_then(|c| Self::get_path(c, remainder))
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            };

        }
        None
    }
}

impl EasyJson for error::Result<JsonValue> {
    fn access<'a>(&'a self, path: &'a str) -> Option<&'a JsonValue> {
        match *self {
            Ok(ref j) => Self::get(j, path),
            Err(ref _e) => {
                // err!(_e);
                None
            }
        }
    }
}
