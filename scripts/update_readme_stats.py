#!/usr/bin/env python3
"""
Update README.md with latest statistics.
"""

import json
import re
from datetime import datetime

STATS_FILE = 'stats_data.json'
README_FILE = 'README.md'

def load_stats():
    """Load stats from JSON file"""
    with open(STATS_FILE, 'r') as f:
        return json.load(f)

def format_number(num):
    """Format number with comma separators"""
    return f'{num:,}'

def update_readme(stats):
    """Update README.md with stats section"""
    with open(README_FILE, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Prepare stats section
    last_updated = datetime.fromisoformat(stats['last_updated']).strftime('%Y-%m-%d %H:%M UTC')
    
    stats_section = f'''<!-- STATS_START -->
## 📊 Repository Statistics / リポジトリ統計

<div align="center">

### 📈 Daily Traffic / 日次トラフィック

![Daily Traffic Stats](docs/stats_graph_daily.png)

### 📊 Cumulative Traffic / 累積トラフィック

![Cumulative Traffic Stats](docs/stats_graph_cumulative.png)

| Metric | Count |
|--------|-------|
| 👁️ **Total Views** / 総閲覧数 | **{format_number(stats['total_views'])}** |
| 📦 **Total Clones** / 総クローン数 | **{format_number(stats['total_clones'])}** |

*Last Updated / 最終更新: {last_updated}*

</div>
<!-- STATS_END -->'''
    
    # Check if stats section exists
    if '<!-- STATS_START -->' in content:
        # Replace existing section
        pattern = r'<!-- STATS_START -->.*?<!-- STATS_END -->'
        content = re.sub(pattern, stats_section, content, flags=re.DOTALL)
    else:
        # Insert after development status section
        insert_marker = '---\n\n## 📚 Legacy Version'
        if insert_marker in content:
            content = content.replace(insert_marker, f'{stats_section}\n\n---\n\n## 📚 Legacy Version')
        else:
            # Fallback: append at end
            content += f'\n\n{stats_section}\n'
    
    # Write back
    with open(README_FILE, 'w', encoding='utf-8') as f:
        f.write(content)
    
    print('README.md updated with stats')

def main():
    stats = load_stats()
    update_readme(stats)

if __name__ == '__main__':
    main()
