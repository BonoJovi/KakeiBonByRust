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

-- SQL_10000013: OBSOLETE - Old TRANSACTIONS table removed
-- This table has been replaced by TRANSACTIONS_HEADER and TRANSACTIONS_DETAIL
-- See SQL_30000002 and SQL_30000003 below for the new schema

-- SQL_10000014: OBSOLETE - Old TRANSACTIONS indexes removed

-- SQL_10000015: Create indexes for I18N tables and I18N_RESOURCES
CREATE INDEX IF NOT EXISTS idx_i18n_lang_key ON I18N_RESOURCES(LANG_CODE, RESOURCE_KEY);
CREATE INDEX IF NOT EXISTS idx_i18n_category ON I18N_RESOURCES(CATEGORY);
CREATE INDEX IF NOT EXISTS idx_category1_i18n_lang ON CATEGORY1_I18N(LANG_CODE);
CREATE INDEX IF NOT EXISTS idx_category2_i18n_lang ON CATEGORY2_I18N(LANG_CODE);
CREATE INDEX IF NOT EXISTS idx_category3_i18n_lang ON CATEGORY3_I18N(LANG_CODE);

-- SQL_10000016: Insert default system language resources
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
(510, 'error.login_failed', 'ja', 'ログインに失敗しました', 'error', 'ログイン失敗メッセージ', datetime('now')),

-- Admin menu
(511, 'menu.admin', 'en', 'Admin', 'menu', 'Admin menu', datetime('now')),
(512, 'menu.admin', 'ja', '管理', 'menu', '管理メニュー', datetime('now')),
(513, 'menu.user_management', 'en', 'User Management', 'menu', 'User management menu item', datetime('now')),
(514, 'menu.user_management', 'ja', 'ユーザー管理', 'menu', 'ユーザー管理メニュー項目', datetime('now')),
(515, 'menu.category_management', 'en', 'Category Management', 'menu', 'Category management menu item', datetime('now')),
(516, 'menu.category_management', 'ja', '費目管理', 'menu', '費目管理メニュー項目', datetime('now')),
(723, 'menu.account_management', 'en', 'Account Management', 'menu', 'Account management menu item', datetime('now')),
(724, 'menu.account_management', 'ja', '口座管理', 'menu', '口座管理メニュー項目', datetime('now')),
(725, 'menu.transaction_management', 'en', 'Transaction Management', 'menu', 'Transaction management menu item', datetime('now')),
(726, 'menu.transaction_management', 'ja', '入出金管理', 'menu', '入出金管理メニュー項目', datetime('now')),
(517, 'menu.back_to_main', 'en', 'Back to Main', 'menu', 'Back to main menu item', datetime('now')),
(518, 'menu.back_to_main', 'ja', 'メインに戻る', 'menu', 'メインに戻るメニュー項目', datetime('now')),

-- Font size menu
(519, 'menu.font_size', 'en', 'Font Size', 'menu', 'Font size menu', datetime('now')),
(520, 'menu.font_size', 'ja', 'フォントサイズ', 'menu', 'フォントサイズメニュー', datetime('now')),
(521, 'font_size.small', 'en', 'Small', 'font_size', 'Small font size', datetime('now')),
(522, 'font_size.small', 'ja', '小', 'font_size', '小フォント', datetime('now')),
(523, 'font_size.medium', 'en', 'Medium', 'font_size', 'Medium font size', datetime('now')),
(524, 'font_size.medium', 'ja', '中', 'font_size', '中フォント', datetime('now')),
(525, 'font_size.large', 'en', 'Large', 'font_size', 'Large font size', datetime('now')),
(526, 'font_size.large', 'ja', '大', 'font_size', '大フォント', datetime('now')),
(527, 'font_size.custom', 'en', 'Custom', 'font_size', 'Custom font size', datetime('now')),
(528, 'font_size.custom', 'ja', 'カスタム', 'font_size', 'カスタムフォント', datetime('now')),
(529, 'font_size.modal_title', 'en', 'Font Size Settings', 'font_size', 'Font size modal title', datetime('now')),
(530, 'font_size.modal_title', 'ja', 'フォントサイズ設定', 'font_size', 'フォントサイズモーダルタイトル', datetime('now')),
(531, 'font_size.preset', 'en', 'Preset:', 'font_size', 'Preset label', datetime('now')),
(532, 'font_size.preset', 'ja', 'プリセット:', 'font_size', 'プリセットラベル', datetime('now')),
(533, 'font_size.percentage', 'en', 'Percentage:', 'font_size', 'Percentage label', datetime('now')),
(534, 'font_size.percentage', 'ja', 'パーセンテージ:', 'font_size', 'パーセンテージラベル', datetime('now')),

-- User Management
(601, 'user_mgmt.title', 'en', 'User Management', 'user_mgmt', 'User management page title', datetime('now')),
(602, 'user_mgmt.title', 'ja', 'ユーザー管理', 'user_mgmt', 'ユーザー管理ページタイトル', datetime('now')),
(603, 'user_mgmt.user_list', 'en', 'User List', 'user_mgmt', 'User list section title', datetime('now')),
(604, 'user_mgmt.user_list', 'ja', 'ユーザー一覧', 'user_mgmt', 'ユーザー一覧セクションタイトル', datetime('now')),
(605, 'user_mgmt.add_user', 'en', 'Add User', 'user_mgmt', 'Add user button', datetime('now')),
(606, 'user_mgmt.add_user', 'ja', 'ユーザー追加', 'user_mgmt', 'ユーザー追加ボタン', datetime('now')),
(607, 'user_mgmt.edit_user', 'en', 'Edit User', 'user_mgmt', 'Edit user modal title', datetime('now')),
(608, 'user_mgmt.edit_user', 'ja', 'ユーザー編集', 'user_mgmt', 'ユーザー編集モーダルタイトル', datetime('now')),
(609, 'user_mgmt.delete_user', 'en', 'Delete User', 'user_mgmt', 'Delete user modal title', datetime('now')),
(610, 'user_mgmt.delete_user', 'ja', 'ユーザー削除', 'user_mgmt', 'ユーザー削除モーダルタイトル', datetime('now')),
(611, 'user_mgmt.user_id', 'en', 'User ID', 'user_mgmt', 'User ID column header', datetime('now')),
(612, 'user_mgmt.user_id', 'ja', 'ユーザーID', 'user_mgmt', 'ユーザーIDカラムヘッダー', datetime('now')),
(613, 'user_mgmt.username', 'en', 'Username:', 'user_mgmt', 'Username column header', datetime('now')),
(614, 'user_mgmt.username', 'ja', 'ユーザー名:', 'user_mgmt', 'ユーザー名カラムヘッダー', datetime('now')),
(615, 'user_mgmt.role', 'en', 'Role', 'user_mgmt', 'Role column header', datetime('now')),
(616, 'user_mgmt.role', 'ja', '権限', 'user_mgmt', '権限カラムヘッダー', datetime('now')),
(617, 'user_mgmt.created_at', 'en', 'Created At', 'user_mgmt', 'Created at column header', datetime('now')),
(618, 'user_mgmt.created_at', 'ja', '作成日時', 'user_mgmt', '作成日時カラムヘッダー', datetime('now')),
(619, 'user_mgmt.updated_at', 'en', 'Updated At', 'user_mgmt', 'Updated at column header', datetime('now')),
(620, 'user_mgmt.updated_at', 'ja', '更新日時', 'user_mgmt', '更新日時カラムヘッダー', datetime('now')),
(621, 'user_mgmt.actions', 'en', 'Actions', 'user_mgmt', 'Actions column header', datetime('now')),
(622, 'user_mgmt.actions', 'ja', '操作', 'user_mgmt', '操作カラムヘッダー', datetime('now')),
(623, 'user_mgmt.edit', 'en', 'Edit', 'user_mgmt', 'Edit button', datetime('now')),
(624, 'user_mgmt.edit', 'ja', '編集', 'user_mgmt', '編集ボタン', datetime('now')),
(625, 'user_mgmt.delete', 'en', 'Delete', 'user_mgmt', 'Delete button', datetime('now')),
(626, 'user_mgmt.delete', 'ja', '削除', 'user_mgmt', '削除ボタン', datetime('now')),
(627, 'user_mgmt.save', 'en', 'Save', 'user_mgmt', 'Save button', datetime('now')),
(628, 'user_mgmt.save', 'ja', '保存', 'user_mgmt', '保存ボタン', datetime('now')),
(629, 'user_mgmt.cancel', 'en', 'Cancel', 'user_mgmt', 'Cancel button', datetime('now')),
(630, 'user_mgmt.cancel', 'ja', 'キャンセル', 'user_mgmt', 'キャンセルボタン', datetime('now')),
(631, 'user_mgmt.password', 'en', 'Password (minimum 16 characters):', 'user_mgmt', 'Password label', datetime('now')),
(632, 'user_mgmt.password', 'ja', 'パスワード (最低16文字):', 'user_mgmt', 'パスワードラベル', datetime('now')),
(633, 'user_mgmt.password_confirm', 'en', 'Password (Confirm):', 'user_mgmt', 'Password confirmation label', datetime('now')),
(634, 'user_mgmt.password_confirm', 'ja', 'パスワード (確認):', 'user_mgmt', 'パスワード確認ラベル', datetime('now')),
(635, 'user_mgmt.role_admin', 'en', 'Admin', 'user_mgmt', 'Admin role badge', datetime('now')),
(636, 'user_mgmt.role_admin', 'ja', '管理者', 'user_mgmt', '管理者権限バッジ', datetime('now')),
(637, 'user_mgmt.role_user', 'en', 'User', 'user_mgmt', 'User role badge', datetime('now')),
(638, 'user_mgmt.role_user', 'ja', '一般', 'user_mgmt', '一般ユーザー権限バッジ', datetime('now')),
(639, 'user_mgmt.delete_confirmation', 'en', 'Are you sure you want to delete this user?', 'user_mgmt', 'Delete confirmation message', datetime('now')),
(640, 'user_mgmt.delete_confirmation', 'ja', 'このユーザーを削除してもよろしいですか？', 'user_mgmt', '削除確認メッセージ', datetime('now')),
(641, 'user_mgmt.no_users', 'en', 'No users found', 'user_mgmt', 'No users message', datetime('now')),
(642, 'user_mgmt.no_users', 'ja', 'ユーザーが見つかりません', 'user_mgmt', 'ユーザーなしメッセージ', datetime('now')),
(643, 'user_mgmt.loading', 'en', 'Loading...', 'user_mgmt', 'Loading message', datetime('now')),
(644, 'user_mgmt.loading', 'ja', '読み込み中...', 'user_mgmt', '読み込み中メッセージ', datetime('now')),
(645, 'user_mgmt.creating', 'en', 'Creating user...', 'user_mgmt', 'Creating user message', datetime('now')),
(646, 'user_mgmt.creating', 'ja', 'ユーザーを作成中...', 'user_mgmt', 'ユーザー作成中メッセージ', datetime('now')),
(647, 'user_mgmt.updating', 'en', 'Updating user...', 'user_mgmt', 'Updating user message', datetime('now')),
(648, 'user_mgmt.updating', 'ja', 'ユーザーを更新中...', 'user_mgmt', 'ユーザー更新中メッセージ', datetime('now')),
(649, 'user_mgmt.deleting', 'en', 'Deleting user...', 'user_mgmt', 'Deleting user message', datetime('now')),
(650, 'user_mgmt.deleting', 'ja', 'ユーザーを削除中...', 'user_mgmt', 'ユーザー削除中メッセージ', datetime('now')),
(651, 'user_mgmt.user_created', 'en', 'User created successfully!', 'user_mgmt', 'User created success message', datetime('now')),
(652, 'user_mgmt.user_created', 'ja', 'ユーザーを作成しました！', 'user_mgmt', 'ユーザー作成成功メッセージ', datetime('now')),
(653, 'user_mgmt.user_updated', 'en', 'User updated successfully!', 'user_mgmt', 'User updated success message', datetime('now')),
(654, 'user_mgmt.user_updated', 'ja', 'ユーザーを更新しました！', 'user_mgmt', 'ユーザー更新成功メッセージ', datetime('now')),
(655, 'user_mgmt.user_deleted', 'en', 'User deleted successfully!', 'user_mgmt', 'User deleted success message', datetime('now')),
(656, 'user_mgmt.user_deleted', 'ja', 'ユーザーを削除しました！', 'user_mgmt', 'ユーザー削除成功メッセージ', datetime('now')),
(657, 'error.load_users_failed', 'en', 'Failed to load users', 'error', 'Load users error', datetime('now')),
(658, 'error.load_users_failed', 'ja', 'ユーザーの読み込みに失敗しました', 'error', 'ユーザー読み込みエラー', datetime('now')),
(659, 'error.save_user_failed', 'en', 'Failed to save user', 'error', 'Save user error', datetime('now')),
(660, 'error.save_user_failed', 'ja', 'ユーザーの保存に失敗しました', 'error', 'ユーザー保存エラー', datetime('now')),
(661, 'error.delete_user_failed', 'en', 'Failed to delete user', 'error', 'Delete user error', datetime('now')),
(662, 'error.delete_user_failed', 'ja', 'ユーザーの削除に失敗しました', 'error', 'ユーザー削除エラー', datetime('now')),

