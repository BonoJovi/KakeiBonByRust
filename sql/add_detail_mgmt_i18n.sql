-- Add i18n resources for transaction detail management feature
-- Date: 2025-11-16 JST
-- Note: Uses INSERT OR IGNORE to avoid duplicates if already exists

-- Common category labels (reusable across screens)
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES 
('common.category1', 'ja', '大分類', 'common', datetime('now')),
('common.category1', 'en', 'Major Category', 'common', datetime('now')),

('common.category2', 'ja', '中分類', 'common', datetime('now')),
('common.category2', 'en', 'Medium Category', 'common', datetime('now')),

('common.category3', 'ja', '小分類', 'common', datetime('now')),
('common.category3', 'en', 'Minor Category', 'common', datetime('now')),

('common.category', 'ja', 'カテゴリ', 'common', datetime('now')),
('common.category', 'en', 'Category', 'common', datetime('now')),

('common.select', 'ja', '選択してください', 'common', datetime('now')),
('common.select', 'en', 'Select', 'common', datetime('now')),

('common.no_data', 'ja', 'データがありません', 'common', datetime('now')),
('common.no_data', 'en', 'No data available', 'common', datetime('now'));

-- Menu
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES 
('menu.back_to_transactions', 'ja', '入出金管理に戻る', 'menu', datetime('now')),
('menu.back_to_transactions', 'en', 'Back to Transactions', 'menu', datetime('now'));

-- Transaction Detail Management Screen
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES 
('detail_mgmt.title', 'ja', '明細管理', 'detail_mgmt', datetime('now')),
('detail_mgmt.title', 'en', 'Transaction Detail Management', 'detail_mgmt', datetime('now')),

('detail_mgmt.transaction_date', 'ja', '取引日', 'detail_mgmt', datetime('now')),
('detail_mgmt.transaction_date', 'en', 'Transaction Date', 'detail_mgmt', datetime('now')),

('detail_mgmt.account', 'ja', '口座', 'detail_mgmt', datetime('now')),
('detail_mgmt.account', 'en', 'Account', 'detail_mgmt', datetime('now')),

('detail_mgmt.shop', 'ja', '店舗', 'detail_mgmt', datetime('now')),
('detail_mgmt.shop', 'en', 'Shop', 'detail_mgmt', datetime('now')),

('detail_mgmt.total_amount', 'ja', '合計金額', 'detail_mgmt', datetime('now')),
('detail_mgmt.total_amount', 'en', 'Total Amount', 'detail_mgmt', datetime('now')),

('detail_mgmt.add_detail', 'ja', '+ 明細追加', 'detail_mgmt', datetime('now')),
('detail_mgmt.add_detail', 'en', '+ Add Detail', 'detail_mgmt', datetime('now')),

('detail_mgmt.edit_detail', 'ja', '明細編集', 'detail_mgmt', datetime('now')),
('detail_mgmt.edit_detail', 'en', 'Edit Detail', 'detail_mgmt', datetime('now')),

('detail_mgmt.item_name', 'ja', '品目名', 'detail_mgmt', datetime('now')),
('detail_mgmt.item_name', 'en', 'Item Name', 'detail_mgmt', datetime('now')),

('detail_mgmt.amount', 'ja', '金額', 'detail_mgmt', datetime('now')),
('detail_mgmt.amount', 'en', 'Amount', 'detail_mgmt', datetime('now')),

('detail_mgmt.amount_excluding_tax', 'ja', '税抜金額', 'detail_mgmt', datetime('now')),
('detail_mgmt.amount_excluding_tax', 'en', 'Amount (Excl. Tax)', 'detail_mgmt', datetime('now')),

('detail_mgmt.amount_including_tax', 'ja', '税込金額', 'detail_mgmt', datetime('now')),
('detail_mgmt.amount_including_tax', 'en', 'Amount (Incl. Tax)', 'detail_mgmt', datetime('now')),

('detail_mgmt.tax', 'ja', '税額', 'detail_mgmt', datetime('now')),
('detail_mgmt.tax', 'en', 'Tax', 'detail_mgmt', datetime('now')),

