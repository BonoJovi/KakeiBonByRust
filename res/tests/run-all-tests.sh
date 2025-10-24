#!/bin/bash

# Run all tests for KakeiBon application
# Usage: ./run-all-tests.sh

set -e

echo "=========================================="
echo "KakeiBon - Running All Tests"
echo "=========================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

TOTAL_PASSED=0
TOTAL_FAILED=0

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$PROJECT_ROOT"

echo "📍 Project root: $PROJECT_ROOT"
echo ""

# 1. Run Rust unit tests
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🦀 Running Rust Unit Tests..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if cargo test --lib 2>&1 | tee /tmp/rust_test.log; then
    RUST_PASSED=$(grep -o "[0-9]\+ passed" /tmp/rust_test.log | head -1 | awk '{print $1}')
    echo -e "${GREEN}✓ Rust tests passed: ${RUST_PASSED}${NC}"
    TOTAL_PASSED=$((TOTAL_PASSED + RUST_PASSED))
else
    RUST_FAILED=$(grep -o "[0-9]\+ failed" /tmp/rust_test.log | head -1 | awk '{print $1}')
    echo -e "${RED}✗ Rust tests failed: ${RUST_FAILED}${NC}"
    TOTAL_FAILED=$((TOTAL_FAILED + RUST_FAILED))
fi
echo ""

# 2. Run JavaScript standalone tests
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📝 Running JavaScript Standalone Tests..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

cd "$PROJECT_ROOT/res/tests"

# Login tests
echo "🔐 Login Tests..."
if node login-test-standalone.js > /tmp/login_test.log 2>&1; then
    LOGIN_PASSED=$(grep -o "Passed: [0-9]\+" /tmp/login_test.log | awk '{print $2}')
    echo -e "${GREEN}✓ Login tests passed: ${LOGIN_PASSED}${NC}"
    TOTAL_PASSED=$((TOTAL_PASSED + LOGIN_PASSED))
else
    LOGIN_FAILED=$(grep -o "Failed: [0-9]\+" /tmp/login_test.log | awk '{print $2}')
    echo -e "${RED}✗ Login tests failed: ${LOGIN_FAILED}${NC}"
    TOTAL_FAILED=$((TOTAL_FAILED + LOGIN_FAILED))
    cat /tmp/login_test.log
fi

# Backend validation tests
echo "🔧 Backend Validation Tests..."
if node backend-validation-standalone.js > /tmp/backend_test.log 2>&1; then
    BACKEND_PASSED=$(grep -o "Passed: [0-9]\+" /tmp/backend_test.log | awk '{print $2}')
    echo -e "${GREEN}✓ Backend validation tests passed: ${BACKEND_PASSED}${NC}"
    TOTAL_PASSED=$((TOTAL_PASSED + BACKEND_PASSED))
else
    BACKEND_FAILED=$(grep -o "Failed: [0-9]\+" /tmp/backend_test.log | awk '{print $2}')
    echo -e "${RED}✗ Backend validation tests failed: ${BACKEND_FAILED}${NC}"
    TOTAL_FAILED=$((TOTAL_FAILED + BACKEND_FAILED))
    cat /tmp/backend_test.log
fi

echo ""

# Summary
echo "=========================================="
echo "📊 Test Summary"
echo "=========================================="
echo "Total Passed: ${TOTAL_PASSED} ✓"
echo "Total Failed: ${TOTAL_FAILED} ✗"
echo "=========================================="

if [ $TOTAL_FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}❌ Some tests failed${NC}"
    exit 1
fi