-- Category Management
(663, 'category_mgmt.title', 'en', 'Category Management', 'category_mgmt', 'Category management page title', datetime('now')),
(664, 'category_mgmt.title', 'ja', '費目管理', 'category_mgmt', '費目管理ページタイトル', datetime('now')),
(665, 'category_mgmt.category_tree', 'en', 'Category Tree', 'category_mgmt', 'Category tree section title', datetime('now')),
(666, 'category_mgmt.category_tree', 'ja', '費目ツリー', 'category_mgmt', '費目ツリーセクションタイトル', datetime('now')),
(667, 'category_mgmt.add_category1', 'en', 'Add Major Category', 'category_mgmt', 'Add major category button', datetime('now')),
(668, 'category_mgmt.add_category1', 'ja', '大分類を追加', 'category_mgmt', '大分類追加ボタン', datetime('now')),
(669, 'category_mgmt.add_category2', 'en', 'Add Medium Category', 'category_mgmt', 'Add medium category button', datetime('now')),
(670, 'category_mgmt.add_category2', 'ja', '中分類を追加', 'category_mgmt', '中分類追加ボタン', datetime('now')),
(671, 'category_mgmt.add_category3', 'en', 'Add Minor Category', 'category_mgmt', 'Add minor category button', datetime('now')),
(672, 'category_mgmt.add_category3', 'ja', '小分類を追加', 'category_mgmt', '小分類追加ボタン', datetime('now')),
(673, 'category_mgmt.edit_category1', 'en', 'Edit Major Category', 'category_mgmt', 'Edit major category modal title', datetime('now')),
(674, 'category_mgmt.edit_category1', 'ja', '大分類を編集', 'category_mgmt', '大分類編集モーダルタイトル', datetime('now')),
(675, 'category_mgmt.edit_category2', 'en', 'Edit Medium Category', 'category_mgmt', 'Edit medium category modal title', datetime('now')),
(676, 'category_mgmt.edit_category2', 'ja', '中分類を編集', 'category_mgmt', '中分類編集モーダルタイトル', datetime('now')),
(677, 'category_mgmt.edit_category3', 'en', 'Edit Minor Category', 'category_mgmt', 'Edit minor category modal title', datetime('now')),
(678, 'category_mgmt.edit_category3', 'ja', '小分類を編集', 'category_mgmt', '小分類編集モーダルタイトル', datetime('now')),
(679, 'category_mgmt.name_ja', 'en', 'Name (Japanese)', 'category_mgmt', 'Japanese name label', datetime('now')),
(680, 'category_mgmt.name_ja', 'ja', '名前（日本語）', 'category_mgmt', '日本語名ラベル', datetime('now')),
(681, 'category_mgmt.name_en', 'en', 'Name (English)', 'category_mgmt', 'English name label', datetime('now')),
(682, 'category_mgmt.name_en', 'ja', '名前（英語）', 'category_mgmt', '英語名ラベル', datetime('now')),
(683, 'category_mgmt.display_order', 'en', 'Display Order', 'category_mgmt', 'Display order label', datetime('now')),
(684, 'category_mgmt.display_order', 'ja', '表示順', 'category_mgmt', '表示順ラベル', datetime('now')),
(685, 'category_mgmt.parent_category', 'en', 'Parent Category', 'category_mgmt', 'Parent category label', datetime('now')),
(686, 'category_mgmt.parent_category', 'ja', '親カテゴリ', 'category_mgmt', '親カテゴリラベル', datetime('now')),
(687, 'category_mgmt.add_sub', 'en', 'Add Subcategory', 'category_mgmt', 'Add subcategory button', datetime('now')),
(688, 'category_mgmt.add_sub', 'ja', 'サブカテゴリ追加', 'category_mgmt', 'サブカテゴリ追加ボタン', datetime('now')),
(689, 'category_mgmt.order', 'en', 'Order', 'category_mgmt', 'Order label', datetime('now')),
(690, 'category_mgmt.order', 'ja', '順序', 'category_mgmt', '順序ラベル', datetime('now')),
(691, 'category_mgmt.no_categories', 'en', 'No categories found', 'category_mgmt', 'No categories message', datetime('now')),
(692, 'category_mgmt.no_categories', 'ja', 'カテゴリがありません', 'category_mgmt', 'カテゴリなしメッセージ', datetime('now')),

-- Common resources (additional)
(693, 'common.edit', 'en', 'Edit', 'common', 'Edit button', datetime('now')),
(694, 'common.edit', 'ja', '編集', 'common', '編集ボタン', datetime('now')),
(695, 'common.save', 'en', 'Save', 'common', 'Save button', datetime('now')),
(696, 'common.save', 'ja', '保存', 'common', '保存ボタン', datetime('now')),
(697, 'common.cancel', 'en', 'Cancel', 'common', 'Cancel button', datetime('now')),
(698, 'common.cancel', 'ja', 'キャンセル', 'common', 'キャンセルボタン', datetime('now')),
(699, 'common.loading', 'en', 'Loading...', 'common', 'Loading message', datetime('now')),
(700, 'common.loading', 'ja', '読み込み中...', 'common', '読み込み中メッセージ', datetime('now')),

-- Category management error messages
(701, 'error.category_name_required', 'en', 'Please enter at least one name (Japanese or English)', 'error', 'Category name required error', datetime('now')),
(702, 'error.category_name_required', 'ja', '名前を少なくとも1つ入力してください（日本語または英語）', 'error', '費目名必須エラー', datetime('now')),
(703, 'error.category_save_failed', 'en', 'Failed to save', 'error', 'Category save error', datetime('now')),
(704, 'error.category_save_failed', 'ja', '保存に失敗しました', 'error', '費目保存エラー', datetime('now')),
(705, 'error.category_move_failed', 'en', 'Failed to move category', 'error', 'Category move error', datetime('now')),
(706, 'error.category_move_failed', 'ja', '費目の移動に失敗しました', 'error', '費目移動エラー', datetime('now')),
(707, 'error.category_load_failed', 'en', 'Failed to load categories', 'error', 'Category load error', datetime('now')),
(708, 'error.category_load_failed', 'ja', '費目の読み込みに失敗しました', 'error', '費目読み込みエラー', datetime('now')),

-- Language and font size error messages
(709, 'error.language_change_failed', 'en', 'Failed to change language', 'error', 'Language change error', datetime('now')),
(710, 'error.language_change_failed', 'ja', '言語の変更に失敗しました', 'error', '言語変更エラー', datetime('now')),
(711, 'error.font_size_change_failed', 'en', 'Failed to change font size', 'error', 'Font size change error', datetime('now')),
(712, 'error.font_size_change_failed', 'ja', 'フォントサイズの変更に失敗しました', 'error', 'フォントサイズ変更エラー', datetime('now')),
(713, 'error.font_size_apply_failed', 'en', 'Failed to apply font size', 'error', 'Font size apply error', datetime('now')),
(714, 'error.font_size_apply_failed', 'ja', 'フォントサイズの適用に失敗しました', 'error', 'フォントサイズ適用エラー', datetime('now')),

-- HTML5 validation messages
(715, 'validation.required', 'en', 'Please fill out this field', 'validation', 'Required field validation message', datetime('now')),
(716, 'validation.required', 'ja', 'このフィールドを入力してください', 'validation', '必須フィールドのバリデーションメッセージ', datetime('now')),
(717, 'common.saving', 'en', 'Saving...', 'common', 'Saving in progress', datetime('now')),
(718, 'common.saving', 'ja', '保存中...', 'common', '保存処理中', datetime('now')),
(719, 'error.category_duplicate_name', 'en', 'Category name "{0}" already exists', 'error', 'Duplicate category name error', datetime('now')),
(720, 'error.category_duplicate_name', 'ja', '費目名「{0}」は既に存在します', 'error', '重複費目名エラー', datetime('now')),
(721, 'category_mgmt.error_load_category', 'en', 'Failed to load category data', 'category_mgmt', 'Category data load error', datetime('now')),
(722, 'category_mgmt.error_load_category', 'ja', '費目データの読み込みに失敗しました', 'category_mgmt', '費目データ読み込みエラー', datetime('now')),

-- Account Management
(727, 'account_mgmt.title', 'en', 'Account Management', 'account_mgmt', 'Account management page title', datetime('now')),
(728, 'account_mgmt.title', 'ja', '口座管理', 'account_mgmt', '口座管理ページタイトル', datetime('now')),
(729, 'account_mgmt.account_list', 'en', 'Account List', 'account_mgmt', 'Account list section title', datetime('now')),
(730, 'account_mgmt.account_list', 'ja', '口座一覧', 'account_mgmt', '口座一覧セクションタイトル', datetime('now')),
(731, 'account_mgmt.add_account', 'en', 'Add Account', 'account_mgmt', 'Add account button', datetime('now')),
(732, 'account_mgmt.add_account', 'ja', '口座追加', 'account_mgmt', '口座追加ボタン', datetime('now')),
(733, 'account_mgmt.edit_account', 'en', 'Edit Account', 'account_mgmt', 'Edit account modal title', datetime('now')),
(734, 'account_mgmt.edit_account', 'ja', '口座編集', 'account_mgmt', '口座編集モーダルタイトル', datetime('now')),
(735, 'account_mgmt.account_code', 'en', 'Account Code', 'account_mgmt', 'Account code label', datetime('now')),
(736, 'account_mgmt.account_code', 'ja', '口座コード', 'account_mgmt', '口座コードラベル', datetime('now')),
(737, 'account_mgmt.account_name', 'en', 'Account Name', 'account_mgmt', 'Account name label', datetime('now')),
(738, 'account_mgmt.account_name', 'ja', '口座名', 'account_mgmt', '口座名ラベル', datetime('now')),
(739, 'account_mgmt.template', 'en', 'Template', 'account_mgmt', 'Template label', datetime('now')),
(740, 'account_mgmt.template', 'ja', 'テンプレート', 'account_mgmt', 'テンプレートラベル', datetime('now')),
(741, 'account_mgmt.initial_balance', 'en', 'Initial Balance', 'account_mgmt', 'Initial balance label', datetime('now')),
(742, 'account_mgmt.initial_balance', 'ja', '初期残高', 'account_mgmt', '初期残高ラベル', datetime('now')),
(743, 'account_mgmt.display_order', 'en', 'Display Order', 'account_mgmt', 'Display order label', datetime('now')),
(744, 'account_mgmt.display_order', 'ja', '表示順', 'account_mgmt', '表示順ラベル', datetime('now')),
(745, 'account_mgmt.no_accounts', 'en', 'No accounts found', 'account_mgmt', 'No accounts message', datetime('now')),
(746, 'account_mgmt.no_accounts', 'ja', '口座が見つかりません', 'account_mgmt', '口座なしメッセージ', datetime('now')),

-- Transaction Management
(747, 'transaction_mgmt.title', 'en', 'Transaction Management', 'transaction_mgmt', 'Transaction management page title', datetime('now')),
(748, 'transaction_mgmt.title', 'ja', '入出金管理', 'transaction_mgmt', '入出金管理ページタイトル', datetime('now')),
(749, 'transaction_mgmt.transaction_list', 'en', 'Transaction List', 'transaction_mgmt', 'Transaction list section title', datetime('now')),
(750, 'transaction_mgmt.transaction_list', 'ja', '入出金一覧', 'transaction_mgmt', '入出金一覧セクションタイトル', datetime('now')),
(751, 'transaction_mgmt.add_transaction', 'en', 'Add Transaction', 'transaction_mgmt', 'Add transaction button', datetime('now')),
(752, 'transaction_mgmt.add_transaction', 'ja', '入出金追加', 'transaction_mgmt', '入出金追加ボタン', datetime('now')),
(753, 'transaction_mgmt.edit_transaction', 'en', 'Edit Transaction', 'transaction_mgmt', 'Edit transaction modal title', datetime('now')),
(754, 'transaction_mgmt.edit_transaction', 'ja', '入出金編集', 'transaction_mgmt', '入出金編集モーダルタイトル', datetime('now')),
(755, 'transaction_mgmt.transaction_date', 'en', 'Transaction Date', 'transaction_mgmt', 'Transaction date label', datetime('now')),
(756, 'transaction_mgmt.transaction_date', 'ja', '取引日時', 'transaction_mgmt', '取引日時ラベル', datetime('now')),
(757, 'transaction_mgmt.category1', 'en', 'Category (Major)', 'transaction_mgmt', 'Major category label', datetime('now')),
(758, 'transaction_mgmt.category1', 'ja', '費目（大分類）', 'transaction_mgmt', '大分類ラベル', datetime('now')),
(759, 'transaction_mgmt.from_account', 'en', 'From Account', 'transaction_mgmt', 'From account label', datetime('now')),
(760, 'transaction_mgmt.from_account', 'ja', '出金元', 'transaction_mgmt', '出金元ラベル', datetime('now')),
(761, 'transaction_mgmt.to_account', 'en', 'To Account', 'transaction_mgmt', 'To account label', datetime('now')),
(762, 'transaction_mgmt.to_account', 'ja', '入金先', 'transaction_mgmt', '入金先ラベル', datetime('now')),
(763, 'transaction_mgmt.total_amount', 'en', 'Total Amount', 'transaction_mgmt', 'Total amount label', datetime('now')),
(764, 'transaction_mgmt.total_amount', 'ja', '合計金額', 'transaction_mgmt', '合計金額ラベル', datetime('now')),
(765, 'transaction_mgmt.tax_rounding', 'en', 'Tax Rounding', 'transaction_mgmt', 'Tax rounding label', datetime('now')),
(766, 'transaction_mgmt.tax_rounding', 'ja', '税額端数処理', 'transaction_mgmt', '税額端数処理ラベル', datetime('now')),
(767, 'transaction_mgmt.round_down', 'en', 'Round Down', 'transaction_mgmt', 'Round down option', datetime('now')),
(768, 'transaction_mgmt.round_down', 'ja', '切り捨て', 'transaction_mgmt', '切り捨てオプション', datetime('now')),
(769, 'transaction_mgmt.round_half', 'en', 'Round Half', 'transaction_mgmt', 'Round half option', datetime('now')),
(770, 'transaction_mgmt.round_half', 'ja', '四捨五入', 'transaction_mgmt', '四捨五入オプション', datetime('now')),
(771, 'transaction_mgmt.round_up', 'en', 'Round Up', 'transaction_mgmt', 'Round up option', datetime('now')),
(772, 'transaction_mgmt.round_up', 'ja', '切り上げ', 'transaction_mgmt', '切り上げオプション', datetime('now')),
(773, 'transaction_mgmt.memo', 'en', 'Memo', 'transaction_mgmt', 'Memo label', datetime('now')),
(774, 'transaction_mgmt.memo', 'ja', 'メモ', 'transaction_mgmt', 'メモラベル', datetime('now')),
(775, 'transaction_mgmt.no_transactions', 'en', 'No transactions found', 'transaction_mgmt', 'No transactions message', datetime('now')),
(776, 'transaction_mgmt.no_transactions', 'ja', '入出金データが見つかりません', 'transaction_mgmt', '入出金なしメッセージ', datetime('now')),
(777, 'transaction_mgmt.show_filters', 'en', 'Show Filters', 'transaction_mgmt', 'Show filters button', datetime('now')),
(778, 'transaction_mgmt.show_filters', 'ja', 'フィルタ表示', 'transaction_mgmt', 'フィルタ表示ボタン', datetime('now')),
(779, 'transaction_mgmt.hide_filters', 'en', 'Hide Filters', 'transaction_mgmt', 'Hide filters button', datetime('now')),
(780, 'transaction_mgmt.hide_filters', 'ja', 'フィルタ非表示', 'transaction_mgmt', 'フィルタ非表示ボタン', datetime('now')),
(781, 'transaction_mgmt.filters', 'en', 'Filters', 'transaction_mgmt', 'Filters title', datetime('now')),
(782, 'transaction_mgmt.filters', 'ja', 'フィルタ', 'transaction_mgmt', 'フィルタタイトル', datetime('now')),
(783, 'transaction_mgmt.date_range', 'en', 'Date Range:', 'transaction_mgmt', 'Date range label', datetime('now')),
(784, 'transaction_mgmt.date_range', 'ja', '期間:', 'transaction_mgmt', '期間ラベル', datetime('now')),
(785, 'transaction_mgmt.start_date', 'en', 'Start', 'transaction_mgmt', 'Start date label', datetime('now')),
(786, 'transaction_mgmt.start_date', 'ja', '開始', 'transaction_mgmt', '開始日ラベル', datetime('now')),
(787, 'transaction_mgmt.end_date', 'en', 'End', 'transaction_mgmt', 'End date label', datetime('now')),
(788, 'transaction_mgmt.end_date', 'ja', '終了', 'transaction_mgmt', '終了日ラベル', datetime('now')),
(789, 'transaction_mgmt.amount_range', 'en', 'Amount Range:', 'transaction_mgmt', 'Amount range label', datetime('now')),
(790, 'transaction_mgmt.amount_range', 'ja', '金額範囲:', 'transaction_mgmt', '金額範囲ラベル', datetime('now')),
(791, 'transaction_mgmt.min_amount', 'en', 'Min', 'transaction_mgmt', 'Minimum amount label', datetime('now')),
(792, 'transaction_mgmt.min_amount', 'ja', '最小', 'transaction_mgmt', '最小金額ラベル', datetime('now')),
(793, 'transaction_mgmt.max_amount', 'en', 'Max', 'transaction_mgmt', 'Maximum amount label', datetime('now')),
(794, 'transaction_mgmt.max_amount', 'ja', '最大', 'transaction_mgmt', '最大金額ラベル', datetime('now')),
(795, 'transaction_mgmt.keyword', 'en', 'Keyword:', 'transaction_mgmt', 'Keyword search label', datetime('now')),
(796, 'transaction_mgmt.keyword', 'ja', 'キーワード:', 'transaction_mgmt', 'キーワード検索ラベル', datetime('now')),
(797, 'transaction_mgmt.apply_filters', 'en', 'Apply Filters', 'transaction_mgmt', 'Apply filters button', datetime('now')),
(798, 'transaction_mgmt.apply_filters', 'ja', 'フィルタ適用', 'transaction_mgmt', 'フィルタ適用ボタン', datetime('now')),
(799, 'transaction_mgmt.clear_filters', 'en', 'Clear Filters', 'transaction_mgmt', 'Clear filters button', datetime('now')),
(800, 'transaction_mgmt.clear_filters', 'ja', 'フィルタクリア', 'transaction_mgmt', 'フィルタクリアボタン', datetime('now')),

