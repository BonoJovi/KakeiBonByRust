// SQL Query Constants
// This module centralizes all SQL queries used in the application

// ============================================================================
// Authentication Service Queries
// ============================================================================

pub const AUTH_GET_USER_BY_NAME: &str = r#"
SELECT USER_ID, NAME, PAW, ROLE, ENTRY_DT, UPDATE_DT 
FROM USERS 
WHERE NAME = ?
"#;

pub const AUTH_GET_NEXT_USER_ID: &str = "SELECT COALESCE(MAX(USER_ID), 0) + 1 as next_id FROM USERS";

pub const AUTH_INSERT_USER: &str = r#"
INSERT INTO USERS (USER_ID, NAME, PAW, ROLE, ENTRY_DT) 
VALUES (?, ?, ?, ?, datetime('now'))
"#;

pub const AUTH_CHECK_TABLE_EXISTS: &str = r#"
SELECT name FROM sqlite_master 
WHERE type='table' AND name='USERS'
"#;

pub const AUTH_COUNT_USERS: &str = "SELECT COUNT(*) as count FROM USERS";

pub const AUTH_COUNT_USERS_BY_ROLE: &str = "SELECT COUNT(*) as count FROM USERS WHERE ROLE = ?";

pub const AUTH_GET_USER_NAME_BY_ID: &str = "SELECT NAME FROM USERS WHERE USER_ID = 1";

pub const AUTH_GET_PASSWORD_BY_ID: &str = "SELECT PAW FROM USERS WHERE USER_ID = 1";

pub const AUTH_GET_ROLE_BY_ID: &str = "SELECT ROLE FROM USERS WHERE USER_ID = 1";

// ============================================================================
// User Management Service Queries
// ============================================================================

pub const USER_LIST_USERS: &str = r#"
SELECT USER_ID, NAME, ROLE, ENTRY_DT, UPDATE_DT 
FROM USERS 
ORDER BY USER_ID
"#;

pub const USER_GET_BY_ID: &str = r#"
SELECT USER_ID, NAME, ROLE, ENTRY_DT, UPDATE_DT 
FROM USERS 
WHERE USER_ID = ?
"#;

pub const USER_CHECK_NAME_EXISTS: &str = "SELECT COUNT(*) as count FROM USERS WHERE NAME = ?";

pub const USER_GET_NEXT_ID: &str = "SELECT COALESCE(MAX(USER_ID), 0) + 1 as next_id FROM USERS";

pub const USER_INSERT: &str = r#"
INSERT INTO USERS (USER_ID, NAME, PAW, ROLE, ENTRY_DT) 
VALUES (?, ?, ?, ?, datetime('now'))
"#;

pub const USER_UPDATE_NAME: &str = r#"
UPDATE USERS 
SET NAME = ?, UPDATE_DT = datetime('now') 
WHERE USER_ID = ?
"#;

pub const USER_CHECK_NAME_EXISTS_EXCLUDING_ID: &str = r#"
SELECT COUNT(*) as count 
FROM USERS 
WHERE NAME = ? AND USER_ID != ?
"#;

pub const USER_UPDATE_PASSWORD: &str = r#"
UPDATE USERS 
SET PAW = ?, UPDATE_DT = datetime('now') 
WHERE USER_ID = ?
"#;

pub const USER_GET_PASSWORD_BY_ID: &str = "SELECT PAW FROM USERS WHERE USER_ID = ?";

pub const USER_DELETE: &str = "DELETE FROM USERS WHERE USER_ID = ?";

// ============================================================================
// Encryption Service Queries
// ============================================================================

pub const ENCRYPTION_LIST_FIELDS: &str = r#"
SELECT FIELD_ID, TABLE_NAME, COLUMN_NAME, DESCRIPTION, IS_ACTIVE 
FROM ENCRYPTED_FIELDS 
WHERE IS_ACTIVE = 1
"#;

