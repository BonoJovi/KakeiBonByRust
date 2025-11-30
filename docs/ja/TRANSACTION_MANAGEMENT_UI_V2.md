# 入出金管理画面 実装ドキュメント (V2)

## 概要

本ドキュメントは、KakeiBonの入出金管理機能の最新実装内容を記録します。

**実装期間**: 2025-11-05 ～ 2025-11-08
**最終更新**: 2025-11-12 22:30 JST
**実装状況**: Phase 1-3 完了（新スキーマ対応、一覧表示、登録機能、多言語化）、明細テーブル正規化完了

---

## アーキテクチャ変更

### スキーマ変更（V2設計）

従来の単一テーブル構造から、ヘッダー・明細分離構造に変更。

#### TRANSACTIONS_HEADER（入出金ヘッダー）
```sql
CREATE TABLE TRANSACTIONS_HEADER (
    TRANSACTION_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    USER_ID INTEGER NOT NULL,
    TRANSACTION_DATE DATETIME NOT NULL,  -- DATE → DATETIME (時分対応)
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    FROM_ACCOUNT_CODE VARCHAR(50) NOT NULL,
    TO_ACCOUNT_CODE VARCHAR(50) NOT NULL,
    TOTAL_AMOUNT INTEGER NOT NULL,
    TAX_ROUNDING_TYPE INTEGER NOT NULL,  -- 0:切り捨て, 1:四捨五入, 2:切り上げ
    MEMO_ID INTEGER,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    FOREIGN KEY(USER_ID) REFERENCES USERS(USER_ID),
    FOREIGN KEY(USER_ID, FROM_ACCOUNT_CODE) REFERENCES ACCOUNTS(USER_ID, ACCOUNT_CODE),
    FOREIGN KEY(USER_ID, TO_ACCOUNT_CODE) REFERENCES ACCOUNTS(USER_ID, ACCOUNT_CODE),
    FOREIGN KEY(MEMO_ID) REFERENCES MEMOS(MEMO_ID)
);
```

#### TRANSACTIONS_DETAIL（入出金明細）
```sql
CREATE TABLE TRANSACTIONS_DETAIL (
    DETAIL_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    TRANSACTION_ID INTEGER NOT NULL,
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(50) NOT NULL,
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
    FOREIGN KEY (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE) REFERENCES CATEGORY2(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE),
    FOREIGN KEY (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE) REFERENCES CATEGORY3(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE),
    FOREIGN KEY (MEMO_ID) REFERENCES MEMOS(MEMO_ID),
    CHECK (ITEM_NAME != '')
);
```

### 主な変更点

1. **日付型変更**: DATE → DATETIME（時分まで入力可能）
2. **税率の移動**: HEADER → DETAIL（混合税率対応）
3. **口座情報**: FROM_ACCOUNT_CODE / TO_ACCOUNT_CODE 追加
4. **メモ分離**: MEMO_ID参照でMEMOSテーブルに外部化
5. **明細テーブルの正規化**（2025-11-12追加）:
   - USER_ID, CATEGORY1_CODEフィールドを追加
   - CATEGORY2/CATEGORY3への複合外部キー制約を設定
   - データ整合性の向上とユーザー単位のカテゴリ管理に対応

---

## 実装済み機能

### Phase 1: データベース・バックエンドAPI ✅

#### バックエンドAPI（Rust）
実装場所: `src/services/transaction.rs`

| API | 説明 | 状態 |
|-----|------|------|
| `save_transaction_header` | ヘッダー保存 | ✅ 実装済み |
| `get_transactions` | 一覧取得 | ✅ 実装済み |
| `delete_transaction` | 削除 | ✅ 実装済み |

**注意**: 明細（DETAIL）管理APIは未実装

#### データ検証
- 日時形式: `YYYY-MM-DD HH:MM:SS` (19文字)
- 金額: 必須、整数値
- 税額端数処理: 0-2の範囲

---

### Phase 2: 入出金登録機能 ✅