-- Common resources
(801, 'common.unspecified', 'en', 'Unspecified', 'common', 'Unspecified option', datetime('now')),
(802, 'common.unspecified', 'ja', '指定なし', 'common', '指定なしオプション', datetime('now')),
(803, 'common.delete', 'en', 'Delete', 'common', 'Delete button', datetime('now')),
(804, 'common.delete', 'ja', '削除', 'common', '削除ボタン', datetime('now')),
(805, 'common.success', 'en', 'Success', 'common', 'Success message', datetime('now')),
(806, 'common.success', 'ja', '成功', 'common', '成功メッセージ', datetime('now')),
(807, 'account_mgmt.add_new', 'en', 'Add New Account', 'account_mgmt', 'Add new account button', datetime('now')),
(808, 'account_mgmt.add_new', 'ja', '新規口座追加', 'account_mgmt', '新規口座追加ボタン', datetime('now')),
(809, 'common.actions', 'en', 'Actions', 'common', 'Actions column header', datetime('now')),
(810, 'common.actions', 'ja', '操作', 'common', '操作カラムヘッダー', datetime('now')),
(811, 'account_mgmt.modal_title_add', 'en', 'Add Account', 'account_mgmt', 'Add account modal title', datetime('now')),
(812, 'account_mgmt.modal_title_add', 'ja', '口座追加', 'account_mgmt', '口座追加モーダルタイトル', datetime('now')),
(813, 'account_mgmt.modal_title_edit', 'en', 'Edit Account', 'account_mgmt', 'Edit account modal title', datetime('now')),
(814, 'account_mgmt.modal_title_edit', 'ja', '口座編集', 'account_mgmt', '口座編集モーダルタイトル', datetime('now')),
(815, 'account_mgmt.delete_confirm', 'en', 'Are you sure you want to delete this account?', 'account_mgmt', 'Delete account confirmation message', datetime('now')),
(816, 'account_mgmt.delete_confirm', 'ja', 'この口座を削除してもよろしいですか？', 'account_mgmt', '口座削除確認メッセージ', datetime('now')),
(817, 'account_mgmt.select_template', 'en', '- Select Template -', 'account_mgmt', 'Template select placeholder', datetime('now')),
(818, 'account_mgmt.select_template', 'ja', '- テンプレートを選択 -', 'account_mgmt', 'テンプレート選択プレースホルダー', datetime('now')),

-- Transaction Management additional resources
(819, 'transaction_mgmt.add_new', 'en', 'Add New Transaction', 'transaction_mgmt', 'Add new transaction button', datetime('now')),
(820, 'transaction_mgmt.add_new', 'ja', '新規入出金追加', 'transaction_mgmt', '新規入出金追加ボタン', datetime('now')),
(821, 'transaction_mgmt.filter', 'en', 'Filter', 'transaction_mgmt', 'Filter button', datetime('now')),
(822, 'transaction_mgmt.filter', 'ja', 'フィルタ', 'transaction_mgmt', 'フィルタボタン', datetime('now')),
(823, 'transaction_mgmt.total', 'en', 'Total', 'transaction_mgmt', 'Total label', datetime('now')),
(824, 'transaction_mgmt.total', 'ja', '合計', 'transaction_mgmt', '合計ラベル', datetime('now')),
(825, 'transaction_mgmt.items', 'en', 'items', 'transaction_mgmt', 'Items label', datetime('now')),
(826, 'transaction_mgmt.items', 'ja', '件', 'transaction_mgmt', '件数ラベル', datetime('now')),
(827, 'transaction_mgmt.page', 'en', 'Page', 'transaction_mgmt', 'Page label', datetime('now')),
(828, 'transaction_mgmt.page', 'ja', 'ページ', 'transaction_mgmt', 'ページラベル', datetime('now')),
(829, 'transaction_mgmt.delete_confirm', 'en', 'Are you sure you want to delete this transaction?', 'transaction_mgmt', 'Delete transaction confirmation message', datetime('now')),
(830, 'transaction_mgmt.delete_confirm', 'ja', 'この入出金データを削除してもよろしいですか？', 'transaction_mgmt', '入出金削除確認メッセージ', datetime('now')),
(831, 'transaction_mgmt.select_category', 'en', '- Select category -', 'transaction_mgmt', 'Category select placeholder', datetime('now')),
(832, 'transaction_mgmt.select_category', 'ja', '- 費目を選択 -', 'transaction_mgmt', '費目選択プレースホルダー', datetime('now')),
(833, 'transaction_mgmt.manage_details', 'en', 'Manage Details', 'transaction_mgmt', 'Manage details button', datetime('now')),
(834, 'transaction_mgmt.manage_details', 'ja', '明細管理', 'transaction_mgmt', '明細管理ボタン', datetime('now')),
(835, 'transaction_mgmt.detail_items', 'en', 'items', 'transaction_mgmt', 'Detail items count label', datetime('now')),
(836, 'transaction_mgmt.detail_items', 'ja', '件', 'transaction_mgmt', '明細件数ラベル', datetime('now')),
(837, 'transaction_mgmt.filter_options', 'en', 'Filter Options', 'transaction_mgmt', 'Filter options title', datetime('now')),
(838, 'transaction_mgmt.filter_options', 'ja', 'フィルタオプション', 'transaction_mgmt', 'フィルタオプションタイトル', datetime('now')),
(839, 'transaction_mgmt.category', 'en', 'Category', 'transaction_mgmt', 'Category filter label', datetime('now')),
(840, 'transaction_mgmt.category', 'ja', '費目', 'transaction_mgmt', '費目フィルタラベル', datetime('now')),
(841, 'common.all', 'en', 'All', 'common', 'All option in filters', datetime('now')),
(842, 'common.all', 'ja', 'すべて', 'common', 'フィルタのすべてオプション', datetime('now')),
(843, 'transaction_mgmt.clear_filter', 'en', 'Clear', 'transaction_mgmt', 'Clear filter button', datetime('now')),
(844, 'transaction_mgmt.clear_filter', 'ja', 'クリア', 'transaction_mgmt', 'フィルタクリアボタン', datetime('now')),
(845, 'transaction_mgmt.apply_filter', 'en', 'Apply', 'transaction_mgmt', 'Apply filter button', datetime('now')),
(846, 'transaction_mgmt.apply_filter', 'ja', '適用', 'transaction_mgmt', 'フィルタ適用ボタン', datetime('now')),
(847, 'transaction_mgmt.min_placeholder', 'en', 'Min', 'transaction_mgmt', 'Minimum amount placeholder', datetime('now')),
(848, 'transaction_mgmt.min_placeholder', 'ja', '最小', 'transaction_mgmt', '最小金額プレースホルダー', datetime('now')),
(849, 'transaction_mgmt.max_placeholder', 'en', 'Max', 'transaction_mgmt', 'Maximum amount placeholder', datetime('now')),
(850, 'transaction_mgmt.max_placeholder', 'ja', '最大', 'transaction_mgmt', '最大金額プレースホルダー', datetime('now')),
(851, 'transaction_mgmt.search_placeholder', 'en', 'Search in memo', 'transaction_mgmt', 'Keyword search placeholder', datetime('now')),
(852, 'transaction_mgmt.search_placeholder', 'ja', 'メモを検索', 'transaction_mgmt', 'キーワード検索プレースホルダー', datetime('now')),

-- Manufacturer Management
(853, 'manufacturer_mgmt.title', 'en', 'Manufacturer Management', 'manufacturer_mgmt', 'Manufacturer management page title', datetime('now')),
(854, 'manufacturer_mgmt.title', 'ja', 'メーカー管理', 'manufacturer_mgmt', 'メーカー管理ページタイトル', datetime('now')),
(855, 'manufacturer_mgmt.list', 'en', 'Manufacturer List', 'manufacturer_mgmt', 'Manufacturer list section title', datetime('now')),
(856, 'manufacturer_mgmt.list', 'ja', 'メーカー一覧', 'manufacturer_mgmt', 'メーカー一覧セクションタイトル', datetime('now')),
(857, 'manufacturer_mgmt.add', 'en', 'Add Manufacturer', 'manufacturer_mgmt', 'Add manufacturer button', datetime('now')),
(858, 'manufacturer_mgmt.add', 'ja', 'メーカー追加', 'manufacturer_mgmt', 'メーカー追加ボタン', datetime('now')),
(859, 'manufacturer_mgmt.edit', 'en', 'Edit Manufacturer', 'manufacturer_mgmt', 'Edit manufacturer modal title', datetime('now')),
(860, 'manufacturer_mgmt.edit', 'ja', 'メーカー編集', 'manufacturer_mgmt', 'メーカー編集モーダルタイトル', datetime('now')),
(861, 'manufacturer_mgmt.code', 'en', 'Manufacturer Code', 'manufacturer_mgmt', 'Manufacturer code label', datetime('now')),
(862, 'manufacturer_mgmt.code', 'ja', 'メーカーコード', 'manufacturer_mgmt', 'メーカーコードラベル', datetime('now')),
(863, 'manufacturer_mgmt.name', 'en', 'Manufacturer Name', 'manufacturer_mgmt', 'Manufacturer name label', datetime('now')),
(864, 'manufacturer_mgmt.name', 'ja', 'メーカー名', 'manufacturer_mgmt', 'メーカー名ラベル', datetime('now')),
(865, 'manufacturer_mgmt.memo', 'en', 'Memo', 'manufacturer_mgmt', 'Memo label', datetime('now')),
(866, 'manufacturer_mgmt.memo', 'ja', 'メモ', 'manufacturer_mgmt', 'メモラベル', datetime('now')),
(867, 'manufacturer_mgmt.display_order', 'en', 'Display Order', 'manufacturer_mgmt', 'Display order label', datetime('now')),
(868, 'manufacturer_mgmt.display_order', 'ja', '表示順', 'manufacturer_mgmt', '表示順ラベル', datetime('now')),
(869, 'manufacturer_mgmt.no_data', 'en', 'No manufacturers found', 'manufacturer_mgmt', 'No manufacturers message', datetime('now')),
(870, 'manufacturer_mgmt.no_data', 'ja', 'メーカーが見つかりません', 'manufacturer_mgmt', 'メーカーなしメッセージ', datetime('now')),
(871, 'manufacturer_mgmt.delete_confirm', 'en', 'Are you sure you want to delete this manufacturer?', 'manufacturer_mgmt', 'Delete confirmation message', datetime('now')),
(872, 'manufacturer_mgmt.delete_confirm', 'ja', 'このメーカーを削除してもよろしいですか？', 'manufacturer_mgmt', '削除確認メッセージ', datetime('now')),

