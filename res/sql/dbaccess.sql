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
(3, 'menu.login', 'en', 'Login', 'menu', 'Login menu item', datetime('now')),
(4, 'menu.login', 'ja', 'ログイン', 'menu', 'ログインメニュー項目', datetime('now')),
(5, 'menu.logout', 'en', 'Logout', 'menu', 'Logout menu item', datetime('now')),
(6, 'menu.logout', 'ja', 'ログアウト', 'menu', 'ログアウトメニュー項目', datetime('now')),
(7, 'menu.quit', 'en', 'Quit', 'menu', 'Quit menu item', datetime('now')),
(8, 'menu.quit', 'ja', '終了', 'menu', '終了メニュー項目', datetime('now')),

-- Settings menu
(9, 'menu.settings', 'en', 'Settings', 'menu', 'Settings menu', datetime('now')),
(10, 'menu.settings', 'ja', '設定', 'menu', '設定メニュー', datetime('now')),
(11, 'menu.language', 'en', 'Language', 'menu', 'Language submenu', datetime('now')),
(12, 'menu.language', 'ja', '言語', 'menu', '言語サブメニュー', datetime('now')),

-- Language options
(13, 'lang.english', 'en', 'English', 'language', 'English language option', datetime('now')),
(14, 'lang.english', 'ja', 'English', 'language', '英語オプション', datetime('now')),
(15, 'lang.japanese', 'en', '日本語 (Japanese)', 'language', 'Japanese language option', datetime('now')),
(16, 'lang.japanese', 'ja', '日本語', 'language', '日本語オプション', datetime('now')),

-- Language names
(17, 'lang.name.en', 'en', 'English', 'language', 'English language name', datetime('now')),
(18, 'lang.name.en', 'ja', '英語', 'language', '英語の名称', datetime('now')),
(19, 'lang.name.ja', 'en', 'Japanese', 'language', 'Japanese language name', datetime('now')),
(20, 'lang.name.ja', 'ja', '日本語', 'language', '日本語の名称', datetime('now')),

-- Messages
(21, 'msg.lang_changed', 'en', 'Language changed to {0}.', 'message', 'Language change confirmation', datetime('now')),
(22, 'msg.lang_changed', 'ja', '言語を{0}に変更しました。', 'message', '言語変更確認メッセージ', datetime('now')),
(23, 'msg.error', 'en', 'Error', 'message', 'Error message title', datetime('now')),
(24, 'msg.error', 'ja', 'エラー', 'message', 'エラーメッセージタイトル', datetime('now')),
(25, 'msg.success', 'en', 'Success', 'message', 'Success message title', datetime('now')),
(26, 'msg.success', 'ja', '成功', 'message', '成功メッセージタイトル', datetime('now')),
(27, 'msg.info', 'en', 'Information', 'message', 'Info message title', datetime('now')),
(28, 'msg.info', 'ja', '情報', 'message', '情報メッセージタイトル', datetime('now')),

-- Admin setup
(101, 'admin.setup_title', 'en', 'Initial Setup - Register Administrator', 'admin', 'Admin setup page title', datetime('now')),
(102, 'admin.setup_title', 'ja', '初期設定 - 管理者登録', 'admin', '管理者設定ページタイトル', datetime('now')),
(103, 'admin.username', 'en', 'Username:', 'admin', 'Username label', datetime('now')),
(104, 'admin.username', 'ja', 'ユーザー名:', 'admin', 'ユーザー名ラベル', datetime('now')),
(105, 'admin.password', 'en', 'Password (minimum 16 characters):', 'admin', 'Password label', datetime('now')),
(106, 'admin.password', 'ja', 'パスワード (最低16文字):', 'admin', 'パスワードラベル', datetime('now')),
(107, 'admin.password_confirm', 'en', 'Password (Confirm):', 'admin', 'Password confirmation label', datetime('now')),
(108, 'admin.password_confirm', 'ja', 'パスワード (確認):', 'admin', 'パスワード確認ラベル', datetime('now')),
(109, 'admin.register_button', 'en', 'Register Administrator', 'admin', 'Register button text', datetime('now')),
(110, 'admin.register_button', 'ja', '管理者を登録', 'admin', '登録ボタンテキスト', datetime('now')),
(111, 'admin.registration_success', 'en', 'Administrator registered successfully! Please login.', 'admin', 'Registration success message', datetime('now')),
(112, 'admin.registration_success', 'ja', '管理者の登録に成功しました！ログインしてください。', 'admin', '登録成功メッセージ', datetime('now')),

