# Testing HTTP Methods for VVO WebAPI Endpoints

This document explains how to verify which HTTP methods (GET vs POST) work correctly for the VVO WebAPI endpoints.

## Background

The VVO WebAPI specification states that certain endpoints should use POST requests with JSON bodies. However, the current `dvb-rs` implementation uses GET requests with query parameters for some of these endpoints, and preliminary testing suggests the specification may be incorrect.

**Endpoints in Question:**
1. **Route Planning API** (`/tr/trips`) - Spec says POST, implementation uses GET
2. **Lines API** (`/stt/lines`) - Spec says POST, implementation uses GET

## Quick Start

Run these test examples to verify which HTTP methods actually work:

```bash
# Test Route Planning API
cargo run --example route_test_methods

# Test Lines API
cargo run --example lines_test_methods

# Test with custom stops
cargo run --example route_test_methods "Hauptbahnhof" "Postplatz"
cargo run --example lines_test_methods "Albertplatz"
```

## What the Tests Do

Each test example will:

1. **Find the specified stop(s)** using the PointFinder API
2. **Test GET method** - Current implementation approach
3. **Test POST method** - Specification's documented approach
4. **Compare results** - Show which method works
5. **Provide recommendations** - Suggest code changes if needed

## Expected Output

### Successful Response
```
✅ GET Method Response:
  Status Code: Ok
  Number of routes: 5

  ✅ GET METHOD WORKS!
```

### Failed Response
```
❌ POST Method Response:
  Status Code: ServiceError
  Message: Connection error
  Number of routes: 0

  ❌ POST METHOD FAILED: ServiceError
```

## Interpreting Results

### If GET works and POST fails:
- ✅ Current implementation is **CORRECT**
- ❌ Specification is **INCORRECT**
- **Action:** Keep using GET, document the finding
- **Action:** Submit issue to kiliankoe/vvo repository

### If POST works and GET fails:
- ❌ Current implementation is **INCORRECT**
- ✅ Specification is **CORRECT**
- **Action:** Update `src/route.rs` and `src/lines.rs` to use POST
- **Action:** Change `.get()` → `.post()` and `.query()` → `.json()`

### If both work:
- API is lenient and accepts both methods
- **Recommendation:** Follow specification (POST) for consistency
- **Action:** Update implementation to use POST method

## Files to Update Based on Results

### Route Planning API (`src/route.rs`)

**If POST is correct:**

Change lines 183-186:
```rust
// BEFORE
let response = reqwest::Client::new()
    .get(ROUTE_URL)
    .query(&params)
```

To:
```rust
// AFTER
let response = reqwest::Client::new()
    .post(ROUTE_URL)
    .json(&params)
```

And lines 202-205 similarly.

### Lines API (`src/lines.rs`)

**If POST is correct:**

Change lines 70-71:
```rust
// BEFORE
let response: DvbResponse<Lines> = reqwest::Client::new()
    .get(LINES_URL)
    .query(&[("format", "json"), ("stopid", stop_id)])
```

To:
```rust
// AFTER
#[derive(Serialize, Debug)]
struct LinesParams<'a> {
    stopid: &'a str,
}

let response: DvbResponse<Lines> = reqwest::Client::new()
    .post(LINES_URL)
    .json(&LinesParams { stopid: stop_id })
```

## Preliminary Testing Results

Manual `curl` testing on 2025-01-20 showed:

### Route Planning API
- GET: ⚠️ Accepts the method, but had date format issues
- POST: ⚠️ Accepts the method, but had date format issues
- **Conclusion:** Both methods may work, needs more testing

### Lines API  
- GET: ✅ **Works perfectly** (Status: Ok)
- POST: ❌ **Fails** (Status: "Connection error")
- **Conclusion:** GET is correct, specification is wrong

## Next Steps

1. **Run the test examples** to verify results
2. **Document your findings** in an issue
3. **Update code** if necessary
4. **Submit PR to kiliankoe/vvo** if specification is incorrect

## Additional Notes

- The test examples use real API calls, so you need internet connectivity
- Rate limiting may apply if you run tests too frequently
- The API may occasionally return errors unrelated to HTTP method
- If tests are inconclusive, try running multiple times at different times

## Related Documents

- [API_COMPLIANCE_REPORT.md](API_COMPLIANCE_REPORT.md) - Full compliance analysis
- [MISSING_FEATURES.md](MISSING_FEATURES.md) - Features not yet implemented
- [VVO WebAPI Specification](https://raw.githubusercontent.com/kiliankoe/vvo/refs/heads/main/documentation/webapi.md)

## Contact

If you discover conclusive results, please:
- Open an issue in this repository with your findings
- Consider submitting a PR to kiliankoe/vvo to correct the specification
- Update this document with confirmed results