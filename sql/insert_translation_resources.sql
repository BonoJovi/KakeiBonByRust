-- Translation resources for Account Management and Transaction Management
-- To be added to I18N_RESOURCES table
-- Starting from RESOURCE_ID 727

-- ============================================================================
-- Account Management (account_mgmt.*)
-- ============================================================================

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
-- Account Management Screen
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

-- ============================================================================
-- Transaction Management (transaction_mgmt.*)
-- ============================================================================

-- Transaction Management Screen
(747, 'transaction_mgmt.title', 'en', 'Transaction Management', 'transaction_mgmt', 'Transaction management page title', datetime('now')),
(748, 'transaction_mgmt.title', 'ja', '入出金管理', 'transaction_mgmt', '入出金管理ページタイトル', datetime('now')),
(749, 'transaction_mgmt.transaction_list', 'en', 'Transaction List', 'transaction_mgmt', 'Transaction list section title', datetime('now')),
(750, 'transaction_mgmt.transaction_list', 'ja', '入出金一覧', 'transaction_mgmt', '入出金一覧セクションタイトル', datetime('now')),
(751, 'transaction_mgmt.add_transaction', 'en', 'Add Transaction', 'transaction_mgmt', 'Add transaction button', datetime('now')),
(752, 'transaction_mgmt.add_transaction', 'ja', '入出金追加', 'transaction_mgmt', '入出金追加ボタン', datetime('now')),
(753, 'transaction_mgmt.edit_transaction', 'en', 'Edit Transaction', 'transaction_mgmt', 'Edit transaction modal title', datetime('now')),
(754, 'transaction_mgmt.edit_transaction', 'ja', '入出金編集', 'transaction_mgmt', '入出金編集モーダルタイトル', datetime('now')),
(755, 'transaction_mgmt.transaction_date', 'en', 'Transaction Date', 'transaction_mgmt', 'Transaction date label', datetime('now')),
(756, 'transaction_mgmt.transaction_date', 'ja', '取引日時', 'transaction_mgmt', '取引日時ラベル', datetime('now')),
(757, 'transaction_mgmt.category1', 'en', 'Category (Major)', 'transaction_mgmt', 'Major category label', datetime('now')),
(758, 'transaction_mgmt.category1', 'ja', '費目（大分類）', 'transaction_mgmt', '大分類ラベル', datetime('now')),
(759, 'transaction_mgmt.from_account', 'en', 'From Account', 'transaction_mgmt', 'From account label', datetime('now')),
(760, 'transaction_mgmt.from_account', 'ja', '出金元', 'transaction_mgmt', '出金元ラベル', datetime('now')),
(761, 'transaction_mgmt.to_account', 'en', 'To Account', 'transaction_mgmt', 'To account label', datetime('now')),
(762, 'transaction_mgmt.to_account', 'ja', '入金先', 'transaction_mgmt', '入金先ラベル', datetime('now')),
(763, 'transaction_mgmt.total_amount', 'en', 'Total Amount', 'transaction_mgmt', 'Total amount label', datetime('now')),
(764, 'transaction_mgmt.total_amount', 'ja', '合計金額', 'transaction_mgmt', '合計金額ラベル', datetime('now')),
(765, 'transaction_mgmt.tax_rounding', 'en', 'Tax Rounding', 'transaction_mgmt', 'Tax rounding label', datetime('now')),
(766, 'transaction_mgmt.tax_rounding', 'ja', '税額端数処理', 'transaction_mgmt', '税額端数処理ラベル', datetime('now')),
(767, 'transaction_mgmt.round_down', 'en', 'Round Down', 'transaction_mgmt', 'Round down option', datetime('now')),
(768, 'transaction_mgmt.round_down', 'ja', '切り捨て', 'transaction_mgmt', '切り捨てオプション', datetime('now')),
(769, 'transaction_mgmt.round_half', 'en', 'Round Half', 'transaction_mgmt', 'Round half option', datetime('now')),
(770, 'transaction_mgmt.round_half', 'ja', '四捨五入', 'transaction_mgmt', '四捨五入オプション', datetime('now')),
(771, 'transaction_mgmt.round_up', 'en', 'Round Up', 'transaction_mgmt', 'Round up option', datetime('now')),
(772, 'transaction_mgmt.round_up', 'ja', '切り上げ', 'transaction_mgmt', '切り上げオプション', datetime('now')),
(773, 'transaction_mgmt.memo', 'en', 'Memo', 'transaction_mgmt', 'Memo label', datetime('now')),
(774, 'transaction_mgmt.memo', 'ja', 'メモ', 'transaction_mgmt', 'メモラベル', datetime('now')),
(775, 'transaction_mgmt.no_transactions', 'en', 'No transactions found', 'transaction_mgmt', 'No transactions message', datetime('now')),
(776, 'transaction_mgmt.no_transactions', 'ja', '入出金データが見つかりません', 'transaction_mgmt', '入出金なしメッセージ', datetime('now')),

