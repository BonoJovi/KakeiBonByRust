// User role constants
pub const ROLE_ADMIN: i64 = 0;
pub const ROLE_USER: i64 = 1;
/// Reserved for test validation of role constants
#[allow(dead_code)]
pub const ROLE_VISIT: i64 = 999;

// Database constants
pub const DB_DIR_NAME: &str = ".kakeibon";
pub const DB_FILE_NAME: &str = "KakeiBonDB.sqlite3";
pub const SQL_INIT_FILE_PATH: &str = "res/sql/dbaccess.sql";

// Language constants
pub const LANG_ENGLISH: &str = "en";
pub const LANG_JAPANESE: &str = "ja";
pub const LANG_DEFAULT: &str = LANG_JAPANESE;

// Font size constants
pub const FONT_SIZE_SMALL: &str = "small";
pub const FONT_SIZE_MEDIUM: &str = "medium";
pub const FONT_SIZE_LARGE: &str = "large";
pub const FONT_SIZE_DEFAULT: &str = FONT_SIZE_MEDIUM;

// Tax rounding mode constants
pub const TAX_ROUND_DOWN: i64 = 0;
pub const TAX_ROUND_HALF_UP: i64 = 1;
pub const TAX_ROUND_UP: i64 = 2;

// Tax inclusion type constants (used in tests)
#[allow(dead_code)]
pub const TAX_INCLUDED: i64 = 0;  // 内税 - tax is included in prices
#[allow(dead_code)]
pub const TAX_EXCLUDED: i64 = 1;  // 外税 - tax is calculated separately

// Recurring scheduled transactions (v2.1.0) — HOLIDAY_SHIFT_TYPE column values
pub const HOLIDAY_SHIFT_NONE: i32 = 0;
pub const HOLIDAY_SHIFT_PREV: i32 = 1;
pub const HOLIDAY_SHIFT_NEXT: i32 = 2;

// Recurring scheduled transactions — PERIOD_UNIT column values
pub const PERIOD_UNIT_DAY: &str = "DAY";
pub const PERIOD_UNIT_WEEK: &str = "WEEK";
pub const PERIOD_UNIT_MONTH: &str = "MONTH";
pub const PERIOD_UNIT_YEAR: &str = "YEAR";

// Recurring scheduled transactions — MONTH_DAY_RULE_TYPE column values
pub const MONTH_DAY_RULE_TYPE_DAY: &str = "DAY";
pub const MONTH_DAY_RULE_TYPE_DAY_OR_END: &str = "DAY_OR_END";
pub const MONTH_DAY_RULE_TYPE_END: &str = "END";
pub const MONTH_DAY_RULE_TYPE_NTH_WEEKDAY: &str = "NTH_WEEKDAY";

// Bounded-field length limits (in characters, not bytes).
// Paired with `validation.max_length` i18n key for the user-facing message.
pub const MAX_NAME_LEN: usize = 128;          // USERS.NAME, CATEGORY*_NAME, ACCOUNTS.ACCOUNT_NAME, SHOPS/MANUFACTURERS/PRODUCTS names
#[allow(dead_code)]
pub const MAX_I18N_NAME_LEN: usize = 256;     // CATEGORY*_I18N.*_NAME_I18N
pub const MAX_ITEM_NAME_LEN: usize = 200;     // TRANSACTIONS_DETAIL.ITEM_NAME, RECURRING_RULE_DETAILS.ITEM_NAME
#[allow(dead_code)]
pub const MAX_RULE_NAME_LEN: usize = 200;     // RECURRING_RULES.RULE_NAME
pub const MAX_MEMO_LEN: usize = 1000;         // MEMOS.MEMO_TEXT (used by transactions and recurring rules)
