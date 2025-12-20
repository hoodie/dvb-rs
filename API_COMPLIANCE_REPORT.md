# VVO WebAPI Compliance Report for dvb-rs

**Generated:** 2025-01-20  
**Specification Reference:** https://raw.githubusercontent.com/kiliankoe/vvo/refs/heads/main/documentation/webapi.md

## Executive Summary

This report evaluates the `dvb-rs` crate's compliance with the VVO WebAPI specification. The crate implements **5 out of 6** documented endpoints with varying degrees of completeness.

**Overall Compliance: ~75%**

### Critical Finding

The specification indicates that two endpoints should use POST, but the current implementation uses GET:
- **Route Planning API** (`/tr/trips`) - Specification says POST, implementation uses GET
- **Lines API** (`/stt/lines`) - Specification says POST, implementation uses GET

**However**, preliminary testing suggests the specification may be incorrect or outdated. Test examples have been created to verify which methods actually work in production.

### Related Documents

- **[MISSING_FEATURES.md](MISSING_FEATURES.md)** - Detailed roadmap of features not yet implemented
- **[examples/route_test_methods.rs](examples/route_test_methods.rs)** - Test tool for Route Planning API
- **[examples/lines_test_methods.rs](examples/lines_test_methods.rs)** - Test tool for Lines API

---

## Endpoint Implementation Status

### ✅ 1. PointFinder API
**Status:** ✅ IMPLEMENTED  
**Endpoint:** `https://webapi.vvo-online.de/tr/pointfinder`  
**Location:** `src/point.rs`  
**HTTP Method:** ✅ POST (Correct)

#### Compliance Details:
- ✅ Correct URL and HTTP method
- ✅ JSON request body with camelCase serialization
- ✅ Core parameters: `query`, `limit`, `stopsOnly`, `assigedstops`
- ✅ Response parsing with custom Point deserializer
- ✅ Handles coordinate queries (`coord:[right]:[up]`)
- ❌ Missing parameters: `regionalOnly`, `stopShortcuts`
- ⚠️  Parameter typo: `assigedstops` (should be `assignedstops`)

#### Recommendations:
1. Add missing `regionalOnly` and `stopShortcuts` parameters
2. Fix typo: `assigedstops` → `assignedstops` (breaking change)
3. Consider deprecation path for parameter rename

---

### ✅ 2. Departure Monitor API
**Status:** ✅ IMPLEMENTED  
**Endpoint:** `https://webapi.vvo-online.de/dm`  
**Location:** `src/monitor.rs`  
**HTTP Method:** ✅ POST (Correct)

#### Compliance Details:
- ✅ Correct URL and HTTP method
- ✅ Core parameters: `stopid`, `limit`, `time`, `isarrival`, `shorttermchanges`, `mot`
- ✅ Response structure with `DepartureMonitor` and `Departure` types
- ✅ Mode of transport filtering with `Mot` enum
- ⚠️  Missing fields in `Departure`: `ScheduledTime`, `Platform`, `Diva`
- ⚠️  Some fields commented out (platform, diva)

#### Recommendations:
1. Add missing `Platform` struct and field to `Departure`
2. Add missing `Diva` struct and field to `Departure`
3. Add `scheduled_time: Option<DvbTime>` field to `Departure`
4. Consider adding convenience methods for delay calculation

---

### ✅ 3. Trip Details API
**Status:** ✅ IMPLEMENTED  
**Endpoint:** `https://webapi.vvo-online.de/dm/trip`  
**Location:** `src/trip.rs`  
**HTTP Method:** ✅ POST (Correct)

#### Compliance Details:
- ✅ Correct URL and HTTP method
- ✅ Core parameters: `tripid`, `time`, `stopid`, `mapdata`
- ✅ Response structure with `Trip` and `Stop` types
- ✅ Position enum (Previous, Current, Next)
- ✅ Platform information included
- ⚠️  Field `scheduled_time: Option<bool>` seems incorrect (should be `Option<DvbTime>`)

#### Recommendations:
1. Fix `scheduled_time` type in `Stop` struct (currently `Option<bool>`, should be `Option<DvbTime>`)
2. Verify coordinate fields match specification (latitude/longitude vs. right/up in GK4)

---

