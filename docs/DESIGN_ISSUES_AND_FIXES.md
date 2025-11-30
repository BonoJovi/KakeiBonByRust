# KakeiBon Design Issues and Fixes

**作成日**: 2025-11-27
**分析者**: Claude (Sonnet 4.5)
**目的**: 論理的矛盾の特定と修正方針の文書化

---

## 📋 Executive Summary

KakeiBonByRustアプリケーションの設計を徹底的に検証した結果、**1つの重大な論理的矛盾**と**2つの軽微な懸念事項**が発見されました。

### 総合評価: **良好だが、セッション管理の統合が未完了**

- ✅ セキュリティ実装: 優秀 (Argon2id, 適切なバリデーション)
- ✅ データベース設計: 優秀 (正規化、制約、インデックス)
- ✅ 多言語化: 優秀 (一貫したi18nアーキテクチャ)
- ❌ セッション管理: **未完了** (実装済みだが使用されていない)
- ⚠️ ロール管理: 軽微な改善余地あり

---

## 🔴 Issue #1: セッション管理の不使用 (重大)

### 問題の詳細

**場所**: `src/lib.rs` のトランザクション管理コマンド群

**発見された箇所** (8箇所):
1. Line 1396: `get_transaction_header`
2. Line 1427: `select_transaction_headers`
3. Line 1456: `update_transaction_header`
4. Line 1485: `get_transaction_header_with_info`
5. Line 1498: `get_transaction_details`
6. Line 1520: `add_transaction_detail`
7. Line 1555: `update_transaction_detail`
8. Line 1581: `delete_transaction_detail`

**問題のコード例**:
```rust
#[tauri::command]
async fn get_transaction_header(
    transaction_id: i64,
    state: tauri::State<'_, AppState>
) -> Result<serde_json::Value, String> {
    let transaction = state.transaction.lock().await;
    // TODO: Get user_id from session/auth
    // For now, use user_id = 2 to match frontend currentUserId
    let user_id = 2;  // ← ハードコード！

    let (header, memo_text) = transaction.get_transaction_header_with_memo(user_id, transaction_id).await
        .map_err(|e| e.to_string())?;
    ...
}
```

### なぜこれが問題か

1. **セキュリティリスク**: 認証なしでデータにアクセス可能
2. **機能不全**: 複数ユーザーのログイン切り替えが正しく機能しない
3. **データ整合性**: 常に `user_id = 2` のデータを操作してしまう
4. **矛盾**: セッション管理機能が完全に実装されているのに使われていない

### セッション管理の実装状況

**✅ 実装済み**:
- `src/services/session.rs`: SessionState完全実装
- `src/lib.rs:154-228`: セッション管理用Tauriコマンド (9個)
- `src/lib.rs:60-88`: `login_user`でセッションに保存
- テストカバレッジ: 10個のテストケース

**❌ 未使用**:
- トランザクション管理コマンド群 (8箇所)
- その他のユーザー依存コマンド

### 影響範囲

#### 直接的な影響
- トランザクションデータ: 常に `user_id = 2` として扱われる
- ユーザー切り替え: 正しく機能しない
- 認証: バイパスされている

#### 間接的な影響
- データベース整合性: 他のユーザーのデータにアクセス可能
- セキュリティ: 認証機構が形骸化
- テスト: ユーザー依存のテストが不可能

---

## 🔧 Issue #1: 修正方針

### ステップ1: ヘルパー関数の作成

**ファイル**: `src/lib.rs`
**場所**: ファイル先頭 (use文の後、最初のTauriコマンドの前)

```rust
/// セッションから認証済みユーザーを取得
///
/// # Returns
/// * `Ok(User)` - セッションに保存されているユーザー情報
/// * `Err(String)` - 未認証の場合のエラーメッセージ
fn get_session_user(state: &tauri::State<'_, AppState>) -> Result<services::session::User, String> {
    state.session.get_user()
        .ok_or_else(|| "Not authenticated. Please login first.".to_string())
}
```

### ステップ2: 各コマンドの修正

**パターン**:

