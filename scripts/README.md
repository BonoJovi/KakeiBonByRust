# Repository Stats Scripts

This directory contains scripts for automatically collecting and visualizing repository statistics.

## Scripts

### `fetch_stats.py`
- Fetches traffic statistics from GitHub API (views and clones)
- Accumulates historical data in a Gist
- Stores data locally for graph generation

### `generate_stats_graph.py`
- Generates traffic statistics graph from accumulated data
- Creates a time-series visualization of views and clones
- Saves graph to `docs/stats_graph.png`

### `update_readme_stats.py`
- Updates README.md with latest statistics
- Inserts/updates the statistics section automatically
- Includes graph and total counts

## Setup

### 1. Create a Personal Access Token (PAT)
1. Go to GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Generate new token with permissions:
   - `repo` (for private repos) or `public_repo` (for public repos)
   - `gist` (to create/update Gist)

### 2. Add Secrets to Repository
1. Go to repository Settings → Secrets and variables → Actions
2. Add the following secrets:
   - `GITHUB_TOKEN` (automatically available, no setup needed)
   - `GIST_ID` (will be auto-generated on first run, then add manually)

### 3. First Run
1. Trigger workflow manually: Actions → Update Repository Stats → Run workflow
2. Check the logs for the generated `GIST_ID`
3. Add `GIST_ID` as a repository secret (optional, for persistence)

## How It Works

```
┌─────────────────────────────────────────────────────────┐
│ GitHub Actions (Daily at 9:00 AM JST)                  │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ 1. Fetch current traffic stats (past 14 days)          │
│    - Views per day                                      │
│    - Clones per day                                     │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ 2. Load historical data from Gist                      │
│    - Merge with current data                            │
│    - Accumulate totals                                  │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ 3. Generate graph (matplotlib)                          │
│    - Time-series chart                                  │
│    - Save to docs/stats_graph.png                       │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ 4. Update README.md                                     │
│    - Embed graph                                        │
│    - Show totals                                        │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│ 5. Commit and push changes                              │
└─────────────────────────────────────────────────────────┘
```

## Dependencies

```bash
pip install requests matplotlib pandas
```

## Manual Testing

```bash
# Set environment variables
export GITHUB_TOKEN="your_token"
export GITHUB_REPOSITORY="BonoJovi/KakeiBonByRust"
export GIST_ID="your_gist_id"  # Optional

# Run scripts
python scripts/fetch_stats.py
python scripts/generate_stats_graph.py
python scripts/update_readme_stats.py
```

## Notes

- GitHub API only provides the last 14 days of traffic data
- Gist stores accumulated historical data indefinitely
- The workflow runs daily to capture all data points
- Use `[skip ci]` in commit messages to avoid triggering other workflows
