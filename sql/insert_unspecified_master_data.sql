-- Insert "Unspecified" master data for transactions
-- This allows NOT NULL constraints while supporting optional fields
-- Created: 2025-01-07

-- 1. Add "Unspecified" account template
INSERT OR IGNORE INTO ACCOUNT_TEMPLATES (TEMPLATE_CODE, TEMPLATE_NAME_JA, TEMPLATE_NAME_EN, DISPLAY_ORDER)
VALUES ('NONE', '指定なし', 'Unspecified', 0);

-- 2. Add "Unspecified" account for each user
INSERT OR IGNORE INTO ACCOUNTS (USER_ID, ACCOUNT_CODE, ACCOUNT_NAME, TEMPLATE_CODE, INITIAL_BALANCE, DISPLAY_ORDER, IS_DISABLED, ENTRY_DT)
SELECT USER_ID, 'NONE', '指定なし', 'NONE', 0, 0, 0, datetime('now')
FROM USERS;

-- 3. Add "Unspecified" CATEGORY2 for each CATEGORY1
INSERT OR IGNORE INTO CATEGORY2 (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY2_NAME, DISPLAY_ORDER, ENTRY_DT)
SELECT USER_ID, CATEGORY1_CODE, 'NONE', '指定なし', 0, datetime('now')
FROM CATEGORY1;

-- 4. Add "Unspecified" CATEGORY3 for each CATEGORY2
INSERT OR IGNORE INTO CATEGORY3 (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, CATEGORY3_NAME, DISPLAY_ORDER, ENTRY_DT)
SELECT USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, 'NONE', '指定なし', 0, datetime('now')
FROM CATEGORY2;

-- 5. Add i18n resources for "Unspecified"
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, ENTRY_DT)
VALUES 
    ('common.unspecified', 'en', 'Unspecified', datetime('now')),
    ('common.unspecified', 'ja', '指定なし', datetime('now'));