```rust
// ❌ 修正前
#[tauri::command]
async fn some_command(
    param1: Type1,
    state: tauri::State<'_, AppState>
) -> Result<ReturnType, String> {
    // TODO: Get user_id from session/auth
    // For now, use user_id = 2 to match frontend currentUserId
    let user_id = 2;

    // ... 処理 ...
}

// ✅ 修正後
#[tauri::command]
async fn some_command(
    param1: Type1,
    state: tauri::State<'_, AppState>
) -> Result<ReturnType, String> {
    let user = get_session_user(&state)?;  // セッションから取得

    // user.user_id を使用
    // ... 処理 ...
}
```

### ステップ3: 修正が必要な全8コマンド

#### 1. get_transaction_header (line 1389-1417)

**変更箇所**:
```rust
// Line 1394-1396 を削除
// TODO: Get user_id from session/auth
// For now, use user_id = 2 to match frontend currentUserId
let user_id = 2;

// 以下を追加
let user = get_session_user(&state)?;

// Line 1398 を変更
// 変更前: transaction.get_transaction_header_with_memo(user_id, transaction_id)
// 変更後: transaction.get_transaction_header_with_memo(user.user_id, transaction_id)
```

#### 2. select_transaction_headers (line 1420-1437)

**変更箇所**:
```rust
// Line 1425-1427 を削除
// TODO: Get user_id from session/auth
// For now, use user_id = 2 to match frontend currentUserId
let user_id = 2;

// 以下を追加
let user = get_session_user(&state)?;

// Line 1431 を変更
// 変更前: transaction.get_transaction_header(user_id, transaction_id)
// 変更後: transaction.get_transaction_header(user.user_id, transaction_id)
```

#### 3. update_transaction_header (line 1440-1472)

**変更箇所**:
```rust
// Line 1454-1456 を削除
// TODO: Get user_id from session/auth
// For now, use user_id = 2 to match frontend currentUserId
let user_id = 2;

// 以下を追加
let user = get_session_user(&state)?;

// Line 1470 を変更
// 変更前: transaction.update_transaction_header(user_id, transaction_id, request)
// 変更後: transaction.update_transaction_header(user.user_id, transaction_id, request)
```

#### 4. get_transaction_header_with_info (line 1479-1489)

**変更箇所**:
```rust
// Line 1484-1485 を削除
// TODO: Get user_id from session/auth
let user_id = 2;

// 以下を追加
let user = get_session_user(&state)?;

// Line 1487 を変更
// 変更前: transaction.get_transaction_header_with_info(user_id, transaction_id)
// 変更後: transaction.get_transaction_header_with_info(user.user_id, transaction_id)
```

#### 5. get_transaction_details (line 1492-1502)

**変更箇所**:
```rust
// Line 1497-1498 を削除
// TODO: Get user_id from session/auth
let user_id = 2;

// 以下を追加
let user = get_session_user(&state)?;

// Line 1500 を変更
// 変更前: transaction.get_transaction_details(user_id, transaction_id)
// 変更後: transaction.get_transaction_details(user.user_id, transaction_id)
```

#### 6. add_transaction_detail (line 1505-1537)

**変更箇所**:
```rust
// Line 1519-1520 を削除
// TODO: Get user_id from session/auth
let user_id = 2;

// 以下を追加
let user = get_session_user(&state)?;

// Line 1535 を変更
// 変更前: transaction.add_transaction_detail(user_id, transaction_id, request)
// 変更後: transaction.add_transaction_detail(user.user_id, transaction_id, request)
```

#### 7. update_transaction_detail (line 1540-1572)

**変更箇所**:
```rust
// Line 1554-1555 を削除
// TODO: Get user_id from session/auth
let user_id = 2;

// 以下を追加
let user = get_session_user(&state)?;

// Line 1570 を変更
// 変更前: transaction.update_transaction_detail(user_id, detail_id, request)
// 変更後: transaction.update_transaction_detail(user.user_id, detail_id, request)
```

#### 8. delete_transaction_detail (line 1575-1585)

**変更箇所**:
```rust
// Line 1580-1581 を削除
// TODO: Get user_id from session/auth
let user_id = 2;

// 以下を追加
let user = get_session_user(&state)?;

// Line 1583 を変更
// 変更前: transaction.delete_transaction_detail(user_id, detail_id)
// 変更後: transaction.delete_transaction_detail(user.user_id, detail_id)
```

### ステップ4: テストと検証

