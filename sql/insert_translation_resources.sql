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
(812, 'account_mgmt.modal_title_add', 'ja', '口座追加', 'account_mgmt', '口座追加モーダルタイトル', datetime('now')),
(813, 'account_mgmt.modal_title_edit', 'en', 'Edit Account', 'account_mgmt', 'Edit account modal title', datetime('now')),
(814, 'account_mgmt.modal_title_edit', 'ja', '口座編集', 'account_mgmt', '口座編集モーダルタイトル', datetime('now')),
(815, 'account_mgmt.delete_confirm', 'en', 'Are you sure you want to delete this account?', 'account_mgmt', 'Delete account confirmation message', datetime('now')),
(816, 'account_mgmt.delete_confirm', 'ja', 'この口座を削除してもよろしいですか？', 'account_mgmt', '口座削除確認メッセージ', datetime('now')),
(817, 'account_mgmt.select_template', 'en', '- Select Template -', 'account_mgmt', 'Template select placeholder', datetime('now')),
(818, 'account_mgmt.select_template', 'ja', '- テンプレートを選択 -', 'account_mgmt', 'テンプレート選択プレースホルダー', datetime('now')),

-- Transaction Management additional resources
(819, 'transaction_mgmt.add_new', 'en', 'Add New Transaction', 'transaction_mgmt', 'Add new transaction button', datetime('now')),
(820, 'transaction_mgmt.add_new', 'ja', '新規入出金追加', 'transaction_mgmt', '新規入出金追加ボタン', datetime('now')),
(821, 'transaction_mgmt.filter', 'en', 'Filter', 'transaction_mgmt', 'Filter button', datetime('now')),
(822, 'transaction_mgmt.filter', 'ja', 'フィルタ', 'transaction_mgmt', 'フィルタボタン', datetime('now')),
(823, 'transaction_mgmt.total', 'en', 'Total', 'transaction_mgmt', 'Total label', datetime('now')),
(824, 'transaction_mgmt.total', 'ja', '合計', 'transaction_mgmt', '合計ラベル', datetime('now')),
(825, 'transaction_mgmt.items', 'en', 'items', 'transaction_mgmt', 'Items label', datetime('now')),
(826, 'transaction_mgmt.items', 'ja', '件', 'transaction_mgmt', '件数ラベル', datetime('now')),
(827, 'transaction_mgmt.page', 'en', 'Page', 'transaction_mgmt', 'Page label', datetime('now')),
(828, 'transaction_mgmt.page', 'ja', 'ページ', 'transaction_mgmt', 'ページラベル', datetime('now')),
(829, 'transaction_mgmt.delete_confirm', 'en', 'Are you sure you want to delete this transaction?', 'transaction_mgmt', 'Delete transaction confirmation message', datetime('now')),
(830, 'transaction_mgmt.delete_confirm', 'ja', 'この入出金データを削除してもよろしいですか？', 'transaction_mgmt', '入出金削除確認メッセージ', datetime('now')),
(831, 'transaction_mgmt.select_category', 'en', '- Select category -', 'transaction_mgmt', 'Category select placeholder', datetime('now')),
(832, 'transaction_mgmt.select_category', 'ja', '- 費目を選択 -', 'transaction_mgmt', '費目選択プレースホルダー', datetime('now')),
(833, 'transaction_mgmt.manage_details', 'en', 'Manage Details', 'transaction_mgmt', 'Manage details button', datetime('now')),
(834, 'transaction_mgmt.manage_details', 'ja', '明細管理', 'transaction_mgmt', '明細管理ボタン', datetime('now')),
(835, 'transaction_mgmt.detail_items', 'en', 'items', 'transaction_mgmt', 'Detail items count label', datetime('now')),
(836, 'transaction_mgmt.detail_items', 'ja', '件', 'transaction_mgmt', '明細件数ラベル', datetime('now')),
(837, 'transaction_mgmt.filter_options', 'en', 'Filter Options', 'transaction_mgmt', 'Filter options title', datetime('now')),
(838, 'transaction_mgmt.filter_options', 'ja', 'フィルタオプション', 'transaction_mgmt', 'フィルタオプションタイトル', datetime('now')),
(839, 'transaction_mgmt.category', 'en', 'Category', 'transaction_mgmt', 'Category filter label', datetime('now')),
(840, 'transaction_mgmt.category', 'ja', '費目', 'transaction_mgmt', '費目フィルタラベル', datetime('now')),
(841, 'common.all', 'en', 'All', 'common', 'All option in filters', datetime('now')),
(842, 'common.all', 'ja', 'すべて', 'common', 'フィルタのすべてオプション', datetime('now')),
(843, 'transaction_mgmt.clear_filter', 'en', 'Clear', 'transaction_mgmt', 'Clear filter button', datetime('now')),
(844, 'transaction_mgmt.clear_filter', 'ja', 'クリア', 'transaction_mgmt', 'フィルタクリアボタン', datetime('now')),
(845, 'transaction_mgmt.apply_filter', 'en', 'Apply', 'transaction_mgmt', 'Apply filter button', datetime('now')),
(846, 'transaction_mgmt.apply_filter', 'ja', '適用', 'transaction_mgmt', 'フィルタ適用ボタン', datetime('now')),
(847, 'transaction_mgmt.min_placeholder', 'en', 'Min', 'transaction_mgmt', 'Minimum amount placeholder', datetime('now')),
(848, 'transaction_mgmt.min_placeholder', 'ja', '最小', 'transaction_mgmt', '最小金額プレースホルダー', datetime('now')),
(849, 'transaction_mgmt.max_placeholder', 'en', 'Max', 'transaction_mgmt', 'Maximum amount placeholder', datetime('now')),
(850, 'transaction_mgmt.max_placeholder', 'ja', '最大', 'transaction_mgmt', '最大金額プレースホルダー', datetime('now')),
(851, 'transaction_mgmt.search_placeholder', 'en', 'Search in memo', 'transaction_mgmt', 'Keyword search placeholder', datetime('now')),
(852, 'transaction_mgmt.search_placeholder', 'ja', 'メモを検索', 'transaction_mgmt', 'キーワード検索プレースホルダー', datetime('now')),

