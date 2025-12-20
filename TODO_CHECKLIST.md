# TODO Checklist for VVO WebAPI Compliance

This checklist tracks the high-priority tasks to improve `dvb-rs` compliance with the VVO WebAPI specification.

**Status Legend:**
- ⬜ Not started
- 🔄 In progress
- ✅ Complete
- ❌ Blocked/Skipped

---

## Phase 1: Verify Specification (CRITICAL)

### Task 1.1: Test Route Planning API HTTP Methods
- ⬜ Run: `cargo run --example route_test_methods`
- ⬜ Document which method works (GET/POST/Both)
- ⬜ Take screenshots or save output
- ⬜ Decision:
  - If GET works: Keep current implementation, mark spec as incorrect
  - If POST works: Proceed to Task 1.3
  - If both work: Decide whether to switch to POST for consistency

**Priority:** 🔴 CRITICAL  
**Estimated Time:** 10 minutes  
**Blockers:** None

---

### Task 1.2: Test Lines API HTTP Methods
- ⬜ Run: `cargo run --example lines_test_methods`
- ⬜ Document which method works (GET/POST/Both)
- ⬜ Take screenshots or save output
- ⬜ Decision:
  - If GET works: Keep current implementation, mark spec as incorrect
  - If POST works: Proceed to Task 1.4
  - If both work: Decide whether to switch to POST for consistency

**Priority:** 🔴 CRITICAL  
**Estimated Time:** 10 minutes  
**Blockers:** None  
**Note:** Preliminary testing suggests GET works, POST fails (spec is wrong)

---

### Task 1.3: Update Route Planning Implementation (If needed)
- ⬜ Open `src/route.rs`
- ⬜ Line 183-186: Change `.get(ROUTE_URL)` to `.post(ROUTE_URL)`
- ⬜ Line 183-186: Change `.query(&params)` to `.json(&params)`
- ⬜ Line 202-205: Same changes for `route_details()` function
- ⬜ Test with existing examples: `cargo run --example routes`
- ⬜ Run: `cargo test`
- ⬜ Commit changes

**Priority:** 🔴 CRITICAL (if POST is correct)  
**Estimated Time:** 15 minutes  
**Blockers:** Task 1.1 must be complete

---

### Task 1.4: Update Lines Implementation (If needed)
- ⬜ Open `src/lines.rs`
- ⬜ Add struct before function:
  ```rust
  #[derive(Serialize, Debug)]
  #[serde(rename_all = "camelCase")]
  struct LinesParams<'a> {
      stopid: &'a str,
  }
  ```
- ⬜ Line 70-71: Change `.get(LINES_URL)` to `.post(LINES_URL)`
- ⬜ Line 70-71: Change `.query(&[...])` to `.json(&LinesParams { stopid: stop_id })`
- ⬜ Test with existing examples: `cargo run --example lines`
- ⬜ Run: `cargo test`
- ⬜ Commit changes

**Priority:** 🔴 CRITICAL (if POST is correct)  
**Estimated Time:** 15 minutes  
**Blockers:** Task 1.2 must be complete  
**Note:** Likely NOT needed (GET appears to be correct)

---

### Task 1.5: Update Specification Documentation
- ⬜ If GET is correct, open issue at https://github.com/kiliankoe/vvo
- ⬜ Reference test results and findings
- ⬜ Suggest corrections to webapi.md
- ⬜ Update local API_COMPLIANCE_REPORT.md with confirmed results
- ⬜ Update TESTING_HTTP_METHODS.md with confirmed results

**Priority:** 🟡 MEDIUM  
**Estimated Time:** 20 minutes  
**Blockers:** Tasks 1.1 and 1.2 must be complete

---

## Phase 2: Fix Known Bugs

### Task 2.1: Fix Stop.scheduled_time Type Bug
- ⬜ Open `src/trip.rs`
- ⬜ Find `pub struct Stop` (around line 27)
- ⬜ Change `pub scheduled_time: Option<bool>` to `pub scheduled_time: Option<DvbTime>`
- ⬜ Test: `cargo run --example route_and_trip`
- ⬜ Run: `cargo test`
- ⬜ Check if any code depends on the bool type
- ⬜ Commit changes

**Priority:** 🔴 HIGH  
**Estimated Time:** 10 minutes  
**Blockers:** None  
**Breaking Change:** Yes (minor)

---

### Task 2.2: Add Missing Departure Monitor Fields
- ⬜ Open `src/monitor.rs`
- ⬜ Uncomment or add `Platform` struct (may exist in other files)
- ⬜ Uncomment or add `Diva` struct (check if it exists in `src/common.rs`)
- ⬜ Add to `Departure` struct:
  - `pub platform: Option<Platform>`
  - `pub diva: Option<Diva>`
  - `pub scheduled_time: Option<DvbTime>`
- ⬜ Test: `cargo run --example departures`
- ⬜ Verify JSON deserialization works
- ⬜ Run: `cargo test`
- ⬜ Commit changes

**Priority:** 🟡 MEDIUM  
**Estimated Time:** 30 minutes  
**Blockers:** None  
**Breaking Change:** No (adding optional fields)