**確認項目**:
1. ✅ ヘルパー関数が追加されている
2. ✅ 全8箇所のTODOコメントが削除されている
3. ✅ 全8箇所で `user_id = 2` のハードコードが削除されている
4. ✅ 全8箇所で `get_session_user(&state)?` が使用されている
5. ✅ コンパイルエラーがない
6. ✅ 既存のテストが通る
7. ✅ ログイン前にコマンドを呼ぶと "Not authenticated" エラーが返る
8. ✅ ログイン後にコマンドが正しく動作する
9. ✅ 複数ユーザーの切り替えが正しく機能する

**テストシナリオ**:
```
1. アプリ起動
2. 管理者でログイン (user_id = 1)
3. トランザクション作成 → user_id = 1 で保存されることを確認
4. ログアウト
5. 一般ユーザーでログイン (user_id = 2)
6. トランザクション作成 → user_id = 2 で保存されることを確認
7. 管理者のトランザクションは見えないことを確認
8. ログアウト
9. ログインせずにトランザクション操作 → エラーになることを確認
```

---

## ⚠️ Issue #2: 管理者ユーザーIDのハードコード (軽微)

### 問題の詳細

**場所**:
- `src/lib.rs:113` (register_admin)
- `src/services/auth.rs:113` (register_admin_user)

**問題のコード**:
```rust
// src/lib.rs:113
sqlx::query(sql_queries::AUTH_INSERT_USER)
    .bind(1)  // USER_ID = 1 for admin ← ハードコード
    .bind(username)
    .bind(password_hash)
    .bind(ROLE_ADMIN)
    .bind(now)
    .execute(&mut *tx)
    .await?;
```

### なぜこれが問題か

1. **拡張性の制限**: 複数の管理者を作成できない
2. **設計の不整合**: 一般ユーザーは自動採番、管理者は固定
3. **将来のリスク**: 管理者機能の拡張時に制約となる

### 影響範囲

#### 現在の影響
- 管理者ユーザーは1人のみ (USER_ID = 1 固定)
- 2人目の管理者を作成しようとするとPRIMARY KEY違反

#### 将来の影響
- 複数管理者が必要になった場合、大規模な変更が必要
- テスト環境で複数の管理者を作成できない

---

## 🔧 Issue #2: 修正方針

### オプション1: 最小限の修正 (推奨)

**方針**: USER_ID = 1 のハードコードを自動採番に変更

**変更箇所**:

#### `src/lib.rs:91-105` (register_admin)

```rust
// ❌ 修正前
#[tauri::command]
async fn register_admin(
    username: String,
    password: String,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    validate_password(&password)?;

    let auth = state.auth.lock().await;

    match auth.register_admin_user(&username, &password).await {
        Ok(_) => Ok("Admin user registered successfully".to_string()),
        Err(e) => Err(format!("Registration failed: {}", e)),
    }
}

// ✅ 修正後 (変更なし - auth.rsの修正のみで対応)
```

#### `src/services/auth.rs:94-138` (register_admin_user)

```rust
// ❌ 修正前
pub async fn register_admin_user(&self, username: &str, password: &str) -> Result<(), AuthError> {
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let password_hash = hash_password(password)?;
    let mut tx = self.pool.begin().await?;

    sqlx::query(sql_queries::AUTH_INSERT_USER)
        .bind(1)  // USER_ID = 1 for admin ← これを削除
        .bind(username)
        .bind(password_hash)
        .bind(ROLE_ADMIN)
        .bind(now)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    // ... 以下省略
}

// ✅ 修正後
pub async fn register_admin_user(&self, username: &str, password: &str) -> Result<(), AuthError> {
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let password_hash = hash_password(password)?;

    // USER_IDを自動採番
    let result = sqlx::query(sql_queries::AUTH_GET_NEXT_USER_ID)
        .fetch_one(&self.pool)
        .await?;
    let next_id: i64 = result.get(0);

    let mut tx = self.pool.begin().await?;

    sqlx::query(sql_queries::AUTH_INSERT_USER)
        .bind(next_id)  // 自動採番されたIDを使用
        .bind(username)
        .bind(password_hash)
        .bind(ROLE_ADMIN)
        .bind(now)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    // 以降は next_id を使用
    let category_service = category::CategoryService::new(self.pool.clone());
    category_service.populate_default_categories(next_id).await
        .map_err(|e| AuthError::DatabaseError(sqlx::Error::Configuration(
            format!("Failed to populate default categories for admin: {}", e).into()
        )))?;

    crate::services::account::initialize_none_account(&self.pool, next_id).await
        .map_err(|e| AuthError::DatabaseError(sqlx::Error::Configuration(
            format!("Failed to initialize NONE account for admin: {}", e).into()
        )))?;

    Ok(())
}
```