pub const ENCRYPTION_GET_NEXT_FIELD_ID: &str = "SELECT COALESCE(MAX(FIELD_ID), 0) + 1 as next_id FROM ENCRYPTED_FIELDS";

pub const ENCRYPTION_INSERT_FIELD: &str = r#"
INSERT INTO ENCRYPTED_FIELDS (FIELD_ID, TABLE_NAME, COLUMN_NAME, DESCRIPTION, IS_ACTIVE, ENTRY_DT) 
VALUES (?, ?, ?, ?, 1, datetime('now'))
"#;

// ============================================================================
// I18N Service Queries
// ============================================================================

pub const I18N_GET_RESOURCE: &str = r#"
SELECT RESOURCE_VALUE 
FROM I18N_RESOURCES 
WHERE RESOURCE_KEY = ? AND LANG_CODE = ?
"#;

pub const I18N_GET_BY_CATEGORY: &str = r#"
SELECT RESOURCE_KEY, RESOURCE_VALUE 
FROM I18N_RESOURCES 
WHERE LANG_CODE = ? AND CATEGORY = ?
"#;

pub const I18N_GET_ALL: &str = r#"
SELECT RESOURCE_KEY, RESOURCE_VALUE 
FROM I18N_RESOURCES 
WHERE LANG_CODE = ?
"#;

pub const I18N_GET_AVAILABLE_LANGUAGES: &str = r#"
SELECT DISTINCT LANG_CODE 
FROM I18N_RESOURCES 
ORDER BY LANG_CODE
"#;

// ============================================================================
// Category Service Queries
// ============================================================================

pub const CATEGORY_COUNT_BY_USER: &str = "SELECT COUNT(*) FROM CATEGORY1 WHERE USER_ID = ?";

pub const CATEGORY1_LIST: &str = r#"
SELECT 
    c.USER_ID,
    c.CATEGORY1_CODE,
    c.DISPLAY_ORDER,
    COALESCE(i18n.CATEGORY1_NAME_I18N, c.CATEGORY1_NAME) as CATEGORY1_NAME,
    c.IS_DISABLED
FROM CATEGORY1 c
LEFT JOIN CATEGORY1_I18N i18n 
    ON c.USER_ID = i18n.USER_ID 
    AND c.CATEGORY1_CODE = i18n.CATEGORY1_CODE 
    AND i18n.LANG_CODE = ?
WHERE c.USER_ID = ? AND c.IS_DISABLED = 0
ORDER BY c.DISPLAY_ORDER
"#;

pub const CATEGORY1_TREE: &str = r#"
SELECT 
    c.CATEGORY1_CODE,
    c.DISPLAY_ORDER,
    COALESCE(i18n.CATEGORY1_NAME_I18N, c.CATEGORY1_NAME) as name
FROM CATEGORY1 c
LEFT JOIN CATEGORY1_I18N i18n 
    ON c.USER_ID = i18n.USER_ID 
    AND c.CATEGORY1_CODE = i18n.CATEGORY1_CODE 
    AND i18n.LANG_CODE = ?
WHERE c.USER_ID = ? AND c.IS_DISABLED = 0
ORDER BY c.DISPLAY_ORDER
"#;

pub const CATEGORY2_TREE: &str = r#"
SELECT 
    c.CATEGORY2_CODE,
    c.DISPLAY_ORDER,
    COALESCE(i18n.CATEGORY2_NAME_I18N, c.CATEGORY2_NAME) as name
FROM CATEGORY2 c
LEFT JOIN CATEGORY2_I18N i18n 
    ON c.USER_ID = i18n.USER_ID 
    AND c.CATEGORY1_CODE = i18n.CATEGORY1_CODE 
    AND c.CATEGORY2_CODE = i18n.CATEGORY2_CODE 
    AND i18n.LANG_CODE = ?