-- Product Management
(873, 'product_mgmt.title', 'en', 'Product Management', 'product_mgmt', 'Product management page title', datetime('now')),
(874, 'product_mgmt.title', 'ja', '商品管理', 'product_mgmt', '商品管理ページタイトル', datetime('now')),
(875, 'product_mgmt.list', 'en', 'Product List', 'product_mgmt', 'Product list section title', datetime('now')),
(876, 'product_mgmt.list', 'ja', '商品一覧', 'product_mgmt', '商品一覧セクションタイトル', datetime('now')),
(877, 'product_mgmt.add', 'en', 'Add Product', 'product_mgmt', 'Add product button', datetime('now')),
(878, 'product_mgmt.add', 'ja', '商品追加', 'product_mgmt', '商品追加ボタン', datetime('now')),
(879, 'product_mgmt.edit', 'en', 'Edit Product', 'product_mgmt', 'Edit product modal title', datetime('now')),
(880, 'product_mgmt.edit', 'ja', '商品編集', 'product_mgmt', '商品編集モーダルタイトル', datetime('now')),
(881, 'product_mgmt.code', 'en', 'Product Code', 'product_mgmt', 'Product code label', datetime('now')),
(882, 'product_mgmt.code', 'ja', '商品コード', 'product_mgmt', '商品コードラベル', datetime('now')),
(883, 'product_mgmt.name', 'en', 'Product Name', 'product_mgmt', 'Product name label', datetime('now')),
(884, 'product_mgmt.name', 'ja', '商品名', 'product_mgmt', '商品名ラベル', datetime('now')),
(885, 'product_mgmt.manufacturer', 'en', 'Manufacturer', 'product_mgmt', 'Manufacturer label', datetime('now')),
(886, 'product_mgmt.manufacturer', 'ja', 'メーカー', 'product_mgmt', 'メーカーラベル', datetime('now')),
(887, 'product_mgmt.manufacturer_none', 'en', '(Not specified)', 'product_mgmt', 'No manufacturer label', datetime('now')),
(888, 'product_mgmt.manufacturer_none', 'ja', '(未指定)', 'product_mgmt', 'メーカー未指定ラベル', datetime('now')),
(889, 'product_mgmt.memo', 'en', 'Memo', 'product_mgmt', 'Memo label', datetime('now')),
(890, 'product_mgmt.memo', 'ja', 'メモ', 'product_mgmt', 'メモラベル', datetime('now')),
(891, 'product_mgmt.display_order', 'en', 'Display Order', 'product_mgmt', 'Display order label', datetime('now')),
(892, 'product_mgmt.display_order', 'ja', '表示順', 'product_mgmt', '表示順ラベル', datetime('now')),
(893, 'product_mgmt.no_data', 'en', 'No products found', 'product_mgmt', 'No products message', datetime('now')),
(894, 'product_mgmt.no_data', 'ja', '商品が見つかりません', 'product_mgmt', '商品なしメッセージ', datetime('now')),
(895, 'product_mgmt.delete_confirm', 'en', 'Are you sure you want to delete this product?', 'product_mgmt', 'Delete confirmation message', datetime('now')),
(896, 'product_mgmt.delete_confirm', 'ja', 'この商品を削除してもよろしいですか？', 'product_mgmt', '削除確認メッセージ', datetime('now')),

-- Error messages
(903, 'manufacturer_mgmt.duplicate_error', 'en', 'This manufacturer name already exists.', 'manufacturer_mgmt', 'Duplicate manufacturer name error', datetime('now')),
(904, 'manufacturer_mgmt.duplicate_error', 'ja', 'このメーカー名は既に存在します。', 'manufacturer_mgmt', '重複メーカー名エラー', datetime('now')),
(905, 'product_mgmt.duplicate_error', 'en', 'This product name already exists.', 'product_mgmt', 'Duplicate product name error', datetime('now')),
(906, 'product_mgmt.duplicate_error', 'ja', 'この商品名は既に存在します。', 'product_mgmt', '重複商品名エラー', datetime('now'));

-- NOTE: Category data will be migrated from existing SQL later

-- ============================================================================
-- Account Management Tables
-- ============================================================================

-- SQL_20000001: Create ACCOUNT_TEMPLATES table
CREATE TABLE IF NOT EXISTS ACCOUNT_TEMPLATES (
    TEMPLATE_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    TEMPLATE_CODE VARCHAR(50) NOT NULL UNIQUE,
    TEMPLATE_NAME_JA TEXT NOT NULL,
    TEMPLATE_NAME_EN TEXT NOT NULL,
    DISPLAY_ORDER INTEGER,
    ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now'))
);

-- SQL_20000002: Create ACCOUNTS table
CREATE TABLE IF NOT EXISTS ACCOUNTS (
    ACCOUNT_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    USER_ID INTEGER NOT NULL,
    ACCOUNT_CODE VARCHAR(50) NOT NULL,
    ACCOUNT_NAME TEXT NOT NULL,
    TEMPLATE_CODE VARCHAR(50) NOT NULL,
    INITIAL_BALANCE INTEGER DEFAULT 0,
    DISPLAY_ORDER INTEGER,
    IS_DISABLED INTEGER DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now')),
    UPDATE_DT DATETIME,
    FOREIGN KEY (USER_ID) REFERENCES USERS(USER_ID) ON DELETE CASCADE,
    FOREIGN KEY (TEMPLATE_CODE) REFERENCES ACCOUNT_TEMPLATES(TEMPLATE_CODE),
    UNIQUE(USER_ID, ACCOUNT_CODE)
);

-- SQL_20000003: Insert account templates
INSERT OR IGNORE INTO ACCOUNT_TEMPLATES (TEMPLATE_CODE, TEMPLATE_NAME_JA, TEMPLATE_NAME_EN, DISPLAY_ORDER) VALUES
('CASH', '現金', 'Cash', 1),
('BANK', '銀行', 'Bank', 2),
('CREDIT', 'クレジットカード', 'Credit Card', 3),
('EMONEY', '電子マネー', 'E-Money', 4),
('OTHER', 'その他', 'Other', 5),
('NONE', '指定なし', 'Unspecified', 0);

-- ============================================================================
-- Shop Management Tables
-- ============================================================================

-- SQL_25000001: Create SHOPS table
CREATE TABLE IF NOT EXISTS SHOPS (
    SHOP_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    USER_ID INTEGER NOT NULL,
    SHOP_NAME TEXT NOT NULL,
    MEMO TEXT,
    DISPLAY_ORDER INTEGER NOT NULL DEFAULT 0,
    IS_DISABLED INTEGER DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now')),
    UPDATE_DT DATETIME,
    FOREIGN KEY (USER_ID) REFERENCES USERS(USER_ID)
);

-- Create indexes for SHOPS
CREATE INDEX IF NOT EXISTS idx_shops_user
    ON SHOPS(USER_ID, DISPLAY_ORDER);

-- ============================================================================
-- Manufacturer and Product Management Tables
-- ============================================================================

-- SQL_26000001: Create MANUFACTURERS table
CREATE TABLE IF NOT EXISTS MANUFACTURERS (
    MANUFACTURER_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    USER_ID INTEGER NOT NULL,
    MANUFACTURER_NAME TEXT NOT NULL,
    MEMO TEXT,
    DISPLAY_ORDER INTEGER NOT NULL DEFAULT 0,
    IS_DISABLED INTEGER DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now')),
    UPDATE_DT DATETIME,
    FOREIGN KEY (USER_ID) REFERENCES USERS(USER_ID) ON DELETE CASCADE,
    UNIQUE(USER_ID, MANUFACTURER_NAME)
);

-- Create indexes for MANUFACTURERS
CREATE INDEX IF NOT EXISTS idx_manufacturers_user
    ON MANUFACTURERS(USER_ID, DISPLAY_ORDER);

-- SQL_26000002: Create PRODUCTS table
CREATE TABLE IF NOT EXISTS PRODUCTS (
    PRODUCT_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    USER_ID INTEGER NOT NULL,
    PRODUCT_NAME TEXT NOT NULL,
    MANUFACTURER_ID INTEGER,
    MEMO TEXT,
    DISPLAY_ORDER INTEGER NOT NULL DEFAULT 0,
    IS_DISABLED INTEGER DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now')),
    UPDATE_DT DATETIME,
    FOREIGN KEY (USER_ID) REFERENCES USERS(USER_ID) ON DELETE CASCADE,
    FOREIGN KEY (MANUFACTURER_ID) REFERENCES MANUFACTURERS(MANUFACTURER_ID) ON DELETE SET NULL,
    UNIQUE(USER_ID, PRODUCT_NAME)
);

-- Create indexes for PRODUCTS
CREATE INDEX IF NOT EXISTS idx_products_user
    ON PRODUCTS(USER_ID, DISPLAY_ORDER);
CREATE INDEX IF NOT EXISTS idx_products_manufacturer
    ON PRODUCTS(MANUFACTURER_ID);

-- ============================================================================
-- Transaction Management Tables
-- ============================================================================

-- SQL_30000001: Create MEMOS table
CREATE TABLE IF NOT EXISTS MEMOS (
    MEMO_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    USER_ID INTEGER NOT NULL,
    MEMO_TEXT TEXT NOT NULL,
    ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now')),
    UPDATE_DT DATETIME,
    FOREIGN KEY (USER_ID) REFERENCES USERS(USER_ID) ON DELETE CASCADE,
    CHECK (MEMO_TEXT != '')
);

-- SQL_30000002: Create TRANSACTIONS_HEADER table
CREATE TABLE IF NOT EXISTS TRANSACTIONS_HEADER (
    TRANSACTION_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    USER_ID INTEGER NOT NULL,
    SHOP_ID INTEGER,
    CATEGORY1_CODE VARCHAR(50) NOT NULL,
    FROM_ACCOUNT_CODE VARCHAR(50) NOT NULL,
    TO_ACCOUNT_CODE VARCHAR(50) NOT NULL,
    TRANSACTION_DATE DATETIME NOT NULL,
    TOTAL_AMOUNT INTEGER NOT NULL,
    TAX_ROUNDING_TYPE INTEGER DEFAULT 0,
    MEMO_ID INTEGER,
    IS_DISABLED INTEGER DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now')),
    UPDATE_DT DATETIME,
    FOREIGN KEY (USER_ID) REFERENCES USERS(USER_ID) ON DELETE CASCADE,
    FOREIGN KEY (USER_ID, CATEGORY1_CODE) REFERENCES CATEGORY1(USER_ID, CATEGORY1_CODE),
    FOREIGN KEY (USER_ID, FROM_ACCOUNT_CODE) REFERENCES ACCOUNTS(USER_ID, ACCOUNT_CODE),
    FOREIGN KEY (USER_ID, TO_ACCOUNT_CODE) REFERENCES ACCOUNTS(USER_ID, ACCOUNT_CODE),
    FOREIGN KEY (MEMO_ID) REFERENCES MEMOS(MEMO_ID)
);

-- SQL_30000003: Create TRANSACTIONS_DETAIL table
CREATE TABLE IF NOT EXISTS TRANSACTIONS_DETAIL (
    DETAIL_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    TRANSACTION_ID INTEGER NOT NULL,
    CATEGORY2_CODE VARCHAR(50) NOT NULL,
    CATEGORY3_CODE VARCHAR(50) NOT NULL,
    ITEM_NAME TEXT NOT NULL,
    AMOUNT INTEGER NOT NULL,
    TAX_AMOUNT INTEGER DEFAULT 0,
    TAX_RATE INTEGER DEFAULT 8,
    MEMO_ID INTEGER,
    ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now')),
    UPDATE_DT DATETIME,
    FOREIGN KEY (TRANSACTION_ID) REFERENCES TRANSACTIONS_HEADER(TRANSACTION_ID) ON DELETE CASCADE,
    FOREIGN KEY (MEMO_ID) REFERENCES MEMOS(MEMO_ID),
    CHECK (ITEM_NAME != '')
);

-- Create indexes for accounts
CREATE INDEX IF NOT EXISTS idx_accounts_user ON ACCOUNTS(USER_ID, ACCOUNT_CODE);
CREATE INDEX IF NOT EXISTS idx_accounts_user_order ON ACCOUNTS(USER_ID, DISPLAY_ORDER);
CREATE INDEX IF NOT EXISTS idx_accounts_template ON ACCOUNTS(TEMPLATE_CODE);

-- Create indexes for memos
CREATE INDEX IF NOT EXISTS idx_memos_user ON MEMOS(USER_ID);
CREATE INDEX IF NOT EXISTS idx_memos_text ON MEMOS(USER_ID, MEMO_TEXT);

-- Create indexes for transactions_header
CREATE INDEX IF NOT EXISTS idx_transactions_header_user ON TRANSACTIONS_HEADER(USER_ID, TRANSACTION_DATE);
CREATE INDEX IF NOT EXISTS idx_transactions_header_accounts ON TRANSACTIONS_HEADER(FROM_ACCOUNT_CODE, TO_ACCOUNT_CODE);
CREATE INDEX IF NOT EXISTS idx_transactions_header_category ON TRANSACTIONS_HEADER(CATEGORY1_CODE);
CREATE INDEX IF NOT EXISTS idx_transactions_header_date ON TRANSACTIONS_HEADER(TRANSACTION_DATE);

-- Create indexes for transactions_detail
CREATE INDEX IF NOT EXISTS idx_transactions_detail_transaction ON TRANSACTIONS_DETAIL(TRANSACTION_ID);
CREATE INDEX IF NOT EXISTS idx_transactions_detail_categories ON TRANSACTIONS_DETAIL(CATEGORY2_CODE, CATEGORY3_CODE);

