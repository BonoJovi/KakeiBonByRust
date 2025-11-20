-- Phase 4-0: Update Category2 and Category3 codes
-- Update category1_code references from numeric to text codes

-- Disable foreign keys temporarily
PRAGMA foreign_keys = OFF;

-- Step 1: Update Category2 codes
-- Update category1_code references
UPDATE CATEGORY2 SET category1_code = 'EXPENSE' WHERE user_id = 1 AND category1_code = '1';
UPDATE CATEGORY2 SET category1_code = 'INCOME' WHERE user_id = 1 AND category1_code = '2';
UPDATE CATEGORY2 SET category1_code = 'TRANSFER' WHERE user_id = 1 AND category1_code = '3';

-- Update category2_code to use meaningful prefixes
UPDATE CATEGORY2 SET category2_code = 'C2_E_' || category2_code WHERE user_id = 1 AND category1_code = 'EXPENSE' AND category2_code NOT LIKE 'C2_%';
UPDATE CATEGORY2 SET category2_code = 'C2_I_' || category2_code WHERE user_id = 1 AND category1_code = 'INCOME' AND category2_code NOT LIKE 'C2_%';
UPDATE CATEGORY2 SET category2_code = 'C2_T_' || category2_code WHERE user_id = 1 AND category1_code = 'TRANSFER' AND category2_code NOT LIKE 'C2_%';

-- Step 2: Update Category3 codes (if any exist)
UPDATE CATEGORY3 SET category1_code = 'EXPENSE' WHERE user_id = 1 AND category1_code = '1';
UPDATE CATEGORY3 SET category1_code = 'INCOME' WHERE user_id = 1 AND category1_code = '2';
UPDATE CATEGORY3 SET category1_code = 'TRANSFER' WHERE user_id = 1 AND category1_code = '3';

UPDATE CATEGORY3 SET category2_code = 'C2_E_' || category2_code WHERE user_id = 1 AND category1_code = 'EXPENSE' AND category2_code NOT LIKE 'C2_%';
UPDATE CATEGORY3 SET category2_code = 'C2_I_' || category2_code WHERE user_id = 1 AND category1_code = 'INCOME' AND category2_code NOT LIKE 'C2_%';
UPDATE CATEGORY3 SET category2_code = 'C2_T_' || category2_code WHERE user_id = 1 AND category1_code = 'TRANSFER' AND category2_code NOT LIKE 'C2_%';

UPDATE CATEGORY3 SET category3_code = 'C3_' || category3_code WHERE user_id = 1 AND category3_code NOT LIKE 'C3_%';

-- Step 3: Update I18N tables if they exist
UPDATE CATEGORY2_I18N SET category1_code = 'EXPENSE' WHERE user_id = 1 AND category1_code = '1';
UPDATE CATEGORY2_I18N SET category1_code = 'INCOME' WHERE user_id = 1 AND category1_code = '2';
UPDATE CATEGORY2_I18N SET category1_code = 'TRANSFER' WHERE user_id = 1 AND category1_code = '3';

UPDATE CATEGORY2_I18N SET category2_code = 'C2_E_' || category2_code WHERE user_id = 1 AND category1_code = 'EXPENSE' AND category2_code NOT LIKE 'C2_%';
UPDATE CATEGORY2_I18N SET category2_code = 'C2_I_' || category2_code WHERE user_id = 1 AND category1_code = 'INCOME' AND category2_code NOT LIKE 'C2_%';
UPDATE CATEGORY2_I18N SET category2_code = 'C2_T_' || category2_code WHERE user_id = 1 AND category1_code = 'TRANSFER' AND category2_code NOT LIKE 'C2_%';

UPDATE CATEGORY3_I18N SET category1_code = 'EXPENSE' WHERE user_id = 1 AND category1_code = '1';
UPDATE CATEGORY3_I18N SET category1_code = 'INCOME' WHERE user_id = 1 AND category1_code = '2';
UPDATE CATEGORY3_I18N SET category1_code = 'TRANSFER' WHERE user_id = 1 AND category1_code = '3';

UPDATE CATEGORY3_I18N SET category2_code = 'C2_E_' || category2_code WHERE user_id = 1 AND category1_code = 'EXPENSE' AND category2_code NOT LIKE 'C2_%';
UPDATE CATEGORY3_I18N SET category2_code = 'C2_I_' || category2_code WHERE user_id = 1 AND category1_code = 'INCOME' AND category2_code NOT LIKE 'C2_%';
UPDATE CATEGORY3_I18N SET category2_code = 'C2_T_' || category2_code WHERE user_id = 1 AND category1_code = 'TRANSFER' AND category2_code NOT LIKE 'C2_%';

UPDATE CATEGORY3_I18N SET category3_code = 'C3_' || category3_code WHERE user_id = 1 AND category3_code NOT LIKE 'C3_%';

-- Re-enable foreign keys
PRAGMA foreign_keys = ON;

-- Verification
SELECT '=== Updated Category2 Data ===' as Info;
SELECT category1_code, category2_code, category2_name, display_order 
FROM CATEGORY2 
WHERE user_id = 1 
ORDER BY category1_code, display_order
LIMIT 10;

SELECT '=== Category3 Count ===' as Info;
SELECT COUNT(*) as count FROM CATEGORY3 WHERE user_id = 1;