-- ============================================================================
-- Shop Management (shop_mgmt.*)
-- ============================================================================

-- Menu item
(921, 'menu.shop_management', 'en', 'Shop Management', 'menu', 'Shop management menu item', datetime('now')),
(922, 'menu.shop_management', 'ja', '店舗管理', 'menu', '店舗管理メニュー項目', datetime('now')),

-- Shop Management Screen
(923, 'shop_mgmt.title', 'en', 'Shop Management', 'shop_mgmt', 'Shop management page title', datetime('now')),
(924, 'shop_mgmt.title', 'ja', '店舗管理', 'shop_mgmt', '店舗管理ページタイトル', datetime('now')),
(925, 'shop_mgmt.add_new', 'en', 'Add New Shop', 'shop_mgmt', 'Add new shop button', datetime('now')),
(926, 'shop_mgmt.add_new', 'ja', '新規店舗追加', 'shop_mgmt', '新規店舗追加ボタン', datetime('now')),
(927, 'shop_mgmt.shop_name', 'en', 'Shop Name', 'shop_mgmt', 'Shop name label', datetime('now')),
(928, 'shop_mgmt.shop_name', 'ja', '店舗名', 'shop_mgmt', '店舗名ラベル', datetime('now')),
(929, 'shop_mgmt.memo', 'en', 'Memo', 'shop_mgmt', 'Memo label', datetime('now')),
(930, 'shop_mgmt.memo', 'ja', 'メモ', 'shop_mgmt', 'メモラベル', datetime('now')),
(931, 'shop_mgmt.modal_title_add', 'en', 'Add Shop', 'shop_mgmt', 'Add shop modal title', datetime('now')),
(932, 'shop_mgmt.modal_title_add', 'ja', '店舗追加', 'shop_mgmt', '店舗追加モーダルタイトル', datetime('now')),
(933, 'shop_mgmt.modal_title_edit', 'en', 'Edit Shop', 'shop_mgmt', 'Edit shop modal title', datetime('now')),
(934, 'shop_mgmt.modal_title_edit', 'ja', '店舗編集', 'shop_mgmt', '店舗編集モーダルタイトル', datetime('now')),
(935, 'shop_mgmt.no_shops', 'en', 'No shops found', 'shop_mgmt', 'No shops message', datetime('now')),
(936, 'shop_mgmt.no_shops', 'ja', '店舗が見つかりません', 'shop_mgmt', '店舗なしメッセージ', datetime('now')),
(937, 'shop_mgmt.delete_confirmation', 'en', 'Delete Confirmation', 'shop_mgmt', 'Delete confirmation title', datetime('now')),
(938, 'shop_mgmt.delete_confirmation', 'ja', '削除確認', 'shop_mgmt', '削除確認タイトル', datetime('now')),
(939, 'shop_mgmt.delete_message', 'en', 'Are you sure you want to delete this shop?', 'shop_mgmt', 'Delete confirmation message', datetime('now')),
(940, 'shop_mgmt.delete_message', 'ja', 'この店舗を削除してもよろしいですか？', 'shop_mgmt', '削除確認メッセージ', datetime('now')),
(941, 'shop_mgmt.duplicate_error', 'en', 'Shop name already exists', 'shop_mgmt', 'Duplicate shop name error', datetime('now')),
(942, 'shop_mgmt.duplicate_error', 'ja', 'この店舗名は既に登録されています', 'shop_mgmt', '重複店舗名エラー', datetime('now')),
(943, 'shop_mgmt.empty_name', 'en', 'Shop name is required', 'shop_mgmt', 'Empty shop name error', datetime('now')),
(944, 'shop_mgmt.empty_name', 'ja', '店舗名を入力してください', 'shop_mgmt', '空店舗名エラー', datetime('now')),
(945, 'shop_mgmt.failed_to_initialize', 'en', 'Failed to initialize', 'shop_mgmt', 'Initialization failure message', datetime('now')),
(946, 'shop_mgmt.failed_to_initialize', 'ja', '初期化に失敗しました', 'shop_mgmt', '初期化失敗メッセージ', datetime('now')),
(947, 'shop_mgmt.failed_to_load', 'en', 'Failed to load shops', 'shop_mgmt', 'Load failure message', datetime('now')),
(948, 'shop_mgmt.failed_to_load', 'ja', '店舗の読み込みに失敗しました', 'shop_mgmt', '読み込み失敗メッセージ', datetime('now')),
(949, 'shop_mgmt.failed_to_delete', 'en', 'Failed to delete shop', 'shop_mgmt', 'Delete failure message', datetime('now')),
(950, 'shop_mgmt.failed_to_delete', 'ja', '店舗の削除に失敗しました', 'shop_mgmt', '削除失敗メッセージ', datetime('now')),