-- ============================================================================
-- I18N Resources Initial Data

-- Translation resources for Account Management, Transaction Management, and Shop Management
-- Starting from RESOURCE_ID 727

-- ============================================================================
-- Account Management (account_mgmt.*)
-- ============================================================================

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
-- Account Management Screen
(727, 'account_mgmt.title', 'en', 'Account Management', 'account_mgmt', 'Account management page title', datetime('now')),
(728, 'account_mgmt.title', 'ja', '口座管理', 'account_mgmt', '口座管理ページタイトル', datetime('now')),
(729, 'account_mgmt.account_list', 'en', 'Account List', 'account_mgmt', 'Account list section title', datetime('now')),
(730, 'account_mgmt.account_list', 'ja', '口座一覧', 'account_mgmt', '口座一覧セクションタイトル', datetime('now')),
(731, 'account_mgmt.add_account', 'en', 'Add Account', 'account_mgmt', 'Add account button', datetime('now')),
(732, 'account_mgmt.add_account', 'ja', '口座追加', 'account_mgmt', '口座追加ボタン', datetime('now')),
(733, 'account_mgmt.edit_account', 'en', 'Edit Account', 'account_mgmt', 'Edit account modal title', datetime('now')),
(734, 'account_mgmt.edit_account', 'ja', '口座編集', 'account_mgmt', '口座編集モーダルタイトル', datetime('now')),
(735, 'account_mgmt.account_code', 'en', 'Account Code', 'account_mgmt', 'Account code label', datetime('now')),
(736, 'account_mgmt.account_code', 'ja', '口座コード', 'account_mgmt', '口座コードラベル', datetime('now')),
(737, 'account_mgmt.account_name', 'en', 'Account Name', 'account_mgmt', 'Account name label', datetime('now')),
(738, 'account_mgmt.account_name', 'ja', '口座名', 'account_mgmt', '口座名ラベル', datetime('now')),
(739, 'account_mgmt.template', 'en', 'Template', 'account_mgmt', 'Template label', datetime('now')),
(740, 'account_mgmt.template', 'ja', 'テンプレート', 'account_mgmt', 'テンプレートラベル', datetime('now')),
(741, 'account_mgmt.initial_balance', 'en', 'Initial Balance', 'account_mgmt', 'Initial balance label', datetime('now')),
(742, 'account_mgmt.initial_balance', 'ja', '初期残高', 'account_mgmt', '初期残高ラベル', datetime('now')),
(743, 'account_mgmt.display_order', 'en', 'Display Order', 'account_mgmt', 'Display order label', datetime('now')),
(744, 'account_mgmt.display_order', 'ja', '表示順', 'account_mgmt', '表示順ラベル', datetime('now')),
(745, 'account_mgmt.no_accounts', 'en', 'No accounts found', 'account_mgmt', 'No accounts message', datetime('now')),
(746, 'account_mgmt.no_accounts', 'ja', '口座が見つかりません', 'account_mgmt', '口座なしメッセージ', datetime('now')),

-- ============================================================================
-- Transaction Management (transaction_mgmt.*)
-- ============================================================================

-- Transaction Management Screen
(747, 'transaction_mgmt.title', 'en', 'Transaction Management', 'transaction_mgmt', 'Transaction management page title', datetime('now')),
(748, 'transaction_mgmt.title', 'ja', '入出金管理', 'transaction_mgmt', '入出金管理ページタイトル', datetime('now')),
(749, 'transaction_mgmt.transaction_list', 'en', 'Transaction List', 'transaction_mgmt', 'Transaction list section title', datetime('now')),
(750, 'transaction_mgmt.transaction_list', 'ja', '入出金一覧', 'transaction_mgmt', '入出金一覧セクションタイトル', datetime('now')),
(751, 'transaction_mgmt.add_transaction', 'en', 'Add Transaction', 'transaction_mgmt', 'Add transaction button', datetime('now')),
(752, 'transaction_mgmt.add_transaction', 'ja', '入出金追加', 'transaction_mgmt', '入出金追加ボタン', datetime('now')),
(753, 'transaction_mgmt.edit_transaction', 'en', 'Edit Transaction', 'transaction_mgmt', 'Edit transaction modal title', datetime('now')),
(754, 'transaction_mgmt.edit_transaction', 'ja', '入出金編集', 'transaction_mgmt', '入出金編集モーダルタイトル', datetime('now')),
(755, 'transaction_mgmt.transaction_date', 'en', 'Transaction Date', 'transaction_mgmt', 'Transaction date label', datetime('now')),
(756, 'transaction_mgmt.transaction_date', 'ja', '取引日時', 'transaction_mgmt', '取引日時ラベル', datetime('now')),
(757, 'transaction_mgmt.category1', 'en', 'Category (Major)', 'transaction_mgmt', 'Major category label', datetime('now')),
(758, 'transaction_mgmt.category1', 'ja', '費目（大分類）', 'transaction_mgmt', '大分類ラベル', datetime('now')),
(759, 'transaction_mgmt.from_account', 'en', 'From Account', 'transaction_mgmt', 'From account label', datetime('now')),
(760, 'transaction_mgmt.from_account', 'ja', '出金口座', 'transaction_mgmt', '出金口座ラベル', datetime('now')),
(761, 'transaction_mgmt.to_account', 'en', 'To Account', 'transaction_mgmt', 'To account label', datetime('now')),
(762, 'transaction_mgmt.to_account', 'ja', '入金口座', 'transaction_mgmt', '入金口座ラベル', datetime('now')),
(763, 'transaction_mgmt.amount', 'en', 'Amount', 'transaction_mgmt', 'Amount label', datetime('now')),
(764, 'transaction_mgmt.amount', 'ja', '金額', 'transaction_mgmt', '金額ラベル', datetime('now')),
(765, 'transaction_mgmt.memo', 'en', 'Memo', 'transaction_mgmt', 'Memo label', datetime('now')),
(766, 'transaction_mgmt.memo', 'ja', 'メモ', 'transaction_mgmt', 'メモラベル', datetime('now')),
(767, 'transaction_mgmt.no_transactions', 'en', 'No transactions found', 'transaction_mgmt', 'No transactions message', datetime('now')),
(768, 'transaction_mgmt.no_transactions', 'ja', '入出金が見つかりません', 'transaction_mgmt', '入出金なしメッセージ', datetime('now')),
(769, 'transaction_mgmt.income', 'en', 'Income', 'transaction_mgmt', 'Income type', datetime('now')),
(770, 'transaction_mgmt.income', 'ja', '収入', 'transaction_mgmt', '収入タイプ', datetime('now')),
(771, 'transaction_mgmt.expense', 'en', 'Expense', 'transaction_mgmt', 'Expense type', datetime('now')),
(772, 'transaction_mgmt.expense', 'ja', '支出', 'transaction_mgmt', '支出タイプ', datetime('now')),
(773, 'transaction_mgmt.transfer', 'en', 'Transfer', 'transaction_mgmt', 'Transfer type', datetime('now')),
(774, 'transaction_mgmt.transfer', 'ja', '振替', 'transaction_mgmt', '振替タイプ', datetime('now')),
(775, 'transaction_mgmt.no_account', 'en', 'No Account', 'transaction_mgmt', 'No account selected label', datetime('now')),
(776, 'transaction_mgmt.no_account', 'ja', '口座なし', 'transaction_mgmt', '口座未選択ラベル', datetime('now')),
(777, 'transaction_mgmt.add_new', 'en', 'Add Transaction', 'transaction_mgmt', 'Add transaction button text', datetime('now')),
(778, 'transaction_mgmt.add_new', 'ja', '入出金追加', 'transaction_mgmt', '入出金追加ボタンテキスト', datetime('now')),
(779, 'transaction_mgmt.selected_account', 'en', 'Selected Account', 'transaction_mgmt', 'Selected account filter label', datetime('now')),
(780, 'transaction_mgmt.selected_account', 'ja', '選択口座', 'transaction_mgmt', '選択口座フィルタラベル', datetime('now')),
(781, 'transaction_mgmt.transaction_type', 'en', 'Transaction Type', 'transaction_mgmt', 'Transaction type filter label', datetime('now')),
(782, 'transaction_mgmt.transaction_type', 'ja', '取引種別', 'transaction_mgmt', '取引種別フィルタラベル', datetime('now')),
(783, 'transaction_mgmt.date_range', 'en', 'Date Range', 'transaction_mgmt', 'Date range filter label', datetime('now')),
(784, 'transaction_mgmt.date_range', 'ja', '日付範囲', 'transaction_mgmt', '日付範囲フィルタラベル', datetime('now')),
(785, 'transaction_mgmt.start_date', 'en', 'Start Date', 'transaction_mgmt', 'Start date label', datetime('now')),
(786, 'transaction_mgmt.start_date', 'ja', '開始日', 'transaction_mgmt', '開始日ラベル', datetime('now')),
(787, 'transaction_mgmt.end_date', 'en', 'End Date', 'transaction_mgmt', 'End date label', datetime('now')),
(788, 'transaction_mgmt.end_date', 'ja', '終了日', 'transaction_mgmt', '終了日ラベル', datetime('now')),
(789, 'transaction_mgmt.amount_range', 'en', 'Amount Range', 'transaction_mgmt', 'Amount range filter label', datetime('now')),
(790, 'transaction_mgmt.amount_range', 'ja', '金額範囲', 'transaction_mgmt', '金額範囲フィルタラベル', datetime('now')),
(791, 'transaction_mgmt.keyword', 'en', 'Keyword', 'transaction_mgmt', 'Keyword search label', datetime('now')),
(792, 'transaction_mgmt.keyword', 'ja', 'キーワード', 'transaction_mgmt', 'キーワード検索ラベル', datetime('now')),
(793, 'transaction_mgmt.showing_items', 'en', 'Showing {start} to {end} of {total} items', 'transaction_mgmt', 'Pagination info', datetime('now')),
(794, 'transaction_mgmt.showing_items', 'ja', '{total}件中 {start}〜{end}件を表示', 'transaction_mgmt', 'ページネーション情報', datetime('now')),
(795, 'transaction_mgmt.previous', 'en', 'Previous', 'transaction_mgmt', 'Previous page button', datetime('now')),
(796, 'transaction_mgmt.previous', 'ja', '前へ', 'transaction_mgmt', '前ページボタン', datetime('now')),
(797, 'transaction_mgmt.next', 'en', 'Next', 'transaction_mgmt', 'Next page button', datetime('now')),
(798, 'transaction_mgmt.next', 'ja', '次へ', 'transaction_mgmt', '次ページボタン', datetime('now')),
(799, 'transaction_mgmt.delete_confirmation', 'en', 'Delete Confirmation', 'transaction_mgmt', 'Delete confirmation title', datetime('now')),
(800, 'transaction_mgmt.delete_confirmation', 'ja', '削除確認', 'transaction_mgmt', '削除確認タイトル', datetime('now')),
(801, 'transaction_mgmt.delete_message', 'en', 'Are you sure you want to delete this transaction?', 'transaction_mgmt', 'Delete confirmation message', datetime('now')),
(802, 'transaction_mgmt.delete_message', 'ja', 'この入出金を削除してもよろしいですか？', 'transaction_mgmt', '削除確認メッセージ', datetime('now')),
(803, 'common.delete', 'en', 'Delete', 'common', 'Delete button', datetime('now')),
(804, 'common.delete', 'ja', '削除', 'common', '削除ボタン', datetime('now')),
(805, 'transaction_mgmt.edit', 'en', 'Edit', 'transaction_mgmt', 'Edit button', datetime('now')),
(806, 'transaction_mgmt.edit', 'ja', '編集', 'transaction_mgmt', '編集ボタン', datetime('now')),
(807, 'account_mgmt.add_new', 'en', 'Add New Account', 'account_mgmt', 'Add new account button', datetime('now')),
(808, 'account_mgmt.add_new', 'ja', '新規口座追加', 'account_mgmt', '新規口座追加ボタン', datetime('now')),
(809, 'common.actions', 'en', 'Actions', 'common', 'Actions column header', datetime('now')),
(810, 'common.actions', 'ja', '操作', 'common', '操作カラムヘッダー', datetime('now')),
(811, 'account_mgmt.modal_title_add', 'en', 'Add Account', 'account_mgmt', 'Add account modal title', datetime('now')),
(812, 'account_mgmt.modal_title_add', 'ja', '口座追加', 'account_mgmt', '口座追加モーダルタイトル', datetime('now')),
(813, 'account_mgmt.modal_title_edit', 'en', 'Edit Account', 'account_mgmt', 'Edit account modal title', datetime('now')),
(814, 'account_mgmt.modal_title_edit', 'ja', '口座編集', 'account_mgmt', '口座編集モーダルタイトル', datetime('now')),
(815, 'transaction_mgmt.no_category', 'en', 'No Category', 'transaction_mgmt', 'No category selected', datetime('now')),
(816, 'transaction_mgmt.no_category', 'ja', '費目なし', 'transaction_mgmt', '費目未選択', datetime('now')),
(817, 'transaction_mgmt.modal_title_add', 'en', 'Add Transaction', 'transaction_mgmt', 'Add transaction modal title', datetime('now')),
(818, 'transaction_mgmt.modal_title_add', 'ja', '入出金追加', 'transaction_mgmt', '入出金追加モーダルタイトル', datetime('now')),
(819, 'transaction_mgmt.modal_title_edit', 'en', 'Edit Transaction', 'transaction_mgmt', 'Edit transaction modal title', datetime('now')),
(820, 'transaction_mgmt.modal_title_edit', 'ja', '入出金編集', 'transaction_mgmt', '入出金編集モーダルタイトル', datetime('now')),
(821, 'transaction_mgmt.main_category', 'en', 'Main Category', 'transaction_mgmt', 'Main category dropdown', datetime('now')),
(822, 'transaction_mgmt.main_category', 'ja', '大分類', 'transaction_mgmt', '大分類ドロップダウン', datetime('now')),
(823, 'transaction_mgmt.middle_category', 'en', 'Middle Category', 'transaction_mgmt', 'Middle category dropdown', datetime('now')),
(824, 'transaction_mgmt.middle_category', 'ja', '中分類', 'transaction_mgmt', '中分類ドロップダウン', datetime('now')),
(825, 'transaction_mgmt.sub_category', 'en', 'Sub Category', 'transaction_mgmt', 'Sub category dropdown', datetime('now')),
(826, 'transaction_mgmt.sub_category', 'ja', '小分類', 'transaction_mgmt', '小分類ドロップダウン', datetime('now')),
(827, 'transaction_mgmt.from', 'en', 'From', 'transaction_mgmt', 'From account label', datetime('now')),
(828, 'transaction_mgmt.from', 'ja', '出金元', 'transaction_mgmt', '出金元ラベル', datetime('now')),
(829, 'transaction_mgmt.to', 'en', 'To', 'transaction_mgmt', 'To account label', datetime('now')),
(830, 'transaction_mgmt.to', 'ja', '入金先', 'transaction_mgmt', '入金先ラベル', datetime('now')),
(831, 'transaction_mgmt.date', 'en', 'Date', 'transaction_mgmt', 'Date column header', datetime('now')),
(832, 'transaction_mgmt.date', 'ja', '日付', 'transaction_mgmt', '日付カラムヘッダー', datetime('now')),
(833, 'transaction_mgmt.manage_details', 'en', 'Manage Details', 'transaction_mgmt', 'Manage details button', datetime('now')),
(834, 'transaction_mgmt.manage_details', 'ja', '明細管理', 'transaction_mgmt', '明細管理ボタン', datetime('now')),
(835, 'transaction_mgmt.detail_items', 'en', 'items', 'transaction_mgmt', 'Detail items count label', datetime('now')),
(836, 'transaction_mgmt.detail_items', 'ja', '件', 'transaction_mgmt', '明細件数ラベル', datetime('now')),
(837, 'transaction_mgmt.filter_options', 'en', 'Filter Options', 'transaction_mgmt', 'Filter options title', datetime('now')),
(838, 'transaction_mgmt.filter_options', 'ja', 'フィルタオプション', 'transaction_mgmt', 'フィルタオプションタイトル', datetime('now')),
(839, 'transaction_mgmt.category', 'en', 'Category', 'transaction_mgmt', 'Category filter label', datetime('now')),
(840, 'transaction_mgmt.category', 'ja', '費目', 'transaction_mgmt', '費目フィルタラベル', datetime('now')),
(841, 'common.all', 'en', 'All', 'common', 'All option in filters', datetime('now')),
(842, 'common.all', 'ja', 'すべて', 'common', 'フィルタのすべてオプション', datetime('now')),
(843, 'transaction_mgmt.clear_filter', 'en', 'Clear', 'transaction_mgmt', 'Clear filter button', datetime('now')),
(844, 'transaction_mgmt.clear_filter', 'ja', 'クリア', 'transaction_mgmt', 'フィルタクリアボタン', datetime('now')),
(845, 'transaction_mgmt.apply_filter', 'en', 'Apply', 'transaction_mgmt', 'Apply filter button', datetime('now')),
(846, 'transaction_mgmt.apply_filter', 'ja', '適用', 'transaction_mgmt', 'フィルタ適用ボタン', datetime('now')),
(847, 'transaction_mgmt.min_placeholder', 'en', 'Min', 'transaction_mgmt', 'Minimum amount placeholder', datetime('now')),
(848, 'transaction_mgmt.min_placeholder', 'ja', '最小', 'transaction_mgmt', '最小金額プレースホルダー', datetime('now')),
(849, 'transaction_mgmt.max_placeholder', 'en', 'Max', 'transaction_mgmt', 'Maximum amount placeholder', datetime('now')),
(850, 'transaction_mgmt.max_placeholder', 'ja', '最大', 'transaction_mgmt', '最大金額プレースホルダー', datetime('now')),
(851, 'transaction_mgmt.search_placeholder', 'en', 'Search in memo', 'transaction_mgmt', 'Keyword search placeholder', datetime('now')),
(852, 'transaction_mgmt.search_placeholder', 'ja', 'メモを検索', 'transaction_mgmt', 'キーワード検索プレースホルダー', datetime('now')),

