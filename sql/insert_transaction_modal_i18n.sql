-- Insert translation resources for transaction management modal
INSERT INTO I18N_RESOURCES (RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, ENTRY_DT) VALUES
-- Transaction modal fields
('transaction_modal.title_add', 'ja', '入出金登録', datetime('now')),
('transaction_modal.title_add', 'en', 'Add Transaction', datetime('now')),
('transaction_modal.title_edit', 'ja', '入出金編集', datetime('now')),
('transaction_modal.title_edit', 'en', 'Edit Transaction', datetime('now')),
('transaction_modal.transaction_date', 'ja', '日付', datetime('now')),
('transaction_modal.transaction_date', 'en', 'Date', datetime('now')),
('transaction_modal.category1', 'ja', '大分類', datetime('now')),
('transaction_modal.category1', 'en', 'Category', datetime('now')),
('transaction_modal.from_account', 'ja', '支払元口座', datetime('now')),
('transaction_modal.from_account', 'en', 'From Account', datetime('now')),
('transaction_modal.to_account', 'ja', '入金先口座', datetime('now')),
('transaction_modal.to_account', 'en', 'To Account', datetime('now')),
('transaction_modal.total_amount', 'ja', '合計金額', datetime('now')),
('transaction_modal.total_amount', 'en', 'Total Amount', datetime('now')),
('transaction_modal.tax_rate', 'ja', '税率(%)', datetime('now')),
('transaction_modal.tax_rate', 'en', 'Tax Rate(%)', datetime('now')),
('transaction_modal.tax_type', 'ja', '税計算', datetime('now')),
('transaction_modal.tax_type', 'en', 'Tax Type', datetime('now')),
('transaction_modal.memo', 'ja', 'メモ', datetime('now')),
('transaction_modal.memo', 'en', 'Memo', datetime('now')),
('transaction_modal.save', 'ja', '保存', datetime('now')),
('transaction_modal.save', 'en', 'Save', datetime('now')),
('transaction_modal.cancel', 'ja', 'キャンセル', datetime('now')),
('transaction_modal.cancel', 'en', 'Cancel', datetime('now')),

-- Tax types
('tax_type.round_up', 'ja', '切り上げ', datetime('now')),
('tax_type.round_up', 'en', 'Round Up', datetime('now')),
('tax_type.round_down', 'ja', '切り捨て', datetime('now')),
('tax_type.round_down', 'en', 'Round Down', datetime('now')),
('tax_type.round', 'ja', '四捨五入', datetime('now')),
('tax_type.round', 'en', 'Round', datetime('now')),

-- Common "Unspecified"
('common.unspecified', 'ja', '指定なし', datetime('now')),
('common.unspecified', 'en', 'Unspecified', datetime('now'))
ON CONFLICT(RESOURCE_KEY, LANG_CODE) DO UPDATE SET RESOURCE_VALUE = excluded.RESOURCE_VALUE;