-- ============================================================================
-- Common (IS_DISABLED feature)
-- ============================================================================

(907, 'common.show_disabled', 'en', 'Show Disabled Items', 'common', 'Show disabled items button', datetime('now')),
(908, 'common.show_disabled', 'ja', '非表示項目を表示', 'common', '非表示項目表示ボタン', datetime('now')),
(909, 'common.hide_disabled', 'en', 'Hide Disabled Items', 'common', 'Hide disabled items button', datetime('now')),
(910, 'common.hide_disabled', 'ja', '非表示項目を隠す', 'common', '非表示項目非表示ボタン', datetime('now')),
(911, 'common.is_disabled', 'en', 'Disabled', 'common', 'Disabled checkbox label', datetime('now')),
(912, 'common.is_disabled', 'ja', '非表示', 'common', '非表示チェックボックスラベル', datetime('now')),
(913, 'common.loading', 'en', 'Loading...', 'common', 'Loading message', datetime('now')),
(914, 'common.loading', 'ja', '読み込み中...', 'common', '読み込み中メッセージ', datetime('now')),
(915, 'common.cancel', 'en', 'Cancel', 'common', 'Cancel button', datetime('now')),
(916, 'common.cancel', 'ja', 'キャンセル', 'common', 'キャンセルボタン', datetime('now')),
(917, 'common.save', 'en', 'Save', 'common', 'Save button', datetime('now')),
(918, 'common.save', 'ja', '保存', 'common', '保存ボタン', datetime('now')),
(919, 'menu.back_to_main', 'en', '← Back to Main', 'menu', 'Back to main menu button', datetime('now')),
(920, 'menu.back_to_main', 'ja', '← メインに戻る', 'menu', 'メインメニューに戻るボタン', datetime('now'));