-- ============================================================================
-- Shop Management (shop_mgmt.*)
-- ============================================================================

-- Menu item
(921, 'menu.shop_management', 'en', 'Shop Management', 'menu', 'Shop management menu item', datetime('now')),
(922, 'menu.shop_management', 'ja', '店舗管理', 'menu', '店舗管理メニュー項目', datetime('now')),

-- Shop Management Screen
(923, 'shop_mgmt.title', 'en', 'Shop Management', 'shop_mgmt', 'Shop management page title', datetime('now')),
(924, 'shop_mgmt.title', 'ja', '店舗管理', 'shop_mgmt', '店舗管理ページタイトル', datetime('now')),
(925, 'shop_mgmt.add_new', 'en', 'Add New Shop', 'shop_mgmt', 'Add new shop button', datetime('now')),
(926, 'shop_mgmt.add_new', 'ja', '新規店舗追加', 'shop_mgmt', '新規店舗追加ボタン', datetime('now')),
(927, 'shop_mgmt.shop_name', 'en', 'Shop Name', 'shop_mgmt', 'Shop name label', datetime('now')),
(928, 'shop_mgmt.shop_name', 'ja', '店舗名', 'shop_mgmt', '店舗名ラベル', datetime('now')),
(929, 'shop_mgmt.memo', 'en', 'Memo', 'shop_mgmt', 'Memo label', datetime('now')),
(930, 'shop_mgmt.memo', 'ja', 'メモ', 'shop_mgmt', 'メモラベル', datetime('now')),
(931, 'shop_mgmt.modal_title_add', 'en', 'Add Shop', 'shop_mgmt', 'Add shop modal title', datetime('now')),
(932, 'shop_mgmt.modal_title_add', 'ja', '店舗追加', 'shop_mgmt', '店舗追加モーダルタイトル', datetime('now')),
(933, 'shop_mgmt.modal_title_edit', 'en', 'Edit Shop', 'shop_mgmt', 'Edit shop modal title', datetime('now')),
(934, 'shop_mgmt.modal_title_edit', 'ja', '店舗編集', 'shop_mgmt', '店舗編集モーダルタイトル', datetime('now')),
(935, 'shop_mgmt.no_shops', 'en', 'No shops found', 'shop_mgmt', 'No shops message', datetime('now')),
(936, 'shop_mgmt.no_shops', 'ja', '店舗が見つかりません', 'shop_mgmt', '店舗なしメッセージ', datetime('now')),
(937, 'shop_mgmt.delete_confirmation', 'en', 'Delete Confirmation', 'shop_mgmt', 'Delete confirmation title', datetime('now')),
(938, 'shop_mgmt.delete_confirmation', 'ja', '削除確認', 'shop_mgmt', '削除確認タイトル', datetime('now')),
(939, 'shop_mgmt.delete_message', 'en', 'Are you sure you want to delete this shop?', 'shop_mgmt', 'Delete confirmation message', datetime('now')),
(940, 'shop_mgmt.delete_message', 'ja', 'この店舗を削除してもよろしいですか？', 'shop_mgmt', '削除確認メッセージ', datetime('now')),
(941, 'shop_mgmt.duplicate_error', 'en', 'Shop name already exists', 'shop_mgmt', 'Duplicate shop name error', datetime('now')),
(942, 'shop_mgmt.duplicate_error', 'ja', 'この店舗名は既に登録されています', 'shop_mgmt', '重複店舗名エラー', datetime('now')),
(943, 'shop_mgmt.empty_name', 'en', 'Shop name is required', 'shop_mgmt', 'Empty shop name error', datetime('now')),
(944, 'shop_mgmt.empty_name', 'ja', '店舗名を入力してください', 'shop_mgmt', '空店舗名エラー', datetime('now')),
(945, 'shop_mgmt.failed_to_initialize', 'en', 'Failed to initialize', 'shop_mgmt', 'Initialization failure message', datetime('now')),
(946, 'shop_mgmt.failed_to_initialize', 'ja', '初期化に失敗しました', 'shop_mgmt', '初期化失敗メッセージ', datetime('now')),
(947, 'shop_mgmt.failed_to_load', 'en', 'Failed to load shops', 'shop_mgmt', 'Load failure message', datetime('now')),
(948, 'shop_mgmt.failed_to_load', 'ja', '店舗の読み込みに失敗しました', 'shop_mgmt', '読み込み失敗メッセージ', datetime('now')),
(949, 'shop_mgmt.failed_to_delete', 'en', 'Failed to delete shop', 'shop_mgmt', 'Delete failure message', datetime('now')),
(950, 'shop_mgmt.failed_to_delete', 'ja', '店舗の削除に失敗しました', 'shop_mgmt', '削除失敗メッセージ', datetime('now')),

-- ============================================================================
-- Common (IS_DISABLED feature)
-- ============================================================================

(907, 'common.show_disabled', 'en', 'Show Disabled Items', 'common', 'Show disabled items button', datetime('now')),
(908, 'common.show_disabled', 'ja', '非表示項目を表示', 'common', '非表示項目表示ボタン', datetime('now')),
(909, 'common.hide_disabled', 'en', 'Hide Disabled Items', 'common', 'Hide disabled items button', datetime('now')),
(910, 'common.hide_disabled', 'ja', '非表示項目を隠す', 'common', '非表示項目非表示ボタン', datetime('now')),
(911, 'common.is_disabled', 'en', 'Disabled', 'common', 'Disabled checkbox label', datetime('now')),
(912, 'common.is_disabled', 'ja', '非表示', 'common', '非表示チェックボックスラベル', datetime('now')),
(913, 'common.loading', 'en', 'Loading...', 'common', 'Loading message', datetime('now')),
(914, 'common.loading', 'ja', '読み込み中...', 'common', '読み込み中メッセージ', datetime('now')),
(915, 'common.cancel', 'en', 'Cancel', 'common', 'Cancel button', datetime('now')),
(916, 'common.cancel', 'ja', 'キャンセル', 'common', 'キャンセルボタン', datetime('now')),
(917, 'common.save', 'en', 'Save', 'common', 'Save button', datetime('now')),
(918, 'common.save', 'ja', '保存', 'common', '保存ボタン', datetime('now')),
(919, 'menu.back_to_main', 'en', '← Back to Main', 'menu', 'Back to main menu button', datetime('now')),
(920, 'menu.back_to_main', 'ja', '← メインに戻る', 'menu', 'メインメニューに戻るボタン', datetime('now'));

