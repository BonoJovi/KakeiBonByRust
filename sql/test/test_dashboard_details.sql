-- Test detail data for dashboard testing
-- Adds TRANSACTIONS_DETAIL records with category2 information

-- 2024/12 boundary
INSERT INTO TRANSACTIONS_DETAIL (TRANSACTION_ID, USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, ITEM_NAME, AMOUNT, ENTRY_DT)
VALUES
(2, 2, 'EXPENSE', 'C2_E_1', '', 'テスト支出12月', -5000, datetime('now')),
(3, 2, 'INCOME', 'C2_I_1', '', '給与12月', 200000, datetime('now'));

-- 2025/01
INSERT INTO TRANSACTIONS_DETAIL (TRANSACTION_ID, USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, ITEM_NAME, AMOUNT, ENTRY_DT)
VALUES
(4, 2, 'EXPENSE', 'C2_E_1', '', '食費', -3500, datetime('now')),
(5, 2, 'EXPENSE', 'C2_E_2', '', '日用品', -12000, datetime('now')),
(6, 2, 'INCOME', 'C2_I_1', '', '給与', 250000, datetime('now'));

-- 2025/02
INSERT INTO TRANSACTIONS_DETAIL (TRANSACTION_ID, USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, ITEM_NAME, AMOUNT, ENTRY_DT)
VALUES
(7, 2, 'EXPENSE', 'C2_E_3', '', '交通費', -8000, datetime('now')),
(8, 2, 'EXPENSE', 'C2_E_4', '', '娯楽', -15000, datetime('now')),
(9, 2, 'INCOME', 'C2_I_1', '', '給与', 250000, datetime('now'));

-- 2025/03
INSERT INTO TRANSACTIONS_DETAIL (TRANSACTION_ID, USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, ITEM_NAME, AMOUNT, ENTRY_DT)
VALUES
(10, 2, 'EXPENSE', 'C2_E_1', '', '食費', -6500, datetime('now')),
(11, 2, 'EXPENSE', 'C2_E_5', '', '医療費', -25000, datetime('now')),
(12, 2, 'INCOME', 'C2_I_1', '', '給与', 250000, datetime('now'));

-- 2025/04
INSERT INTO TRANSACTIONS_DETAIL (TRANSACTION_ID, USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, ITEM_NAME, AMOUNT, ENTRY_DT)
VALUES
(13, 2, 'EXPENSE', 'C2_E_2', '', '日用品', -4200, datetime('now')),
(14, 2, 'EXPENSE', 'C2_E_6', '', '衣服', -18000, datetime('now')),
(15, 2, 'INCOME', 'C2_I_1', '', '給与', 260000, datetime('now'));

-- 2025/05
INSERT INTO TRANSACTIONS_DETAIL (TRANSACTION_ID, USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, ITEM_NAME, AMOUNT, ENTRY_DT)
VALUES
(16, 2, 'EXPENSE', 'C2_E_1', '', '食費', -9800, datetime('now')),
(17, 2, 'EXPENSE', 'C2_E_7', '', '通信費', -32000, datetime('now')),
(18, 2, 'INCOME', 'C2_I_1', '', '給与', 260000, datetime('now'));

-- 2025/06
INSERT INTO TRANSACTIONS_DETAIL (TRANSACTION_ID, USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, ITEM_NAME, AMOUNT, ENTRY_DT)
VALUES
(19, 2, 'EXPENSE', 'C2_E_3', '', '交通費', -5500, datetime('now')),
(20, 2, 'EXPENSE', 'C2_E_8', '', '水道光熱費', -21000, datetime('now')),
(21, 2, 'INCOME', 'C2_I_1', '', '給与', 260000, datetime('now')),
(22, 2, 'INCOME', 'C2_I_2', '', 'ボーナス', 150000, datetime('now'));

-- 2025/07
INSERT INTO TRANSACTIONS_DETAIL (TRANSACTION_ID, USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, ITEM_NAME, AMOUNT, ENTRY_DT)
VALUES
(23, 2, 'EXPENSE', 'C2_E_1', '', '食費', -7800, datetime('now')),
(24, 2, 'EXPENSE', 'C2_E_9', '', '旅行', -45000, datetime('now')),
(25, 2, 'INCOME', 'C2_I_1', '', '給与', 260000, datetime('now'));

-- 2025/08
INSERT INTO TRANSACTIONS_DETAIL (TRANSACTION_ID, USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, ITEM_NAME, AMOUNT, ENTRY_DT)
VALUES
(26, 2, 'EXPENSE', 'C2_E_4', '', '娯楽', -11000, datetime('now')),
(27, 2, 'EXPENSE', 'C2_E_10', '', '教育費', -28000, datetime('now')),
(28, 2, 'INCOME', 'C2_I_1', '', '給与', 260000, datetime('now'));

-- 2025/09
INSERT INTO TRANSACTIONS_DETAIL (TRANSACTION_ID, USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, ITEM_NAME, AMOUNT, ENTRY_DT)
VALUES
(29, 2, 'EXPENSE', 'C2_E_2', '', '日用品', -6200, datetime('now')),
(30, 2, 'EXPENSE', 'C2_E_11', '', '保険料', -19500, datetime('now')),
(31, 2, 'INCOME', 'C2_I_1', '', '給与', 265000, datetime('now'));

-- 2025/10
INSERT INTO TRANSACTIONS_DETAIL (TRANSACTION_ID, USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, ITEM_NAME, AMOUNT, ENTRY_DT)
VALUES
(32, 2, 'EXPENSE', 'C2_E_1', '', '食費', -8500, datetime('now')),
(33, 2, 'EXPENSE', 'C2_E_12', '', '家具家電', -35000, datetime('now')),
(34, 2, 'INCOME', 'C2_I_1', '', '給与', 265000, datetime('now'));

-- 2025/11
INSERT INTO TRANSACTIONS_DETAIL (TRANSACTION_ID, USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, ITEM_NAME, AMOUNT, ENTRY_DT)
VALUES
(35, 2, 'EXPENSE', 'C2_E_3', '', '交通費', -4800, datetime('now')),
(36, 2, 'EXPENSE', 'C2_E_13', '', '交際費', -22000, datetime('now')),
(37, 2, 'INCOME', 'C2_I_1', '', '給与', 265000, datetime('now'));

-- 2025/12
INSERT INTO TRANSACTIONS_DETAIL (TRANSACTION_ID, USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, ITEM_NAME, AMOUNT, ENTRY_DT)
VALUES
(38, 2, 'EXPENSE', 'C2_E_1', '', '食費', -15000, datetime('now')),
(39, 2, 'EXPENSE', 'C2_E_14', '', '年末イベント', -50000, datetime('now')),
(40, 2, 'INCOME', 'C2_I_1', '', '給与', 265000, datetime('now')),
(41, 2, 'INCOME', 'C2_I_2', '', 'ボーナス', 300000, datetime('now'));

-- 2026/01 boundary
INSERT INTO TRANSACTIONS_DETAIL (TRANSACTION_ID, USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, ITEM_NAME, AMOUNT, ENTRY_DT)
VALUES
(42, 2, 'EXPENSE', 'C2_E_1', '', 'お年玉', -10000, datetime('now')),
(43, 2, 'INCOME', 'C2_I_3', '', 'お年玉', 50000, datetime('now'));
