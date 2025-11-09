#!/usr/bin/env python3
"""
Generate traffic statistics graph from accumulated data.
"""

import json
import matplotlib.pyplot as plt
import matplotlib.dates as mdates
from datetime import datetime
import pandas as pd

STATS_FILE = 'stats_data.json'
OUTPUT_FILE = 'docs/stats_graph.png'

def load_stats():
    """Load stats from JSON file"""
    with open(STATS_FILE, 'r') as f:
        return json.load(f)

def generate_graph(data):
    """Generate traffic statistics graph"""
    if not data['views'] and not data['clones']:
        print('No data to plot')
        return
    
    # Convert to pandas DataFrame
    views_df = pd.DataFrame(data['views'])
    clones_df = pd.DataFrame(data['clones'])
    
    if not views_df.empty:
        views_df['timestamp'] = pd.to_datetime(views_df['timestamp'])
    if not clones_df.empty:
        clones_df['timestamp'] = pd.to_datetime(clones_df['timestamp'])
    
    # Create figure
    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(12, 8))
    fig.suptitle('Repository Traffic Statistics', fontsize=16, fontweight='bold')
    
    # Plot views
    if not views_df.empty:
        ax1.plot(views_df['timestamp'], views_df['count'], 
                marker='o', linestyle='-', color='#2196F3', linewidth=2, markersize=4)
        ax1.fill_between(views_df['timestamp'], views_df['count'], alpha=0.3, color='#2196F3')
        ax1.set_ylabel('Views', fontsize=12, fontweight='bold')
        ax1.grid(True, alpha=0.3)
        ax1.set_title(f'Total Views: {data["total_views"]:,}', fontsize=12)
    
    # Plot clones
    if not clones_df.empty:
        ax2.plot(clones_df['timestamp'], clones_df['count'],
                marker='s', linestyle='-', color='#4CAF50', linewidth=2, markersize=4)
        ax2.fill_between(clones_df['timestamp'], clones_df['count'], alpha=0.3, color='#4CAF50')
        ax2.set_ylabel('Clones', fontsize=12, fontweight='bold')
        ax2.set_xlabel('Date', fontsize=12, fontweight='bold')
        ax2.grid(True, alpha=0.3)
        ax2.set_title(f'Total Clones: {data["total_clones"]:,}', fontsize=12)
    
    # Format x-axis
    for ax in [ax1, ax2]:
        ax.xaxis.set_major_formatter(mdates.DateFormatter('%Y-%m-%d'))
        ax.xaxis.set_major_locator(mdates.AutoDateLocator())
        plt.setp(ax.xaxis.get_majorticklabels(), rotation=45, ha='right')
    
    plt.tight_layout()
    plt.savefig(OUTPUT_FILE, dpi=150, bbox_inches='tight')
    print(f'Graph saved to {OUTPUT_FILE}')

def main():
    data = load_stats()
    generate_graph(data)

if __name__ == '__main__':
    main()