-- Additional menu items for Shop, Manufacturer, and Product management (added 2025-11-11)
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES (897, 'menu.shop_management', 'en', 'Shop Management', 'menu', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES (898, 'menu.shop_management', 'ja', '店舗管理', 'menu', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES (899, 'menu.manufacturer_management', 'en', 'Manufacturer Management', 'menu', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES (900, 'menu.manufacturer_management', 'ja', 'メーカー管理', 'menu', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES (901, 'menu.product_management', 'en', 'Product Management', 'menu', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES (902, 'menu.product_management', 'ja', '商品管理', 'menu', datetime('now'));

-- Error messages for Manufacturer and Product management (added 2025-11-11)
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (903, 'manufacturer_mgmt.duplicate_error', 'en', 'This manufacturer name already exists.', 'manufacturer_mgmt', 'Duplicate manufacturer name error', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (904, 'manufacturer_mgmt.duplicate_error', 'ja', 'このメーカー名は既に存在します。', 'manufacturer_mgmt', '重複メーカー名エラー', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (905, 'product_mgmt.duplicate_error', 'en', 'This product name already exists.', 'product_mgmt', 'Duplicate product name error', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (906, 'product_mgmt.duplicate_error', 'ja', 'この商品名は既に存在します。', 'product_mgmt', '重複商品名エラー', datetime('now'));
-- Missing i18n resources - to be added to database
-- Generated: 2024-11-21 JST

-- Common resources (9 keys = 18 records)
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1291, 'common.apply', 'en', 'Apply', 'common', 'Apply button', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1292, 'common.apply', 'ja', '適用', 'common', '適用ボタン', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1293, 'common.category', 'en', 'Category', 'common', 'Category label', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1294, 'common.category', 'ja', 'カテゴリ', 'common', 'カテゴリラベル', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1295, 'common.category2', 'en', 'Medium Category', 'common', 'Medium category label', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1296, 'common.category2', 'ja', '中分類', 'common', '中分類ラベル', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1297, 'common.category3', 'en', 'Minor Category', 'common', 'Minor category label', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1298, 'common.category3', 'ja', '小分類', 'common', '小分類ラベル', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1299, 'common.confirm', 'en', 'Confirm', 'common', 'Confirm button', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1300, 'common.confirm', 'ja', '確認', 'common', '確認ボタン', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1301, 'common.disabled_label', 'en', '(Disabled)', 'common', 'Disabled item label', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1302, 'common.disabled_label', 'ja', '（非表示）', 'common', '非表示項目ラベル', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1303, 'common.error', 'en', 'Error', 'common', 'Error title', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1304, 'common.error', 'ja', 'エラー', 'common', 'エラータイトル', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1305, 'common.no_data', 'en', 'No data', 'common', 'No data message', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1306, 'common.no_data', 'ja', 'データがありません', 'common', 'データなしメッセージ', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1307, 'common.select', 'en', 'Select', 'common', 'Select placeholder', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1308, 'common.select', 'ja', '選択してください', 'common', '選択プレースホルダー', datetime('now'));

-- Manufacturer Management (6 keys = 12 records)
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1309, 'manufacturer_mgmt.delete_manufacturer', 'en', 'Delete Manufacturer', 'manufacturer_mgmt', 'Delete manufacturer button', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1310, 'manufacturer_mgmt.delete_manufacturer', 'ja', 'メーカー削除', 'manufacturer_mgmt', 'メーカー削除ボタン', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1311, 'manufacturer_mgmt.empty_name', 'en', 'Manufacturer name is required', 'manufacturer_mgmt', 'Empty name error', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1312, 'manufacturer_mgmt.empty_name', 'ja', 'メーカー名を入力してください', 'manufacturer_mgmt', '空名前エラー', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1313, 'manufacturer_mgmt.failed_to_delete', 'en', 'Failed to delete manufacturer', 'manufacturer_mgmt', 'Delete failure message', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1314, 'manufacturer_mgmt.failed_to_delete', 'ja', 'メーカーの削除に失敗しました', 'manufacturer_mgmt', '削除失敗メッセージ', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1315, 'manufacturer_mgmt.failed_to_initialize', 'en', 'Failed to initialize', 'manufacturer_mgmt', 'Initialization failure message', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1316, 'manufacturer_mgmt.failed_to_initialize', 'ja', '初期化に失敗しました', 'manufacturer_mgmt', '初期化失敗メッセージ', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1317, 'manufacturer_mgmt.failed_to_load', 'en', 'Failed to load manufacturers', 'manufacturer_mgmt', 'Load failure message', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1318, 'manufacturer_mgmt.failed_to_load', 'ja', 'メーカーの読み込みに失敗しました', 'manufacturer_mgmt', '読み込み失敗メッセージ', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1319, 'manufacturer_mgmt.manufacturer_list', 'en', 'Manufacturer List', 'manufacturer_mgmt', 'Manufacturer list header', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1320, 'manufacturer_mgmt.manufacturer_list', 'ja', 'メーカー一覧', 'manufacturer_mgmt', 'メーカー一覧ヘッダー', datetime('now'));

-- Product Management (6 keys = 12 records)
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1321, 'product_mgmt.delete_product', 'en', 'Delete Product', 'product_mgmt', 'Delete product button', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1322, 'product_mgmt.delete_product', 'ja', '商品削除', 'product_mgmt', '商品削除ボタン', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1323, 'product_mgmt.empty_name', 'en', 'Product name is required', 'product_mgmt', 'Empty name error', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1324, 'product_mgmt.empty_name', 'ja', '商品名を入力してください', 'product_mgmt', '空名前エラー', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1325, 'product_mgmt.failed_to_delete', 'en', 'Failed to delete product', 'product_mgmt', 'Delete failure message', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1326, 'product_mgmt.failed_to_delete', 'ja', '商品の削除に失敗しました', 'product_mgmt', '削除失敗メッセージ', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1327, 'product_mgmt.failed_to_initialize', 'en', 'Failed to initialize', 'product_mgmt', 'Initialization failure message', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1328, 'product_mgmt.failed_to_initialize', 'ja', '初期化に失敗しました', 'product_mgmt', '初期化失敗メッセージ', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1329, 'product_mgmt.failed_to_load', 'en', 'Failed to load products', 'product_mgmt', 'Load failure message', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1330, 'product_mgmt.failed_to_load', 'ja', '商品の読み込みに失敗しました', 'product_mgmt', '読み込み失敗メッセージ', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1331, 'product_mgmt.product_list', 'en', 'Product List', 'product_mgmt', 'Product list header', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1332, 'product_mgmt.product_list', 'ja', '商品一覧', 'product_mgmt', '商品一覧ヘッダー', datetime('now'));

-- Shop Management (2 keys = 4 records)
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1333, 'shop_mgmt.delete_shop', 'en', 'Delete Shop', 'shop_mgmt', 'Delete shop button', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1334, 'shop_mgmt.delete_shop', 'ja', '店舗削除', 'shop_mgmt', '店舗削除ボタン', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1335, 'shop_mgmt.shop_list', 'en', 'Shop List', 'shop_mgmt', 'Shop list header', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1336, 'shop_mgmt.shop_list', 'ja', '店舗一覧', 'shop_mgmt', '店舗一覧ヘッダー', datetime('now'));

-- Transaction Management (5 keys = 10 records)
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1337, 'transaction_mgmt.delete_error', 'en', 'Failed to delete transaction', 'transaction_mgmt', 'Delete error message', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1338, 'transaction_mgmt.delete_error', 'ja', '入出金の削除に失敗しました', 'transaction_mgmt', '削除エラーメッセージ', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1339, 'transaction_mgmt.save_before_details', 'en', 'Please save the transaction before managing details', 'transaction_mgmt', 'Save before details message', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1340, 'transaction_mgmt.save_before_details', 'ja', '明細を管理する前に入出金を保存してください', 'transaction_mgmt', '明細前保存メッセージ', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1341, 'transaction_mgmt.tax_excluded', 'en', 'Tax Excluded', 'transaction_mgmt', 'Tax excluded option', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1342, 'transaction_mgmt.tax_excluded', 'ja', '外税', 'transaction_mgmt', '外税オプション', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1343, 'transaction_mgmt.tax_included', 'en', 'Tax Included', 'transaction_mgmt', 'Tax included option', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1344, 'transaction_mgmt.tax_included', 'ja', '内税', 'transaction_mgmt', '内税オプション', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1345, 'transaction_mgmt.tax_type', 'en', 'Tax Type', 'transaction_mgmt', 'Tax type label', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1346, 'transaction_mgmt.tax_type', 'ja', '税区分', 'transaction_mgmt', '税区分ラベル', datetime('now'));
-- Translation resources for aggregation
-- Auto-generated from database
-- Category: aggregation

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
(1055, 'aggregation.title', 'en', 'Monthly Aggregation', 'aggregation', 'Aggregation page title', datetime('now')),
(1056, 'aggregation.title', 'ja', '月次集計', 'aggregation', '集計ページタイトル', datetime('now')),
(1057, 'aggregation.filter', 'en', 'Filter', 'aggregation', 'Filter section title', datetime('now')),
(1058, 'aggregation.filter', 'ja', 'フィルタ', 'aggregation', 'フィルタセクションタイトル', datetime('now')),
(1059, 'aggregation.year', 'en', 'Year', 'aggregation', 'Year label', datetime('now')),
(1060, 'aggregation.year', 'ja', '年', 'aggregation', '年ラベル', datetime('now')),
(1061, 'aggregation.month', 'en', 'Month', 'aggregation', 'Month label', datetime('now')),
(1062, 'aggregation.month', 'ja', '月', 'aggregation', '月ラベル', datetime('now')),
(1063, 'aggregation.group_by', 'en', 'Group By', 'aggregation', 'Group by label', datetime('now')),
(1064, 'aggregation.group_by', 'ja', '集計単位', 'aggregation', '集計単位ラベル', datetime('now')),
(1065, 'aggregation.category1', 'en', 'Category (Major)', 'aggregation', 'Major category option', datetime('now')),
(1066, 'aggregation.category1', 'ja', '費目（大分類）', 'aggregation', '大分類オプション', datetime('now')),
(1067, 'aggregation.category2', 'en', 'Category (Middle)', 'aggregation', 'Middle category option', datetime('now')),
(1068, 'aggregation.category2', 'ja', '費目（中分類）', 'aggregation', '中分類オプション', datetime('now')),
(1069, 'aggregation.category3', 'en', 'Category (Minor)', 'aggregation', 'Minor category option', datetime('now')),
(1070, 'aggregation.category3', 'ja', '費目（小分類）', 'aggregation', '小分類オプション', datetime('now')),
(1071, 'aggregation.account', 'en', 'Account', 'aggregation', 'Account option', datetime('now')),
(1072, 'aggregation.account', 'ja', '口座', 'aggregation', '口座オプション', datetime('now')),
(1073, 'aggregation.shop', 'en', 'Shop', 'aggregation', 'Shop option', datetime('now')),
(1074, 'aggregation.shop', 'ja', '店舗', 'aggregation', '店舗オプション', datetime('now')),
(1075, 'aggregation.date', 'en', 'Date', 'aggregation', 'Date option', datetime('now')),
(1076, 'aggregation.date', 'ja', '日付', 'aggregation', '日付オプション', datetime('now')),
(1077, 'aggregation.execute', 'en', 'Execute', 'aggregation', 'Execute button', datetime('now')),
(1078, 'aggregation.execute', 'ja', '実行', 'aggregation', '実行ボタン', datetime('now')),
(1079, 'aggregation.results', 'en', 'Results', 'aggregation', 'Results section title', datetime('now')),
(1080, 'aggregation.results', 'ja', '集計結果', 'aggregation', '集計結果セクションタイトル', datetime('now')),
(1081, 'aggregation.group_name', 'en', 'Group', 'aggregation', 'Group name column header', datetime('now')),
(1082, 'aggregation.group_name', 'ja', '集計項目', 'aggregation', '集計項目カラムヘッダー', datetime('now')),
(1083, 'aggregation.total_amount', 'en', 'Net Amount (Income - Expense)', 'aggregation', 'Net amount column header', datetime('now')),
(1084, 'aggregation.total_amount', 'ja', '純増減（収入 - 支出）', 'aggregation', '純増減カラムヘッダー', datetime('now')),
(1085, 'aggregation.count', 'en', 'Count', 'aggregation', 'Count column header', datetime('now')),
(1086, 'aggregation.count', 'ja', '件数', 'aggregation', '件数カラムヘッダー', datetime('now')),
(1087, 'aggregation.avg_amount', 'en', 'Average Net Amount', 'aggregation', 'Average net amount column header', datetime('now')),
(1088, 'aggregation.avg_amount', 'ja', '平均純増減', 'aggregation', '平均純増減カラムヘッダー', datetime('now')),
(1089, 'aggregation.items', 'en', 'items', 'aggregation', 'Items count label', datetime('now')),
(1090, 'aggregation.items', 'ja', '件', 'aggregation', '件数ラベル', datetime('now')),
(1091, 'aggregation.total', 'en', 'Total', 'aggregation', 'Total row label', datetime('now')),
(1092, 'aggregation.total', 'ja', '合計', 'aggregation', '合計行ラベル', datetime('now')),
(1093, 'aggregation.no_results', 'en', 'No results found', 'aggregation', 'No results message', datetime('now')),
(1094, 'aggregation.no_results', 'ja', '集計結果がありません', 'aggregation', '結果なしメッセージ', datetime('now')),
(1095, 'aggregation.error_invalid_year', 'en', 'Please enter a valid year (1900-2100)', 'aggregation', 'Invalid year error', datetime('now')),
(1096, 'aggregation.error_invalid_year', 'ja', '有効な年を入力してください（1900-2100）', 'aggregation', '無効な年エラー', datetime('now')),
(1097, 'aggregation.error_invalid_month', 'en', 'Please select a valid month', 'aggregation', 'Invalid month error', datetime('now')),
(1098, 'aggregation.error_invalid_month', 'ja', '有効な月を選択してください', 'aggregation', '無効な月エラー', datetime('now')),
(1195, 'aggregation.title_daily', 'en', 'Daily Aggregation', 'aggregation', 'Daily aggregation page title', datetime('now')),
(1196, 'aggregation.title_daily', 'ja', '日次集計', 'aggregation', '日次集計ページタイトル', datetime('now')),
(1197, 'aggregation.error_no_date', 'en', 'Please select a date', 'aggregation', 'Error message for missing date', datetime('now')),
(1198, 'aggregation.error_no_date', 'ja', '日付を選択してください', 'aggregation', '日付未選択エラーメッセージ', datetime('now')),
(1199, 'aggregation.account_note', 'en', '※ Account aggregation includes transfers (account-to-account movements)', 'aggregation', 'Account aggregation note', datetime('now')),
(1200, 'aggregation.account_note', 'ja', '※ 口座別集計では振替（口座間移動）も含まれます', 'aggregation', '口座別集計注釈', datetime('now')),
(1203, 'aggregation.title_weekly', 'en', 'Weekly Aggregation', 'aggregation', 'Weekly aggregation page title', datetime('now')),
(1204, 'aggregation.title_weekly', 'ja', '週次集計', 'aggregation', '週次集計ページタイトル', datetime('now')),
(1205, 'aggregation.week', 'en', 'Week', 'aggregation', 'Week label', datetime('now')),
(1206, 'aggregation.week', 'ja', '週', 'aggregation', '週ラベル', datetime('now')),
(1207, 'aggregation.week_start', 'en', 'Week Start', 'aggregation', 'Week start label', datetime('now')),
(1208, 'aggregation.week_start', 'ja', '週の開始', 'aggregation', '週の開始ラベル', datetime('now')),
(1209, 'aggregation.sunday', 'en', 'Sunday', 'aggregation', 'Sunday option', datetime('now')),
(1210, 'aggregation.sunday', 'ja', '日曜', 'aggregation', '日曜オプション', datetime('now')),
(1211, 'aggregation.monday', 'en', 'Monday', 'aggregation', 'Monday option', datetime('now')),
(1212, 'aggregation.monday', 'ja', '月曜', 'aggregation', '月曜オプション', datetime('now')),
(1213, 'aggregation.error_invalid_week', 'en', 'Please enter a valid week (1-53)', 'aggregation', 'Invalid week error', datetime('now')),
(1214, 'aggregation.error_invalid_week', 'ja', '有効な週番号を入力してください（1-53）', 'aggregation', '無効な週エラー', datetime('now')),
(1215, 'aggregation.reference_date', 'en', 'Reference Date', 'aggregation', 'Reference date label for weekly aggregation', datetime('now')),
(1216, 'aggregation.reference_date', 'ja', '基準日', 'aggregation', '週次集計の基準日ラベル', datetime('now')),
(1219, 'aggregation.title_yearly', 'en', 'Yearly Aggregation', 'aggregation', 'Yearly aggregation page title', datetime('now')),
(1220, 'aggregation.title_yearly', 'ja', '年次集計', 'aggregation', '年次集計ページタイトル', datetime('now')),
(1221, 'aggregation.year_start', 'en', 'Year Start', 'aggregation', 'Year start label', datetime('now')),
(1222, 'aggregation.year_start', 'ja', '年度開始', 'aggregation', '年度開始ラベル', datetime('now')),
(1223, 'aggregation.january', 'en', 'January (Calendar Year)', 'aggregation', 'January/Calendar year option', datetime('now')),
(1224, 'aggregation.january', 'ja', '1月（暦年）', 'aggregation', '1月/暦年オプション', datetime('now')),
(1225, 'aggregation.april', 'en', 'April (Fiscal Year)', 'aggregation', 'April/Fiscal year option', datetime('now')),
(1226, 'aggregation.april', 'ja', '4月（会計年度）', 'aggregation', '4月/会計年度オプション', datetime('now')),
(1229, 'aggregation.title_period', 'en', 'Period Aggregation', 'aggregation', 'Period aggregation page title', datetime('now')),
(1230, 'aggregation.title_period', 'ja', '期間別集計', 'aggregation', '期間別集計ページタイトル', datetime('now')),
(1231, 'aggregation.start_date', 'en', 'Start Date', 'aggregation', 'Start date label', datetime('now')),
(1232, 'aggregation.start_date', 'ja', '開始日', 'aggregation', '開始日ラベル', datetime('now')),
(1233, 'aggregation.end_date', 'en', 'End Date', 'aggregation', 'End date label', datetime('now')),
(1234, 'aggregation.end_date', 'ja', '終了日', 'aggregation', '終了日ラベル', datetime('now')),
(1235, 'aggregation.error_no_dates', 'en', 'Please select start and end dates', 'aggregation', 'Missing dates error', datetime('now')),
(1236, 'aggregation.error_no_dates', 'ja', '開始日と終了日を選択してください', 'aggregation', '日付未選択エラー', datetime('now')),
(1237, 'aggregation.error_invalid_date_range', 'en', 'Start date must be before end date', 'aggregation', 'Invalid date range error', datetime('now')),
(1238, 'aggregation.error_invalid_date_range', 'ja', '開始日は終了日より前でなければなりません', 'aggregation', '無効な日付範囲エラー', datetime('now'));
-- Translation resources for detail_mgmt
-- Auto-generated from database
-- Category: detail_mgmt

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
(1239, 'detail_mgmt.title', 'en', 'Transaction Detail Management', 'detail_mgmt', 'Detail management page title', datetime('now')),
(1240, 'detail_mgmt.title', 'ja', '明細管理', 'detail_mgmt', '明細管理ページタイトル', datetime('now')),
(1241, 'detail_mgmt.transaction_date', 'en', 'Transaction Date', 'detail_mgmt', 'Transaction date label', datetime('now')),
(1242, 'detail_mgmt.transaction_date', 'ja', '取引日', 'detail_mgmt', '取引日ラベル', datetime('now')),
(1243, 'detail_mgmt.account', 'en', 'Account', 'detail_mgmt', 'Account label', datetime('now')),
(1244, 'detail_mgmt.account', 'ja', '口座', 'detail_mgmt', '口座ラベル', datetime('now')),
(1245, 'detail_mgmt.shop', 'en', 'Shop', 'detail_mgmt', 'Shop label', datetime('now')),
(1246, 'detail_mgmt.shop', 'ja', '店舗', 'detail_mgmt', '店舗ラベル', datetime('now')),
(1247, 'detail_mgmt.total_amount', 'en', 'Total Amount', 'detail_mgmt', 'Total amount label', datetime('now')),
(1248, 'detail_mgmt.total_amount', 'ja', '合計金額', 'detail_mgmt', '合計金額ラベル', datetime('now')),
(1249, 'detail_mgmt.add_detail', 'en', '+ Add Detail', 'detail_mgmt', 'Add detail button', datetime('now')),
(1250, 'detail_mgmt.add_detail', 'ja', '+ 明細追加', 'detail_mgmt', '明細追加ボタン', datetime('now')),
(1251, 'detail_mgmt.edit_detail', 'en', 'Edit Detail', 'detail_mgmt', 'Edit detail title', datetime('now')),
(1252, 'detail_mgmt.edit_detail', 'ja', '明細編集', 'detail_mgmt', '明細編集タイトル', datetime('now')),
(1253, 'detail_mgmt.item_name', 'en', 'Item Name', 'detail_mgmt', 'Item name label', datetime('now')),
(1254, 'detail_mgmt.item_name', 'ja', '品目名', 'detail_mgmt', '品目名ラベル', datetime('now')),
(1255, 'detail_mgmt.amount', 'en', 'Amount', 'detail_mgmt', 'Amount label', datetime('now')),
(1256, 'detail_mgmt.amount', 'ja', '金額', 'detail_mgmt', '金額ラベル', datetime('now')),
(1257, 'detail_mgmt.amount_excluding_tax', 'en', 'Amount (Excl. Tax)', 'detail_mgmt', 'Amount excluding tax label', datetime('now')),
(1258, 'detail_mgmt.amount_excluding_tax', 'ja', '税抜金額', 'detail_mgmt', '税抜金額ラベル', datetime('now')),
(1259, 'detail_mgmt.amount_including_tax', 'en', 'Amount (Incl. Tax)', 'detail_mgmt', 'Amount including tax label', datetime('now')),
(1260, 'detail_mgmt.amount_including_tax', 'ja', '税込金額', 'detail_mgmt', '税込金額ラベル', datetime('now')),
(1261, 'detail_mgmt.tax', 'en', 'Tax', 'detail_mgmt', 'Tax label', datetime('now')),
(1262, 'detail_mgmt.tax', 'ja', '税額', 'detail_mgmt', '税額ラベル', datetime('now')),
(1263, 'detail_mgmt.tax_rate', 'en', 'Tax Rate (%)', 'detail_mgmt', 'Tax rate label', datetime('now')),
(1264, 'detail_mgmt.tax_rate', 'ja', '税率 (%)', 'detail_mgmt', '税率ラベル', datetime('now')),
(1265, 'detail_mgmt.tax_amount', 'en', 'Tax Amount', 'detail_mgmt', 'Tax amount label', datetime('now')),
(1266, 'detail_mgmt.tax_amount', 'ja', '税額', 'detail_mgmt', '税額ラベル', datetime('now')),
(1267, 'detail_mgmt.memo', 'en', 'Memo', 'detail_mgmt', 'Memo label', datetime('now')),
(1268, 'detail_mgmt.memo', 'ja', 'メモ', 'detail_mgmt', 'メモラベル', datetime('now')),
(1269, 'detail_mgmt.delete_confirm_title', 'en', 'Delete Detail', 'detail_mgmt', 'Delete confirmation title', datetime('now')),
(1270, 'detail_mgmt.delete_confirm_title', 'ja', '明細削除', 'detail_mgmt', '削除確認タイトル', datetime('now')),
(1271, 'detail_mgmt.delete_confirm_message', 'en', 'Are you sure you want to delete this detail?', 'detail_mgmt', 'Delete confirmation message', datetime('now')),
(1272, 'detail_mgmt.delete_confirm_message', 'ja', 'この明細を削除してもよろしいですか？', 'detail_mgmt', '削除確認メッセージ', datetime('now')),
(1273, 'detail_mgmt.error_item_name_required', 'en', 'Item name is required', 'detail_mgmt', 'Item name required error', datetime('now')),
(1274, 'detail_mgmt.error_item_name_required', 'ja', '品目名を入力してください', 'detail_mgmt', '品目名必須エラー', datetime('now')),
(1275, 'detail_mgmt.error_category_required', 'en', 'Category is required', 'detail_mgmt', 'Category required error', datetime('now')),
(1276, 'detail_mgmt.error_category_required', 'ja', 'カテゴリを選択してください', 'detail_mgmt', 'カテゴリ必須エラー', datetime('now')),
(1277, 'detail_mgmt.error_invalid_amount', 'en', 'Please enter a valid amount', 'detail_mgmt', 'Invalid amount error', datetime('now')),
(1278, 'detail_mgmt.error_invalid_amount', 'ja', '有効な金額を入力してください', 'detail_mgmt', '無効な金額エラー', datetime('now')),
(1279, 'detail_mgmt.save_success', 'en', 'Detail saved successfully', 'detail_mgmt', 'Save success message', datetime('now')),
(1280, 'detail_mgmt.save_success', 'ja', '明細を保存しました', 'detail_mgmt', '保存成功メッセージ', datetime('now')),
(1281, 'detail_mgmt.save_error', 'en', 'Failed to save detail', 'detail_mgmt', 'Save error message', datetime('now')),
(1282, 'detail_mgmt.save_error', 'ja', '明細の保存に失敗しました', 'detail_mgmt', '保存エラーメッセージ', datetime('now')),
(1283, 'detail_mgmt.delete_success', 'en', 'Detail deleted successfully', 'detail_mgmt', 'Delete success message', datetime('now')),
(1284, 'detail_mgmt.delete_success', 'ja', '明細を削除しました', 'detail_mgmt', '削除成功メッセージ', datetime('now')),
(1285, 'detail_mgmt.delete_error', 'en', 'Failed to delete detail', 'detail_mgmt', 'Delete error message', datetime('now')),
(1286, 'detail_mgmt.delete_error', 'ja', '明細の削除に失敗しました', 'detail_mgmt', '削除エラーメッセージ', datetime('now')),
(1287, 'detail_mgmt.rounding_warning_title', 'en', 'Rounding Warning', 'detail_mgmt', 'Rounding warning title', datetime('now')),
(1288, 'detail_mgmt.rounding_warning_title', 'ja', '端数調整警告', 'detail_mgmt', '端数調整警告タイトル', datetime('now')),
(1289, 'detail_mgmt.rounding_warning_message', 'en', 'Tax calculation resulted in rounding. The total may not match exactly.', 'detail_mgmt', 'Rounding warning message', datetime('now')),
(1290, 'detail_mgmt.rounding_warning_message', 'ja', '税額計算で端数が発生しました。合計金額が一致しない可能性があります。', 'detail_mgmt', '端数調整警告メッセージ', datetime('now'));
-- Translation resources for language
-- Auto-generated from database
-- Category: language

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
(13, 'lang.english', 'en', 'English', 'language', 'English language option', datetime('now')),
(14, 'lang.english', 'ja', 'English', 'language', '英語オプション', datetime('now')),
(15, 'lang.japanese', 'en', '日本語 (Japanese)', 'language', 'Japanese language option', datetime('now')),
(16, 'lang.japanese', 'ja', '日本語', 'language', '日本語オプション', datetime('now')),
(17, 'lang.name.en', 'en', 'English', 'language', 'English language name', datetime('now')),
(18, 'lang.name.en', 'ja', '英語', 'language', '英語の名称', datetime('now')),
(19, 'lang.name.ja', 'en', 'Japanese', 'language', 'Japanese language name', datetime('now')),
(20, 'lang.name.ja', 'ja', '日本語', 'language', '日本語の名称', datetime('now'));
-- Translation resources for message
-- Auto-generated from database
-- Category: message

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
(21, 'msg.lang_changed', 'en', 'Language changed to {0}.', 'message', 'Language change confirmation', datetime('now')),
(22, 'msg.lang_changed', 'ja', '言語を{0}に変更しました。', 'message', '言語変更確認メッセージ', datetime('now')),
(23, 'msg.error', 'en', 'Error', 'message', 'Error message title', datetime('now')),
(24, 'msg.error', 'ja', 'エラー', 'message', 'エラーメッセージタイトル', datetime('now')),
(25, 'msg.success', 'en', 'Success', 'message', 'Success message title', datetime('now')),
(26, 'msg.success', 'ja', '成功', 'message', '成功メッセージタイトル', datetime('now')),
(27, 'msg.info', 'en', 'Information', 'message', 'Info message title', datetime('now')),
(28, 'msg.info', 'ja', '情報', 'message', '情報メッセージタイトル', datetime('now'));
-- Translation resources for transaction_modal
-- Auto-generated from database
-- Category: transaction_modal

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
(1113, 'transaction_modal.shop', 'en', 'Shop', 'transaction_modal', 'Shop label', datetime('now')),
(1114, 'transaction_modal.shop', 'ja', '店舗', 'transaction_modal', '店舗ラベル', datetime('now')),
(1115, 'transaction_modal.manage_shops', 'en', 'Manage Shops', 'transaction_modal', 'Manage shops button', datetime('now')),
(1116, 'transaction_modal.manage_shops', 'ja', '店舗管理', 'transaction_modal', '店舗管理ボタン', datetime('now'));

-- Additional menu items for admin submenu (added 2024-11-21)
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1347, 'menu.master_data', 'en', 'Master Data', 'menu', 'Master data submenu', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1348, 'menu.master_data', 'ja', 'マスタ管理', 'menu', 'マスタ管理サブメニュー', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1349, 'menu.reports', 'en', 'Reports', 'menu', 'Reports submenu', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1350, 'menu.reports', 'ja', '集計', 'menu', '集計サブメニュー', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1351, 'menu.aggregation_daily', 'en', 'Daily Aggregation', 'menu', 'Daily aggregation menu item', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1352, 'menu.aggregation_daily', 'ja', '日次集計', 'menu', '日次集計メニュー項目', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1353, 'menu.aggregation_weekly', 'en', 'Weekly Aggregation', 'menu', 'Weekly aggregation menu item', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1354, 'menu.aggregation_weekly', 'ja', '週次集計', 'menu', '週次集計メニュー項目', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1355, 'menu.aggregation', 'en', 'Monthly Aggregation', 'menu', 'Monthly aggregation menu item', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1356, 'menu.aggregation', 'ja', '月次集計', 'menu', '月次集計メニュー項目', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1357, 'menu.aggregation_yearly', 'en', 'Yearly Aggregation', 'menu', 'Yearly aggregation menu item', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1358, 'menu.aggregation_yearly', 'ja', '年次集計', 'menu', '年次集計メニュー項目', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1359, 'menu.aggregation_period', 'en', 'Period Aggregation', 'menu', 'Period aggregation menu item', datetime('now'));
INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1360, 'menu.aggregation_period', 'ja', '期間別集計', 'menu', '期間別集計メニュー項目', datetime('now'));