WHERE c.USER_ID = ? AND c.CATEGORY1_CODE = ? AND c.IS_DISABLED = 0
ORDER BY c.DISPLAY_ORDER
"#;

pub const CATEGORY3_TREE: &str = r#"
SELECT 
    c.CATEGORY3_CODE,
    c.DISPLAY_ORDER,
    COALESCE(i18n.CATEGORY3_NAME_I18N, c.CATEGORY3_NAME) as name
FROM CATEGORY3 c
LEFT JOIN CATEGORY3_I18N i18n 
    ON c.USER_ID = i18n.USER_ID 
    AND c.CATEGORY1_CODE = i18n.CATEGORY1_CODE 
    AND c.CATEGORY2_CODE = i18n.CATEGORY2_CODE 
    AND c.CATEGORY3_CODE = i18n.CATEGORY3_CODE 
    AND i18n.LANG_CODE = ?
WHERE c.USER_ID = ? 
    AND c.CATEGORY1_CODE = ? 
    AND c.CATEGORY2_CODE = ? 
    AND c.IS_DISABLED = 0
ORDER BY c.DISPLAY_ORDER
"#;

pub const CATEGORY2_GET_MAX_ORDER: &str = r#"
SELECT COALESCE(MAX(DISPLAY_ORDER), 0) as max_order
FROM CATEGORY2
WHERE USER_ID = ? AND CATEGORY1_CODE = ?
"#;

pub const CATEGORY2_INSERT: &str = r#"
INSERT INTO CATEGORY2 (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, DISPLAY_ORDER, CATEGORY2_NAME, IS_DISABLED, ENTRY_DT)
VALUES (?, ?, ?, ?, ?, 0, datetime('now'))
"#;

pub const CATEGORY2_I18N_INSERT: &str = r#"
INSERT INTO CATEGORY2_I18N (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, LANG_CODE, CATEGORY2_NAME_I18N, ENTRY_DT)
VALUES (?, ?, ?, ?, ?, datetime('now'))
"#;

pub const CATEGORY3_GET_MAX_ORDER: &str = r#"
SELECT COALESCE(MAX(DISPLAY_ORDER), 0) as max_order
FROM CATEGORY3
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ?
"#;

pub const CATEGORY3_INSERT: &str = r#"
INSERT INTO CATEGORY3 (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, DISPLAY_ORDER, CATEGORY3_NAME, IS_DISABLED, ENTRY_DT)
VALUES (?, ?, ?, ?, ?, ?, 0, datetime('now'))
"#;

pub const CATEGORY3_I18N_INSERT: &str = r#"
INSERT INTO CATEGORY3_I18N (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, LANG_CODE, CATEGORY3_NAME_I18N, ENTRY_DT)
VALUES (?, ?, ?, ?, ?, ?, datetime('now'))
"#;

pub const CATEGORY2_CHECK_DUPLICATE_NAME: &str = r#"
SELECT COUNT(*) as count FROM CATEGORY2_I18N 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_NAME_I18N = ? AND LANG_CODE = ?
"#;

pub const CATEGORY2_CHECK_DUPLICATE_NAME_EXCLUDING: &str = r#"
SELECT COUNT(*) as count FROM CATEGORY2_I18N 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE != ? 
AND LANG_CODE = ? AND CATEGORY2_NAME_I18N = ?
"#;

pub const CATEGORY3_CHECK_DUPLICATE_NAME: &str = r#"
SELECT COUNT(*) as count FROM CATEGORY3_I18N 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ? AND CATEGORY3_NAME_I18N = ? AND LANG_CODE = ?
"#;

pub const CATEGORY3_CHECK_DUPLICATE_NAME_EXCLUDING: &str = r#"
SELECT COUNT(*) as count FROM CATEGORY3_I18N 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ? 
AND CATEGORY3_CODE != ? AND LANG_CODE = ? AND CATEGORY3_NAME_I18N = ?
"#;

