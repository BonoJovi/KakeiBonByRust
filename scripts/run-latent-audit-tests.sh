#!/usr/bin/env bash
# Run the isolated latent-audit regression tests (Rust).
#
# A latent-audit test pins a bug that is not fixed yet, so it is EXPECTED to
# fail. It is marked `#[ignore = "latent-audit <ID>"]` and its function name
# starts with `latent_`, which keeps it out of the normal `cargo test` run
# (and therefore out of CI). When the bug is fixed, remove the `#[ignore]` so
# the test joins the regular suite.
#
# Every test of the 2026-09 audit and scan has been fixed and moved into the
# regular suites, so this script currently runs no tests.
#
# Usage: ./scripts/run-latent-audit-tests.sh

set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

echo "=== Rust latent-audit tests ==="
cargo test --lib latent_ -- --ignored --test-threads=1 2>&1 \
    | grep -E '^test .*latent_.* \.\.\. |^test result:' \
    | sed -E 's/^test (.*) \.\.\. (ok|FAILED)$/\2\t\1/'