('detail_mgmt.tax_rate', 'ja', '税率 (%)', 'detail_mgmt', datetime('now')),
('detail_mgmt.tax_rate', 'en', 'Tax Rate (%)', 'detail_mgmt', datetime('now')),

('detail_mgmt.tax_amount', 'ja', '税額', 'detail_mgmt', datetime('now')),
('detail_mgmt.tax_amount', 'en', 'Tax Amount', 'detail_mgmt', datetime('now')),

('detail_mgmt.memo', 'ja', 'メモ', 'detail_mgmt', datetime('now')),
('detail_mgmt.memo', 'en', 'Memo', 'detail_mgmt', datetime('now')),

('detail_mgmt.delete_confirm_title', 'ja', '明細削除', 'detail_mgmt', datetime('now')),
('detail_mgmt.delete_confirm_title', 'en', 'Delete Detail', 'detail_mgmt', datetime('now')),

('detail_mgmt.delete_confirm_message', 'ja', 'この明細を削除してもよろしいですか？', 'detail_mgmt', datetime('now')),
('detail_mgmt.delete_confirm_message', 'en', 'Are you sure you want to delete this detail?', 'detail_mgmt', datetime('now')),

-- Validation messages
('detail_mgmt.error_item_name_required', 'ja', '品目名を入力してください', 'detail_mgmt', datetime('now')),
('detail_mgmt.error_item_name_required', 'en', 'Item name is required', 'detail_mgmt', datetime('now')),

('detail_mgmt.error_category_required', 'ja', 'カテゴリを選択してください', 'detail_mgmt', datetime('now')),
('detail_mgmt.error_category_required', 'en', 'Category is required', 'detail_mgmt', datetime('now')),

('detail_mgmt.error_invalid_amount', 'ja', '有効な金額を入力してください', 'detail_mgmt', datetime('now')),
('detail_mgmt.error_invalid_amount', 'en', 'Please enter a valid amount', 'detail_mgmt', datetime('now')),

-- Success/Error messages
('detail_mgmt.save_success', 'ja', '明細を保存しました', 'detail_mgmt', datetime('now')),
('detail_mgmt.save_success', 'en', 'Detail saved successfully', 'detail_mgmt', datetime('now')),

('detail_mgmt.save_error', 'ja', '明細の保存に失敗しました', 'detail_mgmt', datetime('now')),
('detail_mgmt.save_error', 'en', 'Failed to save detail', 'detail_mgmt', datetime('now')),

('detail_mgmt.delete_success', 'ja', '明細を削除しました', 'detail_mgmt', datetime('now')),
('detail_mgmt.delete_success', 'en', 'Detail deleted successfully', 'detail_mgmt', datetime('now')),

('detail_mgmt.delete_error', 'ja', '明細の削除に失敗しました', 'detail_mgmt', datetime('now')),
('detail_mgmt.delete_error', 'en', 'Failed to delete detail', 'detail_mgmt', datetime('now'));

-- Rounding warning messages (added 2025-11-17)
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES 
('detail_mgmt.rounding_warning_title', 'ja', '端数処理による誤差', 'detail_mgmt', datetime('now')),
('detail_mgmt.rounding_warning_title', 'en', 'Rounding Discrepancy', 'detail_mgmt', datetime('now')),

('detail_mgmt.rounding_warning_message', 'ja', '税込金額から計算した結果、税抜金額に{diff}円の誤差が発生しました。自動計算した税抜金額から税込金額を再計算すると{calculated}円(入力は{userInput}円)になります。正確な金額が必要な場合は、税抜金額を直接入力してください。', 'detail_mgmt', datetime('now')),
('detail_mgmt.rounding_warning_message', 'en', 'A rounding discrepancy of ¥{diff} occurred in the tax-excluded amount. Recalculating the tax-included amount from the calculated tax-excluded amount results in ¥{calculated} (input was ¥{userInput}). For accurate amounts, please enter the tax-excluded amount directly.', 'detail_mgmt', datetime('now'));