### オプション2: 将来を見据えた修正 (発展的)

**方針**: 最初のユーザーを自動的に管理者にする

```rust
pub async fn register_first_user(&self, username: &str, password: &str) -> Result<(), AuthError> {
    // ユーザー数を確認
    let count: i64 = sqlx::query_scalar(sql_queries::AUTH_COUNT_USERS)
        .fetch_one(&self.pool)
        .await?;

    // 最初のユーザーは管理者、それ以外は一般ユーザー
    let role = if count == 0 { ROLE_ADMIN } else { ROLE_USER };

    // USER_IDは常に自動採番
    let next_id: i64 = sqlx::query_scalar(sql_queries::AUTH_GET_NEXT_USER_ID)
        .fetch_one(&self.pool)
        .await?;

    // ... 登録処理
}
```

**メリット**:
- 最初のユーザーが自動的に管理者になる
- 2人目以降の管理者は別の方法で昇格可能
- 柔軟性が高い

**デメリット**:
- 既存の動作との互換性確認が必要

### 推奨: オプション1

**理由**:
- 最小限の変更
- 既存の動作を維持
- 複数管理者の作成が可能になる
- リスクが低い

---

## ⚠️ Issue #3: ROLE_VISIT (999) の未使用 (軽微)

### 問題の詳細

**場所**: `src/consts.rs:4`

**定義**:
```rust
pub const ROLE_ADMIN: i64 = 0;
pub const ROLE_USER: i64 = 1;
pub const ROLE_VISIT: i64 = 999;  // ← どこでも使われていない
```

**検索結果**:
```bash
$ grep -r "ROLE_VISIT" src/ res/
src/consts.rs:pub const ROLE_VISIT: i64 = 999;
# → 定義以外に使用箇所なし
```

### なぜこれが問題か

1. **意図不明**: 将来の実装のための予約か、削除し忘れか不明
2. **保守性**: 使われていない定数はコードの理解を妨げる
3. **混乱**: 開発者が「これは何に使うのか?」と疑問を持つ

---

## 🔧 Issue #3: 修正方針

### オプション1: コメント付きで保持 (推奨)

**将来の実装予定がある場合**:

```rust
pub const ROLE_ADMIN: i64 = 0;
pub const ROLE_USER: i64 = 1;

// 将来の実装予定: 読み取り専用の訪問者ロール
// TODO: ゲストアクセス機能実装時に使用
pub const ROLE_VISIT: i64 = 999;
```

### オプション2: 削除 (シンプル)

**使用予定がない場合**:

```rust
pub const ROLE_ADMIN: i64 = 0;
pub const ROLE_USER: i64 = 1;
// ROLE_VISIT は削除
```

### 推奨: オプション1

**理由**:
- コメントで意図を明確化
- 将来の拡張性を保持
- リスクなし

---

## ✅ その他: 検証済みで問題なし

### 1. パスワードセキュリティ

**検証項目**:
- ✅ Argon2id使用 (業界標準)
- ✅ ランダムソルト生成
- ✅ 最小16文字の要求
- ✅ フロントエンド・バックエンドの一貫性

**結論**: 問題なし

### 2. データベーススキーマ

**検証項目**:
- ✅ 適切な正規化
- ✅ 外部キー制約
- ✅ インデックス最適化
- ✅ CASCADE動作

**結論**: 問題なし

### 3. 多言語化 (i18n)

**検証項目**:
- ✅ 一貫したリソース管理
- ✅ 言語切り替え機能
- ✅ カテゴリ名の多言語対応

**結論**: 問題なし

### 4. トランザクションスキーマ

**検証項目**:
- ✅ ヘッダー・ディテール分離
- ✅ マイグレーション実装
- ✅ テストカバレッジ

**結論**: 問題なし (軽微な懸念はあるが実用上は問題なし)

---

## 📊 優先度付き実装計画

### 🔴 Phase 1: 即時対応 (必須)

**目標**: セッション管理の統合

