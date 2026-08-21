# Frontend Test Index

This document provides a complete index of all frontend tests implemented in JavaScript.

**Last Updated**: 2026-08-21 JST  
**Total Tests**: 697 (jest suites; 22 test files, per `npm test`)

---

For detailed Japanese version with all test cases, see [Japanese Frontend Test Index](../ja/FRONTEND_TEST_INDEX.md).

## Quick Reference

### Common Test Suites (56 tests — helper libraries)

Helper functions invoked from screen tests. Their assertions are counted in
the Screen-Specific totals below; they are listed here for discoverability
and are **not** added again to the grand total.

- **password-validation-tests.js** - Password validation tests (26)
- **username-validation-tests.js** - Username validation tests (20)
- **user-edit-validation-tests.js** - User edit validation tests (23)
- **validation-helpers.js** - Common validation functions

### Screen-Specific Tests (308 tests)
- **admin-setup.test.js** - Admin setup tests (32)
- **user-addition.test.js** - User addition tests (46)
- **admin-edit.test.js** - Admin edit tests (63)
- **general-user-edit.test.js** - General user edit tests (63)
- **login.test.js** - Login tests (58)
- **user-deletion.test.js** - User deletion tests (46)

### Feature-Specific Tests (274 tests)
- **transaction-edit.test.js** - Transaction edit tests (112)
- **transaction-detail-management.test.js** - Transaction detail management tests (51)
- **transaction-detail-tax-calculation.test.js** - Tax calculation tests (17)
- **toast.test.js** - Toast notification tests (14)
- **tax-calc.test.js** - Tax calculation utility tests (10)
- **product-autocomplete.test.js** - Product autocomplete UI tests (10)
- **product-draft.test.js** - Product draft-state tests (11)
- **product-master-jump-draft.test.js** - Product master jump / draft handoff tests (11)
- **modal-double-submit.test.js** - Shared `Modal._handleSave` re-entrancy guard + unhandled-rejection swallow tests (6)
- **master-crud.test.js** - Shared `saveMasterEntry` + `mapMasterErrorCode` (Fable-5 #D3/#D4/#23) tests (24)
- **attach-char-counter-ime.test.js** - `attachCharCounter` baseline + IME composition guard (Fable-5 #D1) tests (8)

### Aggregation Tests (115 tests)
- **aggregation-daily.test.js** - Daily aggregation (16)
- **aggregation-weekly.test.js** - Weekly aggregation (22)
- **aggregation-monthly.test.js** - Monthly aggregation (33)
- **aggregation-yearly.test.js** - Yearly aggregation (21)
- **aggregation-period.test.js** - Period aggregation (23)

### Browser / Standalone (not counted in the jest total)
- **category-management-ui-tests.js** - DOM-based tests, run in a browser session against a rendered category page
- **tax-rounding-tests.js** - Companion to `tax-rounding-tests.html`; pure-function harness, run via the HTML page
- **backend-validation-standalone.js** - Node-standalone runner (`node backend-validation-standalone.js`)
- **login-test-standalone.js** - Node-standalone runner (`node login-test-standalone.js`)
- **aggregation-test-helpers.js** - Shared mock/fixture helpers imported by the aggregation `.test.js` files

---

## Test Statistics Summary

| Category | Test Count |
|----------|------------|
| **Common Test Suites** (helpers — counted inside Screen-Specific) | 56 |
| password-validation-tests.js | 26 |
| username-validation-tests.js | 20 |
| user-edit-validation-tests.js | 23 |
| **Screen-Specific Tests** | **308** |
| admin-setup.test.js | 32 |
| user-addition.test.js | 46 |
| admin-edit.test.js | 63 |
| general-user-edit.test.js | 63 |
| login.test.js | 58 |
| user-deletion.test.js | 46 |
| **Feature-Specific Tests** | **274** |
| transaction-edit.test.js | 112 |
| transaction-detail-management.test.js | 51 |
| transaction-detail-tax-calculation.test.js | 17 |
| toast.test.js | 14 |
| tax-calc.test.js | 10 |
| product-autocomplete.test.js | 10 |
| product-draft.test.js | 11 |
| product-master-jump-draft.test.js | 11 |
| modal-double-submit.test.js | 6 |
| master-crud.test.js | 24 |
| attach-char-counter-ime.test.js | 8 |
| **Aggregation Tests** | **115** |
| aggregation-daily.test.js | 16 |
| aggregation-weekly.test.js | 22 |
| aggregation-monthly.test.js | 33 |
| aggregation-yearly.test.js | 21 |
| aggregation-period.test.js | 23 |
| **Total (jest)** | **697** |

Grand total is Screen + Feature + Aggregation (Common Test Suites are helper
libraries invoked from Screen-Specific files and their assertions are already
counted in those screen totals).

---

## How to Run Tests

### Run all tests

```bash
cd res/tests
npm test
```

### Run specific test file

```bash
npm test admin-setup.test.js
npm test login.test.js
npm test user-deletion.test.js
```

### Run specific test case

```bash
npm test -- --testNamePattern="Empty Password"
npm test -- --testNamePattern="Username Validation"
```

### Generate coverage report

```bash
npm run test:coverage
```

### Standalone tests (Node.js, no dependencies)

```bash
node login-test-standalone.js
node backend-validation-standalone.js
```

### Refreshing the authoritative counts

```bash
cd res/tests
node --experimental-vm-modules node_modules/jest/bin/jest.js --json > /tmp/jest.json
# Per-file counts:
node -e "const j=JSON.parse(require('fs').readFileSync('/tmp/jest.json','utf8')); \
  j.testResults.map(r=>({f:r.name.replace(/^.*\\//,''),n:r.assertionResults.length})) \
  .sort((a,b)=>a.f.localeCompare(b.f)).forEach(r=>console.log(String(r.n).padStart(4)+'  '+r.f)); \
  console.log('total:',j.numTotalTests);"
```

---

## Related Documents

- [Backend Test Index](BACKEND_TEST_INDEX.md) - Complete list of Rust tests
- [Test Overview](TEST_OVERVIEW.md) - Test strategy and execution guide
- [Test Design](TEST_DESIGN.md) - Test architecture and design philosophy
- [Test Results](TEST_RESULTS.md) - Latest test execution results
