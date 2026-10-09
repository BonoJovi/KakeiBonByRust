#!/usr/bin/env bash
# Run the same tests as CI: the Rust tests (`cargo test`) and the frontend
# Jest tests (`npm test` in tests/frontend). Both suites run even when the first
# one fails, and the script exits non-zero when either suite fails.
#
# The Jest dependencies must be installed first: `cd tests/frontend && npm install`.
#
# Usage: ./scripts/run-all-tests.sh

set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

if [[ ! -d tests/frontend/node_modules ]]; then
    echo "tests/frontend/node_modules is missing; run 'cd tests/frontend && npm install' first." >&2
    exit 1
fi

echo "=== Rust tests (cargo test) ==="
cargo test
rust_status=$?

echo
echo "=== Frontend tests (npm test) ==="
(cd tests/frontend && npm test)
js_status=$?

echo
echo "=== Summary ==="
[[ $rust_status -eq 0 ]] && echo "Rust tests:     passed" || echo "Rust tests:     FAILED"
[[ $js_status -eq 0 ]] && echo "Frontend tests: passed" || echo "Frontend tests: FAILED"

[[ $rust_status -eq 0 && $js_status -eq 0 ]]
