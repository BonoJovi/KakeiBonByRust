# 口座管理画面 実装ドキュメント

## 概要

本ドキュメントは、KakeiBonの口座管理画面の実装内容を記録します。

**実装期間**: 2025-11-05 ～ 2025-11-07  
**最終更新**: 2025-11-08 07:40 JST

---

## 機能概要

### 口座管理の目的
- ユーザーが使用する銀行口座、クレジットカード、電子マネーなどを管理
- 入出金データ登録時に口座を選択できるようにする
- テンプレートベースで口座を作成し、実際の口座名をカスタマイズ可能

### 口座データの構造

#### テーブル構成
1. **ACCOUNT_TEMPLATES**（マスターデータ）
   - 全ユーザー共通のテンプレート
   - TEMPLATE_CODE: NONE, CASH, BANK, CREDIT, EMONEY, OTHER
   - 編集不可

2. **ACCOUNTS**（ユーザーごとの口座）
   - `USER_ID` + `ACCOUNT_CODE` で管理
   - `TEMPLATE_CODE` でテンプレートを参照
   - ユーザーが追加・編集・削除可能

#### 「指定なし」口座の特別扱い
- **ACCOUNT_CODE**: `NONE`
- **目的**: イレギュラーな入出金に対応（口座を特定しない取引）
- **自動生成**: ユーザー作成時に自動追加
- **削除不可**: 削除ボタンがグレーアウト表示
- **編集可能**: 名前のカスタマイズは可能

---

## 実装内容

### 1. ユーザーフィルタリング機能

#### 要件
- **管理者（ROLE_ADMIN）**: すべてのユーザーの口座を表示
- **一般ユーザー（ROLE_USER）**: 自分の口座のみ表示

#### バックエンド実装
```rust
// src/lib.rs
#[tauri::command]
async fn get_accounts(
    user_id: i64,
    user_role: i64,
    state: tauri::State<'_, AppState>
) -> Result<Vec<services::account::Account>, String> {
    let db = state.db.lock().await;
    
    // Admin can see all accounts, regular users see only their own
    if user_role == crate::consts::ROLE_ADMIN {
        services::account::get_all_accounts(db.pool()).await
    } else {
        services::account::get_accounts(db.pool(), user_id).await
    }
}
```

#### フロントエンド実装
```javascript
// res/js/account-management.js
const currentUserId = 1;  // TODO: Get from session/auth
const currentUserRole = ROLE_ADMIN;  // TODO: Get from session/auth

async function loadAccounts() {
    accounts = await invoke('get_accounts', {
        userId: currentUserId,
        userRole: currentUserRole
    });
    // ...
}
```

**暫定対応**:
- `currentUserId` と `currentUserRole` は定数としてハードコード
- テスト時は手動で値を変更する必要あり
- セッション管理機能の実装後に改善予定

### 2. NONE口座の自動生成

#### バックエンド実装
```rust
// src/services/account.rs
pub async fn initialize_none_account(pool: &SqlitePool, user_id: i64) -> Result<(), String> {
    // Get NONE template
    let none_template = sqlx::query_as::<_, AccountTemplate>(
        "SELECT * FROM ACCOUNT_TEMPLATES WHERE TEMPLATE_CODE = 'NONE'"
    )
    .fetch_one(pool)
    .await?;
    
    // Create NONE account
    let request = AddAccountRequest {
        account_code: "NONE".to_string(),
        account_name: none_template.template_name_ja.clone(),
        template_code: "NONE".to_string(),
        initial_balance: 0,
    };
    
    add_account(pool, user_id, request).await
}
```

```rust
// src/services/auth.rs
impl AuthService {
    pub async fn create_general_user(&self, name: &str, password: &str) -> Result<(), AuthError> {
        // ...existing code...
        
        // Initialize NONE account for the new user
        crate::services::account::initialize_none_account(&self.pool, next_id).await?;
        
        Ok(())
    }
}
```

### 3. NONE口座の削除防止

#### フロントエンド実装
```javascript
// res/js/account-management.js
function createAccountRow(account) {
    // NONE account cannot be deleted
    const isNoneAccount = account.account_code === 'NONE';
    const deleteButtonHtml = isNoneAccount 
        ? '<button class="btn btn-danger" disabled style="opacity: 0.5; cursor: not-allowed;">Delete</button>'
        : `<button class="btn btn-danger delete-btn" data-code="${escapeHtml(account.account_code)}">Delete</button>`;
    
    // ...render row...
    
    // Delete button (only if not NONE account)
    if (!isNoneAccount) {
        row.querySelector('.delete-btn').addEventListener('click', () => {
            deleteAccount(account.account_code, account.account_name);
        });
    }
    
    return row;
}
```

### 4. 口座のCRUD操作

#### 追加（Add）
```javascript
await invoke('add_account', {
    userId: currentUserId,
    accountCode: accountCode,
    accountName: accountName,
    templateCode: templateCode,
    initialBalance: initialBalance
});
```

#### 更新（Update）
```javascript
await invoke('update_account', {
    userId: currentUserId,
    accountCode: accountCode,
    accountName: accountName,
    templateCode: templateCode,
    initialBalance: initialBalance,
    displayOrder: displayOrder
});
```

#### 削除（Delete）
```javascript
await invoke('delete_account', { 
    userId: currentUserId,
    accountCode: accountCode 
});
```

---

## ファイル構成

### フロントエンド
- `res/account-management.html`: 画面HTML
- `res/js/account-management.js`: ビジネスロジック
- `res/css/account-management.css`: スタイル定義（共通CSSを使用）

