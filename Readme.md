# Dvb API library in Rust

An unofficial Rust library for querying Dresden's public transport system API.

## Features

### `iso8601-serialization`

By default, `DvbTime` deserializes from the DVB API's custom `/Date(...)` format and serializes back to the same format.

Enable the `iso8601-serialization` feature to serialize `DvbTime` as ISO8601/RFC3339 format instead, while still deserializing from the DVB format:

```toml
[dependencies]
dvb = { version = "0.7", features = ["iso8601-serialization"] }
```

This is useful when you need to:
- Receive data from the DVB API (deserializes `/Date(...)` format)
- Pass that data forward in a more standard format (serializes as ISO8601)

**Without the feature:**
```rust
let time = DvbTime::from_str("/Date(1609459200000+0100)/").unwrap();
let json = serde_json::to_string(&time).unwrap();
// json = "\"/Date(1609459200000+0100)/\""
```

**With the feature:**
```rust
let time = DvbTime::from_str("/Date(1609459200000+0100)/").unwrap();
let json = serde_json::to_string(&time).unwrap();
// json = "\"2021-01-01T00:00:00+01:00\""
```
