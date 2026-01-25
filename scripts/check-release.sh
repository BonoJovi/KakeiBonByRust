#!/bin/bash
# check-release.sh - Release Build Integrity Check
#
# This script verifies that the release build completes successfully.
# Run this before creating a release tag to catch any issues early.
#
# Usage: ./scripts/check-release.sh
#
# What it checks:
# 1. Release build (cargo build --release)
# 2. Release tests (cargo test --release)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}========================================${NC}"
echo -e "${YELLOW}  Release Build Integrity Check${NC}"
echo -e "${YELLOW}========================================${NC}"
echo ""

# Change to project root
cd "$(dirname "$0")/.."
PROJECT_ROOT=$(pwd)
echo "Project root: $PROJECT_ROOT"
echo ""

# Step 1: Release Build
echo -e "${YELLOW}[1/2] Building release...${NC}"
if cargo build --release; then
    echo -e "${GREEN}  Release build successful.${NC}"
else
    echo -e "${RED}  Release build FAILED!${NC}"
    exit 1
fi
echo ""

# Step 2: Release Tests
echo -e "${YELLOW}[2/2] Running release tests...${NC}"
if cargo test --release; then
    echo -e "${GREEN}  Release tests passed.${NC}"
else
    echo -e "${RED}  Release tests FAILED!${NC}"
    exit 1
fi
echo ""

# Summary
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}  All checks passed!${NC}"
echo -e "${GREEN}  Ready for release.${NC}"
echo -e "${GREEN}========================================${NC}"