-- User setup
(201, 'user.setup_title', 'en', 'Register General User Account', 'user', 'User setup page title', datetime('now')),
(202, 'user.setup_title', 'ja', '一般ユーザーアカウント登録', 'user', 'ユーザー設定ページタイトル', datetime('now')),
(203, 'user.setup_description', 'en', 'Create a standard user account for daily use.', 'user', 'User setup description', datetime('now')),
(204, 'user.setup_description', 'ja', '普段使うための標準ユーザーアカウントを作成します。', 'user', 'ユーザー設定説明', datetime('now')),
(205, 'user.username', 'en', 'Username:', 'user', 'Username label', datetime('now')),
(206, 'user.username', 'ja', 'ユーザー名:', 'user', 'ユーザー名ラベル', datetime('now')),
(207, 'user.password', 'en', 'Password (minimum 16 characters):', 'user', 'Password label', datetime('now')),
(208, 'user.password', 'ja', 'パスワード (最低16文字):', 'user', 'パスワードラベル', datetime('now')),
(209, 'user.password_confirm', 'en', 'Password (Confirm):', 'user', 'Password confirmation label', datetime('now')),
(210, 'user.password_confirm', 'ja', 'パスワード (確認):', 'user', 'パスワード確認ラベル', datetime('now')),
(211, 'user.register_button', 'en', 'Register User', 'user', 'Register button text', datetime('now')),
(212, 'user.register_button', 'ja', 'ユーザーを登録', 'user', '登録ボタンテキスト', datetime('now')),
(213, 'user.registration_success', 'en', 'User registered successfully!', 'user', 'Registration success message', datetime('now')),
(214, 'user.registration_success', 'ja', 'ユーザーの登録に成功しました！', 'user', '登録成功メッセージ', datetime('now')),

-- Login
(301, 'login.title', 'en', 'Login', 'login', 'Login page title', datetime('now')),
(302, 'login.title', 'ja', 'ログイン', 'login', 'ログインページタイトル', datetime('now')),
(303, 'login.username', 'en', 'Username:', 'login', 'Username label', datetime('now')),
(304, 'login.username', 'ja', 'ユーザー名:', 'login', 'ユーザー名ラベル', datetime('now')),
(305, 'login.password', 'en', 'Password:', 'login', 'Password label', datetime('now')),
(306, 'login.password', 'ja', 'パスワード:', 'login', 'パスワードラベル', datetime('now')),
(307, 'login.button', 'en', 'Login', 'login', 'Login button text', datetime('now')),
(308, 'login.button', 'ja', 'ログイン', 'login', 'ログインボタンテキスト', datetime('now')),
(309, 'login.success', 'en', 'Login successful!', 'login', 'Login success message', datetime('now')),
(310, 'login.success', 'ja', 'ログインに成功しました！', 'login', 'ログイン成功メッセージ', datetime('now')),

-- App
(401, 'app.welcome', 'en', 'Welcome to KakeiBon', 'app', 'Welcome message', datetime('now')),
(402, 'app.welcome', 'ja', 'KakeiBonへようこそ', 'app', 'ウェルカムメッセージ', datetime('now')),
(403, 'app.logged_in', 'en', 'You are logged in successfully!', 'app', 'Logged in message', datetime('now')),
(404, 'app.logged_in', 'ja', 'ログインに成功しました！', 'app', 'ログイン完了メッセージ', datetime('now')),

-- Error messages
(501, 'error.password_empty', 'en', 'Password cannot be empty!', 'error', 'Empty password error', datetime('now')),
(502, 'error.password_empty', 'ja', 'パスワードを入力してください！', 'error', '空のパスワードエラー', datetime('now')),
(503, 'error.password_too_short', 'en', 'Password must be at least 16 characters long!', 'error', 'Password too short error', datetime('now')),
(504, 'error.password_too_short', 'ja', 'パスワードは最低16文字必要です！', 'error', 'パスワード短すぎエラー', datetime('now')),
(505, 'error.password_mismatch', 'en', 'Passwords do not match!', 'error', 'Password mismatch error', datetime('now')),
(506, 'error.password_mismatch', 'ja', 'パスワードが一致しません！', 'error', 'パスワード不一致エラー', datetime('now')),
(507, 'error.registration_failed', 'en', 'Registration failed', 'error', 'Registration failure message', datetime('now')),
(508, 'error.registration_failed', 'ja', '登録に失敗しました', 'error', '登録失敗メッセージ', datetime('now')),
(509, 'error.login_failed', 'en', 'Login failed', 'error', 'Login failure message', datetime('now')),
(510, 'error.login_failed', 'ja', 'ログインに失敗しました', 'error', 'ログイン失敗メッセージ', datetime('now'));

-- NOTE: Category data will be migrated from existing SQL later
