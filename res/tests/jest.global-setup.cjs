// Jest globalSetup — pin `TZ=Asia/Tokyo` before worker processes spawn.
//
// Why this file exists:
//   `res/tests/format-local-date.test.js` builds Date values via
//   `Date.UTC(...)` and asserts on their local-getter view, which
//   only differs from the UTC view when the runtime TZ is not UTC.
//   Without a pin, CI (Ubuntu, TZ=UTC by default) makes the local
//   and UTC views collapse to the same date and the tests can no
//   longer distinguish a good implementation from a UTC-regressing
//   one.
//
// Why here (globalSetup) and not the package.json script:
//   `TZ=X node ...` in an npm script works on POSIX shells but not
//   in Windows `cmd.exe`, which doesn't parse `NAME=VALUE cmd` as
//   an env-var assignment (CodeRabbit on #134). A `.cjs`
//   globalSetup file runs in the Jest main process before workers
//   are forked; workers inherit the mutated env, so Node in each
//   worker starts with TZ set. This is cross-platform (pure Node)
//   and adds no dev dependency.
//
// Why `.cjs`:
//   `res/tests/package.json` sets `"type": "module"`, so a `.js`
//   file would be loaded as ESM. Jest 29's globalSetup handling
//   for ESM has known caveats; `.cjs` sidesteps them.

module.exports = async function () {
    process.env.TZ = 'Asia/Tokyo';
};
