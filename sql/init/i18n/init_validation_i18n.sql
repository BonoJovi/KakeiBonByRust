-- Translation resources for validation
-- Auto-generated from database
-- Category: validation

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
(715, 'validation.required', 'en', 'Please fill out this field', 'validation', 'Required field validation message', datetime('now')),
(716, 'validation.required', 'ja', 'このフィールドを入力してください', 'validation', '必須フィールドのバリデーションメッセージ', datetime('now')),
(2343, 'validation.invalid_period_start_day', 'en', 'Start day must be between 1 and 31', 'validation', 'Invalid period start day', datetime('now')),
(2344, 'validation.invalid_period_start_day', 'ja', '起算日は 1〜31 の範囲で指定してください', 'validation', '起算日範囲エラー', datetime('now')),
(2345, 'validation.invalid_period_start_month', 'en', 'Start month must be between 1 and 12', 'validation', 'Invalid period start month', datetime('now')),
(2346, 'validation.invalid_period_start_month', 'ja', '起算月は 1〜12 の範囲で指定してください', 'validation', '起算月範囲エラー', datetime('now')),
(2359, 'validation.invalid_month_period_holiday_shift', 'en', 'Month period holiday shift must be 0, 1, or 2', 'validation', 'Invalid month period holiday shift', datetime('now')),
(2360, 'validation.invalid_month_period_holiday_shift', 'ja', '月次起算日のシフト設定は 0 / 1 / 2 のいずれかである必要があります', 'validation', '月次起算日シフト範囲エラー', datetime('now'));
