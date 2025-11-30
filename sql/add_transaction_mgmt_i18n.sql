-- Add i18n resources for transaction management feature
-- Date: 2025-11-06
-- Note: Uses INSERT OR IGNORE to avoid duplicates if already exists

-- Menu
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES 
('menu.transaction_management', 'ja', '入出金管理', 'menu', datetime('now')),
('menu.transaction_management', 'en', 'Transaction Management', 'menu', datetime('now'));

-- Transaction Management Screen
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES 
('transaction_mgmt.title', 'ja', '入出金管理', 'transaction_mgmt', datetime('now')),
('transaction_mgmt.title', 'en', 'Transaction Management', 'transaction_mgmt', datetime('now')),

('transaction_mgmt.add_new', 'ja', '+ 新規追加', 'transaction_mgmt', datetime('now')),
('transaction_mgmt.add_new', 'en', '+ Add New Transaction', 'transaction_mgmt', datetime('now')),

('transaction_mgmt.filter', 'ja', '🔍 フィルター', 'transaction_mgmt', datetime('now')),
('transaction_mgmt.filter', 'en', '🔍 Filter', 'transaction_mgmt', datetime('now')),

('transaction_mgmt.filter_options', 'ja', 'フィルター設定', 'transaction_mgmt', datetime('now')),
('transaction_mgmt.filter_options', 'en', 'Filter Options', 'transaction_mgmt', datetime('now')),

('transaction_mgmt.date_range', 'ja', '日付範囲:', 'transaction_mgmt', datetime('now')),
('transaction_mgmt.date_range', 'en', 'Date Range:', 'transaction_mgmt', datetime('now')),

('transaction_mgmt.category', 'ja', 'カテゴリ:', 'transaction_mgmt', datetime('now')),
('transaction_mgmt.category', 'en', 'Category:', 'transaction_mgmt', datetime('now')),

('transaction_mgmt.amount_range', 'ja', '金額範囲:', 'transaction_mgmt', datetime('now')),
('transaction_mgmt.amount_range', 'en', 'Amount Range:', 'transaction_mgmt', datetime('now')),

('transaction_mgmt.keyword', 'ja', 'キーワード:', 'transaction_mgmt', datetime('now')),
('transaction_mgmt.keyword', 'en', 'Keyword:', 'transaction_mgmt', datetime('now')),

('transaction_mgmt.clear_filter', 'ja', 'クリア', 'transaction_mgmt', datetime('now')),
('transaction_mgmt.clear_filter', 'en', 'Clear', 'transaction_mgmt', datetime('now')),

('transaction_mgmt.apply_filter', 'ja', '適用', 'transaction_mgmt', datetime('now')),
('transaction_mgmt.apply_filter', 'en', 'Apply', 'transaction_mgmt', datetime('now')),

('transaction_mgmt.total', 'ja', '合計:', 'transaction_mgmt', datetime('now')),
('transaction_mgmt.total', 'en', 'Total:', 'transaction_mgmt', datetime('now')),

('transaction_mgmt.items', 'ja', '件', 'transaction_mgmt', datetime('now')),
('transaction_mgmt.items', 'en', 'items', 'transaction_mgmt', datetime('now')),

('transaction_mgmt.page', 'ja', 'ページ', 'transaction_mgmt', datetime('now')),
('transaction_mgmt.page', 'en', 'Page', 'transaction_mgmt', datetime('now')),

('transaction_mgmt.coming_soon', 'ja', '入出金登録機能は近日公開予定です', 'transaction_mgmt', datetime('now')),
('transaction_mgmt.coming_soon', 'en', 'Transaction registration feature coming soon!', 'transaction_mgmt', datetime('now'));