### ⚠️ 4. Route Planning (Trip Query) API
**Status:** ⚠️ METHOD NEEDS VERIFICATION  
**Endpoint:** `https://webapi.vvo-online.de/tr/trips`  
**Location:** `src/route.rs`  
**HTTP Method:** ⚠️ GET (Specification says POST)

#### Issues Requiring Verification:
- ⚠️ **Uses GET instead of POST** (lines 184, 203)
- ⚠️ **Uses `.query(&params)` instead of `.json(&params)`**
- ⚠️ **Specification may be incorrect** - needs real-world testing
- ⚠️ Run `cargo run --example route_test_methods` to verify

#### Compliance Details:
- ❌ Wrong HTTP method (GET vs POST)
- ✅ Core parameters present: `origin`, `destination`, `time`, `isarrivaltime`, `shorttermchanges`
- ⚠️  Added parameters not in spec: `format`, `via` (marked with TODO comments)
- ❌ Missing complex parameters: `mobilitySettings`, `standardSettings`
- ⚠️  Comprehensive response types but missing some optional fields

#### Specification Requirements:
According to the spec, the request should be:
```json
{
  "destination": "33000016",
  "isarrivaltime": false,
  "mobilitySettings": {
    "mobilityRestriction": "None"
  },
  "origin": "33000028",
  "shorttermchanges": true,
  "standardSettings": {
    "footpathToStop": 5,
    "includeAlternativeStops": true,
    "maxChanges": "Unlimited",
    "mot": ["Tram", "CityBus", ...],
    "walkingSpeed": "Normal"
  },
  "time": "2017-12-08T21:36:42.775Z"
}
```

#### Testing Required:

**Run the test example to determine correct method:**
```bash
cargo run --example route_test_methods
```

This will test both GET and POST methods and provide clear output on which works.

#### Potential Changes (if POST is correct):
1. Change from GET to POST method
2. Use `.json(&params)` instead of `.query(&params)`
3. Add `MobilitySettings` and `StandardSettings` structs (see MISSING_FEATURES.md)

#### Potential Changes (if GET is correct):
1. Keep current implementation
2. Document that specification is incorrect
3. Submit PR to kiliankoe/vvo repository to fix documentation

---

### ⚠️ 5. Lines API
**Status:** ⚠️ METHOD NEEDS VERIFICATION  
**Endpoint:** `https://webapi.vvo-online.de/stt/lines`  
**Location:** `src/lines.rs`  
**HTTP Method:** ⚠️ GET (Specification says POST)

#### Issues Requiring Verification:
- ⚠️ **Uses GET with query parameters instead of POST with JSON body** (line 70-71)
- ⚠️ **Preliminary testing suggests GET works, POST fails!**
- ⚠️ Run `cargo run --example lines_test_methods` to verify

#### Compliance Details:
- ❌ Wrong HTTP method (GET vs POST)
- ✅ Response structure appears correct
- ✅ Comprehensive type definitions for `Line`, `Direction`, `TimeTable`
- ✅ Includes `Diva` information

#### Specification Requirements:
According to the spec, the request should be:
```bash
curl -X "POST" "https://webapi.vvo-online.de/stt/lines" \
     -H 'Content-Type: application/json; charset=utf-8' \
     -d $'{ "stopid": "33000293" }'
```

Current implementation:
```rust
.get(LINES_URL)
.query(&[("format", "json"), ("stopid", stop_id)])
```

#### Testing Required:

**Run the test example to determine correct method:**
```bash
cargo run --example lines_test_methods
```

This will test both GET and POST methods and provide clear output on which works.

**Note:** Preliminary manual testing showed:
- ✅ GET with query parameters: **Works** (Status: Ok)
- ❌ POST with JSON body: **Fails** (Status: "Connection error")

This suggests the specification may be incorrect and the current implementation is correct.

#### Recommended Action:
1. Run comprehensive tests with the provided example
2. If GET continues to work and POST fails, keep current implementation
3. Document findings and submit issue to kiliankoe/vvo repository

---

### ❌ 6. Route Changes API
**Status:** ❌ NOT IMPLEMENTED  
**Endpoint:** `https://webapi.vvo-online.de/rc`  
**Location:** N/A

#### Missing Implementation:
The route changes endpoint provides information about service disruptions, construction work, and schedule changes. This is a valuable feature for real-time transit applications.

