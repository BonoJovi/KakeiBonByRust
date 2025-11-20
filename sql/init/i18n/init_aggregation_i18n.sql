-- Translation resources for Aggregation feature
-- To be added to I18N_RESOURCES table
-- Starting from RESOURCE_ID 1053

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
-- Menu item
(1053, 'menu.aggregation', 'en', 'Aggregation', 'menu', 'Aggregation menu item', datetime('now')),
(1054, 'menu.aggregation', 'ja', '集計', 'menu', '集計メニュー項目', datetime('now')),

-- Aggregation Screen
(1055, 'aggregation.title', 'en', 'Monthly Aggregation', 'aggregation', 'Aggregation page title', datetime('now')),
(1056, 'aggregation.title', 'ja', '月次集計', 'aggregation', '集計ページタイトル', datetime('now')),
(1057, 'aggregation.filter', 'en', 'Filter', 'aggregation', 'Filter section title', datetime('now')),
(1058, 'aggregation.filter', 'ja', 'フィルタ', 'aggregation', 'フィルタセクションタイトル', datetime('now')),

-- Filter labels
(1059, 'aggregation.year', 'en', 'Year', 'aggregation', 'Year label', datetime('now')),
(1060, 'aggregation.year', 'ja', '年', 'aggregation', '年ラベル', datetime('now')),
(1061, 'aggregation.month', 'en', 'Month', 'aggregation', 'Month label', datetime('now')),
(1062, 'aggregation.month', 'ja', '月', 'aggregation', '月ラベル', datetime('now')),
(1063, 'aggregation.group_by', 'en', 'Group By', 'aggregation', 'Group by label', datetime('now')),
(1064, 'aggregation.group_by', 'ja', '集計単位', 'aggregation', '集計単位ラベル', datetime('now')),

-- Group by options
(1065, 'aggregation.category1', 'en', 'Category (Major)', 'aggregation', 'Major category option', datetime('now')),
(1066, 'aggregation.category1', 'ja', '費目（大分類）', 'aggregation', '大分類オプション', datetime('now')),
(1067, 'aggregation.category2', 'en', 'Category (Middle)', 'aggregation', 'Middle category option', datetime('now')),
(1068, 'aggregation.category2', 'ja', '費目（中分類）', 'aggregation', '中分類オプション', datetime('now')),
(1069, 'aggregation.category3', 'en', 'Category (Minor)', 'aggregation', 'Minor category option', datetime('now')),
(1070, 'aggregation.category3', 'ja', '費目（小分類）', 'aggregation', '小分類オプション', datetime('now')),
(1071, 'aggregation.account', 'en', 'Account', 'aggregation', 'Account option', datetime('now')),
(1072, 'aggregation.account', 'ja', '口座', 'aggregation', '口座オプション', datetime('now')),
(1073, 'aggregation.shop', 'en', 'Shop', 'aggregation', 'Shop option', datetime('now')),
(1074, 'aggregation.shop', 'ja', '店舗', 'aggregation', '店舗オプション', datetime('now')),
(1075, 'aggregation.date', 'en', 'Date', 'aggregation', 'Date option', datetime('now')),
(1076, 'aggregation.date', 'ja', '日付', 'aggregation', '日付オプション', datetime('now')),

-- Execute button
(1077, 'aggregation.execute', 'en', 'Execute', 'aggregation', 'Execute button', datetime('now')),
(1078, 'aggregation.execute', 'ja', '実行', 'aggregation', '実行ボタン', datetime('now')),

-- Results section
(1079, 'aggregation.results', 'en', 'Results', 'aggregation', 'Results section title', datetime('now')),
(1080, 'aggregation.results', 'ja', '集計結果', 'aggregation', '集計結果セクションタイトル', datetime('now')),

-- Result table headers
(1081, 'aggregation.group_name', 'en', 'Group', 'aggregation', 'Group name column header', datetime('now')),
(1082, 'aggregation.group_name', 'ja', '集計項目', 'aggregation', '集計項目カラムヘッダー', datetime('now')),
(1083, 'aggregation.total_amount', 'en', 'Total Amount', 'aggregation', 'Total amount column header', datetime('now')),
(1084, 'aggregation.total_amount', 'ja', '合計金額', 'aggregation', '合計金額カラムヘッダー', datetime('now')),
(1085, 'aggregation.count', 'en', 'Count', 'aggregation', 'Count column header', datetime('now')),
(1086, 'aggregation.count', 'ja', '件数', 'aggregation', '件数カラムヘッダー', datetime('now')),
(1087, 'aggregation.avg_amount', 'en', 'Average', 'aggregation', 'Average amount column header', datetime('now')),
(1088, 'aggregation.avg_amount', 'ja', '平均金額', 'aggregation', '平均金額カラムヘッダー', datetime('now')),

-- Other labels
(1089, 'aggregation.items', 'en', 'items', 'aggregation', 'Items count label', datetime('now')),
(1090, 'aggregation.items', 'ja', '件', 'aggregation', '件数ラベル', datetime('now')),
(1091, 'aggregation.total', 'en', 'Total', 'aggregation', 'Total row label', datetime('now')),
(1092, 'aggregation.total', 'ja', '合計', 'aggregation', '合計行ラベル', datetime('now')),
(1093, 'aggregation.no_results', 'en', 'No results found', 'aggregation', 'No results message', datetime('now')),
(1094, 'aggregation.no_results', 'ja', '集計結果がありません', 'aggregation', '結果なしメッセージ', datetime('now')),

-- Error messages
(1095, 'aggregation.error_invalid_year', 'en', 'Please enter a valid year (1900-2100)', 'aggregation', 'Invalid year error', datetime('now')),
(1096, 'aggregation.error_invalid_year', 'ja', '有効な年を入力してください（1900-2100）', 'aggregation', '無効な年エラー', datetime('now')),
(1097, 'aggregation.error_invalid_month', 'en', 'Please select a valid month', 'aggregation', 'Invalid month error', datetime('now')),
(1098, 'aggregation.error_invalid_month', 'ja', '有効な月を選択してください', 'aggregation', '無効な月エラー', datetime('now'));
