-- Add transfer categories (Phase 1: Minimal structure)
-- This SQL adds missing category data for TRANSFER type transactions
-- Phase 2 will add account management features

-- Insert CATEGORY2 for TRANSFER
INSERT INTO CATEGORY2 (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, DISPLAY_ORDER, CATEGORY2_NAME, IS_DISABLED, ENTRY_DT) VALUES
(1, 'TRANSFER', 'C2_T_1', 1, '振替', 0, datetime('now'));

-- Insert CATEGORY2_I18N for TRANSFER
INSERT INTO CATEGORY2_I18N (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, LANG_CODE, CATEGORY2_NAME_I18N, ENTRY_DT) VALUES
(1, 'TRANSFER', 'C2_T_1', 'ja', '振替', datetime('now')),
(1, 'TRANSFER', 'C2_T_1', 'en', 'Transfer', datetime('now'));

-- Insert CATEGORY3 for TRANSFER (generic/universal)
INSERT INTO CATEGORY3 (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, DISPLAY_ORDER, CATEGORY3_NAME, IS_DISABLED, ENTRY_DT) VALUES
(1, 'TRANSFER', 'C2_T_1', 'C3_T_1_1', 1, '振替', 0, datetime('now'));

-- Insert CATEGORY3_I18N for TRANSFER
INSERT INTO CATEGORY3_I18N (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, LANG_CODE, CATEGORY3_NAME_I18N, ENTRY_DT) VALUES
(1, 'TRANSFER', 'C2_T_1', 'C3_T_1_1', 'ja', '振替', datetime('now')),
(1, 'TRANSFER', 'C2_T_1', 'C3_T_1_1', 'en', 'Transfer', datetime('now'));

-- Verify the data
SELECT 'CATEGORY2 for TRANSFER:' as info;
SELECT CATEGORY2_CODE, CATEGORY2_NAME FROM CATEGORY2 WHERE CATEGORY1_CODE = 'TRANSFER';

SELECT 'CATEGORY3 for TRANSFER:' as info;
SELECT CATEGORY3_CODE, CATEGORY3_NAME FROM CATEGORY3 WHERE CATEGORY1_CODE = 'TRANSFER';

-- Note: Phase 2 will add:
-- - ACCOUNTS table (account master)
-- - FROM_ACCOUNT_ID and TO_ACCOUNT_ID columns to TRANSACTIONS table
-- - Account balance tracking and aggregation features