### バックエンド
- `src/services/account.rs`: 口座関連のビジネスロジック
- `src/services/auth.rs`: ユーザー作成時のNONE口座生成
- `src/lib.rs`: Tauriコマンド定義

### データベース
- `sql/create_accounts_table.sql`: テーブル定義
- `sql/fix_transactions_foreign_keys.sql`: 外部キー制約修正
- `sql/insert_default_accounts_for_existing_users.sql`: 既存ユーザーへのNONE口座追加

---

## 主要な関数

### バックエンド（Rust）
```rust
// 口座一覧取得（ユーザーフィルタ付き）
pub async fn get_accounts(pool: &SqlitePool, user_id: i64) -> Result<Vec<Account>, String>

// 全口座取得（管理者用）
pub async fn get_all_accounts(pool: &SqlitePool) -> Result<Vec<Account>, String>

// 口座追加
pub async fn add_account(pool: &SqlitePool, user_id: i64, request: AddAccountRequest) -> Result<String, String>

// 口座更新
pub async fn update_account(pool: &SqlitePool, user_id: i64, request: UpdateAccountRequest) -> Result<String, String>

// 口座削除
pub async fn delete_account(pool: &SqlitePool, user_id: i64, account_code: &str) -> Result<String, String>

// NONE口座初期化
pub async fn initialize_none_account(pool: &SqlitePool, user_id: i64) -> Result<(), String>

// テンプレート一覧取得
pub async fn get_account_templates(pool: &SqlitePool) -> Result<Vec<AccountTemplate>, String>
```

### フロントエンド（JavaScript）
```javascript
// 口座一覧読み込み
async function loadAccounts()

// テンプレート読み込み
async function loadTemplates()

// モーダル開閉
function openModal(mode, account = null)
function closeModal()

// 口座行の生成
function createAccountRow(account)

// 保存処理
async function saveAccount()

// 削除処理
async function deleteAccount(accountCode, accountName)
```

---

## データ構造

### AccountTemplate
```rust
pub struct AccountTemplate {
    pub template_id: i64,
    pub template_code: String,
    pub template_name_ja: String,
    pub template_name_en: String,
    pub display_order: Option<i64>,
    pub entry_dt: String,
}
```

### Account
```rust
pub struct Account {
    pub account_id: i64,
    pub user_id: i64,
    pub account_code: String,
    pub account_name: String,
    pub template_code: String,
    pub initial_balance: i64,
    pub display_order: Option<i64>,
    pub is_disabled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

### AddAccountRequest
```rust
pub struct AddAccountRequest {
    pub account_code: String,
    pub account_name: String,
    pub template_code: String,
    pub initial_balance: i64,
}
```

### UpdateAccountRequest
```rust
pub struct UpdateAccountRequest {
    pub account_code: String,
    pub account_name: String,
    pub template_code: String,
    pub initial_balance: i64,
    pub display_order: i64,
}
```

---

## テスト方法

### 管理者としてテスト
```javascript
// res/js/account-management.js
const currentUserId = 1;
const currentUserRole = ROLE_ADMIN;
```
**期待動作**: すべてのユーザーの口座が表示される

### 一般ユーザーとしてテスト
```javascript
// res/js/account-management.js
const currentUserId = 2;
const currentUserRole = ROLE_USER;
```
**期待動作**: USER_ID=2の口座のみ表示される

### NONE口座の削除防止テスト
1. 口座一覧で「指定なし」口座を確認
2. 削除ボタンがグレーアウトしていることを確認
3. クリックしても反応しないことを確認

### 新規ユーザー作成テスト
1. 管理者でログイン
2. ユーザー管理画面で新規ユーザーを作成
3. 新規ユーザーでログイン
4. 口座管理画面を開く
5. 「指定なし」口座が自動的に存在することを確認

---

## 既知の制限事項

### セッション管理未実装
- **現状**: `currentUserId` と `currentUserRole` をハードコード
- **影響**: テスト時に手動でコード変更が必要
- **対応予定**: セッション管理機能を実装し、自動的にログインユーザー情報を取得

### 口座コードの重複チェック
- **現状**: バックエンドでUNIQUE制約があるがフロントエンドでの事前チェックなし
- **影響**: 重複時にエラーダイアログが表示される
- **改善案**: リアルタイムバリデーションの追加

### テンプレート選択の制約
- **現状**: すべてのテンプレートが選択可能
- **改善案**: テンプレートごとの制約（例：NONEは1つのみ）を実装

---

## 将来の改善予定

### Phase 2: セッション管理
- localStorage/sessionStorageを使用したセッション管理
- ログイン状態の永続化
- 自動ログアウト機能

### Phase 3: バリデーション強化
- 口座コードの重複チェック（リアルタイム）
- 口座名の文字数制限チェック
- 初期残高の妥当性チェック

### Phase 4: UI/UX改善
- 口座のドラッグ＆ドロップでの並び替え
- 口座のグループ化機能（表示のみ）
- 口座の有効/無効切り替え機能

### Phase 5: インポート/エクスポート
- CSV形式での口座データのエクスポート
- CSV形式での口座データのインポート

---

## 関連ドキュメント

- [口座管理API仕様](./API_ACCOUNT_ja.md)
- [データベース設計](./DATABASE_CONFIGURATION.md)
- [トラブルシューティング](./TROUBLESHOOTING.md)

---

**最終更新**: 2025-11-08 07:40 JST

**作成者**: AI Assistant  
**監修**: Yoshihiro NAKAHARA (bonojovi@zundou.org)
