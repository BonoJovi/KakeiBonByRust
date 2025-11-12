# 入出金管理機能 設計仕様書 v2.0（ヘッダ・明細構造）

## 概要

本ドキュメントは、KakeiBonの入出金管理機能のデータベース設計とUI/UX設計を記録します。

**作成日**: 2025-11-07 00:36 JST  
**設計方針**: ヘッダ・明細構造（正規化版）  
**状態**: 設計完了、実装前

---

## 設計方針

### なぜヘッダ・明細構造にするのか

#### 背景
- 買い物では複数商品を一度に購入することが多い
- 「何を買ったか」を後から確認したいニーズがある（特に主婦・主夫）
- 商品ごとの価格履歴を追跡したい（栄養価の高い商品を安く買いたい）
- レシート単位での管理が自然

#### フラット構造との比較

| 観点 | フラット構造 | ヘッダ・明細構造 |
|------|-------------|----------------|
| シンプルさ | ⭕ シンプル | ❌ 複雑 |
| 買い物の表現 | ❌ 同じ日付・店名で複数レコード | ⭕ 1ヘッダ・複数明細 |
| 商品別履歴 | ❌ 困難 | ⭕ 容易 |
| 一括編集 | ❌ 困難 | ⭕ 容易 |
| 将来の拡張性 | ❌ 低い | ⭕ 高い |

**結論**: ユーザビリティと将来性を重視し、ヘッダ・明細構造を採用

---

## データベース設計

### 1. 口座マスタ（ACCOUNTS）

```sql
CREATE TABLE ACCOUNTS (
    ACCOUNT_ID INTEGER PRIMARY KEY,
    USER_ID INTEGER NOT NULL,
    ACCOUNT_CODE VARCHAR(50) NOT NULL,      -- ユーザー内で一意
    ACCOUNT_NAME TEXT NOT NULL,             -- 例: "財布", "三菱UFJ銀行"
    ACCOUNT_TYPE VARCHAR(20),               -- 現金/銀行/クレジット等（任意）
    INITIAL_BALANCE INTEGER DEFAULT 0,      -- アプリ開始時点残高
    DISPLAY_ORDER INTEGER,
    IS_DISABLED INTEGER DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    FOREIGN KEY (USER_ID) REFERENCES USERS(USER_ID),
    UNIQUE(USER_ID, ACCOUNT_CODE)
);

CREATE INDEX idx_accounts_user ON ACCOUNTS(USER_ID, ACCOUNT_CODE);
```

#### 設計のポイント
- **1つのマスタテーブル**: FROM口座とTO口座を別マスタにせず、1つのテーブルで管理
  - コード実装量が少ない
  - 汎用性が高い
  - 管理が楽
- **INITIAL_BALANCE**: ユーザーが手動で設定（口座作成時）
- **論理削除**: IS_DISABLED=1で無効化（物理削除はしない）

### 2. ヘッダテーブル（TRANSACTION_HEADERS）

```sql
CREATE TABLE TRANSACTION_HEADERS (
    HEADER_ID INTEGER PRIMARY KEY,
    USER_ID INTEGER NOT NULL,
    TRANSACTION_DATE DATE NOT NULL,
    CATEGORY1_CODE VARCHAR(50) NOT NULL,    -- 大分類（支出/収入/振替）
    FROM_ACCOUNT_CODE VARCHAR(50),          -- 出金元口座コード
    TO_ACCOUNT_CODE VARCHAR(50),            -- 入金先口座コード
    DESCRIPTION TEXT NOT NULL,              -- 店名や概要
    TOTAL_AMOUNT INTEGER NOT NULL,          -- レシート合計金額（ユーザー入力）
    TAX_ROUNDING_TYPE VARCHAR(20),          -- 税の丸め方式
    MEMO TEXT,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    FOREIGN KEY (USER_ID) REFERENCES USERS(USER_ID),
    FOREIGN KEY (USER_ID, CATEGORY1_CODE) REFERENCES CATEGORY1(USER_ID, CATEGORY1_CODE),
    FOREIGN KEY (USER_ID, FROM_ACCOUNT_CODE) REFERENCES ACCOUNTS(USER_ID, ACCOUNT_CODE),
    FOREIGN KEY (USER_ID, TO_ACCOUNT_CODE) REFERENCES ACCOUNTS(USER_ID, ACCOUNT_CODE)
);

CREATE INDEX idx_headers_user_date ON TRANSACTION_HEADERS(USER_ID, TRANSACTION_DATE);
CREATE INDEX idx_headers_category1 ON TRANSACTION_HEADERS(USER_ID, CATEGORY1_CODE);
CREATE INDEX idx_headers_from_account ON TRANSACTION_HEADERS(USER_ID, FROM_ACCOUNT_CODE);
CREATE INDEX idx_headers_to_account ON TRANSACTION_HEADERS(USER_ID, TO_ACCOUNT_CODE);
```