**タスク**:
1. ✅ `get_session_user()` ヘルパー関数を追加
2. ✅ 8つのトランザクション管理コマンドを修正
3. ✅ 全TODOコメントを削除
4. ✅ テストと検証

**所要時間**: 1-2時間
**リスク**: 低 (既存のセッション実装を使うだけ)

---

### 🟡 Phase 2: 計画的対応 (推奨)

**目標**: 管理者ユーザーの柔軟化

**タスク**:
1. ✅ `auth.rs:register_admin_user` を修正
2. ✅ USER_ID自動採番に変更
3. ✅ テストと検証

**所要時間**: 30分-1時間
**リスク**: 低 (既存のロジックとほぼ同じ)

---

### 🟢 Phase 3: オプション (任意)

**目標**: コードの明確化

**タスク**:
1. ✅ `ROLE_VISIT` にコメント追加
2. ✅ または削除

**所要時間**: 5分
**リスク**: なし

---

## 🧪 テスト計画

### Unit Tests

**新規テスト**:
```rust
#[cfg(test)]
mod session_tests {
    use super::*;

    #[test]
    fn test_get_session_user_not_authenticated() {
        // セッションが空の場合、エラーが返ることを確認
    }

    #[test]
    fn test_get_session_user_authenticated() {
        // セッションにユーザーが保存されている場合、正しく取得できることを確認
    }
}
```

### Integration Tests

**テストシナリオ**:
1. 複数ユーザーのログイン切り替え
2. 未認証でのAPI呼び出し
3. ユーザー毎のデータ分離

---

## 📝 実装チェックリスト

実装時に使用するチェックリスト:

### Phase 1: セッション管理統合

- [ ] `src/lib.rs` に `get_session_user()` 関数を追加
- [ ] `get_transaction_header` (line 1389) を修正
- [ ] `select_transaction_headers` (line 1420) を修正
- [ ] `update_transaction_header` (line 1440) を修正
- [ ] `get_transaction_header_with_info` (line 1479) を修正
- [ ] `get_transaction_details` (line 1492) を修正
- [ ] `add_transaction_detail` (line 1505) を修正
- [ ] `update_transaction_detail` (line 1540) を修正
- [ ] `delete_transaction_detail` (line 1575) を修正
- [ ] 全TODOコメントが削除されていることを確認
- [ ] `cargo build` が成功することを確認
- [ ] `cargo test` が成功することを確認
- [ ] 手動テスト: ログイン → トランザクション操作
- [ ] 手動テスト: 未ログイン → トランザクション操作 (エラー確認)
- [ ] 手動テスト: ユーザー切り替え → データ分離確認

### Phase 2: 管理者ユーザー柔軟化

- [ ] `src/services/auth.rs:register_admin_user` を修正
- [ ] `cargo build` が成功することを確認
- [ ] `cargo test` が成功することを確認
- [ ] 手動テスト: 管理者ユーザー作成 (複数回)

### Phase 3: コード明確化

- [ ] `src/consts.rs` の `ROLE_VISIT` にコメント追加 or 削除
- [ ] `cargo build` が成功することを確認

---

## 🎯 完了後の期待される状態

### セキュリティ
- ✅ 全てのトランザクション操作が認証必須
- ✅ ユーザー毎のデータ分離が保証される
- ✅ セッション管理が完全に機能する

### 機能性
- ✅ 複数ユーザーのログイン切り替えが正しく動作
- ✅ 複数の管理者ユーザーを作成可能
- ✅ データの整合性が保たれる

### 保守性
- ✅ TODOコメントが全て解消される
- ✅ コードの意図が明確になる
- ✅ 将来の拡張が容易になる

---

## 📚 参考情報

### 関連ファイル

**セッション管理**:
- `src/services/session.rs` - SessionState実装
- `src/lib.rs:154-228` - セッション管理Tauriコマンド
- `src/lib.rs:60-88` - login_user実装

**トランザクション管理**:
- `src/services/transaction.rs` - TransactionService実装
- `src/lib.rs:1357-1851` - トランザクション管理Tauriコマンド

**認証管理**:
- `src/services/auth.rs` - AuthService実装
- `src/security.rs` - パスワードハッシュ化

### コーディング規約

`.ai-context/CONVENTIONS.md` を参照:
- SQL queries centralization
- Constants externalization
- Error handling patterns

---

**最終更新**: 2025-11-27
**次回レビュー**: Phase 1実装完了後