pub const CATEGORY2_CHECK_DUPLICATE_CODE: &str = r#"
SELECT COUNT(*) as count FROM CATEGORY2 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ?
"#;

pub const CATEGORY3_CHECK_DUPLICATE_CODE: &str = r#"
SELECT COUNT(*) as count FROM CATEGORY3 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ? AND CATEGORY3_CODE = ?
"#;

pub const CATEGORY2_UPDATE: &str = r#"
UPDATE CATEGORY2 
SET CATEGORY2_NAME = ?, UPDATE_DT = datetime('now') 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ?
"#;

pub const CATEGORY2_I18N_UPDATE: &str = r#"
UPDATE CATEGORY2_I18N 
SET CATEGORY2_NAME_I18N = ?, UPDATE_DT = datetime('now') 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ? AND LANG_CODE = ?
"#;

pub const CATEGORY3_UPDATE: &str = r#"
UPDATE CATEGORY3 
SET CATEGORY3_NAME = ?, UPDATE_DT = datetime('now') 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ? AND CATEGORY3_CODE = ?
"#;

pub const CATEGORY3_I18N_UPDATE: &str = r#"
UPDATE CATEGORY3_I18N 
SET CATEGORY3_NAME_I18N = ?, UPDATE_DT = datetime('now') 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ? AND CATEGORY3_CODE = ? AND LANG_CODE = ?
"#;

pub const CATEGORY2_GET_FOR_EDIT: &str = r#"
SELECT 
    c.CATEGORY2_CODE,
    c.CATEGORY2_NAME,
    COALESCE(i18n_ja.CATEGORY2_NAME_I18N, c.CATEGORY2_NAME) as name_ja,
    COALESCE(i18n_en.CATEGORY2_NAME_I18N, c.CATEGORY2_NAME) as name_en
FROM CATEGORY2 c
LEFT JOIN CATEGORY2_I18N i18n_ja 
    ON c.USER_ID = i18n_ja.USER_ID 
    AND c.CATEGORY1_CODE = i18n_ja.CATEGORY1_CODE 
    AND c.CATEGORY2_CODE = i18n_ja.CATEGORY2_CODE 
    AND i18n_ja.LANG_CODE = 'ja'
LEFT JOIN CATEGORY2_I18N i18n_en 
    ON c.USER_ID = i18n_en.USER_ID 
    AND c.CATEGORY1_CODE = i18n_en.CATEGORY1_CODE 
    AND c.CATEGORY2_CODE = i18n_en.CATEGORY2_CODE 
    AND i18n_en.LANG_CODE = 'en'
WHERE c.USER_ID = ? AND c.CATEGORY1_CODE = ? AND c.CATEGORY2_CODE = ?
"#;

pub const CATEGORY3_GET_FOR_EDIT: &str = r#"
SELECT 
    c.CATEGORY3_CODE,
    c.CATEGORY3_NAME,
    COALESCE(i18n_ja.CATEGORY3_NAME_I18N, c.CATEGORY3_NAME) as name_ja,
    COALESCE(i18n_en.CATEGORY3_NAME_I18N, c.CATEGORY3_NAME) as name_en
FROM CATEGORY3 c
LEFT JOIN CATEGORY3_I18N i18n_ja 
    ON c.USER_ID = i18n_ja.USER_ID 
    AND c.CATEGORY1_CODE = i18n_ja.CATEGORY1_CODE 
    AND c.CATEGORY2_CODE = i18n_ja.CATEGORY2_CODE 
    AND c.CATEGORY3_CODE = i18n_ja.CATEGORY3_CODE 
    AND i18n_ja.LANG_CODE = 'ja'
LEFT JOIN CATEGORY3_I18N i18n_en 
    ON c.USER_ID = i18n_en.USER_ID 
    AND c.CATEGORY1_CODE = i18n_en.CATEGORY1_CODE 
    AND c.CATEGORY2_CODE = i18n_en.CATEGORY2_CODE 
    AND c.CATEGORY3_CODE = i18n_en.CATEGORY3_CODE 
    AND i18n_en.LANG_CODE = 'en'
