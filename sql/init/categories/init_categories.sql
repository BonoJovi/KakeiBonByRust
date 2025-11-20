-- Phase 4-0: Initialize template user category data
-- Update existing Category1 codes and add I18N records

-- Step 1: Update Category1 codes to meaningful names
UPDATE CATEGORY1 SET category1_code = 'EXPENSE' WHERE user_id = 1 AND category1_code = '1';
UPDATE CATEGORY1 SET category1_code = 'INCOME' WHERE user_id = 1 AND category1_code = '2';
UPDATE CATEGORY1 SET category1_code = 'TRANSFER' WHERE user_id = 1 AND category1_code = '3';

-- Step 2: Add I18N records for Category1 (Japanese)
INSERT OR REPLACE INTO CATEGORY1_I18N (user_id, category1_code, lang_code, category1_name_i18n, entry_dt)
VALUES
  (1, 'EXPENSE', 'ja', '支出', datetime('now')),
  (1, 'INCOME', 'ja', '収入', datetime('now')),
  (1, 'TRANSFER', 'ja', '振替', datetime('now'));

-- Step 3: Add I18N records for Category1 (English)
INSERT OR REPLACE INTO CATEGORY1_I18N (user_id, category1_code, lang_code, category1_name_i18n, entry_dt)
VALUES
  (1, 'EXPENSE', 'en', 'Expense', datetime('now')),
  (1, 'INCOME', 'en', 'Income', datetime('now')),
  (1, 'TRANSFER', 'en', 'Transfer', datetime('now'));

-- Verification queries
SELECT '=== Category1 Data ===' as Info;
SELECT user_id, category1_code, display_order, category1_name, is_disabled 
FROM CATEGORY1 
WHERE user_id = 1 
ORDER BY display_order;

SELECT '=== Category1 I18N Data ===' as Info;
SELECT user_id, category1_code, lang_code, category1_name_i18n 
FROM CATEGORY1_I18N 
WHERE user_id = 1 
ORDER BY category1_code, lang_code;
