# セキュリティ設計仕様書

**最終更新**: 2024-12-05 03:52 JST  
**対象バージョン**: 開発中

---

## 目次

1. [概要](#概要)
2. [パスワード管理](#パスワード管理)
3. [データ暗号化](#データ暗号化)
4. [認証・認可](#認証認可)
5. [セキュリティ境界](#セキュリティ境界)
6. [脅威モデル](#脅威モデル)

---

## 概要

KakeiBonは、個人の財務データを扱うデスクトップアプリケーションとして、以下のセキュリティ原則に基づいて設計されています：

### セキュリティ原則

1. **データ保護**: 機密データは暗号化して保存
2. **最小権限**: ユーザーロールに応じたアクセス制御
3. **防御の深層化**: 複数層でのセキュリティ対策
4. **透明性**: セキュリティ実装の可監査性

### セキュリティスコープ

**保護対象**:
- ユーザーパスワード
- メモフィールド（ENCRYPTED_FIELDS管理）
- セッション情報

**対象外**:
- ネットワーク通信（デスクトップアプリのため）
- 外部API連携（現時点で未実装）

---

## パスワード管理

### ハッシュアルゴリズム: Argon2id

**選定理由**:
- OWASP推奨の最新パスワードハッシュアルゴリズム
- GPU/ASIC攻撃に対する耐性
- サイドチャネル攻撃への耐性

**実装**: `src/security.rs`

```rust
// Argon2idでパスワードをハッシュ化
pub fn hash_password(password: &str) -> Result<String, SecurityError>

// ハッシュ値と照合
pub fn verify_password(password: &str, hash: &str) -> Result<bool, SecurityError>
```

**パラメータ**:
- デフォルト設定（Argon2::default()）
- ランダムソルト（OsRng）
- PHC形式で保存

**保存場所**: `USERS.PASSWORD_HASH`（TEXT型、PHC形式）

### パスワードポリシー

**最小要件**（`src/validation.rs`で実装）:
```rust
pub const MIN_PASSWORD_LENGTH: usize = 16;
```

**検証ルール**:
- 長さ: 最低16文字
- 文字種: 制限なし（Unicode対応）
- 強度チェック: クライアント側で推奨

---

## データ暗号化

### 暗号化アルゴリズム: AES-256-GCM

**選定理由**:
- 認証付き暗号化（AEAD）
- データ改ざん検出機能
- 高速かつ安全

**実装**: `src/crypto.rs`

```rust
pub struct Crypto {
    cipher: Aes256Gcm,
}

// 暗号化（Base64エンコード）
pub fn encrypt(&self, plaintext: &str) -> Result<String, CryptoError>

// 復号化
pub fn decrypt(&self, ciphertext: &str) -> Result<String, CryptoError>
```

**データ形式**:
```
Base64(nonce[12bytes] || ciphertext || auth_tag[16bytes])
```

### 暗号化キー管理

**キー派生**: Argon2id（パスワードベース）

```rust
pub fn derive_key(password: &str, salt: &str) -> Result<[u8; 32], SecurityError>
```

**キーパラメータ**:
- 出力長: 32バイト（256ビット）
- ソルト: ユーザー名（一意性保証）
- パラメータ: Argon2::default()

**キーライフサイクル**:
1. ユーザーログイン時にパスワードから派生
2. メモリ上で保持（Tauri State管理）
3. ログアウト時に破棄

### 暗号化対象データ

**ENCRYPTED_FIELDS テーブル**:
```sql
CREATE TABLE ENCRYPTED_FIELDS (
    TABLE_NAME TEXT NOT NULL,
    RECORD_ID INTEGER NOT NULL,
    FIELD_NAME TEXT NOT NULL,
    ENCRYPTED_VALUE TEXT NOT NULL,
    CREATED_AT TEXT DEFAULT (datetime('now', 'localtime')),
    UPDATED_AT TEXT DEFAULT (datetime('now', 'localtime')),
    PRIMARY KEY (TABLE_NAME, RECORD_ID, FIELD_NAME)
);
```

**用途**:
- SHOPS.MEMO（店舗メモ）
- MANUFACTURERS.MEMO（メーカーメモ）
- PRODUCTS.MEMO（商品メモ）
- TRANSACTIONS_DETAIL.MEMO（取引明細メモ）

**暗号化フロー**:
1. ユーザーがメモを入力
2. フロントエンドがTauri APIを呼び出し
3. バックエンドが暗号化してENCRYPTED_FIELDSに保存
4. 元のテーブルにはプレースホルダーまたはNULL

**復号化フロー**:
1. データ取得時にENCRYPTED_FIELDSを結合
2. バックエンドで復号化
3. 平文をフロントエンドに返却

---

## 認証・認可

### ユーザーロール

**ROLE定数** (`src/consts.rs`):
```rust
pub const ROLE_ADMIN: i64 = 0;  // 管理者
pub const ROLE_USER: i64 = 1;   // 一般ユーザー
```

**権限マトリクス**:

| 機能 | Admin | User |
|------|-------|------|
| ユーザー管理 | ✅ | ❌ |
| 自分のパスワード変更 | ✅ | ✅ |
| データ入力・閲覧 | ✅ | ✅ |
| マスタデータ管理 | ✅ | ✅ |
| 集計・レポート | ✅ | ✅ |

### セッション管理

**実装方法**: Tauri State（メモリ内管理）

**セッション情報**:
```rust
struct SessionState {
    user_id: Option<i64>,
    username: Option<String>,
    role: Option<i64>,
    encryption_key: Option<[u8; 32]>,
}
```

**ライフサイクル**:
1. ログイン成功時に初期化
2. アプリケーション実行中は保持
3. ログアウト/アプリ終了時にクリア

**セキュリティ特性**:
- ディスクに保存されない（メモリのみ）
- プロセス終了で自動破棄
- 他プロセスからアクセス不可

### 認可チェック

**実装パターン**（各Tauri Commandで実施）:

```rust
#[tauri::command]
async fn admin_only_operation(state: State<'_, SessionState>) -> Result<(), String> {
    let role = state.role.lock().unwrap();
    if *role != Some(ROLE_ADMIN) {
        return Err("Admin権限が必要です".to_string());
    }
    // 処理続行
}
```

**適用箇所**:
- `create_user`, `update_user`, `delete_user` - Admin専用
- その他のAPI - ログイン済みチェックのみ

---

## セキュリティ境界

### アーキテクチャ層

```
┌─────────────────────────────────────┐
│   Frontend (HTML/JS)                │  ← ユーザー入力検証
├─────────────────────────────────────┤
│   Tauri IPC Layer                   │  ← 型安全なAPI境界
├─────────────────────────────────────┤
│   Backend (Rust)                    │  ← 認証・認可・暗号化
│   - src/security.rs                 │
│   - src/crypto.rs                   │
│   - src/validation.rs               │
├─────────────────────────────────────┤
│   Database (SQLite)                 │  ← 暗号化データ保存
└─────────────────────────────────────┘
```

### 信頼境界

**非信頼領域**:
- フロントエンド（JS）- ユーザー入力を受け付け

**信頼境界**:
- Tauri IPC - 型チェック、シリアライゼーション検証

**信頼領域**:
- Rustバックエンド - すべてのセキュリティロジック
- データベース - 暗号化データの永続化

### データフロー

**書き込み（暗号化）**:
```
User Input → Frontend Validation → Tauri IPC → Backend Validation
  → Password Hashing/Data Encryption → Database
```

**読み取り（復号化）**:
```
Database → Backend (Decryption) → Tauri IPC → Frontend Display
```

---

## 脅威モデル

### 対策済みの脅威

#### 1. パスワード漏洩
**脅威**: データベースファイルが盗まれた場合
**対策**: Argon2idハッシュ化（逆算不可能）
**残存リスク**: 弱いパスワードは辞書攻撃に脆弱

#### 2. 機密データ漏洩
**脅威**: データベースファイルが盗まれた場合
**対策**: AES-256-GCM暗号化（ユーザーパスワード必要）
**残存リスク**: パスワードが判明すれば復号可能

#### 3. データ改ざん
**脅威**: 暗号化データが改ざんされる
**対策**: AES-GCMの認証タグで検出
**残存リスク**: 改ざん検出のみ（防止ではない）

#### 4. SQLインジェクション
**脅威**: 悪意ある入力でDB操作
**対策**: sqlxのプリペアドステートメント
**残存リスク**: ほぼゼロ

#### 5. 権限昇格
**脅威**: 一般ユーザーがAdmin権限取得
**対策**: バックエンドでのロールチェック
**残存リスク**: 実装バグ以外ではゼロ

### 想定される脅威（対策未実装）

#### 1. メモリダンプ攻撃
**脅威**: 実行中プロセスのメモリを読み取られる
**現状**: 平文の暗号化キーがメモリに存在
**対策案**: 将来的にHSMやTPM連携を検討

#### 2. ローカル特権昇格
**脅威**: OS管理者がプロセスメモリを読み取る
**現状**: 防御不可能（デスクトップアプリの限界）
**対策案**: 信頼できる環境での実行を前提

#### 3. タイミング攻撃
**脅威**: パスワード検証の時間差で推測
**現状**: Argon2の定数時間比較で軽減済み
**残存リスク**: 微小（実用上問題なし）

### セキュリティ前提条件

1. **信頼できる実行環境**: ユーザーのデバイスは悪意のあるソフトウェアに感染していない
2. **物理的セキュリティ**: デバイスへの物理アクセスは制限されている
3. **OS保護**: OSレベルのアクセス制御が有効
4. **強力なパスワード**: ユーザーが十分に強力なパスワードを設定

---

## 監査とテスト

### セキュリティテスト

**実装済み** (`src/security.rs`, `src/crypto.rs`):
- パスワードハッシュ化テスト
- パスワード検証テスト
- 暗号化/復号化テスト
- エラーケーステスト

**テスト実行**:
```bash
cargo test
```

### 監査ログ

**現状**: 未実装

**将来的な実装案**:
- ログインイベント
- ユーザー管理操作
- データ暗号化/復号化操作
- 権限チェック失敗

---

## 関連ドキュメント

- [API仕様書（共通）](../api/API_COMMON.md) - 認証・暗号化API
- [API仕様書（ユーザー管理）](../api/API_USER.md) - ユーザーCRUD API
- [アーキテクチャ設計](./ARCHITECTURE.md) - システム全体構造
- [データベース設計](./DATABASE_DESIGN.md) - ENCRYPTED_FIELDSテーブル詳細

---

**ドキュメント作成ガイドライン評価**:
- ✅ 最新コードとの突合（security.rs, crypto.rs確認済み）
- ✅ コード引用最小化（必要な関数シグネチャのみ）
- ✅ 日本語版作成
- ⏳ 英語版は次ステップ
- ✅ ガイドライン遵守確認完了