WHERE c.USER_ID = ? AND c.CATEGORY1_CODE = ? AND c.CATEGORY2_CODE = ? AND c.CATEGORY3_CODE = ?
"#;

// ============================================================================
// Category Order Management Queries
// ============================================================================

pub const CATEGORY2_GET_ORDER: &str = r#"
SELECT DISPLAY_ORDER FROM CATEGORY2 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ?
"#;

pub const CATEGORY2_UPDATE_ORDER: &str = r#"
UPDATE CATEGORY2 SET DISPLAY_ORDER = ?, UPDATE_DT = datetime('now')
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ?
"#;

pub const CATEGORY2_GET_SIBLING_BY_ORDER: &str = r#"
SELECT CATEGORY2_CODE FROM CATEGORY2
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND DISPLAY_ORDER = ?
"#;

pub const CATEGORY3_GET_ORDER: &str = r#"
SELECT DISPLAY_ORDER FROM CATEGORY3 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ? AND CATEGORY3_CODE = ?
"#;

pub const CATEGORY3_UPDATE_ORDER: &str = r#"
UPDATE CATEGORY3 SET DISPLAY_ORDER = ?, UPDATE_DT = datetime('now')
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ? AND CATEGORY3_CODE = ?
"#;

pub const CATEGORY3_GET_SIBLING_BY_ORDER: &str = r#"
SELECT CATEGORY3_CODE FROM CATEGORY3
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ? AND DISPLAY_ORDER = ?
"#;

// ============================================================================
// Database Service Queries
// ============================================================================

pub const DB_PRAGMA_WAL: &str = "PRAGMA journal_mode = WAL;";

pub const DB_TEST_CONNECTION: &str = "SELECT 1 as test";

// ============================================================================
// Test-Only Queries (用于测试代码)
// ============================================================================

pub const TEST_AUTH_GET_USER_NAME_BY_ID: &str = "SELECT NAME FROM USERS WHERE USER_ID = 1";

pub const TEST_AUTH_GET_PASSWORD_BY_ID: &str = "SELECT PAW FROM USERS WHERE USER_ID = 1";

pub const TEST_AUTH_GET_ROLE_BY_ID: &str = "SELECT ROLE FROM USERS WHERE USER_ID = 1";

// Test queries for category service
pub const TEST_CATEGORY_GET_CATEGORY2_NAME: &str = "SELECT CATEGORY2_NAME FROM CATEGORY2 WHERE USER_ID = ? AND CATEGORY2_CODE = ?";

pub const TEST_CATEGORY_GET_CATEGORY2_I18N_NAME: &str = "SELECT CATEGORY2_NAME_I18N FROM CATEGORY2_I18N WHERE USER_ID = ? AND CATEGORY2_CODE = ? AND LANG_CODE = ?";

pub const TEST_CATEGORY_GET_FIRST_CATEGORY2_CODE: &str = "SELECT CATEGORY2_CODE FROM CATEGORY2 WHERE USER_ID = ? LIMIT 1";

pub const TEST_CATEGORY_GET_CATEGORY3_NAME: &str = "SELECT CATEGORY3_NAME FROM CATEGORY3 WHERE USER_ID = ? AND CATEGORY3_CODE = ?";

pub const TEST_CATEGORY2_GET_DISPLAY_ORDER: &str = "SELECT DISPLAY_ORDER FROM CATEGORY2 WHERE USER_ID = ? AND CATEGORY2_CODE = ?";

pub const TEST_CATEGORY3_GET_DISPLAY_ORDER: &str = "SELECT DISPLAY_ORDER FROM CATEGORY3 WHERE USER_ID = ? AND CATEGORY3_CODE = ?";

pub const TEST_CATEGORY2_COUNT: &str = "SELECT COUNT(*) FROM CATEGORY2 WHERE USER_ID = ?";

