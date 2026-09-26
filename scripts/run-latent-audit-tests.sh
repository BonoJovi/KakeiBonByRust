#!/usr/bin/env bash
# Run the isolated latent-audit regression tests (2026-09 audit).
#
# These tests pin bugs listed in
# work/bug_list_opus_5_5_latent_audit_2026-09-26.md and are EXPECTED to fail
# until each bug is fixed. They are excluded from the normal `cargo test` /
# `npm test` runs (and therefore from CI):
#   - Rust: `#[ignore = "latent-audit <ID>"]`, function names start with `latent_`
#   - Jest: files under res/tests/latent-audit/ (testPathIgnorePatterns)
#
# When a bug is fixed, remove the `#[ignore]` (Rust) or move the test file out
# of latent-audit/ (Jest) so the test joins the regular suite.
#
# Usage: ./scripts/run-latent-audit-tests.sh [rust|js]   (default: both)

set -uo pipefail
cd "$(dirname "$0")/.."

target="${1:-all}"

if [[ "$target" == "all" || "$target" == "rust" ]]; then
    echo "=== Rust latent-audit tests ==="
    cargo test --lib latent_ -- --ignored 2>&1 \
        | grep -E '^test .*latent_.* \.\.\. |^test result:' \
        | sed -E 's/^test (.*) \.\.\. (ok|FAILED)$/\2\t\1/'
    echo
fi

if [[ "$target" == "all" || "$target" == "js" ]]; then
    echo "=== Jest latent-audit tests ==="
    (cd res/tests && npm run -s test:latent -- --verbose 2>&1) \
        | grep -E '^\s+(✓|✕)|^Tests:'
fi
