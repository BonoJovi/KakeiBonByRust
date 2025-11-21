-- Translation resources for Menu submenu reorganization
-- Adds "Master Data" and "Reports" submenu labels
-- To be added to I18N_RESOURCES table

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
-- Master Data submenu
(2001, 'menu.master_data', 'en', 'Master Data', 'menu', 'Master data submenu label', datetime('now')),
(2002, 'menu.master_data', 'ja', 'マスタ管理', 'menu', 'マスタ管理サブメニューラベル', datetime('now')),

-- Reports submenu
(2003, 'menu.reports', 'en', 'Reports', 'menu', 'Reports submenu label', datetime('now')),
(2004, 'menu.reports', 'ja', '集計', 'menu', '集計サブメニューラベル', datetime('now'));