#### 登録モーダル

**表示項目**:
- 取引日時（datetime-local input）
- 費目（大分類）- SELECTボックス
- 出金元口座 - SELECTボックス
- 入金先口座 - SELECTボックス
- 合計金額
- 税額端数処理（切り捨て/四捨五入/切り上げ）
- メモ（textarea）
- 明細管理ボタン（未実装）

**動作**:
1. 費目選択: 動的ロード（CATEGORY1テーブルから取得）
2. 口座選択: ユーザーの口座のみ表示（currentUserIdでフィルタ）
3. 日時初期値: OS現地時間、時刻は00:00
4. バリデーション: 必須項目チェック
5. 保存: `save_transaction_header` API呼び出し

**制限事項**:
- 明細機能は未実装（現在はヘッダーのみ）
- 編集機能は未実装

---

### Phase 3: 一覧表示 ✅

#### 表示項目
- 取引日時（YYYY-MM-DD HH:MM形式）
- 費目名（CATEGORY1_NAME）
- 口座情報（FROM_ACCOUNT_NAME → TO_ACCOUNT_NAME）
- 合計金額（カテゴリ別色分け）
  - 支出: 赤色
  - 収入: 緑色
  - 振替: 青色
- メモ（最大20文字、超過時「...」、ホバーで全文表示）
- 操作ボタン（編集✏️、削除🗑️）

#### データ取得
複数テーブルのJOIN:
```sql
SELECT 
    t.*, 
    c1.CATEGORY1_NAME,
    a1.ACCOUNT_NAME as FROM_ACCOUNT_NAME,
    a2.ACCOUNT_NAME as TO_ACCOUNT_NAME,
    m.MEMO_TEXT
FROM TRANSACTIONS_HEADER t
LEFT JOIN CATEGORY1 c1 ON ...
LEFT JOIN ACCOUNTS a1 ON ...
LEFT JOIN ACCOUNTS a2 ON ...
LEFT JOIN MEMOS m ON ...
```

#### ページネーション
- 50件/ページ
- 総件数表示
- 前/次ページボタン

---

### Phase 4: フィルター機能 ✅

#### フィルター項目
- 期間: 開始日～終了日
- 費目: カテゴリ1/2/3（連動SELECTボックス）
- 金額範囲: 最小～最大
- キーワード: メモ検索

**注意**: 
- カテゴリ2/3、キーワード検索は現在無効化（明細テーブル未対応）
- 実装済みフィルター: 期間、カテゴリ1、金額範囲

---

### Phase 5: 多言語化 ✅

#### 翻訳リソース（34件）

**ボタン・ラベル**:
- `transaction_mgmt.add_new`: 新規入出金追加
- `transaction_mgmt.filter`: フィルタ
- `transaction_mgmt.total`: 合計
- `transaction_mgmt.items`: 件
- `transaction_mgmt.page`: ページ

**モーダル**:
- `transaction_mgmt.select_category`: - 費目を選択 -
- `transaction_mgmt.manage_details`: 明細管理
- `transaction_mgmt.delete_confirm`: 削除確認メッセージ

**フィルター**:
- `transaction_mgmt.filter_options`: フィルタオプション
- `transaction_mgmt.clear_filter`: クリア
- `transaction_mgmt.apply_filter`: 適用
- `transaction_mgmt.min_placeholder`: 最小
- `transaction_mgmt.max_placeholder`: 最大
- `transaction_mgmt.search_placeholder`: メモを検索

**共通**:
- `common.all`: すべて
- `common.unspecified`: 指定なし

#### バグ修正
JavaScriptで動的にSELECTボックスを更新する際、`innerHTML`で上書きしていた問題を修正。
`data-i18n`属性を持つデフォルトoptionを保持するように変更。

---

## ファイル構成

### フロントエンド
- `res/transaction-management.html`: 画面HTML
- `res/js/transaction-management.js`: ビジネスロジック
- `res/css/transaction-management.css`: スタイル定義

