#!/bin/bash

# Database access script for KakeiBon
# Opens the official database file with sqlite3

set -e

# Database path (from src/consts.rs)
DB_DIR="$HOME/.kakeibon"
DB_FILE="KakeiBonDB.sqlite3"
DB_PATH="$DB_DIR/$DB_FILE"

# Check if database exists
if [ ! -f "$DB_PATH" ]; then
    echo "Error: Database file not found at $DB_PATH"
    echo "Please run the application first to initialize the database."
    exit 1
fi

# Open sqlite3 with the database
echo "Opening database: $DB_PATH"
sqlite3 "$DB_PATH" "$@"