-- Filter related
(777, 'transaction_mgmt.show_filters', 'en', 'Show Filters', 'transaction_mgmt', 'Show filters button', datetime('now')),
(778, 'transaction_mgmt.show_filters', 'ja', 'フィルタ表示', 'transaction_mgmt', 'フィルタ表示ボタン', datetime('now')),
(779, 'transaction_mgmt.hide_filters', 'en', 'Hide Filters', 'transaction_mgmt', 'Hide filters button', datetime('now')),
(780, 'transaction_mgmt.hide_filters', 'ja', 'フィルタ非表示', 'transaction_mgmt', 'フィルタ非表示ボタン', datetime('now')),
(781, 'transaction_mgmt.filters', 'en', 'Filters', 'transaction_mgmt', 'Filters title', datetime('now')),
(782, 'transaction_mgmt.filters', 'ja', 'フィルタ', 'transaction_mgmt', 'フィルタタイトル', datetime('now')),
(783, 'transaction_mgmt.date_range', 'en', 'Date Range:', 'transaction_mgmt', 'Date range label', datetime('now')),
(784, 'transaction_mgmt.date_range', 'ja', '期間:', 'transaction_mgmt', '期間ラベル', datetime('now')),
(785, 'transaction_mgmt.start_date', 'en', 'Start', 'transaction_mgmt', 'Start date label', datetime('now')),
(786, 'transaction_mgmt.start_date', 'ja', '開始', 'transaction_mgmt', '開始日ラベル', datetime('now')),
(787, 'transaction_mgmt.end_date', 'en', 'End', 'transaction_mgmt', 'End date label', datetime('now')),
(788, 'transaction_mgmt.end_date', 'ja', '終了', 'transaction_mgmt', '終了日ラベル', datetime('now')),
(789, 'transaction_mgmt.amount_range', 'en', 'Amount Range:', 'transaction_mgmt', 'Amount range label', datetime('now')),
(790, 'transaction_mgmt.amount_range', 'ja', '金額範囲:', 'transaction_mgmt', '金額範囲ラベル', datetime('now')),
(791, 'transaction_mgmt.min_amount', 'en', 'Min', 'transaction_mgmt', 'Minimum amount label', datetime('now')),
(792, 'transaction_mgmt.min_amount', 'ja', '最小', 'transaction_mgmt', '最小金額ラベル', datetime('now')),
(793, 'transaction_mgmt.max_amount', 'en', 'Max', 'transaction_mgmt', 'Maximum amount label', datetime('now')),
(794, 'transaction_mgmt.max_amount', 'ja', '最大', 'transaction_mgmt', '最大金額ラベル', datetime('now')),
(795, 'transaction_mgmt.keyword', 'en', 'Keyword:', 'transaction_mgmt', 'Keyword search label', datetime('now')),
(796, 'transaction_mgmt.keyword', 'ja', 'キーワード:', 'transaction_mgmt', 'キーワード検索ラベル', datetime('now')),
(797, 'transaction_mgmt.apply_filters', 'en', 'Apply Filters', 'transaction_mgmt', 'Apply filters button', datetime('now')),
(798, 'transaction_mgmt.apply_filters', 'ja', 'フィルタ適用', 'transaction_mgmt', 'フィルタ適用ボタン', datetime('now')),
(799, 'transaction_mgmt.clear_filters', 'en', 'Clear Filters', 'transaction_mgmt', 'Clear filters button', datetime('now')),
(800, 'transaction_mgmt.clear_filters', 'ja', 'フィルタクリア', 'transaction_mgmt', 'フィルタクリアボタン', datetime('now')),

-- Common resources
(801, 'common.unspecified', 'en', 'Unspecified', 'common', 'Unspecified option', datetime('now')),
(802, 'common.unspecified', 'ja', '指定なし', 'common', '指定なしオプション', datetime('now')),
(803, 'common.delete', 'en', 'Delete', 'common', 'Delete button', datetime('now')),
(804, 'common.delete', 'ja', '削除', 'common', '削除ボタン', datetime('now')),
(805, 'common.success', 'en', 'Success', 'common', 'Success message', datetime('now')),
(806, 'common.success', 'ja', '成功', 'common', '成功メッセージ', datetime('now')),
(807, 'account_mgmt.add_new', 'en', 'Add New Account', 'account_mgmt', 'Add new account button', datetime('now')),
(808, 'account_mgmt.add_new', 'ja', '新規口座追加', 'account_mgmt', '新規口座追加ボタン', datetime('now')),
(809, 'common.actions', 'en', 'Actions', 'common', 'Actions column header', datetime('now')),
(810, 'common.actions', 'ja', '操作', 'common', '操作カラムヘッダー', datetime('now')),
(811, 'account_mgmt.modal_title_add', 'en', 'Add Account', 'account_mgmt', 'Add account modal title', datetime('now')),
(812, 'account_mgmt.modal_title_add', 'ja', '口座追加', 'account_mgmt', '口座追加モーダルタイトル', datetime('now'));