#### 設計のポイント

##### 大分類と口座の関係

| 大分類 | FROM_ACCOUNT | TO_ACCOUNT | 例 |
|--------|--------------|------------|-----|
| **支出** | ✅ 必須 | ❌ NULL | 財布 → NULL（お店で使った） |
| **収入** | ❌ NULL | ✅ 必須 | NULL → 銀行口座（給与振込） |
| **振替** | ✅ 必須 | ✅ 必須 | A銀行 → B銀行（口座間移動） |

##### 大分類をヘッダに持つ理由
1. **整合性**: 全明細が同じ大分類になる（データの一貫性）
2. **簡素化**: 明細のカテゴリ選択は中分類・小分類のみ（2階層）
3. **口座制御**: 大分類に応じて口座欄の表示/必須を制御

##### TAX_ROUNDING_TYPEの値

| 値 | 意味 |
|---|------|
| NULL | 明細合計とレシート合計が一致 |
| 'ROUND_UP' | 切り上げパターンで一致 |
| 'ROUND_DOWN' | 切り捨てパターンで一致 |
| 'ROUND_HALF' | 四捨五入パターンで一致 |
| 'MANUAL' | 手動調整（不一致を承知で保存） |

### 3. 明細テーブル（TRANSACTIONS_DETAIL）

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

CREATE INDEX idx_details_transaction ON TRANSACTIONS_DETAIL(TRANSACTION_ID);
CREATE INDEX idx_details_user_categories ON TRANSACTIONS_DETAIL(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE);
```

#### 設計のポイント

##### ヘッダと明細の関係
- **必須制約**: ヘッダは必ず最低1件の明細を持つ
- **カスケード削除**: ヘッダ削除時に明細も自動削除
- **カテゴリ階層**: CATEGORY1はヘッダから、CATEGORY2/3は明細で選択

##### データ整合性の強化（2025-11-12更新）
- **USER_IDの追加**: ユーザー単位でのカテゴリ参照を明確化
- **CATEGORY1_CODEの追加**: ヘッダから大分類を継承し、カテゴリ階層を保証
- **複合外部キー制約**: CATEGORY2/CATEGORY3への参照整合性を確保
  - CATEGORY2: (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE)
  - CATEGORY3: (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE)
- **メモの分離**: MEMO_ID参照でMEMOSテーブルに外部化（同一メモの再利用可能）

##### 税の扱い

###### 背景
日本では店舗が税の端数処理（切り上げ/切り捨て/四捨五入）を自由に選択可能。
そのため、レシート合計と商品の本体価格×税率の計算結果が一致しないことがある。

###### 税計算フロー

```
[明細入力時]
1. ユーザーが本体価格（税抜き）を入力
2. システムが税額を自動計算: tax_amount = round(amount × tax_rate / 100)
3. ユーザーが税額を手動修正可能（小数点誤差対策）
4. 税込金額 = amount + tax_amount （リアルタイム表示）

[ヘッダ保存時]
1. 全明細の税込金額を合計: calculated_total = Σ(amount + tax_amount)
2. レシート合計と比較: calculated_total vs total_amount
3. 一致判定:
   a) 一致 → OK (tax_rounding_type = NULL)
   b) 不一致 → 3パターンで再計算
      - 切り上げ、切り捨て、四捨五入でいずれかに一致？
      - 一致した → tax_rounding_type に保存
      - 不一致 → 警告表示、ユーザー判断で保存可 (tax_rounding_type = 'MANUAL')
```

###### 税率のデフォルト値
- **デフォルト8%**: 日常の買い物は食料品（軽減税率）が中心
- **変更可能**: 日用品など10%の商品にも対応
- **将来拡張**: Phase 5で費目マスタに税率フィールド追加を検討

##### 金額の制約
- **0円以上**: マイナス不可、0円はOK
  - 理由: 振替で残高が0円になるケースなど、予測できない0円データの発生を許容
  - 操作上のデッドロックを防ぐ

---

## UI/UX 設計

### 画面遷移フロー

```
[入出金一覧画面]
  - ヘッダ情報のみ表示（1行1ヘッダ）
  - 表示項目: 日付、大分類、FROM/TO口座、摘要、合計金額
  
  ├─ [+ 新規追加] → ① ヘッダ登録モーダル
  └─ ヘッダ行 [編集] or ダブルクリック → ② ヘッダ編集モーダル

