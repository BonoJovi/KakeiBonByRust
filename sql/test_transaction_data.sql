-- Test transaction data for USER_ID = 1
-- This SQL file contains sample transaction data for testing the transaction management feature

-- Insert sample transactions (EXPENSE)
INSERT INTO TRANSACTIONS (USER_ID, TRANSACTION_DATE, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, AMOUNT, DESCRIPTION, MEMO, ENTRY_DT) VALUES
-- Food expenses
(1, '2025-11-01', 'EXPENSE', 'C2_E_1', 'C3_E_1_1', 3500, 'スーパーマーケット', '週末の食材購入', datetime('now')),
(1, '2025-11-02', 'EXPENSE', 'C2_E_1', 'C3_E_1_2', 1200, 'コンビニ', '昼食', datetime('now')),
(1, '2025-11-03', 'EXPENSE', 'C2_E_1', 'C3_E_1_3', 2800, 'ファミレス', '家族で夕食', datetime('now')),
(1, '2025-11-04', 'EXPENSE', 'C2_E_1', 'C3_E_1_1', 4200, 'スーパーマーケット', '週中の買い物', datetime('now')),
(1, '2025-11-05', 'EXPENSE', 'C2_E_1', 'C3_E_1_5', '500', 'カフェ', 'コーヒー', datetime('now')),

-- Transportation expenses
(1, '2025-11-01', 'EXPENSE', 'C2_E_2', 'C3_E_2_1', 15000, '定期券', '11月分定期代', datetime('now')),
(1, '2025-11-02', 'EXPENSE', 'C2_E_2', 'C3_E_2_2', 1500, 'タクシー', '帰宅時', datetime('now')),
(1, '2025-11-03', 'EXPENSE', 'C2_E_2', 'C3_E_2_4', 8000, 'ガソリンスタンド', '満タン給油', datetime('now')),

-- Housing expenses
(1, '2025-11-01', 'EXPENSE', 'C2_E_3', 'C3_E_3_1', 80000, '家賃', '11月分', datetime('now')),
(1, '2025-11-05', 'EXPENSE', 'C2_E_3', 'C3_E_3_2', 12000, '電気代', '10月分', datetime('now')),

-- Utilities
(1, '2025-11-01', 'EXPENSE', 'C2_E_4', 'C3_E_4_1', 5000, '携帯電話', '11月分', datetime('now')),
(1, '2025-11-03', 'EXPENSE', 'C2_E_4', 'C3_E_4_2', 6000, 'インターネット', '11月分', datetime('now')),

-- Medical expenses
(1, '2025-11-02', 'EXPENSE', 'C2_E_5', 'C3_E_5_1', 3000, '内科クリニック', '風邪の診察', datetime('now')),
(1, '2025-11-02', 'EXPENSE', 'C2_E_5', 'C3_E_5_2', 1500, '薬局', '処方箋', datetime('now')),

-- Education expenses
(1, '2025-11-01', 'EXPENSE', 'C2_E_6', 'C3_E_6_1', 25000, '学習塾', '11月分', datetime('now')),
(1, '2025-11-04', 'EXPENSE', 'C2_E_6', 'C3_E_6_2', 3800, '書店', '参考書購入', datetime('now')),

-- Entertainment
(1, '2025-11-03', 'EXPENSE', 'C2_E_7', 'C3_E_7_1', 2500, '映画館', '家族で鑑賞', datetime('now')),
(1, '2025-11-05', 'EXPENSE', 'C2_E_7', 'C3_E_7_3', 8000, 'カラオケ', '友人と', datetime('now')),

-- Clothing
(1, '2025-11-04', 'EXPENSE', 'C2_E_8', 'C3_E_8_1', 15000, 'デパート', '冬服購入', datetime('now')),

-- Personal care
(1, '2025-11-02', 'EXPENSE', 'C2_E_9', 'C3_E_9_1', 3500, '美容院', 'カット', datetime('now')),

-- Other expenses
(1, '2025-11-05', 'EXPENSE', 'C2_E_10', 'C3_E_10_1', 2000, '100円ショップ', '日用品', datetime('now'));

-- Insert sample transactions (INCOME)
INSERT INTO TRANSACTIONS (USER_ID, TRANSACTION_DATE, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, AMOUNT, DESCRIPTION, MEMO, ENTRY_DT) VALUES
-- Salary
(1, '2025-11-01', 'INCOME', 'C2_I_1', 'C3_I_1_1', 300000, '給料', '10月分給与', datetime('now')),

-- Side income
(1, '2025-11-03', 'INCOME', 'C2_I_2', 'C3_I_2_1', 50000, 'フリーランス', 'Web制作案件', datetime('now')),

-- Investment income
(1, '2025-11-01', 'INCOME', 'C2_I_3', 'C3_I_3_2', 5000, '株式配当', '配当金入金', datetime('now')),

-- Other income
(1, '2025-11-04', 'INCOME', 'C2_I_6', 'C3_I_6_1', 3000, 'フリマ', '不用品販売', datetime('now'));

-- Insert sample transactions (TRANSFER)
INSERT INTO TRANSACTIONS (USER_ID, TRANSACTION_DATE, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, AMOUNT, DESCRIPTION, MEMO, ENTRY_DT) VALUES
-- Transfers
(1, '2025-11-01', 'TRANSFER', 'C2_T_1', 'C3_T_1_1', 100000, '貯金', '定期預金へ', datetime('now')),
(1, '2025-11-05', 'TRANSFER', 'C2_T_1', 'C3_T_1_2', 50000, '投資口座へ', '積立投資', datetime('now'));

-- Add some transactions for previous month (October) for testing date filters
INSERT INTO TRANSACTIONS (USER_ID, TRANSACTION_DATE, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, AMOUNT, DESCRIPTION, MEMO, ENTRY_DT) VALUES
(1, '2025-10-15', 'EXPENSE', 'C2_E_1', 'C3_E_1_1', 4000, 'スーパーマーケット', '10月中旬の買い物', datetime('now')),
(1, '2025-10-20', 'EXPENSE', 'C2_E_2', 'C3_E_2_1', 15000, '定期券', '10月分定期代', datetime('now')),
(1, '2025-10-25', 'INCOME', 'C2_I_1', 'C3_I_1_1', 300000, '給料', '9月分給与', datetime('now')),
(1, '2025-10-28', 'EXPENSE', 'C2_E_3', 'C3_E_3_1', 80000, '家賃', '10月分', datetime('now'));

-- Summary of test data:
-- Total transactions: 31
-- EXPENSE: 21 transactions, Various categories
-- INCOME: 4 transactions, Salary, Side income, Investment, Other
-- TRANSFER: 2 transactions, Savings
-- Date range: 2025-10-15 to 2025-11-05
-- Total amount: Expense ~200,000 yen, Income ~358,000 yen, Transfer ~150,000 yen
