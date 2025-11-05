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

-- SQL_10000013: Create TRANSACTIONS table (入出金データ)
CREATE TABLE IF NOT EXISTS TRANSACTIONS (
    TRANSACTION_ID INTEGER NOT NULL,
    USER_ID INTEGER NOT NULL,
    TRANSACTION_DATE DATE NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    CATEGORY2_CODE VARCHAR(64) NOT NULL,
    CATEGORY3_CODE VARCHAR(64) NOT NULL,
    AMOUNT INTEGER NOT NULL,
    DESCRIPTION VARCHAR(500),
    MEMO TEXT,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(TRANSACTION_ID AUTOINCREMENT),
    FOREIGN KEY(USER_ID) REFERENCES USERS(USER_ID) ON DELETE CASCADE,
    FOREIGN KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE) 
        REFERENCES CATEGORY3(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE)
);

-- SQL_10000014: Create indexes for TRANSACTIONS table
CREATE INDEX IF NOT EXISTS idx_transactions_user ON TRANSACTIONS(USER_ID);
CREATE INDEX IF NOT EXISTS idx_transactions_date ON TRANSACTIONS(USER_ID, TRANSACTION_DATE DESC);
CREATE INDEX IF NOT EXISTS idx_transactions_category1 ON TRANSACTIONS(USER_ID, CATEGORY1_CODE);
CREATE INDEX IF NOT EXISTS idx_transactions_category2 ON TRANSACTIONS(USER_ID, CATEGORY2_CODE);
CREATE INDEX IF NOT EXISTS idx_transactions_category3 ON TRANSACTIONS(USER_ID, CATEGORY3_CODE);
CREATE INDEX IF NOT EXISTS idx_transactions_amount ON TRANSACTIONS(USER_ID, AMOUNT);

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
(722, 'category_mgmt.error_load_category', 'ja', '費目データの読み込みに失敗しました', 'category_mgmt', '費目データ読み込みエラー', datetime('now'));

-- NOTE: Category data will be migrated from existing SQL later
