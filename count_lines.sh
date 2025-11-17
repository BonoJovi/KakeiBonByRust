#!/bin/bash
# Count lines of code in the project
# Usage: ./count_lines.sh

echo "================================================"
echo "  KakeiBon - Code Statistics"
echo "================================================"
echo ""

# Rust source code
RUST_LINES=$(find src/ -name "*.rs" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}')
echo "Rust (src/)          : ${RUST_LINES:-0} lines"

# JavaScript
JS_LINES=$(find res/js -name "*.js" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}')
echo "JavaScript (res/js/) : ${JS_LINES:-0} lines"

# HTML
HTML_LINES=$(find res/ -name "*.html" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}')
echo "HTML (res/)          : ${HTML_LINES:-0} lines"

# CSS
CSS_LINES=$(find res/css -name "*.css" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}')
echo "CSS (res/css/)       : ${CSS_LINES:-0} lines"

# SQL
SQL_LINES=$(find sql/ -name "*.sql" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}')
echo "SQL (sql/)           : ${SQL_LINES:-0} lines"

echo ""
echo "------------------------------------------------"

# Calculate total
TOTAL=$((RUST_LINES + JS_LINES + HTML_LINES + CSS_LINES + SQL_LINES))
echo "Total                : ${TOTAL} lines"

echo "================================================"
echo ""

# Optionally output as JSON for programmatic use
if [ "$1" = "--json" ]; then
    echo "{"
    echo "  \"rust\": ${RUST_LINES:-0},"
    echo "  \"javascript\": ${JS_LINES:-0},"
    echo "  \"html\": ${HTML_LINES:-0},"
    echo "  \"css\": ${CSS_LINES:-0},"
    echo "  \"sql\": ${SQL_LINES:-0},"
    echo "  \"total\": ${TOTAL}"
    echo "}"
fi
