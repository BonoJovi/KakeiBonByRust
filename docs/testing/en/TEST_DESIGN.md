# Test Design

How KakeiBon's tests are built, and why.

## Policy

### 1. Test the real code, never a copy of it

A test must run the app's own code. A test that copies a rule into the test
file and checks the copy keeps passing while the app changes, and the copy
drifts from the app without anyone noticing. KakeiBon had such tests until
2026-10. Some of the copies described behavior the app does not have:

- The password copy counted UTF-16 units, so it accepted
  `password🔐🔐🔐🔐123`. The app counts characters (code points) like the
  backend, so that password has 15 characters and is refused.
- The amount copy read `1,000` as 1000. The app refuses that input.

### 2. Check what the user sees and what is sent

A frontend test fills in the real form, submits it, and checks:

- the message shown (an i18n key, because the test's i18n returns the key), and
- the Tauri commands sent through `invoke`, with their arguments.

### 3. Keep the backend as the final check

Rules such as the 16-character password minimum are checked in both the
frontend (for immediate feedback) and the backend (`src/validation.rs`, for
safety). Each side has its own tests.

## Frontend (JavaScript, Jest + jsdom)

### Page tests

Most screens are page scripts: `res/js/<screen>.js` registers a
`DOMContentLoaded` listener and works on the DOM of `res/<screen>.html`.
`tests/frontend/pages/_page-harness.js` boots such a page in jsdom:

1. `mockPageModules(jest, { invoke, user })`, **before** the page module is
   imported, replaces Tauri `invoke` and the modules that only draw the
   window frame (menu bar, font size, language menu, indicators, window fit),
   plus session, toast and i18n, with `jest.unstable_mockModule`.
2. `loadPageBody('<screen>.html')` puts the real page markup into jsdom.
3. The test imports the real page module and calls `bootPage()`, which fires
   `DOMContentLoaded` and waits for the page to finish loading.

```javascript
// tests/frontend/pages/index-setup-validation.test.js (shortened)
import { jest } from '@jest/globals';
import { mockPageModules, loadPageBody, bootPage, flush, callsOf } from './_page-harness.js';

const { invoke } = mockPageModules(jest, {
    keepMenu: true,
    invoke: (cmd) => (cmd === 'check_needs_setup' ? true : 'ok'),
});

loadPageBody('index.html');
await import('../../../res/js/menu.js');
await bootPage();

test('should refuse with the right message and not register when the password has 15 characters', async () => {
    document.getElementById('admin-username').value = 'alice';
    document.getElementById('admin-password').value = '123456789012345';
    document.getElementById('admin-password-confirm').value = '123456789012345';
    document.getElementById('admin-setup-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
    await flush(10);

    expect(document.getElementById('setup-message').textContent).toBe('error.password_too_short');
    expect(callsOf(invoke, 'register_admin')).toHaveLength(0);
});
```

### Submitting a form

- `form.dispatchEvent(new Event('submit', ...))` runs the page's own checks.
  Use it to test the app's code.
- Clicking the submit button also runs the browser's field checks first
  (`required`, `min`, `step`, `type="number"`), as a browser does. Use it to
  test what those attributes stop before the app's code runs.

### Checks shared by several screens

When several screens must pass the same checks, put the cases in a helper
next to the tests that use it, and give each test file only what differs
(for example the commands it expects). Example:
`tests/frontend/pages/_user-self-edit-checks.js`, used by the administrator
and general-user user management tests. Helper files do not end in
`.test.js`, so Jest does not collect them.

### Pure helper modules

Modules that only compute a value (for example `res/js/parse-amount-strict.js`,
`res/js/tax-calc.js`) are imported and tested directly, without a page.

### Proving that a test can fail

After writing a test, break the app code it covers on purpose (for example
count UTF-16 units instead of characters) and check that the test fails. Then
restore the code with `git checkout`.

### Jest setup

`tests/frontend/package.json`:

- `"type": "module"` and `node --experimental-vm-modules` for ES modules
- `"testEnvironment": "jsdom"`, `"testMatch": ["**/*.test.js"]`
- `latent-audit/` is ignored by default (`npm run test:latent` runs it)

Imports must include the `.js` extension.

## Backend (Rust)

Rust tests live next to the code (`#[cfg(test)]` modules) and share:

- `src/test_helpers.rs`: database setup (`setup_test_db`, `create_test_admin`, ...)
- `src/validation_tests.rs`: password test suites reused by several modules

## Test names

New tests use the form `should <expected result> when <condition>`. See the
Testing Conventions in `docs/developer/en/guides/CODING_STANDARDS_en.md`.

## Troubleshooting

| Error | Fix |
|-------|-----|
| `SyntaxError: Cannot use import statement outside a module` | Run through `npm test` (it adds `--experimental-vm-modules`), not `npx jest` |
| A mock has no effect | Call `mockPageModules` before the page module is imported |
| The page shows nothing after boot | `await bootPage()` and `await flush()` after actions that call `invoke` |

## Related documents

- [Test Overview](TEST_OVERVIEW.md)
- [Frontend Test Index](FRONTEND_TEST_INDEX.md)
- [Backend Test Index](BACKEND_TEST_INDEX.md)
- [Jest: ECMAScript Modules](https://jestjs.io/docs/ecmascript-modules)
