-- Translation resources for dashboard
-- Category: dashboard, menu

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
-- Menu item
(2001, 'menu.dashboard', 'en', 'Dashboard', 'menu', 'Dashboard menu item', datetime('now')),
(2002, 'menu.dashboard', 'ja', 'ダッシュボード', 'menu', 'ダッシュボードメニュー項目', datetime('now')),

-- Dashboard page
(2003, 'dashboard.title', 'en', 'Dashboard', 'dashboard', 'Dashboard page title', datetime('now')),
(2004, 'dashboard.title', 'ja', 'ダッシュボード', 'dashboard', 'ダッシュボードページタイトル', datetime('now')),

(2005, 'dashboard.filter', 'en', 'Filter', 'dashboard', 'Filter section header', datetime('now')),
(2006, 'dashboard.filter', 'ja', 'フィルター', 'dashboard', 'フィルターセクションヘッダー', datetime('now')),

(2007, 'dashboard.year', 'en', 'Year:', 'dashboard', 'Year label', datetime('now')),
(2008, 'dashboard.year', 'ja', '年:', 'dashboard', '年ラベル', datetime('now')),

(2009, 'dashboard.month', 'en', 'Month:', 'dashboard', 'Month label', datetime('now')),
(2010, 'dashboard.month', 'ja', '月:', 'dashboard', '月ラベル', datetime('now')),

(2011, 'dashboard.trend_months', 'en', 'Aggregation Period:', 'dashboard', 'Aggregation period label', datetime('now')),
(2012, 'dashboard.trend_months', 'ja', '集計期間:', 'dashboard', '集計期間ラベル', datetime('now')),

(2013, 'dashboard.execute', 'en', 'Execute', 'dashboard', 'Execute button', datetime('now')),
(2014, 'dashboard.execute', 'ja', '集計', 'dashboard', '集計ボタン', datetime('now')),

-- Chart titles
(2015, 'dashboard.expense_by_category', 'en', 'Expense by Category', 'dashboard', 'Pie chart title', datetime('now')),
(2016, 'dashboard.expense_by_category', 'ja', 'カテゴリ別支出', 'dashboard', '円グラフタイトル', datetime('now')),

(2017, 'dashboard.category_comparison', 'en', 'Category Comparison', 'dashboard', 'Bar chart title', datetime('now')),
(2018, 'dashboard.category_comparison', 'ja', 'カテゴリ別比較', 'dashboard', '棒グラフタイトル', datetime('now')),

(2019, 'dashboard.monthly_trend', 'en', 'Monthly Trend', 'dashboard', 'Line chart title', datetime('now')),
(2020, 'dashboard.monthly_trend', 'ja', '月別推移', 'dashboard', '折れ線グラフタイトル', datetime('now')),

-- Chart labels
(2021, 'dashboard.expense', 'en', 'Expense', 'dashboard', 'Expense label', datetime('now')),
(2022, 'dashboard.expense', 'ja', '支出', 'dashboard', '支出ラベル', datetime('now')),

(2023, 'dashboard.income', 'en', 'Income', 'dashboard', 'Income label', datetime('now')),
(2024, 'dashboard.income', 'ja', '収入', 'dashboard', '収入ラベル', datetime('now')),

(2025, 'dashboard.balance', 'en', 'Balance', 'dashboard', 'Balance label', datetime('now')),
(2026, 'dashboard.balance', 'ja', '収支', 'dashboard', '収支ラベル', datetime('now')),

-- Messages
(2027, 'dashboard.no_data', 'en', 'No data available', 'dashboard', 'No data message', datetime('now')),
(2028, 'dashboard.no_data', 'ja', 'データがありません', 'dashboard', 'データなしメッセージ', datetime('now')),

(2029, 'dashboard.loading', 'en', 'Loading...', 'dashboard', 'Loading message', datetime('now')),
(2030, 'dashboard.loading', 'ja', '読み込み中...', 'dashboard', '読み込み中メッセージ', datetime('now')),

-- Errors
(2031, 'dashboard.error_invalid_year', 'en', 'Invalid year', 'dashboard', 'Invalid year error', datetime('now')),
(2032, 'dashboard.error_invalid_year', 'ja', '無効な年です', 'dashboard', '無効な年エラー', datetime('now')),

(2033, 'dashboard.error_invalid_month', 'en', 'Invalid month', 'dashboard', 'Invalid month error', datetime('now')),
(2034, 'dashboard.error_invalid_month', 'ja', '無効な月です', 'dashboard', '無効な月エラー', datetime('now')),

-- Access control
(2035, 'dashboard.admin_access_denied', 'en', 'Dashboard is not available for administrator accounts. Please login as a regular user.', 'dashboard', 'Admin access denied message', datetime('now')),
(2036, 'dashboard.admin_access_denied', 'ja', 'ダッシュボードは管理者アカウントでは利用できません。一般ユーザーでログインしてください。', 'dashboard', '管理者アクセス拒否メッセージ', datetime('now')),

-- Period display suffixes
(2037, 'dashboard.year_suffix', 'en', '/', 'dashboard', 'Year suffix for period display', datetime('now')),
(2038, 'dashboard.year_suffix', 'ja', '年', 'dashboard', '期間表示の年サフィックス', datetime('now')),
(2039, 'dashboard.month_suffix', 'en', '', 'dashboard', 'Month suffix for period display', datetime('now')),
(2040, 'dashboard.month_suffix', 'ja', '月', 'dashboard', '期間表示の月サフィックス', datetime('now'));