**Notes:**
- Platform may already exist in `src/trip.rs` - can reuse or move to `src/common.rs`
- Diva may already exist in `src/lines.rs` - check before creating

---

## Phase 3: Add Missing Features

### Task 3.1: Implement Route Changes API
- ⬜ Create `src/route_changes.rs`
- ⬜ Follow implementation guide in MISSING_FEATURES.md section 1
- ⬜ Add structs: `RouteChanges`, `RouteChange`, `ValidityPeriod`
- ⬜ Add enum: `ChangeType` (Scheduled, Unscheduled)
- ⬜ Add async function: `route_changes(shortterm: bool)`
- ⬜ Export from `src/lib.rs`
- ⬜ Create example: `examples/route_changes.rs`
- ⬜ Test against real API
- ⬜ Update documentation
- ⬜ Commit changes

**Priority:** 🟢 MEDIUM-LOW  
**Estimated Time:** 2-3 hours  
**Blockers:** None

---

### Task 3.2: Add Optional PointFinder Parameters
- ⬜ Open `src/point.rs`
- ⬜ Add to `Params` struct:
  - `pub regional_only: Option<bool>`
  - `pub stop_shortcuts: Option<bool>`
- ⬜ Update `Default` impl to set these to None
- ⬜ Test with examples
- ⬜ Update documentation
- ⬜ Commit changes

**Priority:** 🟢 LOW  
**Estimated Time:** 30 minutes  
**Blockers:** None  
**Breaking Change:** No

---

### Task 3.3: Add Advanced Route Planning Parameters
- ⬜ Open `src/route.rs`
- ⬜ Follow implementation guide in MISSING_FEATURES.md section 3
- ⬜ Add structs: `MobilitySettings`, `StandardSettings`
- ⬜ Add enums: `MobilityRestriction`, `MaxChanges`, `WalkingSpeed`
- ⬜ Update `Params` struct to include these
- ⬜ Update examples to show usage
- ⬜ Test against real API
- ⬜ Commit changes

**Priority:** 🟢 LOW  
**Estimated Time:** 3-4 hours  
**Blockers:** Phase 1 should be complete (verify correct HTTP method)  
**Breaking Change:** No (adding optional fields)

---

## Phase 4: Testing and Quality

### Task 4.1: Add Integration Tests
- ⬜ Create `tests/integration_tests.rs`
- ⬜ Add tests for each endpoint against real API
- ⬜ Add retry logic for flaky API
- ⬜ Document how to run tests
- ⬜ Consider CI/CD integration

**Priority:** 🟡 MEDIUM  
**Estimated Time:** 4-6 hours  
**Blockers:** Phase 2 should be complete

---

### Task 4.2: Fix Parameter Typo (Breaking Change)
- ⬜ Plan for next major version (v0.8.0 or v1.0.0)
- ⬜ Add deprecation warning to `assigedstops` field
- ⬜ Add new field `assigned_stops`
- ⬜ Make both work for transition period
- ⬜ Document migration in CHANGELOG
- ⬜ In next major version, remove old field

**Priority:** 🟢 LOW  
**Estimated Time:** 1 hour  
**Blockers:** None  
**Breaking Change:** Yes (save for major version bump)

---

## Phase 5: Documentation

### Task 5.1: Update README.md
- ⬜ Add link to compliance report
- ⬜ Document which HTTP methods are used
- ⬜ Note any specification discrepancies
- ⬜ Add more examples
- ⬜ Add troubleshooting section

**Priority:** 🟡 MEDIUM  
**Estimated Time:** 1 hour  
**Blockers:** Phase 1 should be complete

---

### Task 5.2: Update CHANGELOG.md
- ⬜ Document all changes from this compliance work
- ⬜ Note any breaking changes
- ⬜ Add migration guides if needed
- ⬜ Reference specification discrepancies

**Priority:** 🟡 MEDIUM  
**Estimated Time:** 30 minutes  
**Blockers:** Should be done continuously

---

## Progress Tracking

**Phase 1:** ⬜⬜⬜⬜⬜ (0/5 complete)  
**Phase 2:** ⬜⬜ (0/2 complete)  
**Phase 3:** ⬜⬜⬜ (0/3 complete)  
**Phase 4:** ⬜⬜ (0/2 complete)  
**Phase 5:** ⬜⬜ (0/2 complete)  

**Overall Progress:** 0/14 tasks complete (0%)

---

## Quick Start: What To Do Right Now

1. ✅ Read this checklist
2. ⬜ Run `cargo run --example route_test_methods`
3. ⬜ Run `cargo run --example lines_test_methods`
4. ⬜ Based on results, complete Phase 1 tasks
5. ⬜ Fix the `Stop.scheduled_time` type bug (Task 2.1)
6. ⬜ Decide priority for remaining tasks based on your use case

---

## Notes

- **Test early, test often:** Run the HTTP method tests before making any changes
- **One task at a time:** Don't skip ahead, especially in Phase 1
- **Document as you go:** Update this checklist with findings and decisions
- **Commit frequently:** Each completed task should be a commit
- **Ask for help:** If blocked, refer to the detailed guides or open an issue

---

**Last Updated:** 2025-01-20  
**Next Review:** After Phase 1 completion