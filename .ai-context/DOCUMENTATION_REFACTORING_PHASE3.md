# Phase 3: API Documentation Structure Refactoring

**Created**: 2025-12-04 18:00 JST  
**Status**: Proposal

## 目的

API_REFERENCE.mdの位置づけを見直し、ドキュメント全体の構造を最適化する。

---

## 現状分析

### 現在のドキュメント構成

```
docs/developer/{en,ja}/api/
├── API_REFERENCE.md         # 統合リファレンス（Phase2で作成）
├── API_AUTH.md              # 認証API
├── API_USER.md              # ユーザー管理API
├── API_COMMON.md            # 共通API
├── API_CATEGORY.md          # 費目管理API
├── API_TRANSACTION.md       # 入出金管理API
├── API_AGGREGATION.md       # 集計API
├── API_ACCOUNT.md           # 口座管理API
├── API_SETTINGS.md          # 設定API
├── API_MASTER_DATA.md       # マスタデータAPI
├── API_MANUFACTURER.md      # メーカー管理API（旧）
├── API_PRODUCT.md           # 商品管理API（旧）
└── API_SHOP.md              # 店舗管理API（旧）
```

### API_REFERENCE.mdの現在の役割

- **Phase2で統合された6つのAPI**:
  - API_CATEGORY.md
  - API_TRANSACTION.md
  - API_AGGREGATION.md
  - API_SHOP.md
  - API_MANUFACTURER.md
  - API_PRODUCT.md

- **統合方針**: コード例を最小限にし、APIの構造とパラメータに焦点

### 問題点

1. **ドキュメントの重複**: API_REFERENCE.mdと個別ドキュメントで情報が重複
2. **メンテナンス負荷**: 同じ情報を複数箇所で更新する必要がある
3. **位置づけの不明確さ**: どちらを正とすべきか曖昧
4. **ナビゲーションの複雑さ**: ユーザーがどちらを見るべきか迷う

---

## Phase3の提案

### 1. API_REFERENCE.mdの新しい位置づけ

**提案**: API_REFERENCE.mdを**エントリーポイント兼インデックス**として再定義

#### 変更内容

1. **概要セクション**:
   - プロジェクト全体のAPI構成を説明
   - 各機能領域の簡潔な説明

2. **クイックリファレンス**:
   - 全APIの一覧表（機能別/画面別）
   - コマンド名とパラメータの要約

3. **詳細ドキュメントへのリンク**:
   - 各機能領域の詳細ドキュメントへの明確なリンク
   - 「詳細は〇〇を参照」という導線

4. **共通仕様**:
   - 全APIで共通する仕様（呼び出し方法、エラーハンドリング等）
   - データベーステーブル一覧

#### 構成案

```markdown
# API Reference

## 概要
全APIのエントリーポイント。詳細は各機能別ドキュメントを参照。

## クイックリファレンス
| 機能領域 | コマンド数 | 詳細ドキュメント |
|---------|----------|---------------|
| 認証 | 3 | [API_AUTH.md](./API_AUTH.md) |
| ユーザー管理 | 5 | [API_USER.md](./API_USER.md) |
| 費目管理 | 12 | [API_CATEGORY.md](./API_CATEGORY.md) |
| ... | ... | ... |

## 共通仕様
- 呼び出し方法
- 共通パラメータ
- エラーハンドリング
- データベーステーブル一覧

## 機能領域別API一覧

### 認証API
- `verify_login` - ログイン認証
- `register_initial_admin` - 初期管理者登録
- ...
詳細: [API_AUTH.md](./API_AUTH.md)

### ユーザー管理API
- `get_users` - ユーザー一覧取得
- ...
詳細: [API_USER.md](./API_USER.md)
```

---

### 2. 個別ドキュメントの役割

**変更なし**: 各APIの詳細仕様を記載（実装例、SQL、ユースケース等）

- API_AUTH.md
- API_USER.md
- API_COMMON.md
- API_CATEGORY.md
- API_TRANSACTION.md
- API_AGGREGATION.md
- API_ACCOUNT.md
- API_SETTINGS.md
- API_MASTER_DATA.md

---

### 3. 旧ドキュメントの扱い

**Phase2で統合された6つのドキュメント**:

#### オプション1: 削除（推奨）

**理由**:
- API_REFERENCE.mdが詳細ドキュメントへの導線を提供
- 個別ドキュメントに詳細が残る
- メンテナンス負荷を削減

**対象**:
- API_MANUFACTURER.md（削除候補）
- API_PRODUCT.md（削除候補）
- API_SHOP.md（削除候補）

**保持する情報**:
- API_MASTER_DATA.mdに統合済み
- API_REFERENCE.mdでインデックス化

#### オプション2: アーカイブ

**対象**: 削除が不安な場合
- `docs/developer/{en,ja}/api/archived/` に移動
- README.mdに「アーカイブ済み、API_REFERENCE.md参照」と記載

---

### 4. ドキュメント構成の最終形

```
docs/developer/{en,ja}/api/
├── API_REFERENCE.md         # エントリーポイント兼インデックス
├── API_AUTH.md              # 認証API詳細
├── API_USER.md              # ユーザー管理API詳細
├── API_COMMON.md            # 共通API詳細
├── API_CATEGORY.md          # 費目管理API詳細
├── API_TRANSACTION.md       # 入出金管理API詳細
├── API_AGGREGATION.md       # 集計API詳細
├── API_ACCOUNT.md           # 口座管理API詳細
├── API_SETTINGS.md          # 設定API詳細
└── API_MASTER_DATA.md       # マスタデータAPI詳細（店舗・メーカー・商品）
```

---

## メリット

1. **明確な導線**: API_REFERENCE.md → 詳細ドキュメント
2. **メンテナンス性向上**: 情報の重複を排除
3. **見つけやすさ向上**: エントリーポイントから全体を俯瞰
4. **段階的な学習**: 概要 → 詳細へスムーズに移行

---

## 実装ステップ

### Step 1: API_REFERENCE.md の再構成

1. **現在の詳細APIセクションを削除**:
   - Category Management API
   - Transaction Management API
   - Aggregation API
   - Shop Management API
   - Manufacturer Management API
   - Product Management API

2. **新しいセクションを追加**:
   - クイックリファレンス（全API一覧表）
   - 機能領域別API一覧（コマンド名と簡潔な説明のみ）
   - 詳細ドキュメントへのリンク

3. **共通仕様セクションを強化**:
   - 呼び出し方法
   - 共通パラメータ
   - エラーハンドリング
   - データベーステーブル一覧
   - テストカバレッジ

### Step 2: 旧ドキュメントの処理

- API_MANUFACTURER.md（削除 or アーカイブ）
- API_PRODUCT.md（削除 or アーカイブ）
- API_SHOP.md（削除 or アーカイブ）

### Step 3: TODO.mdの更新

- Phase3完了をマーク
- 削除/アーカイブしたドキュメントを記録

### Step 4: コミット＆プッシュ

- Phase3完了のコミット

---

## ドキュメント作成ガイドラインへの追加事項

なし（Phase3は既存ドキュメントの再構成のみ）

---

## 評価プロセス

### Phase3完了後の確認事項

1. API_REFERENCE.mdがエントリーポイントとして機能しているか
2. 個別ドキュメントへの導線が明確か
3. 情報の重複が排除されているか
4. 旧ドキュメントが適切に処理されているか

---

**Next Step**: 開発者承認後、Step 1から実装開始
