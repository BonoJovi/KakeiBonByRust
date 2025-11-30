-- Add missing font size modal translation resources
-- Date: 2025-11-18

INSERT INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES
(1049, 'font_size.description', 'ja', '希望のフォントサイズをカスタマイズ（10-30px）', 'font_size', 'フォントサイズ説明', datetime('now', 'localtime')),
(1050, 'font_size.description', 'en', 'Customize your preferred font size (10-30px)', 'font_size', 'Font size description', datetime('now', 'localtime')),
(1051, 'font_size.label', 'ja', 'フォントサイズ：', 'font_size', 'フォントサイズラベル', datetime('now', 'localtime')),
(1052, 'font_size.label', 'en', 'Font Size:', 'font_size', 'Font size label', datetime('now', 'localtime'));
