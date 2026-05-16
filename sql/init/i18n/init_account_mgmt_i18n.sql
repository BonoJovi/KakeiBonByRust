-- Translation resources for account_mgmt
-- Auto-generated from database
-- Category: account_mgmt

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
(727, 'account_mgmt.title', 'en', 'Account Management', 'account_mgmt', 'Account management page title', datetime('now')),
(728, 'account_mgmt.title', 'ja', '口座管理', 'account_mgmt', '口座管理ページタイトル', datetime('now')),
(729, 'account_mgmt.account_list', 'en', 'Account List', 'account_mgmt', 'Account list section title', datetime('now')),
(730, 'account_mgmt.account_list', 'ja', '口座一覧', 'account_mgmt', '口座一覧セクションタイトル', datetime('now')),
(731, 'account_mgmt.add_account', 'en', 'Add Account', 'account_mgmt', 'Add account button', datetime('now')),
(732, 'account_mgmt.add_account', 'ja', '口座追加', 'account_mgmt', '口座追加ボタン', datetime('now')),
(733, 'account_mgmt.edit_account', 'en', 'Edit Account', 'account_mgmt', 'Edit account modal title', datetime('now')),
(734, 'account_mgmt.edit_account', 'ja', '口座編集', 'account_mgmt', '口座編集モーダルタイトル', datetime('now')),
(735, 'account_mgmt.account_code', 'en', 'Account Code', 'account_mgmt', 'Account code label', datetime('now')),
(736, 'account_mgmt.account_code', 'ja', '口座コード', 'account_mgmt', '口座コードラベル', datetime('now')),
(737, 'account_mgmt.account_name', 'en', 'Account Name', 'account_mgmt', 'Account name label', datetime('now')),
(738, 'account_mgmt.account_name', 'ja', '口座名', 'account_mgmt', '口座名ラベル', datetime('now')),
(739, 'account_mgmt.template', 'en', 'Template', 'account_mgmt', 'Template label', datetime('now')),
(740, 'account_mgmt.template', 'ja', 'テンプレート', 'account_mgmt', 'テンプレートラベル', datetime('now')),
(741, 'account_mgmt.initial_balance', 'en', 'Initial Balance', 'account_mgmt', 'Initial balance label', datetime('now')),
(742, 'account_mgmt.initial_balance', 'ja', '初期残高', 'account_mgmt', '初期残高ラベル', datetime('now')),
(743, 'account_mgmt.display_order', 'en', 'Display Order', 'account_mgmt', 'Display order label', datetime('now')),
(744, 'account_mgmt.display_order', 'ja', '表示順', 'account_mgmt', '表示順ラベル', datetime('now')),
(745, 'account_mgmt.no_accounts', 'en', 'No accounts found', 'account_mgmt', 'No accounts message', datetime('now')),
(746, 'account_mgmt.no_accounts', 'ja', '口座が見つかりません', 'account_mgmt', '口座なしメッセージ', datetime('now')),
(807, 'account_mgmt.add_new', 'en', 'Add New Account', 'account_mgmt', 'Add new account button', datetime('now')),
(808, 'account_mgmt.add_new', 'ja', '新規口座追加', 'account_mgmt', '新規口座追加ボタン', datetime('now')),
(811, 'account_mgmt.modal_title_add', 'en', 'Add Account', 'account_mgmt', 'Add account modal title', datetime('now')),
(812, 'account_mgmt.modal_title_add', 'ja', '口座追加', 'account_mgmt', '口座追加モーダルタイトル', datetime('now')),
(813, 'account_mgmt.modal_title_edit', 'en', 'Edit Account', 'account_mgmt', 'Edit account modal title', datetime('now')),
(814, 'account_mgmt.modal_title_edit', 'ja', '口座編集', 'account_mgmt', '口座編集モーダルタイトル', datetime('now')),
(815, 'account_mgmt.delete_confirm', 'en', 'Are you sure you want to delete this account?', 'account_mgmt', 'Delete account confirmation message', datetime('now')),
(816, 'account_mgmt.delete_confirm', 'ja', 'この口座を削除してもよろしいですか？', 'account_mgmt', '口座削除確認メッセージ', datetime('now')),
(817, 'account_mgmt.select_template', 'en', '- Select Template -', 'account_mgmt', 'Template select placeholder', datetime('now')),
(818, 'account_mgmt.select_template', 'ja', '- テンプレートを選択 -', 'account_mgmt', 'テンプレート選択プレースホルダー', datetime('now'));

-- Issue #50 follow-up: i18n keys for alert→toast migration. Code
-- adoption is in a separate PR; this is the data-side prerequisite.
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES
(2301, 'account_mgmt.failed_to_initialize', 'en', 'Failed to initialize', 'account_mgmt', 'Initialization error message', datetime('now')),
(2302, 'account_mgmt.failed_to_initialize', 'ja', '初期化に失敗しました', 'account_mgmt', '初期化エラーメッセージ', datetime('now')),
(2303, 'account_mgmt.failed_to_load_templates', 'en', 'Failed to load templates', 'account_mgmt', 'Template load error', datetime('now')),
(2304, 'account_mgmt.failed_to_load_templates', 'ja', 'テンプレートの読み込みに失敗しました', 'account_mgmt', 'テンプレート読み込みエラー', datetime('now')),
(2305, 'account_mgmt.failed_to_save', 'en', 'Failed to save account', 'account_mgmt', 'Save error message', datetime('now')),
(2306, 'account_mgmt.failed_to_save', 'ja', '口座の保存に失敗しました', 'account_mgmt', '保存エラーメッセージ', datetime('now')),
(2307, 'account_mgmt.failed_to_delete', 'en', 'Failed to delete account', 'account_mgmt', 'Delete error message', datetime('now')),
(2308, 'account_mgmt.failed_to_delete', 'ja', '口座の削除に失敗しました', 'account_mgmt', '削除エラーメッセージ', datetime('now')),
(2309, 'account_mgmt.update_success', 'en', 'Account updated successfully', 'account_mgmt', 'Update success message', datetime('now')),
(2310, 'account_mgmt.update_success', 'ja', '口座を更新しました', 'account_mgmt', '更新成功メッセージ', datetime('now')),
(2311, 'account_mgmt.add_success', 'en', 'Account added successfully', 'account_mgmt', 'Add success message', datetime('now')),
(2312, 'account_mgmt.add_success', 'ja', '口座を追加しました', 'account_mgmt', '追加成功メッセージ', datetime('now')),
(2313, 'account_mgmt.delete_success', 'en', 'Account deleted successfully', 'account_mgmt', 'Delete success message', datetime('now')),
(2314, 'account_mgmt.delete_success', 'ja', '口座を削除しました', 'account_mgmt', '削除成功メッセージ', datetime('now'));