#### Specification Requirements:
```bash
curl -X "POST" "https://webapi.vvo-online.de/rc" \
     -H 'Content-Type: application/json; charset=utf-8' \
     -d $'{ "shortterm": true }'
```

**Response Structure:**
```json
{
  "Changes": [
    {
      "Description": "<HTML description>",
      "Id": "511595",
      "LineIds": ["428296"],
      "PublishDate": "/Date(1512400560000+0100)/",
      "Title": "Dresden - Construction work",
      "Type": "Scheduled",
      "ValidityPeriods": [
        {
          "Begin": "/Date(1512529200000+0100)/",
          "End": "/Date(1512788400000+0100)/"
        }
      ]
    }
  ],
  "Status": { "Code": "Ok" },
  "ExpirationTime": "/Date(1512565171371+0100)/"
}
```

#### Implementation Recommendation:
Create `src/route_changes.rs` with the following structure:

```rust
use crate::{DvbResponse, error::Result, time::DvbTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RouteChanges {
    #[serde(default)]
    pub changes: Vec<RouteChange>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RouteChange {
    pub id: String,
    pub title: String,
    pub description: String,
    pub r#type: ChangeType,
    pub publish_date: DvbTime,
    #[serde(default)]
    pub line_ids: Vec<String>,
    #[serde(default)]
    pub validity_periods: Vec<ValidityPeriod>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChangeType {
    Scheduled,
    Unscheduled,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ValidityPeriod {
    pub begin: DvbTime,
    pub end: DvbTime,
}

const ROUTE_CHANGES_URL: &str = "https://webapi.vvo-online.de/rc";

#[derive(Serialize, Debug)]
pub struct Params {
    pub shortterm: bool,
}

pub async fn route_changes(shortterm: bool) -> Result<DvbResponse<RouteChanges>> {
    let response = reqwest::Client::new()
        .post(ROUTE_CHANGES_URL)
        .json(&Params { shortterm })
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
```

---

## Additional Observations

### 1. Mode of Transport (Mot) Enum
**Location:** `src/common.rs`

The specification lists these modes:
- Tram ✅
- CityBus ✅
- IntercityBus ✅
- SuburbanRailway ✅
- Train ✅
- Cableway ✅
- Ferry ✅
- HailedSharedTaxi ✅

Additional modes in implementation:
- PlusBus ✅ (regional express bus service)
- Footpath ✅ (walking segments)
- RapidTransit ✅ (S-Bahn)
- Bus (generic, in addition to CityBus)

**Status:** Good - includes all specified modes plus reasonable extensions

---

### 2. Date/Time Handling
**Location:** `src/time.rs`

- ✅ Custom `DvbTime` type handles Microsoft JSON date format (`/Date(timestamp+timezone)/`)
- ✅ Optional ISO8601 serialization via feature flag
- ✅ Proper deserialization from DVB format
- ✅ Integration with `chrono` crate

**Status:** Excellent implementation

---

### 3. Error Handling
**Location:** `src/error.rs`

- ✅ Custom `Result` type
- ✅ Wraps reqwest and serde_json errors
- ⚠️  Could expose API-level errors (Status.Code != "Ok")

**Recommendation:** Add API error variants for non-Ok status codes

---

### 4. Response Wrapper
**Location:** `src/common.rs`

The `DvbResponse<T>` wrapper correctly implements:
- ✅ Status code checking
- ✅ Optional expiration time
- ✅ Flattened content with `#[serde(flatten)]`
- ✅ Deref trait for ergonomic access
- ⚠️  Status message field present but private

---

### 5. Point Types (POI Handling)
**Location:** `src/poi.rs`

- ✅ Handles different point ID types: Stop, Address, Coords, POI
- ✅ Custom parsing for complex ID formats (streetID, poiID, coord)
- ✅ Type-safe enum for point types

**Status:** Good implementation

---

## Undocumented Endpoints

The specification mentions additional endpoints that are not yet documented:

### 1. Map Pins
- **Endpoint:** `https://webapi.vvo-online.de/map/pins`
- **Purpose:** Map markers for stops/POIs
- **Status:** Not implemented (no specification available)

### 2. Handy Ticket
- **Endpoint:** `https://webapi.vvo-online.de/tr/handyticket`
- **Purpose:** Mobile ticketing
- **Status:** Not implemented (no specification available)

These are marked as "Not Yet Documented" in the specification and require further investigation.

