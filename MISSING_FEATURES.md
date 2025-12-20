# Missing Features and Enhancement Roadmap

This document tracks features and improvements that are not yet implemented in `dvb-rs` but are documented in the VVO WebAPI specification or would enhance the library's functionality.

**Last Updated:** 2025-01-20  
**Specification Reference:** https://raw.githubusercontent.com/kiliankoe/vvo/refs/heads/main/documentation/webapi.md

---

## High Priority Missing Features

### 1. Route Changes API (Not Implemented)

**Endpoint:** `https://webapi.vvo-online.de/rc`  
**Method:** POST  
**Priority:** High  
**Complexity:** Low

**Description:**  
Get information about route changes, service disruptions, construction work, and schedule modifications. This is valuable for real-time transit applications to inform users about service alerts.

**Request Parameters:**
- `shortterm` (Bool) - Filter for short-term changes

**Response Structure:**
```json
{
  "Changes": [
    {
      "Id": "511595",
      "Title": "Dresden - Construction work",
      "Description": "<HTML description>",
      "Type": "Scheduled" | "Unscheduled",
      "PublishDate": "/Date(timestamp)/",
      "LineIds": ["428296"],
      "ValidityPeriods": [
        {
          "Begin": "/Date(timestamp)/",
          "End": "/Date(timestamp)/"
        }
      ]
    }
  ],
  "Status": { "Code": "Ok" }
}
```

**Implementation Suggestion:**  
Create `src/route_changes.rs` with:
- `RouteChanges` struct containing list of changes
- `RouteChange` struct with all fields
- `ChangeType` enum (Scheduled, Unscheduled)
- `ValidityPeriod` struct with begin/end times
- `route_changes(shortterm: bool)` async function

**Estimated Effort:** 2-3 hours

---

### 2. Missing Fields in Departure Monitor

**Location:** `src/monitor.rs`  
**Priority:** High  
**Complexity:** Low

**Missing Fields in `Departure` struct:**
- `scheduled_time: Option<DvbTime>` - The originally scheduled departure time
- `platform: Option<Platform>` - Platform/track information
- `diva: Option<Diva>` - DIVA system identifiers

**Current State:**  
Fields are commented out in the code:
```rust
pub struct Departure {
    pub id: String,
    pub line_name: String,
    pub direction: String,
    // platform  <- commented out
    pub mot: Mot,
    pub real_time: Option<DvbTime>,
    pub state: Option<ArrivalState>,
    pub route_changes: Option<Vec<String>>,
    // diva      <- commented out
}
```

**Required Structs:**
```rust
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Platform {
    pub name: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Diva {
    pub network: String,
    pub number: String,
}
```

**Estimated Effort:** 1 hour

---

### 3. Advanced Route Planning Parameters

**Location:** `src/route.rs`  
**Priority:** Medium  
**Complexity:** Medium

**Missing Parameter Structures:**

According to the specification, the route planning endpoint accepts complex nested parameters:

```rust
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MobilitySettings {
    pub mobility_restriction: MobilityRestriction,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum MobilityRestriction {
    None,
    Wheelchair,
    // Add other variants as needed
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandardSettings {
    pub footpath_to_stop: Option<u32>,
    pub include_alternative_stops: Option<bool>,
    pub max_changes: Option<MaxChanges>,
    pub mot: Option<Vec<Mot>>,
    pub walking_speed: Option<WalkingSpeed>,
}

#[derive(Serialize, Debug)]
pub enum MaxChanges {
    Unlimited,
    Limited(u32),
}

#[derive(Serialize, Debug)]
pub enum WalkingSpeed {
    Slow,
    Normal,
    Fast,
}
```

**Update `Params` struct:**
```rust
pub struct Params<'a> {
    pub origin: &'a str,
    pub destination: &'a str,
    pub time: DvbTime,
    pub isarrivaltime: bool,
    pub shorttermchanges: bool,
    pub mobility_settings: Option<MobilitySettings>,
    pub standard_settings: Option<StandardSettings>,
    pub via: Option<&'a str>,
}
```

**Estimated Effort:** 3-4 hours

---

## Medium Priority Missing Features

### 4. Additional PointFinder Parameters

**Location:** `src/point.rs`  
**Priority:** Medium  
**Complexity:** Low

**Missing Parameters:**
- `regionalOnly` (Bool) - Include only stops in VVO area if true
- `stopShortcuts` (Bool) - Include stop shortcuts if true

**Current Params:**
```rust
pub struct Params<'a> {
    pub query: &'a str,
    pub limit: Option<u32>,
    pub stops_only: bool,
    pub assigedstops: bool,  // Note: typo in parameter name
    pub dvb: bool,
    pub format: Format,
}
```

**Should Add:**
```rust
pub struct Params<'a> {
    pub query: &'a str,
    pub limit: Option<u32>,
    pub stops_only: bool,
    pub assigedstops: bool,
    pub regional_only: Option<bool>,     // NEW
    pub stop_shortcuts: Option<bool>,    // NEW
    pub dvb: bool,
    pub format: Format,
}
```

**Estimated Effort:** 30 minutes

---

### 5. Fix Type Issue in Trip Details

**Location:** `src/trip.rs`  
**Priority:** Medium  
**Complexity:** Low

**Issue:**  
The `scheduled_time` field in `Stop` struct has incorrect type:

