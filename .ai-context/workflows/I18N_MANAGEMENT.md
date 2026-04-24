# I18N Resource Management

## Critical: RESOURCE_ID Conflicts

`INSERT OR IGNORE` silently skips duplicate IDs. **Always check MAX ID first.**

```bash
grep -oP '^\(\K[0-9]+' res/sql/dbaccess.sql | sort -n | tail -1
```

## Adding Resources

1. Check current MAX RESOURCE_ID
2. Use `MAX_ID + 1` as starting ID
3. Add to **both** files:
   - `res/sql/dbaccess.sql` (primary — executed on DB init)
   - Corresponding `sql/init/i18n/init_*_i18n.sql` (categorized reference)

## Key Naming

Pattern: `<category>.<specific_key>`

Categories: `menu.*`, `common.*`, `user_mgmt.*`, `account_mgmt.*`, `category_mgmt.*`, `shop_mgmt.*`, `manufacturer_mgmt.*`, `product_mgmt.*`, `transaction_mgmt.*`, `aggregation.*`

## ID Allocation

| Range | Category |
|-------|----------|
| 1-726 | Core (User, Category, Settings) |
| 727-852 | Account/Transaction |
| 853-906 | Manufacturer/Product |
| 907-920 | Common (IS_DISABLED) |
| 921-950 | Shop |
| 951-2068 | Transaction details, dashboard |
| 2069-2086 | Aggregation errors, scheduled |
| 2087+ | Available |

## DB Init Flow

`src/db.rs` reads `res/sql/dbaccess.sql` on first launch to create all tables and insert resources.

## Verify

```bash
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "SELECT COUNT(*), MAX(RESOURCE_ID) FROM I18N_RESOURCES;"
```
