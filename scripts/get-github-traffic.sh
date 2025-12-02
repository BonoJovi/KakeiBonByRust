#!/bin/bash
################################################################################
# GitHub Traffic Data Extraction Script
# 
# Description: Fetches clone and view statistics from GitHub API
# Usage: ./get-github-traffic.sh [--json|--csv|--table]
# 
# Created: 2025-12-02
################################################################################

set -euo pipefail

# Configuration
REPO_OWNER="BonoJovi"
REPO_NAME="KakeiBonByRust"
OUTPUT_FORMAT="${1:-table}"  # Default to table format

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

################################################################################
# Functions
################################################################################

print_header() {
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}GitHub Traffic Data - ${REPO_OWNER}/${REPO_NAME}${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo ""
}

fetch_clones() {
    gh api "repos/${REPO_OWNER}/${REPO_NAME}/traffic/clones"
}

fetch_views() {
    gh api "repos/${REPO_OWNER}/${REPO_NAME}/traffic/views"
}

fetch_referrers() {
    gh api "repos/${REPO_OWNER}/${REPO_NAME}/traffic/popular/referrers"
}

fetch_paths() {
    gh api "repos/${REPO_OWNER}/${REPO_NAME}/traffic/popular/paths"
}

format_table() {
    local data_type=$1
    local data=$2
    
    echo -e "${GREEN}${data_type} Statistics:${NC}"
    echo "----------------------------------------"
    printf "%-12s %8s %8s\n" "Date" "Count" "Uniques"
    echo "----------------------------------------"
    
    echo "$data" | jq -r '.'"${data_type,,}"'[] | "\(.timestamp | split("T")[0]) \(.count) \(.uniques)"' | \
    while read -r timestamp count uniques; do
        printf "%-12s %8s %8s\n" "$timestamp" "$count" "$uniques"
    done
    
    # Summary
    local total_count=$(echo "$data" | jq '.count')
    local total_uniques=$(echo "$data" | jq '.uniques')
    echo "----------------------------------------"
    printf "%-12s %8s %8s\n" "TOTAL" "$total_count" "$total_uniques"
    echo ""
}

format_referrers() {
    local data=$1
    
    echo -e "${GREEN}Referrers (Traffic Sources):${NC}"
    echo "----------------------------------------"
    
    local count=$(echo "$data" | jq 'length')
    
    if [ "$count" -eq 0 ]; then
        echo "No referrer data available (direct traffic or not tracked)"
    else
        printf "%-30s %8s %8s\n" "Source" "Count" "Uniques"
        echo "----------------------------------------"
        
        echo "$data" | jq -r '.[] | "\(.referrer) \(.count) \(.uniques)"' | \
        while read -r referrer count uniques; do
            printf "%-30s %8s %8s\n" "$referrer" "$count" "$uniques"
        done
    fi
    
    echo ""
}

format_paths() {
    local data=$1
    
    echo -e "${GREEN}Popular Paths:${NC}"
    echo "----------------------------------------"
    
    local count=$(echo "$data" | jq 'length')
    
    if [ "$count" -eq 0 ]; then
        echo "No path data available"
    else
        printf "%-50s %8s %8s\n" "Path" "Count" "Uniques"
        echo "----------------------------------------"
        
        echo "$data" | jq -r '.[] | "\(.path) \(.count) \(.uniques)"' | \
        while read -r path count uniques; do
            # Truncate path if too long
            local display_path="$path"
            if [ ${#path} -gt 50 ]; then
                display_path="${path:0:47}..."
            fi
            printf "%-50s %8s %8s\n" "$display_path" "$count" "$uniques"
        done
    fi
    
    echo ""
}

format_csv() {
    local data_type=$1
    local data=$2
    
    echo "${data_type},Date,Count,Uniques"
    echo "$data" | jq -r '.'"${data_type,,}"'[] | "\(.timestamp | split("T")[0]),\(.count),\(.uniques)"' | \
    while read -r line; do
        echo "${data_type},$line"
    done
}

format_json() {
    local clones=$1
    local views=$2
    local referrers=$3
    local paths=$4
    
    jq -n \
        --argjson clones "$clones" \
        --argjson views "$views" \
        --argjson referrers "$referrers" \
        --argjson paths "$paths" \
        '{
            timestamp: (now | strftime("%Y-%m-%d %H:%M:%S")),
            repository: "'"${REPO_OWNER}/${REPO_NAME}"'",
            clones: $clones,
            views: $views,
            referrers: $referrers,
            paths: $paths
        }'
}

################################################################################
# Main
################################################################################

main() {
    # Check if gh CLI is available
    if ! command -v gh &> /dev/null; then
        echo -e "${RED}Error: GitHub CLI (gh) is not installed${NC}" >&2
        echo "Install: https://cli.github.com/" >&2
        exit 1
    fi
    
    # Check authentication
    if ! gh auth status &> /dev/null; then
        echo -e "${RED}Error: Not authenticated with GitHub CLI${NC}" >&2
        echo "Run: gh auth login" >&2
        exit 1
    fi
    
    # Fetch data
    echo -e "${YELLOW}Fetching traffic data...${NC}"
    local clones_data=$(fetch_clones)
    local views_data=$(fetch_views)
    local referrers_data=$(fetch_referrers)
    local paths_data=$(fetch_paths)
    
    # Output based on format
    case "$OUTPUT_FORMAT" in
        --json|-j)
            format_json "$clones_data" "$views_data" "$referrers_data" "$paths_data"
            ;;
        --csv|-c)
            format_csv "Clones" "$clones_data"
            format_csv "Views" "$views_data"
            ;;
        --table|-t|*)
            print_header
            format_table "Clones" "$clones_data"
            format_table "Views" "$views_data"
            format_referrers "$referrers_data"
            format_paths "$paths_data"
            
            # Last updated
            echo -e "${YELLOW}Last Updated: $(date '+%Y-%m-%d %H:%M:%S %Z')${NC}"
            ;;
    esac
}

# Show help
if [[ "${1:-}" == "--help" ]] || [[ "${1:-}" == "-h" ]]; then
    cat << EOF
Usage: $0 [OPTIONS]

Fetch GitHub repository traffic statistics (clones and views).

OPTIONS:
    --table, -t     Display in table format (default)
    --json, -j      Output as JSON
    --csv, -c       Output as CSV
    --help, -h      Show this help message

EXAMPLES:
    $0                  # Table format
    $0 --json           # JSON format
    $0 --csv > data.csv # Save as CSV

REQUIREMENTS:
    - GitHub CLI (gh) must be installed and authenticated
    - Repository: ${REPO_OWNER}/${REPO_NAME}
EOF
    exit 0
fi

main