```rust
pub struct Stop {
    // ... other fields
    pub scheduled_time: Option<bool>,  // ❌ WRONG
    pub time: DvbTime,
}
```

**Should Be:**
```rust
pub struct Stop {
    // ... other fields
    pub scheduled_time: Option<DvbTime>,  // ✅ CORRECT
    pub time: DvbTime,
}
```

**Estimated Effort:** 5 minutes (+ testing)

---

## Low Priority Missing Features

### 6. Parameter Name Typo Fix

**Location:** `src/point.rs`  
**Priority:** Low (Breaking Change)  
**Complexity:** Low

**Issue:**  
Parameter `assigedstops` should be `assignedstops`

**Migration Path:**
1. Add `assigned_stops` field with correct spelling
2. Deprecate `assigedstops` with `#[deprecated]` attribute
3. Make both fields work (use `assigned_stops` if set, otherwise `assigedstops`)
4. Remove deprecated field in next major version

**Estimated Effort:** 30 minutes + deprecation period

---

### 7. Enhanced Error Handling

**Location:** `src/error.rs`  
**Priority:** Low  
**Complexity:** Medium

**Current State:**  
The error handling doesn't distinguish between API-level errors and transport errors.

**Enhancement:**  
Add specific error variants for API status codes:

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("API returned an error: {code} - {message}")]
    ApiError {
        code: String,
        message: Option<String>,
    },
    
    #[error("Invalid request: {0}")]
    ValidationError(String),
    
    #[error("Service unavailable")]
    ServiceError,
    
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),
}
```

**Benefits:**
- Better error messages for users
- Ability to retry on specific errors
- Distinguish between client and server errors

**Estimated Effort:** 2-3 hours

---

### 8. Convenience Methods for Common Use Cases

**Priority:** Low  
**Complexity:** Low-Medium

**Suggestions:**

#### Departure Monitor Extensions
```rust
impl DepartureMonitor {
    /// Get all departures for a specific line
    pub fn departures_for_line(&self, line_name: &str) -> Vec<&Departure> { ... }
    
    /// Get departures within next N minutes
    pub fn departures_within(&self, minutes: u32) -> Vec<&Departure> { ... }
    
    /// Check if a specific line is delayed
    pub fn is_line_delayed(&self, line_name: &str) -> bool { ... }
}
```

#### Departure Extensions
```rust
impl Departure {
    /// Calculate delay in minutes
    pub fn delay_minutes(&self) -> Option<i64> { ... }
    
    /// Is this departure delayed?
    pub fn is_delayed(&self) -> bool { ... }
    
    /// Get departure time (real or scheduled)
    pub fn effective_time(&self) -> Option<&DvbTime> { ... }
}
```

**Estimated Effort:** 2-4 hours

---

## Undocumented/Speculative Features

These endpoints are mentioned but not documented in the specification:

### 9. Map Pins API

**Endpoint:** `https://webapi.vvo-online.de/map/pins`  
**Status:** Mentioned but not documented  
**Priority:** Low (requires reverse engineering)

### 10. Handy Ticket API

**Endpoint:** `https://webapi.vvo-online.de/tr/handyticket`  
**Status:** Mentioned but not documented  
**Priority:** Low (may require authentication)

---

## Testing and Quality Improvements

### 11. Integration Tests

**Priority:** High  
**Complexity:** Medium

**Current State:**  
Limited testing against real API endpoints.

**Needed:**
- Integration tests that make real API calls
- Mock server tests with recorded responses
- Regression tests for breaking changes
- Performance benchmarks

**Estimated Effort:** 1-2 days

---

### 12. Documentation Improvements

**Priority:** Medium  
**Complexity:** Low

**Needed:**
- More usage examples in documentation
- API rate limiting guidelines
- Best practices guide
- Migration guides for breaking changes
- Comparison with other transit APIs

**Estimated Effort:** 4-6 hours

---

## Summary

| Priority | Feature | Complexity | Estimated Effort |
|----------|---------|------------|------------------|
| High | Route Changes API | Low | 2-3 hours |
| High | Missing Departure Fields | Low | 1 hour |
| Medium | Advanced Route Planning | Medium | 3-4 hours |
| Medium | Additional PointFinder Params | Low | 30 minutes |
| Medium | Fix Trip Stop Type | Low | 5 minutes |
| Low | Parameter Name Typo | Low | 30 minutes |
| Low | Enhanced Error Handling | Medium | 2-3 hours |
| Low | Convenience Methods | Low-Medium | 2-4 hours |
| High | Integration Tests | Medium | 1-2 days |
| Medium | Documentation | Low | 4-6 hours |

**Total Estimated Effort:** ~3-4 days of focused development

---

## Contributing

If you'd like to contribute any of these features:

1. Check existing issues and PRs to avoid duplicate work
2. Open an issue to discuss the approach before major changes
3. Follow the existing code style and patterns
4. Add tests for new functionality
5. Update documentation and examples
6. Consider backward compatibility

For breaking changes, please coordinate with maintainers to plan for next major version release.

---

## Notes

- Some features may require validation against the real API before implementation
- Priorities may shift based on user feedback and use cases
- Complexity estimates are approximate and may vary based on developer experience
- Test the HTTP method fixes (route planning and lines) before implementing other features