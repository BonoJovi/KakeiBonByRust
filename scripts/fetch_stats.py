#!/usr/bin/env python3
"""
Fetch GitHub repository traffic statistics and accumulate historical data.
Data is stored in a Gist for persistence across workflow runs.
"""

import os
import sys
import json
import requests
from datetime import datetime

REPO = os.environ.get('GITHUB_REPOSITORY', 'BonoJovi/KakeiBonByRust')
TOKEN = os.environ.get('GITHUB_TOKEN', '').strip()  # Remove whitespace/newlines
GIST_ID = os.environ.get('GIST_ID', '').strip()  # Remove whitespace/newlines
STATS_FILE = 'stats_history.json'

def fetch_github_traffic(endpoint):
    """Fetch traffic data from GitHub API"""
    url = f'https://api.github.com/repos/{REPO}/traffic/{endpoint}'
    headers = {'Authorization': f'token {TOKEN}'}
    response = requests.get(url, headers=headers)
    response.raise_for_status()
    return response.json()

def fetch_gist_data():
    """Fetch historical data from Gist"""
    if not GIST_ID:
        return {'views': [], 'clones': []}
    
    url = f'https://api.github.com/gists/{GIST_ID}'
    headers = {'Authorization': f'token {TOKEN}'}
    try:
        response = requests.get(url, headers=headers)
        response.raise_for_status()
        gist_data = response.json()
        content = gist_data['files'][STATS_FILE]['content']
        return json.loads(content)
    except (requests.exceptions.RequestException, KeyError):
        return {'views': [], 'clones': []}

def update_gist_data(data):
    """Update or create Gist with new data"""
    content = json.dumps(data, indent=2)
    
    if GIST_ID:
        # Update existing Gist
        url = f'https://api.github.com/gists/{GIST_ID}'
        payload = {
            'files': {
                STATS_FILE: {'content': content}
            }
        }
    else:
        # Create new Gist
        url = 'https://api.github.com/gists'
        payload = {
            'description': f'Traffic stats for {REPO}',
            'public': False,
            'files': {
                STATS_FILE: {'content': content}
            }
        }
    
    headers = {'Authorization': f'token {TOKEN}'}
    response = requests.post(url, headers=headers, json=payload)
    response.raise_for_status()
    
    if not GIST_ID:
        new_gist_id = response.json()['id']
        print(f'Created new Gist: {new_gist_id}')
        print(f'Please add this to your workflow as GIST_ID secret: {new_gist_id}')

def accumulate_stats(historical, current, key='timestamp'):
    """Merge current stats with historical data, avoiding duplicates"""
    existing_keys = {item[key] for item in historical}
    
    for item in current:
        if item[key] not in existing_keys:
            historical.append(item)
    
    # Sort by timestamp
    historical.sort(key=lambda x: x[key])
    return historical

def main():
    if not TOKEN:
        print('Error: GITHUB_TOKEN not found', file=sys.stderr)
        sys.exit(1)
    
    print('Fetching traffic statistics...')
    
    # Fetch current data from GitHub
    views_data = fetch_github_traffic('views')
    clones_data = fetch_github_traffic('clones')
    
    # Fetch historical data from Gist
    historical_data = fetch_gist_data()
    
    # Accumulate data
    historical_data['views'] = accumulate_stats(
        historical_data.get('views', []),
        views_data.get('views', [])
    )
    historical_data['clones'] = accumulate_stats(
        historical_data.get('clones', []),
        clones_data.get('clones', [])
    )
    
    # Add totals
    historical_data['total_views'] = sum(v['count'] for v in historical_data['views'])
    historical_data['total_clones'] = sum(c['count'] for c in historical_data['clones'])
    historical_data['last_updated'] = datetime.utcnow().isoformat()
    
    # Save locally for graph generation
    with open('stats_data.json', 'w') as f:
        json.dump(historical_data, f, indent=2)
    
    # Update Gist
    update_gist_data(historical_data)
    
    print(f'Total views: {historical_data["total_views"]}')
    print(f'Total clones: {historical_data["total_clones"]}')
    print('Stats updated successfully!')

if __name__ == '__main__':
    main()