① [ヘッダ登録モーダル]
  - 日付
  - 大分類（支出/収入/振替）← ドロップダウン変更で口座欄の表示切替
  - 出金元口座（大分類が支出or振替の場合のみ表示）
  - 入金先口座（大分類が収入or振替の場合のみ表示）
  - 店名/摘要
  - レシート合計金額
  - メモ
  - [次へ（明細追加）] → ヘッダ保存後、②へ遷移
  - [キャンセル]

② [ヘッダ編集モーダル]
  - ヘッダ情報編集（上記と同じ項目）
  - 明細一覧表示（テーブル形式）
    | 中分類 | 小分類 | 商品名 | 本体価格 | 税率 | 税額 | 税込 | 操作 |
  - 明細合計（税込）: xxx円（自動計算）
  - レシート合計: xxx円
  - 差異: ±xx円（差異がある場合は警告表示）
  - [+ 明細追加] → ③ 明細登録モーダル
  - 明細行 [編集] or ダブルクリック → ③ 明細編集モーダル
  - 明細行 [削除] → 確認後削除（ただし最後の1件は削除不可）
  - [保存] / [キャンセル]

③ [明細登録/編集モーダル]（サブモーダル）
  - 中分類（ドロップダウン）※大分類は親から引き継ぎ
  - 小分類（ドロップダウン）※中分類選択後に絞り込み
  - 商品名/摘要
  - 本体価格（税抜き）← メインの入力
  - 税率（%）デフォルト8.00%
  - 税額 自動計算（グレー表示）← 修正可能
  - 税込金額（表示のみ）← リアルタイム計算
  - メモ
  - [保存] → ②に戻る
  - [保存して続けて追加] → 保存後、フォームクリアして続けて入力
  - [キャンセル] → ②に戻る
```

### 一覧画面の表示イメージ

```
入出金一覧
-------------------------------------------------------------
日付        | 大分類 | 口座      | 摘要            | 金額    | 操作
-------------------------------------------------------------
2025-11-06  | 支出   | 財布 →   | イオンで買い物  | 1,062円 | [編集][削除]
2025-11-05  | 支出   | 財布 →   | ガソリン        | 3,500円 | [編集][削除]
2025-11-01  | 収入   | → 銀行   | 11月分給与      | 250,000円| [編集][削除]
2025-10-30  | 振替   | A銀行→B銀行| 資金移動       | 50,000円 | [編集][削除]
```

### ヘッダ編集モーダルの表示イメージ

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  入出金編集
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

日付: [2025-11-06]
大分類: [支出 ▼]
出金元: [財布 ▼]
店名: [イオンで買い物____________]
レシート合計: [1062] 円
メモ: [________________________]

明細一覧
┌─────────────────────────────────────┐
│中分類  │小分類  │商品名    │本体│税率│税額│税込│操作│
├─────────────────────────────────────┤
│飲料    │乳製品  │牛乳 1L   │276│ 8% │ 22 │298│[編][削]│
│主食    │パン    │食パン6枚 │119│ 8% │  9 │128│[編][削]│
│食材    │卵      │卵 10個   │220│ 8% │ 17 │237│[編][削]│
│消耗品  │紙製品  │ティッシュ│368│10% │ 36 │404│[編][削]│
└─────────────────────────────────────┘
                         明細合計: 1,067円
                         差異: +5円 ⚠️

ℹ️ 明細合計とレシート合計に差異があります。
   レシート合計を修正するか、このまま保存できます。

[+ 明細追加]            [保存] [キャンセル]
```

### 明細登録モーダルの表示イメージ

```
━━━━━━━━━━━━━━━━━━━━━━━━━
  明細追加
━━━━━━━━━━━━━━━━━━━━━━━━━

大分類: 支出（親から継承）

中分類: [食費 ▼]
小分類: [飲料 ▼]

商品名: [牛乳 1L____________]

本体価格（税抜き）: [276] 円
税率: [8.00] %
税額: [22] 円 ← 自動計算（修正可）
─────────────────────
税込金額: 298円

メモ: [__________________]

[保存] [保存して続けて追加] [キャンセル]
```

---

## バリデーションルール

| 項目 | ルール | エラーメッセージ |
|------|--------|-----------------|
| **大分類と口座** | 支出→FROM必須/TO=NULL | "支出の場合、出金元口座を選択してください" |
| | 収入→FROM=NULL/TO必須 | "収入の場合、入金先口座を選択してください" |
| | 振替→両方必須 | "振替の場合、出金元と入金先の両方を選択してください" |
| **明細件数** | 必ず1件以上 | "明細が1件もありません。明細を追加してください" |
| **金額** | 0円以上の整数 | "金額は0以上の整数を入力してください" |
| **税率** | 0%以上 | "税率は0以上の数値を入力してください" |
| **合計一致** | レシート合計 vs 明細合計 | 警告表示（保存は可能） |
| **口座コード** | ACCOUNTSテーブルに存在 | "選択された口座が見つかりません" |
| **最後の明細削除** | 明細が1件の時は削除不可 | "最後の明細は削除できません" |

