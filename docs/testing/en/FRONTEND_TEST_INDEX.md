# Frontend Test Index

This document provides a complete index of all frontend tests implemented in JavaScript.

**Last Updated**: 2025-12-06 06:45 JST  
**Total Tests**: 262+

---

For detailed Japanese version with all test cases, see [Japanese Frontend Test Index](../ja/FRONTEND_TEST_INDEX.md).

## Quick Reference

### Common Test Suites (56 tests)
- **password-validation-tests.js** - Password validation tests (26)
- **username-validation-tests.js** - Username validation tests (20)
- **user-edit-validation-tests.js** - User edit validation tests (23)
- **validation-helpers.js** - Common validation functions

### Screen-Specific Tests (206 tests)
- **admin-setup.test.js** - Admin setup tests (29)
- **user-addition.test.js** - User addition tests (49)
- **admin-edit.test.js** - Admin edit tests (63)
- **general-user-edit.test.js** - General user edit tests (63)
- **login.test.js** - Login tests (58)
- **user-deletion.test.js** - User deletion tests (46)

### Feature-Specific Tests (118+ tests)
- **transaction-edit.test.js** - Transaction edit tests (112)
- **transaction-detail-management.test.js** - Transaction detail tests
- **transaction-detail-tax-calculation.test.js** - Tax calculation tests
- **category-management-ui-tests.js** - Category management UI tests

### Aggregation Tests (many tests)
- **aggregation-daily.test.js** - Daily aggregation
- **aggregation-weekly.test.js** - Weekly aggregation
- **aggregation-monthly.test.js** - Monthly aggregation
- **aggregation-yearly.test.js** - Yearly aggregation
- **aggregation-period.test.js** - Period aggregation

---

## Test Statistics Summary

| Category | Test Count |
|----------|------------|
| **Common Test Suites** | **56** |
| password-validation-tests.js | 26 |
| username-validation-tests.js | 20 |
| user-edit-validation-tests.js | 23 |
| **Screen-Specific Tests** | **206** |
| admin-setup.test.js | 29 |
| user-addition.test.js | 49 |
| admin-edit.test.js | 63 |
| general-user-edit.test.js | 63 |
| login.test.js | 58 |
| user-deletion.test.js | 46 |
| **Feature-Specific Tests** | **118+** |
| transaction-edit.test.js | 112 |
| transaction-detail-management.test.js | 6+ |
| transaction-detail-tax-calculation.test.js | many |
| **Aggregation Tests** | **many** |
| **Total** | **262+** |

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

---

## Related Documents

- [Backend Test Index](BACKEND_TEST_INDEX.md) - Complete list of Rust tests
- [Test Overview](TEST_OVERVIEW.md) - Test strategy and execution guide
- [Test Design](TEST_DESIGN.md) - Test architecture and design philosophy
- [Test Results](TEST_RESULTS.md) - Latest test execution results
