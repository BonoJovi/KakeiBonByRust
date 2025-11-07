# SQL Scripts

This directory contains SQL scripts for database initialization and migration.

## Directory Structure

- `phase4/` - Phase 4 (Category Management Frontend) related scripts

## Phase 4 Scripts

### phase4-0_init_categories.sql
Initialize template user (USER_ID=1) category data:
- Update Category1 codes to meaningful names (EXPENSE, INCOME, TRANSFER)
- Add I18N records for Japanese and English

### phase4-0_update_category_codes.sql
Update existing category codes:
- Update Category2/3 codes to use prefixes (C2_E_1, C3_1 format)
- Update I18N tables accordingly

## Usage

Execute with sqlite3:
```bash
# Use the db.sh script (recommended)
../db.sh < sql/phase4/phase4-0_init_categories.sql

# Or specify the full path
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 < sql/phase4/phase4-0_init_categories.sql
```

**Note:** These scripts are for reference and initial setup. After Phase 4-0, 
the database should already be in the correct state.
