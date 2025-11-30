-- Add missing translation resources for management screens
-- Date: 2025-11-14

-- Shop Management
INSERT OR REPLACE INTO I18N_RESOURCES (RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES
('shop_mgmt.shop_list', 'ja', '店舗一覧', 'shop_mgmt', datetime('now')),
('shop_mgmt.shop_list', 'en', 'Shop List', 'shop_mgmt', datetime('now')),
('shop_mgmt.manage_shop', 'ja', '店舗管理', 'shop_mgmt', datetime('now')),
('shop_mgmt.manage_shop', 'en', 'Manage Shop', 'shop_mgmt', datetime('now')),
('shop_mgmt.delete_shop', 'ja', '店舗削除', 'shop_mgmt', datetime('now')),
('shop_mgmt.delete_shop', 'en', 'Delete Shop', 'shop_mgmt', datetime('now'));

-- Account Management
INSERT OR REPLACE INTO I18N_RESOURCES (RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES
('account_mgmt.account_list', 'ja', '口座一覧', 'account_mgmt', datetime('now')),
('account_mgmt.account_list', 'en', 'Account List', 'account_mgmt', datetime('now'));

-- Manufacturer Management
INSERT OR REPLACE INTO I18N_RESOURCES (RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES
('manufacturer_mgmt.manufacturer_list', 'ja', 'メーカー一覧', 'manufacturer_mgmt', datetime('now')),
('manufacturer_mgmt.manufacturer_list', 'en', 'Manufacturer List', 'manufacturer_mgmt', datetime('now')),
('manufacturer_mgmt.delete_manufacturer', 'ja', 'メーカー削除', 'manufacturer_mgmt', datetime('now')),
('manufacturer_mgmt.delete_manufacturer', 'en', 'Delete Manufacturer', 'manufacturer_mgmt', datetime('now'));

-- Product Management
INSERT OR REPLACE INTO I18N_RESOURCES (RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES
('product_mgmt.product_list', 'ja', '商品一覧', 'product_mgmt', datetime('now')),
('product_mgmt.product_list', 'en', 'Product List', 'product_mgmt', datetime('now')),
('product_mgmt.delete_product', 'ja', '商品削除', 'product_mgmt', datetime('now')),
('product_mgmt.delete_product', 'en', 'Delete Product', 'product_mgmt', datetime('now'));

-- Transaction Modal
INSERT OR REPLACE INTO I18N_RESOURCES (RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES
('transaction_modal.shop', 'ja', '店舗', 'transaction_modal', datetime('now')),
('transaction_modal.shop', 'en', 'Shop', 'transaction_modal', datetime('now')),
('transaction_modal.manage_shops', 'ja', '店舗管理', 'transaction_modal', datetime('now')),
('transaction_modal.manage_shops', 'en', 'Manage Shops', 'transaction_modal', datetime('now'));

-- Common (Apply button)
INSERT OR REPLACE INTO I18N_RESOURCES (RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES
('common.apply', 'ja', '適用', 'common', datetime('now')),
('common.apply', 'en', 'Apply', 'common', datetime('now'));