pub const TEST_CATEGORY3_COUNT: &str = "SELECT COUNT(*) FROM CATEGORY3 WHERE USER_ID = ?";

// ============================================================================
// Category Initialization Queries
// ============================================================================

pub const CATEGORY_INSERT_CATEGORY1: &str = r#"
INSERT INTO CATEGORY1 (USER_ID, CATEGORY1_CODE, DISPLAY_ORDER, CATEGORY1_NAME, IS_DISABLED, ENTRY_DT) 
VALUES (?, ?, ?, ?, 0, ?)
"#;

pub const CATEGORY_INSERT_CATEGORY1_I18N: &str = r#"
INSERT INTO CATEGORY1_I18N (USER_ID, CATEGORY1_CODE, LANG_CODE, CATEGORY1_NAME_I18N, ENTRY_DT) 
VALUES (?, ?, ?, ?, ?)
"#;

// ============================================================================
// Test-only Data Setup Queries
// ============================================================================

// Test data setup queries
pub const TEST_CATEGORY_INSERT_CATEGORY1: &str = "INSERT INTO CATEGORY1 (USER_ID, CATEGORY1_CODE, DISPLAY_ORDER, CATEGORY1_NAME, IS_DISABLED, ENTRY_DT) VALUES (?, ?, ?, ?, ?, datetime('now'))";

pub const TEST_CATEGORY_INSERT_CATEGORY1_I18N: &str = "INSERT INTO CATEGORY1_I18N (USER_ID, CATEGORY1_CODE, LANG_CODE, CATEGORY1_NAME_I18N, ENTRY_DT) VALUES (?, ?, ?, ?, datetime('now'))";

pub const TEST_USER_GET_PASSWORD_BY_ID: &str = "SELECT PAW FROM USERS WHERE USER_ID = ?";

// ============================================================================
// Transaction Management Service Queries
// ============================================================================

// Transaction CRUD operations
pub const TRANSACTION_INSERT: &str = r#"
INSERT INTO TRANSACTIONS 
(USER_ID, TRANSACTION_DATE, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, AMOUNT, DESCRIPTION, MEMO, ENTRY_DT)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))
"#;

pub const TRANSACTION_SELECT_BY_ID: &str = r#"
SELECT TRANSACTION_ID, USER_ID, TRANSACTION_DATE, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, 
       AMOUNT, DESCRIPTION, MEMO, ENTRY_DT, UPDATE_DT
FROM TRANSACTIONS 
WHERE USER_ID = ? AND TRANSACTION_ID = ?
"#;

pub const TRANSACTION_UPDATE: &str = r#"
UPDATE TRANSACTIONS 
SET TRANSACTION_DATE = ?, CATEGORY1_CODE = ?, CATEGORY2_CODE = ?, CATEGORY3_CODE = ?, 
    AMOUNT = ?, DESCRIPTION = ?, MEMO = ?, UPDATE_DT = datetime('now')
WHERE USER_ID = ? AND TRANSACTION_ID = ?
"#;

pub const TRANSACTION_DELETE: &str = r#"
DELETE FROM TRANSACTIONS 
WHERE USER_ID = ? AND TRANSACTION_ID = ?
"#;

// Transaction list queries (base queries - WHERE clause is built dynamically)
pub const TRANSACTION_COUNT_BASE: &str = "SELECT COUNT(*) FROM TRANSACTIONS WHERE ";

pub const TRANSACTION_LIST_BASE: &str = r#"
SELECT TRANSACTION_ID, USER_ID, TRANSACTION_DATE, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, 
       AMOUNT, DESCRIPTION, MEMO, ENTRY_DT, UPDATE_DT
FROM TRANSACTIONS 
WHERE 
"#;

pub const TRANSACTION_LIST_ORDER: &str = " ORDER BY TRANSACTION_DATE DESC, TRANSACTION_ID DESC";

