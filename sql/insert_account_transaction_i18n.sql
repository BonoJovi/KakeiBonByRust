-- Insert i18n resources for account management and transaction management screens
-- Run this after initial database setup
-- Created: 2025-01-07

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES
-- Account Management (727-742)
(727, 'account_mgmt.title', 'en', 'Account Management', 'account_mgmt', 'Account management title', datetime('now')),
(728, 'account_mgmt.title', 'ja', '口座管理', 'account_mgmt', '口座管理タイトル', datetime('now')),
(729, 'account_mgmt.add_new', 'en', 'Add New Account', 'account_mgmt', 'Add new account button', datetime('now')),
(730, 'account_mgmt.add_new', 'ja', '新規口座追加', 'account_mgmt', '新規口座追加ボタン', datetime('now')),
(731, 'account_mgmt.modal_title_add', 'en', 'Add Account', 'account_mgmt', 'Add account modal title', datetime('now')),
(732, 'account_mgmt.modal_title_add', 'ja', '口座追加', 'account_mgmt', '口座追加モーダルタイトル', datetime('now')),
(733, 'account_mgmt.account_code', 'en', 'Account Code', 'account_mgmt', 'Account code label', datetime('now')),
(734, 'account_mgmt.account_code', 'ja', '口座コード', 'account_mgmt', '口座コードラベル', datetime('now')),
(735, 'account_mgmt.account_name', 'en', 'Account Name', 'account_mgmt', 'Account name label', datetime('now')),
(736, 'account_mgmt.account_name', 'ja', '口座名', 'account_mgmt', '口座名ラベル', datetime('now')),
(737, 'account_mgmt.template', 'en', 'Template', 'account_mgmt', 'Template label', datetime('now')),
(738, 'account_mgmt.template', 'ja', 'テンプレート', 'account_mgmt', 'テンプレートラベル', datetime('now')),
(739, 'account_mgmt.initial_balance', 'en', 'Initial Balance', 'account_mgmt', 'Initial balance label', datetime('now')),
(740, 'account_mgmt.initial_balance', 'ja', '初期残高', 'account_mgmt', '初期残高ラベル', datetime('now')),

-- Transaction Management (743-766)
(743, 'transaction_mgmt.title', 'en', 'Transaction List', 'transaction_mgmt', 'Transaction list title', datetime('now')),
(744, 'transaction_mgmt.title', 'ja', '入出金一覧', 'transaction_mgmt', '入出金一覧タイトル', datetime('now')),
(745, 'transaction_mgmt.add_new', 'en', 'Add New Transaction', 'transaction_mgmt', 'Add new transaction button', datetime('now')),
(746, 'transaction_mgmt.add_new', 'ja', '新規入出金登録', 'transaction_mgmt', '新規入出金登録ボタン', datetime('now')),
(747, 'transaction_mgmt.filter', 'en', 'Filter', 'transaction_mgmt', 'Filter button', datetime('now')),
(748, 'transaction_mgmt.filter', 'ja', '絞り込み', 'transaction_mgmt', '絞り込みボタン', datetime('now')),
(749, 'transaction_mgmt.filter_options', 'en', 'Filter Options', 'transaction_mgmt', 'Filter options title', datetime('now')),
(750, 'transaction_mgmt.filter_options', 'ja', '絞り込み条件', 'transaction_mgmt', '絞り込み条件タイトル', datetime('now')),
(751, 'transaction_mgmt.date_range', 'en', 'Date Range', 'transaction_mgmt', 'Date range label', datetime('now')),
(752, 'transaction_mgmt.date_range', 'ja', '日付範囲', 'transaction_mgmt', '日付範囲ラベル', datetime('now')),
(753, 'transaction_mgmt.category', 'en', 'Category', 'transaction_mgmt', 'Category label', datetime('now')),
(754, 'transaction_mgmt.category', 'ja', '費目', 'transaction_mgmt', '費目ラベル', datetime('now')),
(755, 'transaction_mgmt.amount_range', 'en', 'Amount Range', 'transaction_mgmt', 'Amount range label', datetime('now')),
(756, 'transaction_mgmt.amount_range', 'ja', '金額範囲', 'transaction_mgmt', '金額範囲ラベル', datetime('now')),
(757, 'transaction_mgmt.keyword', 'en', 'Keyword', 'transaction_mgmt', 'Keyword label', datetime('now')),
(758, 'transaction_mgmt.keyword', 'ja', 'キーワード', 'transaction_mgmt', 'キーワードラベル', datetime('now')),
(759, 'transaction_mgmt.apply_filter', 'en', 'Apply Filter', 'transaction_mgmt', 'Apply filter button', datetime('now')),
(760, 'transaction_mgmt.apply_filter', 'ja', '絞り込み実行', 'transaction_mgmt', '絞り込み実行ボタン', datetime('now')),
(761, 'transaction_mgmt.clear_filter', 'en', 'Clear Filter', 'transaction_mgmt', 'Clear filter button', datetime('now')),
(762, 'transaction_mgmt.clear_filter', 'ja', '絞り込み解除', 'transaction_mgmt', '絞り込み解除ボタン', datetime('now')),
(763, 'transaction_mgmt.total', 'en', 'Total', 'transaction_mgmt', 'Total label', datetime('now')),
(764, 'transaction_mgmt.total', 'ja', '合計', 'transaction_mgmt', '合計ラベル', datetime('now')),
(765, 'transaction_mgmt.items', 'en', 'items', 'transaction_mgmt', 'Items label', datetime('now')),
(766, 'transaction_mgmt.items', 'ja', '件', 'transaction_mgmt', '件ラベル', datetime('now')),
(767, 'transaction_mgmt.page', 'en', 'Page', 'transaction_mgmt', 'Page label', datetime('now')),
(768, 'transaction_mgmt.page', 'ja', 'ページ', 'transaction_mgmt', 'ページラベル', datetime('now')),

-- Common (already exists, but adding missing ones if needed)
(769, 'common.all', 'en', 'All', 'common', 'All option', datetime('now')),
(770, 'common.all', 'ja', 'すべて', 'common', 'すべてオプション', datetime('now'));

-- Add missing no_transactions resource
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES
(771, 'transaction_mgmt.no_transactions', 'en', 'No transactions found', 'transaction_mgmt', 'No transactions message', datetime('now')),
(772, 'transaction_mgmt.no_transactions', 'ja', '入出金データがありません', 'transaction_mgmt', '入出金データなしメッセージ', datetime('now'));
