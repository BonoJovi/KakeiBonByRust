// User role constants
pub const ROLE_ADMIN: i64 = 0;
pub const ROLE_USER: i64 = 1;

// Reserved for future implementation: Read-only visitor role
// TODO: Implement guest/visitor access feature with limited permissions
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

// Tax inclusion type constants
pub const TAX_INCLUDED: i64 = 0;  // 内税 - tax is included in prices
pub const TAX_EXCLUDED: i64 = 1;  // 外税 - tax is calculated separately
