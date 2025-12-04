# 口座管理API リファレンス

**最終更新**: 2025-12-05 02:05 JST

## 概要

本ドキュメントは、口座管理画面（account-management.html）で使用されるAPIの仕様を定義します。ユーザーが複数の口座（現金、銀行口座等）を管理する機能を提供します。

---

## 目次

1. [口座テンプレートAPI](#口座テンプレートapi)
2. [口座管理API](#口座管理api)
3. [データ構造](#データ構造)

---

## 口座テンプレートAPI

### get_account_templates

口座テンプレートの一覧を取得します。

**パラメータ:** なし

**戻り値:**
- `Vec<AccountTemplate>`: テンプレートの配列

**AccountTemplate構造:**
```javascript
{
    template_id: number,
    template_code: string,      // "CASH", "BANK", "CREDIT_CARD"など
    template_name_ja: string,   // "現金", "銀行口座", "クレジットカード"
    template_name_en: string,   // "Cash", "Bank Account", "Credit Card"
    display_order: number,
    entry_dt: string
}
```

**使用例:**
```javascript
const templates = await invoke('get_account_templates');

// テンプレート選択用のプルダウンを作成
templates.forEach(template => {
    const option = document.createElement('option');
    option.value = template.template_code;
    option.textContent = template.template_name_ja;
    selectElement.appendChild(option);
});
```

**用途:**
- 口座作成時のテンプレート選択
- テンプレートに応じた表示アイコンの決定

**注意:**
- テンプレートは固定データ（ユーザーによる追加・編集不可）
- ACCOUNT_TEMPLATESテーブルから取得

---

## 口座管理API

### get_accounts

ログイン中のユーザーの口座一覧を取得します。

**パラメータ:** なし

**戻り値:**
- `Vec<Account>`: 口座の配列

**Account構造:**
```javascript
{
    account_id: number,
    user_id: number,
    account_code: string,       // "CASH", "MIZUHO_001"など
    account_name: string,       // "現金", "みずほ銀行普通預金"
    template_code: string,      // "CASH", "BANK"など
    initial_balance: number,    // 初期残高
    display_order: number,
    is_disabled: number,        // 0=有効, 1=無効
    entry_dt: string,
    update_dt: string | null
}
```

**使用例:**
```javascript
const accounts = await invoke('get_accounts');

accounts.forEach(account => {
    console.log(`${account.account_name}: ${account.initial_balance}円`);
});
```

**ロール別動作:**
- **管理者（role=0）**: すべてのユーザーの口座を取得
- **一般ユーザー（role=1）**: 自分の口座のみ取得

**フィルタ:**
- `IS_DISABLED = 0`のみ取得（論理削除済みは除外）

**ソート:**
- `DISPLAY_ORDER`, `ACCOUNT_CODE`の順

---

### add_account

新しい口座を追加します。

**パラメータ:**
- `account_code` (String): 口座コード（大文字に自動変換）
- `account_name` (String): 口座名
- `template_code` (String): テンプレートコード
- `initial_balance` (i64): 初期残高

**戻り値:**
- `String`: "Account added successfully"

**使用例:**
```javascript
try {
    await invoke('add_account', {
        accountCode: 'MIZUHO_001',
        accountName: 'みずほ銀行普通預金',
        templateCode: 'BANK',
        initialBalance: 100000
    });
    
    alert('口座を追加しました');
    await loadAccounts();
} catch (error) {
    alert(`追加失敗: ${error}`);
}
```

**自動処理:**
1. **コードの正規化**: 小文字→大文字、トリム処理
2. **表示順の自動設定**: 最大値+1
3. **is_disabled**: 0（有効）に設定

**バリデーション:**
- 口座コードの重複チェック（同一ユーザー内）
- 口座名は必須

**エラー:**
- `"Account code 'XXX' already exists"`: コードが重複
- `"Failed to add account: ..."`: データベースエラー

---

### update_account

口座情報を更新します。

**パラメータ:**
- `account_code` (String): 口座コード（変更不可、識別用）
- `account_name` (String): 新しい口座名
- `template_code` (String): 新しいテンプレートコード
- `initial_balance` (i64): 新しい初期残高
- `display_order` (i64): 新しい表示順

**戻り値:**
- `String`: "Account updated successfully"

**使用例:**
```javascript
await invoke('update_account', {
    accountCode: 'MIZUHO_001',
    accountName: 'みずほ銀行普通預金（更新）',
    templateCode: 'BANK',
    initialBalance: 150000,
    displayOrder: 2
});
```

**注意:**
- `account_code`は変更できません（主キー）
- 他のフィールドはすべて更新可能

**バリデーション:**
- 自分自身を除いて口座コード重複チェック

---

### delete_account

口座を論理削除します。

**パラメータ:**
- `account_code` (String): 口座コード

**戻り値:**
- `String`: "Account deleted successfully"

**使用例:**
```javascript
if (confirm('この口座を削除してもよろしいですか？')) {
    try {
        await invoke('delete_account', { accountCode: 'MIZUHO_001' });
        alert('口座を削除しました');
        await loadAccounts();
    } catch (error) {
        alert(`削除失敗: ${error}`);
    }
}
```

**動作:**
- 論理削除（`IS_DISABLED = 1`）
- 物理削除ではありません

**注意:**
- トランザクションで使用中の口座も削除可能
- 削除後もトランザクションの履歴は保持される

---

## データ構造

### AccountTemplate

```rust
pub struct AccountTemplate {
    pub template_id: i64,
    pub template_code: String,        // "CASH", "BANK", "CREDIT_CARD"...
    pub template_name_ja: String,     // 日本語名
    pub template_name_en: String,     // 英語名
    pub display_order: i64,
    pub entry_dt: String,
}
```

**標準テンプレート（例）:**
- CASH: 現金 / Cash
- BANK: 銀行口座 / Bank Account
- CREDIT_CARD: クレジットカード / Credit Card
- E_MONEY: 電子マネー / Electronic Money
- SECURITIES: 証券口座 / Securities Account

---

### Account

```rust
pub struct Account {
    pub account_id: i64,
    pub user_id: i64,
    pub account_code: String,       // 大文字（自動正規化）
    pub account_name: String,
    pub template_code: String,      // テンプレート参照
    pub initial_balance: i64,       // 初期残高
    pub display_order: i64,
    pub is_disabled: i64,           // 0=有効, 1=無効
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

**account_codeの命名規則（推奨）:**
- CASH: 現金
- {銀行名}_{番号}: 例: MIZUHO_001, UFJ_002
- CREDIT_{カード名}: 例: CREDIT_RAKUTEN

---

### AddAccountRequest

```rust
pub struct AddAccountRequest {
    pub account_code: String,
    pub account_name: String,
    pub template_code: String,
    pub initial_balance: i64,
}
```

---

### UpdateAccountRequest

```rust
pub struct UpdateAccountRequest {
    pub account_code: String,       // 識別用（変更不可）
    pub account_name: String,
    pub template_code: String,
    pub initial_balance: i64,
    pub display_order: i64,
}
```

---

## エラーハンドリング

### 共通エラーパターン

| エラーメッセージ | 原因 | 対処方法 |
|----------------|------|---------|
| `"User not authenticated"` | セッション未認証 | ログインが必要 |
| `"Account code 'XXX' already exists"` | コードが重複 | 別のコードを使用 |
| `"Failed to get account templates: ..."` | DB接続エラー | データベース確認 |
| `"Failed to add account: ..."` | 追加エラー | データベース確認 |
| `"Failed to update account: ..."` | 更新エラー | データベース確認 |
| `"Failed to delete account: ..."` | 削除エラー | データベース確認 |

### フロントエンドでのエラーハンドリング例

```javascript
// 口座追加
async function addAccount(code, name, templateCode, initialBalance) {
    try {
        await invoke('add_account', {
            accountCode: code,
            accountName: name,
            templateCode,
            initialBalance
        });
        
        alert('口座を追加しました');
        return true;
    } catch (error) {
        if (error.includes('already exists')) {
            alert('この口座コードは既に使用されています');
        } else {
            alert(`エラー: ${error}`);
        }
        return false;
    }
}

// 口座更新
async function updateAccount(code, name, templateCode, initialBalance, displayOrder) {
    try {
        await invoke('update_account', {
            accountCode: code,
            accountName: name,
            templateCode,
            initialBalance,
            displayOrder
        });
        
        alert('口座を更新しました');
        return true;
    } catch (error) {
        alert(`更新エラー: ${error}`);
        return false;
    }
}
```

---

## 口座コードの正規化

### normalize_account_code関数

口座コードは自動的に正規化されます：

```rust
fn normalize_account_code(code: &str) -> String {
    code.trim().to_uppercase()
}
```

**例:**
- 入力: `"mizuho_001"` → 保存: `"MIZUHO_001"`
- 入力: `" cash "` → 保存: `"CASH"`
- 入力: `"Ufj_002"` → 保存: `"UFJ_002"`

**理由:**
- コードの一貫性を保証
- 重複チェックの精度向上
- 大文字小文字による重複を防止

---

## 使用例：口座管理画面の実装

### 口座一覧表示

```javascript
async function loadAccounts() {
    try {
        const accounts = await invoke('get_accounts');
        const templates = await invoke('get_account_templates');
        
        // テンプレートコードから名前を取得するマップ
        const templateMap = new Map();
        templates.forEach(t => {
            templateMap.set(t.template_code, t.template_name_ja);
        });
        
        const tbody = document.getElementById('account-table-body');
        tbody.innerHTML = '';
        
        accounts.forEach(account => {
            const row = document.createElement('tr');
            row.innerHTML = `
                <td>${account.account_code}</td>
                <td>${account.account_name}</td>
                <td>${templateMap.get(account.template_code) || account.template_code}</td>
                <td>${account.initial_balance.toLocaleString()}円</td>
                <td>
                    <button onclick="editAccount('${account.account_code}')">編集</button>
                    <button onclick="deleteAccount('${account.account_code}')">削除</button>
                </td>
            `;
            tbody.appendChild(row);
        });
    } catch (error) {
        console.error('口座一覧の読み込みエラー:', error);
    }
}
```

### 口座追加フォーム

```javascript
async function handleAddAccount(event) {
    event.preventDefault();
    
    const code = document.getElementById('account-code').value;
    const name = document.getElementById('account-name').value;
    const templateCode = document.getElementById('template-select').value;
    const balance = parseInt(document.getElementById('initial-balance').value);
    
    try {
        await invoke('add_account', {
            accountCode: code,
            accountName: name,
            templateCode,
            initialBalance: balance
        });
        
        alert('口座を追加しました');
        event.target.reset();
        await loadAccounts();
    } catch (error) {
        alert(`エラー: ${error}`);
    }
}
```

---

## セキュリティ考慮事項

### ユーザー分離

1. **セッションユーザーID**: 各APIで自動取得
2. **データ分離**: 一般ユーザーは自分の口座のみアクセス可能
3. **管理者権限**: 管理者はすべてのユーザーの口座を閲覧可能

### コードの一意性

1. **ユーザー単位**: 同一ユーザー内でコード重複不可
2. **異なるユーザー**: 同じコードを使用可能
3. **大文字正規化**: 大文字小文字の違いによる重複を防止

### 論理削除

1. **データ保持**: 削除後もデータは保持
2. **トランザクション整合性**: 過去のトランザクションは保護
3. **再有効化**: 必要に応じてIS_DISABLED=0で復活可能

---

## テストカバレッジ

**AccountService:**
- ✅ テンプレート取得テスト
- ✅ 口座一覧取得テスト（ユーザー別）
- ✅ 口座追加テスト
- ✅ 口座更新テスト
- ✅ 口座削除テスト
- ✅ コード重複チェック
- ✅ コード正規化テスト

---

## 関連ドキュメント

### 実装ファイル

- 口座サービス: `src/services/account.rs`
- SQL定義: `src/sql_queries.rs`
- Tauri Commands: `src/lib.rs`

### その他のAPIリファレンス

- [共通API](./API_COMMON.md) - セッション管理
- [入出金管理API](./API_TRANSACTION.md) - 口座の利用

---

**変更履歴:**
- 2025-12-05: 初版作成（実装コードに基づく）
