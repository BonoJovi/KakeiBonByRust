#!/usr/bin/env python3
"""
Generate traffic statistics graphs (daily and cumulative) from accumulated data.
"""

import json
import matplotlib.pyplot as plt
import matplotlib.dates as mdates
from datetime import datetime
import pandas as pd
import numpy as np

STATS_FILE = 'stats_data.json'
OUTPUT_FILE_DAILY = 'docs/stats_graph_daily.png'
OUTPUT_FILE_CUMULATIVE = 'docs/stats_graph_cumulative.png'

def load_stats():
    """Load stats from JSON file"""
    with open(STATS_FILE, 'r') as f:
        return json.load(f)

def generate_daily_graph(data):
    """Generate daily traffic statistics graph"""
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
    
    # Create figure with reduced height (5.5 instead of 8)
    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(12, 5.5))
    fig.suptitle('Daily Repository Traffic', fontsize=14, fontweight='bold')
    
    # Plot views
    if not views_df.empty:
        ax1.plot(views_df['timestamp'], views_df['count'], 
                marker='o', linestyle='-', color='#2196F3', linewidth=2, markersize=4)
        ax1.fill_between(views_df['timestamp'], views_df['count'], alpha=0.3, color='#2196F3')
        ax1.set_ylabel('Daily Views', fontsize=11, fontweight='bold')
        ax1.grid(True, alpha=0.3)
        ax1.set_title(f'Total Views: {data["total_views"]:,}', fontsize=11)
    
    # Plot clones
    if not clones_df.empty:
        ax2.plot(clones_df['timestamp'], clones_df['count'],
                marker='s', linestyle='-', color='#4CAF50', linewidth=2, markersize=4)
        ax2.fill_between(clones_df['timestamp'], clones_df['count'], alpha=0.3, color='#4CAF50')
        ax2.set_ylabel('Daily Clones', fontsize=11, fontweight='bold')
        ax2.set_xlabel('Date', fontsize=11, fontweight='bold')
        ax2.grid(True, alpha=0.3)
        ax2.set_title(f'Total Clones: {data["total_clones"]:,}', fontsize=11)
    
    # Format x-axis
    for ax in [ax1, ax2]:
        ax.xaxis.set_major_formatter(mdates.DateFormatter('%Y-%m-%d'))
        ax.xaxis.set_major_locator(mdates.AutoDateLocator())
        plt.setp(ax.xaxis.get_majorticklabels(), rotation=45, ha='right')
    
    plt.tight_layout()
    plt.savefig(OUTPUT_FILE_DAILY, dpi=150, bbox_inches='tight')
    print(f'Daily graph saved to {OUTPUT_FILE_DAILY}')

def generate_cumulative_graph(data):
    """Generate cumulative traffic statistics graph"""
    if not data['views'] and not data['clones']:
        print('No data to plot')
        return
    
    # Convert to pandas DataFrame
    views_df = pd.DataFrame(data['views'])
    clones_df = pd.DataFrame(data['clones'])
    
    if not views_df.empty:
        views_df['timestamp'] = pd.to_datetime(views_df['timestamp'])
        views_df = views_df.sort_values('timestamp')
        views_df['cumulative'] = views_df['count'].cumsum()
    
    if not clones_df.empty:
        clones_df['timestamp'] = pd.to_datetime(clones_df['timestamp'])
        clones_df = clones_df.sort_values('timestamp')
        clones_df['cumulative'] = clones_df['count'].cumsum()
    
    # Create figure with reduced height
    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(12, 5.5))
    fig.suptitle('Cumulative Repository Traffic', fontsize=14, fontweight='bold')
    
    # Plot cumulative views
    if not views_df.empty:
        ax1.plot(views_df['timestamp'], views_df['cumulative'], 
                marker='o', linestyle='-', color='#2196F3', linewidth=2, markersize=4)
        ax1.fill_between(views_df['timestamp'], views_df['cumulative'], alpha=0.3, color='#2196F3')
        ax1.set_ylabel('Cumulative Views', fontsize=11, fontweight='bold')
        ax1.grid(True, alpha=0.3)
        ax1.set_title(f'Total Views: {data["total_views"]:,}', fontsize=11)
    
    # Plot cumulative clones
    if not clones_df.empty:
        ax2.plot(clones_df['timestamp'], clones_df['cumulative'],
                marker='s', linestyle='-', color='#4CAF50', linewidth=2, markersize=4)
        ax2.fill_between(clones_df['timestamp'], clones_df['cumulative'], alpha=0.3, color='#4CAF50')
        ax2.set_ylabel('Cumulative Clones', fontsize=11, fontweight='bold')
        ax2.set_xlabel('Date', fontsize=11, fontweight='bold')
        ax2.grid(True, alpha=0.3)
        ax2.set_title(f'Total Clones: {data["total_clones"]:,}', fontsize=11)
    
    # Format x-axis
    for ax in [ax1, ax2]:
        ax.xaxis.set_major_formatter(mdates.DateFormatter('%Y-%m-%d'))
        ax.xaxis.set_major_locator(mdates.AutoDateLocator())
        plt.setp(ax.xaxis.get_majorticklabels(), rotation=45, ha='right')
    
    plt.tight_layout()
    plt.savefig(OUTPUT_FILE_CUMULATIVE, dpi=150, bbox_inches='tight')
    print(f'Cumulative graph saved to {OUTPUT_FILE_CUMULATIVE}')

def main():
    data = load_stats()
    generate_daily_graph(data)
    generate_cumulative_graph(data)

if __name__ == '__main__':
    main()
