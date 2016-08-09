use hyper::client::Client;
use json::{parse, JsonValue};

use std::io::Read;

use error::*;

/// Commonalities of all API endpoints.
pub trait APIEndPoint {
    fn url(&self) -> String;

    /// Returns the content of the API endpoints response.
    fn get_raw(&self) -> Result<String> {
        let client = Client::new();
        let mut res = try!(client.get(&self.url()).send());
        let mut res_content = String::new();
        try!(res.read_to_string(&mut res_content));
        Ok(res_content)
    }

    /// Returns the content of the API endpoints response parsed as `JsonValue`.
    fn get(&self) -> Result<JsonValue> {
        self.get_raw().and_then(|r| parse(&r).map_err(Into::into))
    }
}