---

## Priority Issues Summary

### 🔴 High Priority (Requires Immediate Attention)

1. **Verify HTTP Methods for Route Planning and Lines APIs**
   - Specification says POST, implementation uses GET
   - Preliminary testing suggests specification may be incorrect
   - **ACTION:** Run test examples to determine correct methods
   - **Tools:** `examples/route_test_methods.rs` and `examples/lines_test_methods.rs`

2. **Fix Type Error in Trip Details** (`src/trip.rs`)
   - `Stop.scheduled_time` is `Option<bool>`, should be `Option<DvbTime>`
   - This is a clear bug that should be fixed

### 🟡 Medium Priority (Feature Gaps)

3. **Route Changes API not implemented**
   - See [MISSING_FEATURES.md](MISSING_FEATURES.md) for implementation details

4. **Missing Departure Monitor fields** (`src/monitor.rs`)
   - `scheduled_time`, `platform`, `diva` fields commented out
   - See [MISSING_FEATURES.md](MISSING_FEATURES.md) for details

5. **Advanced Route Planning parameters**
   - `mobilitySettings` and `standardSettings` not implemented
   - See [MISSING_FEATURES.md](MISSING_FEATURES.md) for complete spec

### 🟢 Low Priority (Polish and Refinements)

See [MISSING_FEATURES.md](MISSING_FEATURES.md) for complete list including:
- Additional PointFinder parameters
- Parameter name typo fix
- Enhanced error handling
- Convenience methods
- Documentation improvements

---

## Testing Recommendations

### Immediate: Verify HTTP Methods

Run the provided test examples to determine correct API methods:

```bash
# Test Route Planning API (GET vs POST)
cargo run --example route_test_methods

# Test Lines API (GET vs POST)  
cargo run --example lines_test_methods
```

These examples will:
- Test both HTTP methods against the real API
- Show clear success/failure for each method
- Provide recommendations on which method to use
- Suggest documentation updates if specification is incorrect

### Future: Comprehensive Testing

1. **Integration Tests:** Tests against real API endpoints
2. **Mock Tests:** Unit tests with recorded responses  
3. **Compliance Tests:** Verify all parameters and response fields

See [MISSING_FEATURES.md](MISSING_FEATURES.md) section 11 for details.

---

## Recommended Action Plan

### Step 1: Verify Specification (Immediate)

1. Run `cargo run --example route_test_methods` to test Route Planning API
2. Run `cargo run --example lines_test_methods` to test Lines API
3. Document findings
4. If specification is incorrect, submit issue/PR to kiliankoe/vvo repository

### Step 2: Quick Fixes (Next Release)

1. Fix `Stop.scheduled_time` type (`Option<bool>` → `Option<DvbTime>`)
2. Update any HTTP methods if testing proves they need changing
3. Add missing Departure Monitor fields (platform, diva, scheduled_time)

### Step 3: Feature Additions (Future Releases)

See [MISSING_FEATURES.md](MISSING_FEATURES.md) for prioritized roadmap including:
- Route Changes API implementation
- Advanced route planning parameters
- Enhanced error handling
- Convenience methods
- Comprehensive testing

---

## Conclusion

The `dvb-rs` crate provides a solid foundation for accessing the VVO WebAPI with good type safety and error handling. The most critical finding is that **the specification may contain errors** regarding HTTP methods for two endpoints.

### Strengths:
- ✅ Well-structured, idiomatic Rust code
- ✅ Good type safety with custom deserializers
- ✅ Excellent date/time handling with optional ISO8601 serialization
- ✅ Most core endpoints implemented (5 out of 6)
- ✅ Appears to work correctly despite specification discrepancies

### Critical Actions Required:
1. **Test HTTP methods** using provided examples (`route_test_methods`, `lines_test_methods`)
2. **Document findings** and update specification if incorrect
3. **Fix type bug** in `Stop.scheduled_time` field

### Future Enhancements:
- Add Route Changes API (important for service alerts)
- Complete missing fields in existing types
- Add advanced route planning parameters
- Implement comprehensive testing

See **[MISSING_FEATURES.md](MISSING_FEATURES.md)** for detailed implementation roadmap.

---

**Report Author:** AI Code Analysis  
**Last Updated:** 2025-01-20  
**Specification Version:** As of 2025-01-20 from kiliankoe/vvo repository
