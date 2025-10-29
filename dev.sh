#!/bin/bash

# Development script for KakeiBon
# This script copies SQL constants before running cargo tauri dev

set -e

echo "Copying SQL constants from src/ to src-tauri/src/..."
cp src/sql_queries.rs src-tauri/src/sql_queries.rs

echo "Starting cargo tauri dev..."
cargo tauri dev