---

## 実装フェーズ

### Phase 0: 口座マスタ管理 🔴 **← 現在ここ**
- [ ] ACCOUNTSテーブル作成SQL
- [ ] 口座管理バックエンドAPI（Rust）
  - `add_account`
  - `get_accounts`
  - `update_account`
  - `delete_account`（論理削除）
- [ ] 口座管理画面（HTML/JS/CSS）
- [ ] デフォルト口座の自動生成
  - ユーザー作成時に「現金」「銀行口座」などを自動追加

### Phase 1: テーブル作成とマイグレーション
- [ ] TRANSACTION_HEADERS/DETAILSテーブル作成SQL
- [ ] 既存TRANSACTIONSテーブルからの移行スクリプト
  - 既存データは全て「ヘッダ+明細1件」として移行

### Phase 2: ヘッダ管理
- [ ] ヘッダ登録モーダル（HTML/JS/CSS）
- [ ] ヘッダ編集モーダル（HTML/JS/CSS）
- [ ] バックエンドAPI（Rust）
  - `add_transaction_header`
  - `get_transaction_header`
  - `update_transaction_header`
  - `delete_transaction_header`

### Phase 3: 明細管理
- [ ] 明細登録/編集モーダル（サブモーダル）
- [ ] 税計算ロジック（フロントエンド＋バックエンド）
- [ ] バックエンドAPI（Rust）
  - `add_transaction_detail`
  - `get_transaction_details`
  - `update_transaction_detail`
  - `delete_transaction_detail`

### Phase 4: 一覧画面更新
- [ ] ヘッダ一覧表示への対応
- [ ] フィルター機能の更新（口座フィルター追加）
- [ ] 集計機能（口座別、カテゴリ別）

### Phase 5: 将来の拡張（後回し）
- [ ] 費目マスタに税率フィールド追加
- [ ] 月次残高テーブル実装
- [ ] 口座残高の自動計算と表示
- [ ] グラフ表示機能

---

## 技術的な考慮事項

### トランザクション制御

ヘッダと明細は不可分な関係なので、必ずトランザクション内で処理：

```rust
// ヘッダ登録時
let mut tx = pool.begin().await?;

// 1. ヘッダを挿入
let header_id = insert_header(&mut tx, header_data).await?;

// 2. 明細を挿入（最低1件必須）
for detail in details {
    insert_detail(&mut tx, header_id, detail).await?;
}

// 3. コミット
tx.commit().await?;
```

### 既存データの移行

```sql
-- 既存TRANSACTIONSからの移行例
INSERT INTO TRANSACTION_HEADERS (
    USER_ID, TRANSACTION_DATE, CATEGORY1_CODE, 
    FROM_ACCOUNT_CODE, DESCRIPTION, TOTAL_AMOUNT
)
SELECT 
    USER_ID, TRANSACTION_DATE, CATEGORY1_CODE,
    'DEFAULT_ACCOUNT', DESCRIPTION, AMOUNT
FROM TRANSACTIONS;

-- 対応する明細を1件ずつ作成
INSERT INTO TRANSACTION_DETAILS (
    HEADER_ID, CATEGORY2_CODE, CATEGORY3_CODE,
    AMOUNT, TAX_AMOUNT, ITEM_DESCRIPTION
)
SELECT 
    h.HEADER_ID, t.CATEGORY2_CODE, t.CATEGORY3_CODE,
    t.AMOUNT, 0, t.DESCRIPTION
FROM TRANSACTIONS t
JOIN TRANSACTION_HEADERS h ON (適切なJOIN条件);
```

---

## 参考資料

### 関連ドキュメント
- [入出金管理 要件定義](../TRANSACTION_REQUIREMENTS.md)
- [入出金管理画面 実装ドキュメント v1](./TRANSACTION_MANAGEMENT_UI.md)
- [費目管理API](./API_CATEGORY_ja.md)

### データベーススキーマ
- `res/sql/dbaccess.sql`

### 既存実装
- フラット構造の実装: `src/services/transaction.rs`（Phase 1でリファクタリング）

---

**作成日**: 2025-11-07 00:36 JST
**最終更新**: 2025-11-12 22:35 JST
**更新内容**: TRANSACTIONS_DETAILテーブルの正規化（USER_ID, CATEGORY1_CODE追加、外部キー制約強化）
