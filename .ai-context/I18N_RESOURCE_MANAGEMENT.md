# I18N Resource Management Guide

## Overview

This document provides guidelines for managing internationalization (i18n) resources in the KakeiBon project to avoid common pitfalls and ensure consistency.

**Last Updated**: 2025-11-12 JST

---

## Critical Warning: Resource ID Conflicts

### Problem

When adding new i18n resources to `res/sql/dbaccess.sql` or `sql/insert_translation_resources.sql`, **ALWAYS check the current maximum RESOURCE_ID first**. Resource IDs can conflict if multiple developers (including AI assistants like Claude Code) add resources independently.

**Why this matters**:
- The SQL uses `INSERT OR IGNORE` statements
- If a RESOURCE_ID already exists, the new resource will be **silently skipped**
- This leads to missing translations that are hard to debug

### Real Example (2025-11-12)

```
Problem:
- Claude Code added resources with IDs 853-906 (manufacturer/product management)
- We tried to add shop_mgmt resources with IDs 853-882
- Result: shop_mgmt resources were NOT registered (silently skipped)

Solution:
- Checked MAX(RESOURCE_ID) in database → found 906
- Changed shop_mgmt resources to IDs 921-950
- Now they register correctly
```

---

## Step-by-Step: Adding New I18N Resources

### 1. Check Current Maximum RESOURCE_ID

**Before adding any resources**, run:

```bash
# Check database
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "SELECT MAX(RESOURCE_ID) FROM I18N_RESOURCES;"

# OR check the SQL file
grep -E "^\([0-9]+," res/sql/dbaccess.sql | tail -1
```

### 2. Choose Starting ID

- Use `MAX_ID + 1` as your starting ID
- Reserve enough IDs for your feature (e.g., +50 for safety)
- Document the ID range you're using

### 3. Add Resources

Add to **both** files:
1. `res/sql/dbaccess.sql` (primary init file)
2. `sql/insert_translation_resources.sql` (reference/backup)

### 4. Maintain Consistency

- Keep resources organized by category
- Use consistent naming: `category.specific_key`
- Add descriptive comments

---

## Resource ID Allocation (Current Status)

| ID Range | Category | Status | Notes |
|----------|----------|--------|-------|
| 1-726 | Core resources | ✅ Allocated | User, Category, Settings, etc. |
| 727-852 | Account/Transaction Management | ✅ Allocated | Added 2025-11-08 |
| 853-906 | Manufacturer/Product Management | ✅ Allocated | Added by Claude Code |
| 907-920 | Common (IS_DISABLED, etc.) | ✅ Allocated | Added 2025-11-12 |
| 921-950 | Shop Management | ✅ Allocated | Added 2025-11-12 |
| 951+ | **AVAILABLE** | 🆓 Free | Use this range for new resources |

**Important**: Update this table when adding new resources!

---

## Database Initialization Flow

### Files and Execution Order

1. **`res/sql/dbaccess.sql`**
   - Primary database initialization file
   - Executed by `src/db.rs` when creating new database
   - Contains ALL necessary resources for app startup

2. **`sql/insert_translation_resources.sql`**
   - Reference file for manual updates
   - Not automatically executed
   - Keep in sync with `dbaccess.sql`

### When Database is Created

```rust
// src/db.rs
pub async fn initialize(&self) -> Result<(), sqlx::Error> {
    let sql_path = get_sql_file_path();  // -> res/sql/dbaccess.sql
    let sql_content = std::fs::read_to_string(&sql_path)?;
    // Executes all statements in dbaccess.sql
}
```

### Testing New Resources

```bash
# 1. Backup current database
cp ~/.kakeibon/KakeiBonDB.sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3.backup_$(date +%Y%m%d_%H%M%S)

# 2. Delete database to force re-initialization
rm ~/.kakeibon/KakeiBonDB.sqlite3*

# 3. Restart app (creates fresh database from dbaccess.sql)

# 4. Verify resources were added
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "SELECT RESOURCE_KEY, RESOURCE_VALUE FROM I18N_RESOURCES WHERE RESOURCE_ID BETWEEN 951 AND 970;"
```

---

## Common Pitfalls

### 1. WAL File Issues

**Symptom**: Changes in `dbaccess.sql` don't appear in database

**Cause**: Write-Ahead Log (WAL) not checkpointed

**Solution**:
```bash
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "PRAGMA wal_checkpoint(FULL);"
```

### 2. Hardcoded Text in JavaScript

