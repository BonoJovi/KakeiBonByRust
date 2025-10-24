-- SQL_10000001: Create USERS table
CREATE TABLE IF NOT EXISTS USERS (
    USER_ID INTEGER NOT NULL,
    NAME VARCHAR(128) NOT NULL UNIQUE,
    PAW VARCHAR(128) NOT NULL,
    ROLE INTEGER NOT NULL,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID)
);

-- SQL_10000002: Create ENCRYPTED_FIELDS table
-- This table manages which fields are encrypted in the database
CREATE TABLE IF NOT EXISTS ENCRYPTED_FIELDS (
    FIELD_ID INTEGER NOT NULL,
    TABLE_NAME VARCHAR(128) NOT NULL,
    COLUMN_NAME VARCHAR(128) NOT NULL,
    DESCRIPTION VARCHAR(256),
    IS_ACTIVE INTEGER NOT NULL DEFAULT 1,
    ENTRY_DT DATETIME NOT NULL,
    PRIMARY KEY(FIELD_ID),
    UNIQUE(TABLE_NAME, COLUMN_NAME)
);

-- SQL_10000003: Create I18N_RESOURCES table
-- This table manages system multilingual text resources (menus, messages, labels)
CREATE TABLE IF NOT EXISTS I18N_RESOURCES (
    RESOURCE_ID INTEGER NOT NULL,
    RESOURCE_KEY VARCHAR(256) NOT NULL,
    LANG_CODE VARCHAR(10) NOT NULL,
    RESOURCE_VALUE TEXT NOT NULL,
    CATEGORY VARCHAR(64),
    DESCRIPTION VARCHAR(512),
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(RESOURCE_ID),
    UNIQUE(RESOURCE_KEY, LANG_CODE)
);

-- SQL_10000004: Create CATEGORY1 table (Major category)
CREATE TABLE IF NOT EXISTS CATEGORY1 (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    DISPLAY_ORDER INTEGER NOT NULL,
    CATEGORY1_NAME VARCHAR(128) NOT NULL,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE)
);

-- SQL_10000005: Create index for CATEGORY1
CREATE INDEX IF NOT EXISTS idx_category1_order ON CATEGORY1(USER_ID, DISPLAY_ORDER);
CREATE INDEX IF NOT EXISTS idx_category1_disabled ON CATEGORY1(USER_ID, IS_DISABLED);

-- SQL_10000006: Create CATEGORY2 table (Middle category)
CREATE TABLE IF NOT EXISTS CATEGORY2 (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    CATEGORY2_CODE VARCHAR(64) NOT NULL,
    DISPLAY_ORDER INTEGER NOT NULL,
    CATEGORY2_NAME VARCHAR(128) NOT NULL,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE),
    FOREIGN KEY(USER_ID, CATEGORY1_CODE) REFERENCES CATEGORY1(USER_ID, CATEGORY1_CODE) ON DELETE CASCADE
);

-- SQL_10000007: Create index for CATEGORY2
CREATE INDEX IF NOT EXISTS idx_category2_parent ON CATEGORY2(USER_ID, CATEGORY1_CODE);
CREATE INDEX IF NOT EXISTS idx_category2_order ON CATEGORY2(USER_ID, CATEGORY1_CODE, DISPLAY_ORDER);
CREATE INDEX IF NOT EXISTS idx_category2_disabled ON CATEGORY2(USER_ID, IS_DISABLED);

-- SQL_10000008: Create CATEGORY3 table (Minor category)
CREATE TABLE IF NOT EXISTS CATEGORY3 (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    CATEGORY2_CODE VARCHAR(64) NOT NULL,
    CATEGORY3_CODE VARCHAR(64) NOT NULL,
    DISPLAY_ORDER INTEGER NOT NULL,
    CATEGORY3_NAME VARCHAR(128) NOT NULL,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE),
    FOREIGN KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE) REFERENCES CATEGORY2(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE) ON DELETE CASCADE
);

-- SQL_10000009: Create index for CATEGORY3
CREATE INDEX IF NOT EXISTS idx_category3_parent ON CATEGORY3(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE);
CREATE INDEX IF NOT EXISTS idx_category3_order ON CATEGORY3(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, DISPLAY_ORDER);
CREATE INDEX IF NOT EXISTS idx_category3_disabled ON CATEGORY3(USER_ID, IS_DISABLED);

-- SQL_10000010: Create CATEGORY1_I18N table for multilingual support
CREATE TABLE IF NOT EXISTS CATEGORY1_I18N (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    LANG_CODE VARCHAR(10) NOT NULL,
    CATEGORY1_NAME_I18N VARCHAR(256) NOT NULL,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE, LANG_CODE),
    FOREIGN KEY(USER_ID, CATEGORY1_CODE) REFERENCES CATEGORY1(USER_ID, CATEGORY1_CODE) ON DELETE CASCADE
);

