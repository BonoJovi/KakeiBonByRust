#!/bin/bash

# Simple script to check i18n resources consistency

DB_PATH="$HOME/.kakeibon/KakeiBonDB.sqlite3"

echo "=== I18N Resources Consistency Check ==="
echo ""

# Check if database exists
if [ ! -f "$DB_PATH" ]; then
    echo "Error: Database not found at $DB_PATH"
    exit 1
fi

# Create temp files
TEMP_DIR=$(mktemp -d)
CODE_KEYS="$TEMP_DIR/code_keys.txt"
DB_KEYS="$TEMP_DIR/db_keys.txt"
MISSING="$TEMP_DIR/missing.txt"

# Extract keys from HTML files
echo "Extracting keys from HTML files..."
grep -roh 'data-i18n="[^"]*"' res/*.html 2>/dev/null | sed 's/data-i18n="//;s/"//' > "$CODE_KEYS"

# Extract keys from JS files
echo "Extracting keys from JS files..."
grep -roh "i18n\.t('[^']*')" res/js/*.js 2>/dev/null | sed "s/i18n\.t('//;s/')//" >> "$CODE_KEYS"

# Sort and deduplicate
sort -u "$CODE_KEYS" -o "$CODE_KEYS"

# Get keys from database
echo "Querying database..."
sqlite3 "$DB_PATH" "SELECT DISTINCT resource_key FROM i18n_resources ORDER BY resource_key;" > "$DB_KEYS"

# Find missing keys
echo ""
echo "Checking for missing resources..."
echo ""

> "$MISSING"
while read -r key; do
    if ! grep -Fxq "$key" "$DB_KEYS"; then
        echo "$key" >> "$MISSING"
    fi
done < "$CODE_KEYS"

# Display results
MISSING_COUNT=$(wc -l < "$MISSING" | tr -d ' ')

if [ "$MISSING_COUNT" -eq 0 ]; then
    echo "✓ All resources found in database!"
else
    echo "✗ Found $MISSING_COUNT missing resource(s):"
    echo ""
    
    while read -r key; do
        echo "  Missing: $key"
        
        # Find where used
        HTML_FILES=$(grep -l "data-i18n=\"$key\"" res/*.html 2>/dev/null)
        if [ -n "$HTML_FILES" ]; then
            echo "    Used in HTML: $(basename $HTML_FILES)"
        fi
        
        JS_FILES=$(grep -l "i18n\.t('$key')" res/js/*.js 2>/dev/null)
        if [ -n "$JS_FILES" ]; then
            echo "    Used in JS: $(basename $JS_FILES)"
        fi
        echo ""
    done < "$MISSING"
    
    # Generate INSERT statements
    echo "=== INSERT Statements ==="
    echo ""
    
    MAX_ID=$(sqlite3 "$DB_PATH" "SELECT MAX(resource_id) FROM i18n_resources;")
    NEXT_ID=$((MAX_ID + 1))
    
    while read -r key; do
        CATEGORY=$(echo "$key" | cut -d. -f1)
        EN_VALUE="TODO"
        JA_VALUE="TODO"
        
        echo "-- $key"
        echo "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES ($NEXT_ID, '$key', 'en', '$EN_VALUE', '$CATEGORY', 'TODO', datetime('now'));"
        NEXT_ID=$((NEXT_ID + 1))
        echo "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES ($NEXT_ID, '$key', 'ja', '$JA_VALUE', '$CATEGORY', 'TODO', datetime('now'));"
        NEXT_ID=$((NEXT_ID + 1))
        echo ""
    done < "$MISSING"
fi

# Summary
echo "=== Summary ==="
CODE_COUNT=$(wc -l < "$CODE_KEYS" | tr -d ' ')
DB_COUNT=$(wc -l < "$DB_KEYS" | tr -d ' ')
echo "Keys in code: $CODE_COUNT"
echo "Keys in DB: $DB_COUNT"
echo "Missing: $MISSING_COUNT"

# Cleanup
rm -rf "$TEMP_DIR"

exit 0
