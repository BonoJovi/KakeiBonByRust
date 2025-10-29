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

// ============================================================================
// Database Service Queries
// ============================================================================

pub const DB_PRAGMA_WAL: &str = "PRAGMA journal_mode = WAL;";

pub const DB_TEST_CONNECTION: &str = "SELECT 1 as test";
