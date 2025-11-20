# SQL Scripts

This directory contains SQL scripts for database initialization and migration.

## Directory Structure

- `init/` - Initialization scripts for database setup
  - `categories/` - Category master data initialization
  - `i18n/` - Translation resources initialization
- Root directory - Migration scripts and other utilities

## Initialization Scripts

### init/categories/init_categories.sql
Initialize template user (USER_ID=1) category data:
- Update Category1 codes to meaningful names (EXPENSE, INCOME, TRANSFER)
- Add I18N records for Japanese and English

### init/categories/update_category_codes.sql
Update existing category codes:
- Update Category2/3 codes to use prefixes (C2_E_1, C3_1 format)
- Update I18N tables accordingly

### init/i18n/init_aggregation_i18n.sql
Initialize translation resources for Monthly Aggregation feature:
- Add I18N records for aggregation screen (menu, filters, results)
- Resource IDs: 1053-1098 (46 records)

## Usage

Execute with sqlite3:
```bash
# Use the db.sh script (recommended)
./db.sh < sql/init/categories/init_categories.sql
./db.sh < sql/init/i18n/init_aggregation_i18n.sql

# Or specify the full path
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 < sql/init/categories/init_categories.sql
```

**Note:** These scripts are for reference and initial setup.