**Problem**:
```javascript
// ❌ BAD: Hardcoded English text
tbody.innerHTML = `<tr><td colspan="5">No accounts found</td></tr>`;
```

**Solution**:
```javascript
// ✅ GOOD: Use i18n
tbody.innerHTML = `<tr><td colspan="5">${i18n.t('account_mgmt.no_accounts')}</td></tr>`;
```

### 3. Duplicate Resource Keys

Always check if a resource key already exists before adding:

```bash
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "SELECT * FROM I18N_RESOURCES WHERE RESOURCE_KEY = 'shop_mgmt.title';"
```

---

## Resource Naming Conventions

### Pattern

```
<category>.<specific_key>
```

### Categories

- `menu.*` - Menu items (File, Language, etc.)
- `common.*` - Common UI elements (buttons, labels)
- `user_mgmt.*` - User management screen
- `account_mgmt.*` - Account management screen
- `category_mgmt.*` - Category management screen
- `shop_mgmt.*` - Shop management screen
- `manufacturer_mgmt.*` - Manufacturer management screen
- `product_mgmt.*` - Product management screen
- `transaction_mgmt.*` - Transaction management screen

### Key Naming

- `title` - Page/modal title
- `add_new` - Add new item button
- `modal_title_add` - Add modal title
- `modal_title_edit` - Edit modal title
- `no_<items>` - Empty state message
- `delete_confirmation` - Delete modal title
- `delete_message` - Delete confirmation message
- `failed_to_<action>` - Error messages

---

## Checklist for Adding I18N Resources

- [ ] Check current `MAX(RESOURCE_ID)` in database
- [ ] Choose appropriate starting ID (MAX_ID + 1 or higher)
- [ ] Add resources to `res/sql/dbaccess.sql`
- [ ] Add same resources to `sql/insert_translation_resources.sql`
- [ ] Update this document's "Resource ID Allocation" table
- [ ] Test by recreating database from scratch
- [ ] Verify all resources appear correctly in UI
- [ ] Commit both SQL files together

---

## Reference: SQL File Structure

### res/sql/dbaccess.sql

```sql
-- ============================================================================
-- I18N Resources Initial Data

-- Translation resources for Account Management, Transaction Management, and Shop Management
-- Starting from RESOURCE_ID 727

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
-- Account Management (727-746)
(727, 'account_mgmt.title', 'en', 'Account Management', 'account_mgmt', 'Account management page title', datetime('now')),
(728, 'account_mgmt.title', 'ja', '口座管理', 'account_mgmt', '口座管理ページタイトル', datetime('now')),
-- ... more resources ...

-- Shop Management (921-950)
(921, 'menu.shop_management', 'en', 'Shop Management', 'menu', 'Shop management menu item', datetime('now')),
(922, 'menu.shop_management', 'ja', '店舗管理', 'menu', '店舗管理メニュー項目', datetime('now')),
-- ... more resources ...
```

---

## For AI Assistants (Claude, etc.)

**When asked to add i18n resources**:

1. ✅ DO: Check `MAX(RESOURCE_ID)` first
2. ✅ DO: Ask user to confirm starting ID
3. ✅ DO: Update both `dbaccess.sql` and `insert_translation_resources.sql`
4. ✅ DO: Provide SQL to verify resources were added
5. ❌ DON'T: Assume ID ranges are available
6. ❌ DON'T: Reuse existing IDs
7. ❌ DON'T: Add resources without checking conflicts

---

## Troubleshooting

### Resources not showing in UI

1. Check if resources exist in database:
   ```bash
   sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "SELECT * FROM I18N_RESOURCES WHERE RESOURCE_KEY LIKE 'shop_mgmt.%';"
   ```

2. Check resource count:
   ```bash
   sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "SELECT COUNT(*), MAX(RESOURCE_ID) FROM I18N_RESOURCES;"
   ```

3. Check for ID conflicts:
   ```bash
   sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "SELECT RESOURCE_ID, COUNT(*) FROM I18N_RESOURCES GROUP BY RESOURCE_ID HAVING COUNT(*) > 2;"
   ```

4. Force checkpoint and verify:
   ```bash
   sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "PRAGMA wal_checkpoint(FULL);"
   ```

5. If still not working, recreate database from scratch

---

## See Also

- [I18N Implementation](../docs/ja/I18N_IMPLEMENTATION.md)
- [I18N Resources](../docs/ja/I18N_RESOURCES.md)
- [Database Configuration](../docs/ja/DATABASE_CONFIGURATION.md)
