#!/bin/bash

# Development script for KakeiBon
# Wrapper for cargo tauri dev (SQL sync is handled by beforeDevCommand in tauri.conf.json)

set -e

echo "Starting cargo tauri dev..."
cargo tauri dev
