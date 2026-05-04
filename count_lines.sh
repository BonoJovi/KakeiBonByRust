#!/bin/bash
# Count lines of code and documentation in the project
# Usage: ./count_lines.sh [--json]

echo "================================================"
echo "  KakeiBon - Code & Documentation Statistics"
echo "================================================"
echo ""

# --- Source code ---
RUST_LINES=$(find src/ -name "*.rs" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}')
echo "Rust (src/)              : ${RUST_LINES:-0} lines"

JS_LINES=$(find res/js -name "*.js" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}')
echo "JavaScript (res/js/)     : ${JS_LINES:-0} lines"

HTML_LINES=$(find res/ -name "*.html" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}')
echo "HTML (res/)              : ${HTML_LINES:-0} lines"

CSS_LINES=$(find res/css -name "*.css" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}')
echo "CSS (res/css/)           : ${CSS_LINES:-0} lines"

# SQL: combine sql/ (legacy migration scripts) and res/sql/ (live DDL)
SQL_LINES=$(find sql/ res/sql/ -name "*.sql" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}')
echo "SQL (sql/ + res/sql/)    : ${SQL_LINES:-0} lines"

# --- Documentation ---
MD_ROOT_LINES=$(find . -maxdepth 1 -name "*.md" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}')
echo "Markdown (root *.md)     : ${MD_ROOT_LINES:-0} lines"

DOCS_LINES=$(find docs/ -name "*.md" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}')
echo "Documentation (docs/*.md): ${DOCS_LINES:-0} lines"

echo ""
echo "------------------------------------------------"

# --- Subtotals ---
CODE_TOTAL=$((${RUST_LINES:-0} + ${JS_LINES:-0} + ${HTML_LINES:-0} + ${CSS_LINES:-0} + ${SQL_LINES:-0}))
DOC_TOTAL=$((${MD_ROOT_LINES:-0} + ${DOCS_LINES:-0}))
GRAND_TOTAL=$((CODE_TOTAL + DOC_TOTAL))

echo "Code total               : ${CODE_TOTAL} lines"
echo "Documentation total      : ${DOC_TOTAL} lines"
echo "Grand total              : ${GRAND_TOTAL} lines"

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
    echo "  \"markdown_root\": ${MD_ROOT_LINES:-0},"
    echo "  \"docs\": ${DOCS_LINES:-0},"
    echo "  \"code_total\": ${CODE_TOTAL},"
    echo "  \"doc_total\": ${DOC_TOTAL},"
    echo "  \"grand_total\": ${GRAND_TOTAL}"
    echo "}"
fi