### バックエンド
- `src/services/transaction.rs`: トランザクションロジック
- `src/sql_queries.rs`: SQLクエリ定義
- `src/lib.rs`: Tauriコマンド登録

### データベース
- `sql/create_transactions_header_table.sql`: ヘッダーテーブル作成
- `sql/create_transactions_detail_table.sql`: 明細テーブル作成
- `sql/migrate_transaction_date_to_datetime.sql`: 日付型マイグレーション

---

## テスト状況

### 動作確認済み ✅
1. ✅ 入出金登録（ヘッダーのみ）
2. ✅ 一覧表示（3件登録済み）
3. ✅ 口座名・カテゴリ名表示
4. ✅ メモ表示（20文字制限、ツールチップ）
5. ✅ 削除機能
6. ✅ 多言語切り替え（日本語・英語）
7. ✅ ユーザーフィルタリング（USER_ID=2で動作確認）

### 未テスト項目
- ページネーション（50件超のデータ）
- フィルター機能（期間、カテゴリ1、金額範囲）
- レスポンシブデザイン（狭い画面）

---

## 既知の問題・制限事項

### 暫定対応
1. **セッション管理未実装**: 
   - `currentUserId`, `currentUserRole`を定数で管理
   - テスト時は手動で変更が必要

2. **明細機能未実装**:
   - TRANSACTIONS_DETAILテーブルへのCRUD未実装
   - 「明細管理」ボタンは表示のみ

3. **編集機能未実装**:
   - 編集ボタンは表示されるが機能しない

4. **カテゴリ2/3フィルター無効**:
   - 明細テーブルとのJOIN未実装

5. **キーワード検索無効**:
   - メモ検索の実装が不完全

---

## 今後の実装予定

### 優先度: 高
1. **明細管理機能**:
   - TRANSACTIONS_DETAIL CRUD実装
   - 明細入力モーダル作成
   - 税率・金額の明細単位管理

2. **編集機能**:
   - 既存トランザクション編集
   - ヘッダー・明細の同時更新

### 優先度: 中
3. **フィルター完全実装**:
   - カテゴリ2/3フィルター（明細JOIN）
   - キーワード検索（メモ全文検索）

4. **セッション管理**:
   - ログインユーザー情報の永続化
   - currentUserId/currentUserRoleの動的取得

### 優先度: 低
5. **UI/UX改善**:
   - ローディングインジケーター
   - エラーメッセージの詳細化
   - 操作フィードバック強化

---

## 変更履歴

### 2025-11-12
- TRANSACTIONS_DETAILテーブルの正規化完了
- USER_ID, CATEGORY1_CODEフィールドを追加
- CATEGORY2/CATEGORY3への複合外部キー制約を設定
- 自動マイグレーション機能実装（既存データの保全）
- テストケース追加（151テスト全て成功）

### 2025-11-08
- 多言語化完了（34リソース追加）
- カテゴリ選択バグ修正（innerHTML問題）
- 口座名・メモ表示実装
- ドキュメント更新

### 2025-11-07
- 口座管理画面実装完了
- NONE口座自動生成
- ユーザーフィルタリング実装

### 2025-11-06
- 入出金登録機能実装
- 日時入力対応（datetime-local）
- 税率設計変更（ヘッダー→明細）

### 2025-11-05
- 新スキーマ設計（V2）
- データベーステーブル作成
- 一覧表示基本実装

---

## 参考資料

- [TRANSACTION_DESIGN_V2.md](./TRANSACTION_DESIGN_V2.md) - V2設計詳細
- [ACCOUNT_MANAGEMENT_UI.md](./ACCOUNT_MANAGEMENT_UI.md) - 口座管理画面
- [I18N_IMPLEMENTATION.md](./I18N_IMPLEMENTATION.md) - 多言語化実装

---

**作成者**: AI Assistant  
**監修**: Yoshihiro NAKAHARA (bonojovi@zundou.org)
