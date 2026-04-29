# Add i18n Resources

Procedure for safely adding new i18n resources without ID conflicts.

## Steps

### 1. Check Current Max RESOURCE_ID

```bash
grep -oP '^\(\K[0-9]+' res/sql/dbaccess.sql | sort -n | tail -1
```

Or from database:
```bash
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "SELECT MAX(RESOURCE_ID) FROM I18N_RESOURCES;"
```

### 2. Choose Starting ID

Use `MAX_ID + 1`. Reserve enough IDs for the feature.

### 3. Add to BOTH Files

1. `res/sql/dbaccess.sql` (primary — executed on DB init)
2. Corresponding file in `sql/init/i18n/` (categorized reference)

Both files must have the same entries.

### 4. Format

```sql
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
(ID, 'category.key', 'en', 'English text', 'category', 'description', datetime('now')),
(ID+1, 'category.key', 'ja', 'Japanese text', 'category', 'description', datetime('now'));
```

### 5. Key Naming Convention

`<category>.<specific_key>` — e.g., `shop_mgmt.title`, `aggregation.error_invalid_year`

### 6. Verify

Delete DB and restart app, or:
```bash
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "SELECT RESOURCE_KEY, RESOURCE_VALUE FROM I18N_RESOURCES WHERE RESOURCE_ID >= NEW_START_ID;"
```

## Critical Warning

`INSERT OR IGNORE` silently skips duplicate IDs. Always check max ID first!
