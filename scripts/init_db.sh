#!/bin/bash

# Database initialization script for KakeiBon
# Creates a fresh database with all tables and initial data

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Database path
DB_DIR="$HOME/.kakeibon"
DB_FILE="KakeiBonDB.sqlite3"
DB_PATH="$DB_DIR/$DB_FILE"

# SQL directory
SQL_DIR="$(cd "$(dirname "$0")/.." && pwd)/sql/init"

echo -e "${YELLOW}=== KakeiBon Database Initialization ===${NC}"
echo ""

# Check if database exists
if [ -f "$DB_PATH" ]; then
    echo -e "${RED}Warning: Database already exists at $DB_PATH${NC}"
    echo -e "${YELLOW}This will DELETE the existing database!${NC}"
    read -p "Are you sure you want to continue? (yes/no): " confirm
    if [ "$confirm" != "yes" ]; then
        echo "Initialization cancelled."
        exit 0
    fi
    
    # Backup existing database
    BACKUP_FILE="${DB_PATH}.backup_$(date +%Y%m%d_%H%M%S)"
    echo "Creating backup: $BACKUP_FILE"
    cp -p "$DB_PATH" "$BACKUP_FILE"
    
    # Remove existing database
    rm "$DB_PATH"
    echo -e "${GREEN}✓ Old database removed${NC}"
fi

# Create directory if not exists
mkdir -p "$DB_DIR"
echo -e "${GREEN}✓ Directory created: $DB_DIR${NC}"

# Create new database
touch "$DB_PATH"
echo -e "${GREEN}✓ New database created${NC}"
echo ""

# Function to execute SQL files
execute_sql_files() {
    local category=$1
    local dir=$2
    
    echo -e "${YELLOW}--- $category ---${NC}"
    
    if [ ! -d "$dir" ]; then
        echo -e "${RED}✗ Directory not found: $dir${NC}"
        return 1
    fi
    
    local count=0
    for sql_file in "$dir"/*.sql; do
        if [ -f "$sql_file" ]; then
            local filename=$(basename "$sql_file")
            echo -n "  Executing: $filename ... "
            if sqlite3 "$DB_PATH" < "$sql_file" 2>/dev/null; then
                echo -e "${GREEN}✓${NC}"
                ((count++))
            else
                echo -e "${RED}✗ Failed${NC}"
                return 1
            fi
        fi
    done
    
    echo -e "${GREEN}✓ Completed: $count files${NC}"
    echo ""
}

# Execute SQL files in order
echo -e "${YELLOW}=== Initializing Database ===${NC}"
echo ""

# 1. Create tables
execute_sql_files "Creating Tables" "$SQL_DIR/tables"

# 2. Insert i18n resources
execute_sql_files "Inserting Translation Resources" "$SQL_DIR/i18n"

echo -e "${GREEN}=== Database initialization completed successfully! ===${NC}"
echo ""
echo "Database location: $DB_PATH"

# Show statistics
echo ""
echo -e "${YELLOW}=== Database Statistics ===${NC}"
sqlite3 "$DB_PATH" <<EOF
.headers off
SELECT 'Tables: ' || COUNT(*) FROM sqlite_master WHERE type='table';
SELECT 'Translation resources: ' || COUNT(*) FROM I18N_RESOURCES;
EOF

echo ""
echo -e "${GREEN}✓ All done!${NC}"