-- ============================================================================
-- Additional Management Screen Resources (added 2024-11-14)
-- ============================================================================

-- Shop Management
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
(921, 'shop_mgmt.shop_list', 'en', 'Shop List', 'shop_mgmt', 'Shop list section title', datetime('now')),
(922, 'shop_mgmt.shop_list', 'ja', '店舗一覧', 'shop_mgmt', '店舗一覧セクションタイトル', datetime('now')),
(923, 'shop_mgmt.manage_shop', 'en', 'Manage Shop', 'shop_mgmt', 'Manage shop button', datetime('now')),
(924, 'shop_mgmt.manage_shop', 'ja', '店舗管理', 'shop_mgmt', '店舗管理ボタン', datetime('now')),
(925, 'shop_mgmt.delete_shop', 'en', 'Delete Shop', 'shop_mgmt', 'Delete shop modal title', datetime('now')),
(926, 'shop_mgmt.delete_shop', 'ja', '店舗削除', 'shop_mgmt', '店舗削除モーダルタイトル', datetime('now')),

-- Manufacturer Management
(927, 'manufacturer_mgmt.manufacturer_list', 'en', 'Manufacturer List', 'manufacturer_mgmt', 'Manufacturer list section title', datetime('now')),
(928, 'manufacturer_mgmt.manufacturer_list', 'ja', 'メーカー一覧', 'manufacturer_mgmt', 'メーカー一覧セクションタイトル', datetime('now')),
(929, 'manufacturer_mgmt.delete_manufacturer', 'en', 'Delete Manufacturer', 'manufacturer_mgmt', 'Delete manufacturer modal title', datetime('now')),
(930, 'manufacturer_mgmt.delete_manufacturer', 'ja', 'メーカー削除', 'manufacturer_mgmt', 'メーカー削除モーダルタイトル', datetime('now')),

-- Product Management
(931, 'product_mgmt.product_list', 'en', 'Product List', 'product_mgmt', 'Product list section title', datetime('now')),
(932, 'product_mgmt.product_list', 'ja', '商品一覧', 'product_mgmt', '商品一覧セクションタイトル', datetime('now')),
(933, 'product_mgmt.delete_product', 'en', 'Delete Product', 'product_mgmt', 'Delete product modal title', datetime('now')),
(934, 'product_mgmt.delete_product', 'ja', '商品削除', 'product_mgmt', '商品削除モーダルタイトル', datetime('now')),

-- Transaction Modal
(935, 'transaction_modal.shop', 'en', 'Shop', 'transaction_modal', 'Shop label', datetime('now')),
(936, 'transaction_modal.shop', 'ja', '店舗', 'transaction_modal', '店舗ラベル', datetime('now')),
(937, 'transaction_modal.manage_shops', 'en', 'Manage Shops', 'transaction_modal', 'Manage shops button', datetime('now')),
(938, 'transaction_modal.manage_shops', 'ja', '店舗管理', 'transaction_modal', '店舗管理ボタン', datetime('now')),

-- Common (Apply button)
(939, 'common.apply', 'en', 'Apply', 'common', 'Apply button', datetime('now')),
(940, 'common.apply', 'ja', '適用', 'common', '適用ボタン', datetime('now'));