-- SQL_10000011: Create CATEGORY2_I18N table for multilingual support
CREATE TABLE IF NOT EXISTS CATEGORY2_I18N (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    CATEGORY2_CODE VARCHAR(64) NOT NULL,
    LANG_CODE VARCHAR(10) NOT NULL,
    CATEGORY2_NAME_I18N VARCHAR(256) NOT NULL,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, LANG_CODE),
    FOREIGN KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE) REFERENCES CATEGORY2(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE) ON DELETE CASCADE
);

-- SQL_10000012: Create CATEGORY3_I18N table for multilingual support
CREATE TABLE IF NOT EXISTS CATEGORY3_I18N (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    CATEGORY2_CODE VARCHAR(64) NOT NULL,
    CATEGORY3_CODE VARCHAR(64) NOT NULL,
    LANG_CODE VARCHAR(10) NOT NULL,
    CATEGORY3_NAME_I18N VARCHAR(256) NOT NULL,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, LANG_CODE),
    FOREIGN KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE) REFERENCES CATEGORY3(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE) ON DELETE CASCADE
);

-- SQL_10000013: Create indexes for I18N tables and I18N_RESOURCES
CREATE INDEX IF NOT EXISTS idx_i18n_lang_key ON I18N_RESOURCES(LANG_CODE, RESOURCE_KEY);
CREATE INDEX IF NOT EXISTS idx_i18n_category ON I18N_RESOURCES(CATEGORY);
CREATE INDEX IF NOT EXISTS idx_category1_i18n_lang ON CATEGORY1_I18N(LANG_CODE);
CREATE INDEX IF NOT EXISTS idx_category2_i18n_lang ON CATEGORY2_I18N(LANG_CODE);
CREATE INDEX IF NOT EXISTS idx_category3_i18n_lang ON CATEGORY3_I18N(LANG_CODE);

-- SQL_10000014: Insert default system language resources
-- Menu items
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES 
-- File menu
(1, 'menu.file', 'en', 'File', 'menu', 'File menu', datetime('now')),
(2, 'menu.file', 'ja', 'ファイル', 'menu', 'ファイルメニュー', datetime('now')),
(3, 'menu.quit', 'en', 'Quit', 'menu', 'Quit menu item', datetime('now')),
(4, 'menu.quit', 'ja', '終了', 'menu', '終了メニュー項目', datetime('now')),

-- Settings menu
(5, 'menu.settings', 'en', 'Settings', 'menu', 'Settings menu', datetime('now')),
(6, 'menu.settings', 'ja', '設定', 'menu', '設定メニュー', datetime('now')),
(7, 'menu.language', 'en', 'Language', 'menu', 'Language submenu', datetime('now')),
(8, 'menu.language', 'ja', '言語', 'menu', '言語サブメニュー', datetime('now')),

-- Language options
(9, 'lang.english', 'en', 'English', 'language', 'English language option', datetime('now')),
(10, 'lang.english', 'ja', 'English', 'language', '英語オプション', datetime('now')),
(11, 'lang.japanese', 'en', '日本語 (Japanese)', 'language', 'Japanese language option', datetime('now')),
(12, 'lang.japanese', 'ja', '日本語', 'language', '日本語オプション', datetime('now')),

-- Language names
(13, 'lang.name.en', 'en', 'English', 'language', 'English language name', datetime('now')),
(14, 'lang.name.en', 'ja', '英語', 'language', '英語の名称', datetime('now')),
(15, 'lang.name.ja', 'en', 'Japanese', 'language', 'Japanese language name', datetime('now')),
(16, 'lang.name.ja', 'ja', '日本語', 'language', '日本語の名称', datetime('now')),

-- Messages
(17, 'msg.lang_changed', 'en', 'Language changed to {0}.', 'message', 'Language change confirmation', datetime('now')),
(18, 'msg.lang_changed', 'ja', '言語を{0}に変更しました。', 'message', '言語変更確認メッセージ', datetime('now')),
(19, 'msg.error', 'en', 'Error', 'message', 'Error message title', datetime('now')),
(20, 'msg.error', 'ja', 'エラー', 'message', 'エラーメッセージタイトル', datetime('now')),
(21, 'msg.success', 'en', 'Success', 'message', 'Success message title', datetime('now')),
(22, 'msg.success', 'ja', '成功', 'message', '成功メッセージタイトル', datetime('now')),
(23, 'msg.info', 'en', 'Information', 'message', 'Info message title', datetime('now')),
(24, 'msg.info', 'ja', '情報', 'message', '情報メッセージタイトル', datetime('now'));

-- NOTE: Category data will be migrated from existing SQL later
