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

    # Determine date range: last 14 days (same as GitHub Traffic)
    all_dates = []
    if not views_df.empty:
        all_dates.extend(views_df['timestamp'].tolist())
    if not clones_df.empty:
        all_dates.extend(clones_df['timestamp'].tolist())

    if all_dates:
        max_date = max(all_dates)
        # Show last 14 days of data
        min_date = max_date - pd.Timedelta(days=13)  # 14 days including max_date

        # Filter data to last 14 days
        if not views_df.empty:
            views_df = views_df[views_df['timestamp'] >= min_date]
        if not clones_df.empty:
            clones_df = clones_df[clones_df['timestamp'] >= min_date]

        # Add padding: 1 day before and after for display
        x_min = min_date - pd.Timedelta(days=1)
        x_max = max_date + pd.Timedelta(days=1)
    else:
        x_min = x_max = None
        min_date = max_date = None

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
        if x_min and x_max:
            ax.set_xlim(x_min, x_max)
        ax.xaxis.set_major_formatter(mdates.DateFormatter('%Y-%m-%d'))
        # Show daily ticks for 14-day period
        ax.xaxis.set_major_locator(mdates.DayLocator(interval=1))
        plt.setp(ax.xaxis.get_majorticklabels(), rotation=45, ha='right')

        # Hide edge labels (leftmost and rightmost date labels)
        labels = ax.get_xticklabels()
        if len(labels) > 2:
            labels[0].set_visible(False)
            labels[-1].set_visible(False)

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

    if not clones_df.empty:
        clones_df['timestamp'] = pd.to_datetime(clones_df['timestamp'])
        clones_df = clones_df.sort_values('timestamp')

    # Determine date range: last 14 days (same as GitHub Traffic)
    all_dates = []
    if not views_df.empty:
        all_dates.extend(views_df['timestamp'].tolist())
    if not clones_df.empty:
        all_dates.extend(clones_df['timestamp'].tolist())

    if all_dates:
        max_date = max(all_dates)
        # Show last 14 days of data
        min_date = max_date - pd.Timedelta(days=13)  # 14 days including max_date

        # Filter data to last 14 days
        if not views_df.empty:
            views_df = views_df[views_df['timestamp'] >= min_date]
            views_df['cumulative'] = views_df['count'].cumsum()
        if not clones_df.empty:
            clones_df = clones_df[clones_df['timestamp'] >= min_date]
            clones_df['cumulative'] = clones_df['count'].cumsum()

        # Add padding: 1 day before and after for display
        x_min = min_date - pd.Timedelta(days=1)
        x_max = max_date + pd.Timedelta(days=1)
    else:
        x_min = x_max = None
        min_date = max_date = None

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
        if x_min and x_max:
            ax.set_xlim(x_min, x_max)
        ax.xaxis.set_major_formatter(mdates.DateFormatter('%Y-%m-%d'))
        # Show daily ticks for 14-day period
        ax.xaxis.set_major_locator(mdates.DayLocator(interval=1))
        plt.setp(ax.xaxis.get_majorticklabels(), rotation=45, ha='right')

        # Hide edge labels (leftmost and rightmost date labels)
        labels = ax.get_xticklabels()
        if len(labels) > 2:
            labels[0].set_visible(False)
            labels[-1].set_visible(False)

    plt.tight_layout()
    plt.savefig(OUTPUT_FILE_CUMULATIVE, dpi=150, bbox_inches='tight')
    print(f'Cumulative graph saved to {OUTPUT_FILE_CUMULATIVE}')

def main():
    data = load_stats()
    generate_daily_graph(data)
    generate_cumulative_graph(data)

if __name__ == '__main__':
    main()
