# 税計算ロジック リファクタプラン

**作成日**: 2026-04-26
**ステータス**: 設計フェーズ（実装未着手）
**対象範囲**: ヘッダー総額の自動計算（フロント） + 集計クエリ（バックエンド）

---

## 1. 背景

ダッシュボードのカテゴリ別比較で、特定カテゴリの集計値が手計算と一致しないケースが見つかった。
4 月の食費で、`丸め(SUM(AMOUNT) × 1.08)` と `SUM(AMOUNT_INCLUDING_TAX)` の差が 107 円。
原因は **明細単位で税計算→丸めを行ってから足し込む** 構造により、丸め誤差が累積していたこと。

直近のロールバック前に当面の重複計上バグ（明細 JOIN による行膨張）は修正したが、
その置き換え式 `SUM(COALESCE(td.AMOUNT_INCLUDING_TAX, td.AMOUNT))` は
「個別丸め値の単純和」のため、誤差累積を防げない。本プランでは抜本的に方針を変える。

---

## 2. 設計合意事項（ユーザー確認済み）

| 項目 | 決定 |
|---|---|
| 1 取引内の総額計算 | **税率ごとに `SUM(税抜AMOUNT)` → 税計算 → 取引の `TAX_ROUNDING_TYPE` で丸め → 税率分を合算** |
| 取引をまたぐ集計 | **既に整数化された値をそのまま `SUM`、追加の丸めはしない** |
| `AMOUNT` の意味 | **常に税抜き** |
| `AMOUNT_INCLUDING_TAX` | コンビニ等の税込み記載レシート対応のため UI / DB ともに残す |
| 修正範囲のスコープ | フロント自動計算（A）と 集計クエリ（B）を **別 PR に分割** |
| 重複計上修正の扱い | 暫定修正はロールバック済み。本プランで根本対応に置き換え |

### なぜ取引をまたいだ丸めをしないか

各取引の総額は `TAX_ROUNDING_TYPE` に従って既に整数化されている。
整数の合計に再度丸めをかけても値は変わらないため、無意味な操作になる。
さらに、複数取引で `TAX_ROUNDING_TYPE` が異なる場合、集計時にどれを採用するか
統一できないという問題も避けられる。

---

## 3. 現状の仕様把握

### 3.1 関連スキーマ

```sql
TRANSACTIONS_HEADER:
  TOTAL_AMOUNT          INTEGER NOT NULL    -- 取引全体の総額（整数）
  TAX_ROUNDING_TYPE     INTEGER             -- 0: 切り捨て / 1: 四捨五入 / 2: 切り上げ
  TAX_INCLUDED_TYPE     INTEGER NOT NULL    -- 0: 税込 / 1: 税抜（DEFAULT 1）

TRANSACTIONS_DETAIL:
  AMOUNT                INTEGER NOT NULL    -- 税抜き（合意により定義）
  TAX_AMOUNT            INTEGER DEFAULT 0
  TAX_RATE              INTEGER DEFAULT 8   -- 8% / 10% など
  AMOUNT_INCLUDING_TAX  INTEGER             -- 税込（NULL 可、UI 表示や入力補助用）
```

### 3.2 集計クエリの現状（`src/services/aggregation.rs`）

ロールバック後の状態。`build_query` は全グルーピングで `SUM(th.TOTAL_AMOUNT)` を集計。
Category2 / Category3 / Product の場合は `INNER JOIN TRANSACTIONS_DETAIL` で行が膨張するので、
**th.TOTAL_AMOUNT × 明細行数** という重複計上バグが残っている（再発状態）。

```rust
// build_query 抜粋（現在の状態）
SUM(CASE
    WHEN th.CATEGORY1_CODE = 'EXPENSE' THEN -th.TOTAL_AMOUNT
    WHEN th.CATEGORY1_CODE = 'INCOME'  THEN  th.TOTAL_AMOUNT
    WHEN th.CATEGORY1_CODE = 'TRANSFER' THEN 0
    ELSE th.TOTAL_AMOUNT
END) as total_amount
```

### 3.3 フロントの税計算ロジック（`res/js/transaction-detail-management.js`）

**明細単位** の税計算は実装済み（`calculateFromExcludingTax` / `calculateFromIncludingTax`、
ファイル先頭〜250 行付近）。`applyTaxRounding(excluded * rate / 100, taxRoundingType)` で
明細ごとに税額を丸めている。

**ヘッダーの TOTAL_AMOUNT を明細から自動計算するロジックは現状未実装**（要再調査）。
現状はユーザーがヘッダー画面で総額を直接入力していると想定される。
仕様変更により、明細を保存・更新したタイミングでヘッダー総額を再計算して反映する必要がある。

### 3.4 既知の不整合データ

- 4 月の `EXPENSE` ヘッダー 44 件中、`TAX_INCLUDED_TYPE = 1`（税抜）は 1 件
- そのうち `TRANSACTION_ID = 15` でヘッダー 2,921 円・明細(税込)合計 3,009 円 → 88 円差
- もう 1 件は外税ヘッダーで `丸め(SUM × 1.08)` と `SUM(AMOUNT_INCLUDING_TAX)` の差が 2 円以上
- 累積した結果、ダッシュボード食費（C2_E_1）でユーザー手計算との差 107 円

---

## 4. 修正案 A — フロント自動計算（別 PR）

### 4.1 ゴール

明細を追加・編集・削除したとき、ヘッダー画面 / 詳細画面の総額表示と DB の `TOTAL_AMOUNT` が
正しい計算式で更新される。

### 4.2 計算式

```
正:  total = Σ_税率( round_by_type( SUM(税抜) × (100 + 税率) / 100, TAX_ROUNDING_TYPE ) )
誤:  total = SUM( round_by_type( 税抜 × (100 + 税率) / 100, TAX_ROUNDING_TYPE ) )   ← 累積誤差
```

### 4.3 タスク（着手前に詳細化）

1. ヘッダー総額を「明細から自動計算する」ボタン or トリガー仕様の確認（ユーザー手入力との共存）
2. 計算ヘルパー関数を JS に追加（テスト容易な純関数）
3. バックエンドの `add_transaction_detail` / `update_transaction_detail` 後に
   ヘッダー総額を再計算する API を追加 or フロントで再計算→ヘッダー更新コール
4. ユニットテスト（複数税率混在 / 各 ROUNDING_TYPE / 端数累積回避を検証）
5. 既存データへの遡及計算（過去取引の `TOTAL_AMOUNT` を再計算するスクリプト）— 要 GO 判断

### 4.4 オープン課題

- ユーザーがヘッダー画面で総額を直接編集できるかどうか（自動計算と手動の優先順位）
- 明細未入力の取引（ヘッダーのみ）の扱い — 現状 0 件だが将来発生したら？
- 過去データの遡及計算をやるか、or 表示時に再計算するか

---

## 5. 修正案 B — 集計クエリ（別 PR、本プランの主軸）

### 5.1 ゴール

- 重複計上バグ（detail JOIN による行膨張）を再発させずに修正
- 明細レベルのカテゴリ集計（Category2 / Category3 / Product）でも正確な総額を返す
- ヘッダーレベルのグルーピング（Category1 / Account / Shop / Date）は影響を最小化

### 5.2 SQL 設計（明細レベル集計）

サブクエリで `(取引, グループキー, 税率)` 単位の整数値を作り、外側で集計する。

```sql
SELECT
    sub.group_key,
    sub.group_name,
    SUM(sub.signed_amount)        AS total_amount,
    COUNT(DISTINCT sub.txn_id)    AS count,
    CAST(AVG(sub.signed_amount) AS INTEGER) AS avg_amount
FROM (
    SELECT
        th.TRANSACTION_ID                                AS txn_id,
        td.CATEGORY1_CODE || '/' || td.CATEGORY2_CODE    AS group_key,
        COALESCE(c2i.CATEGORY2_NAME_I18N, c2.CATEGORY2_NAME) AS group_name,
        -- 取引×グループ×税率 単位で税抜 SUM して税計算 + 取引の丸め方式で整数化
        CASE th.TAX_ROUNDING_TYPE
            WHEN 0 THEN CAST(SUM(td.AMOUNT) * (100 + td.TAX_RATE) / 100 AS INTEGER)         -- floor
            WHEN 1 THEN CAST(ROUND(SUM(td.AMOUNT) * (100.0 + td.TAX_RATE) / 100.0) AS INTEGER) -- half-up
            WHEN 2 THEN -CAST(-SUM(td.AMOUNT) * (100 + td.TAX_RATE) / 100 AS INTEGER)        -- ceil
        END
        * CASE th.CATEGORY1_CODE
            WHEN 'EXPENSE' THEN -1
            WHEN 'TRANSFER' THEN 0
            ELSE 1
        END AS signed_amount
    FROM TRANSACTIONS_HEADER th
    INNER JOIN TRANSACTIONS_DETAIL td
        ON th.USER_ID = td.USER_ID AND th.TRANSACTION_ID = td.TRANSACTION_ID
    LEFT JOIN CATEGORY2 c2 ON ...
    LEFT JOIN CATEGORY2_I18N c2i ON ... AND c2i.LANG_CODE = ?
    WHERE { date_filter, scheduled_filter, ... }
    GROUP BY th.TRANSACTION_ID, group_key, group_name, td.TAX_RATE, th.TAX_ROUNDING_TYPE
) sub
GROUP BY sub.group_key, sub.group_name
ORDER BY ...
```

- **取引×カテゴリ×税率** 単位で税抜 SUM → 税計算 → ヘッダーの `TAX_ROUNDING_TYPE` で丸める
- 上位クエリは整数値を `SUM` するだけ（追加丸めなし）
- `count` は `COUNT(DISTINCT txn_id)` で取引数（明細数ではなく）

### 5.3 ヘッダーレベルのグルーピング

`Category1` / `Shop` / `Date` は `th.TOTAL_AMOUNT` を使い続ける。
ただし、**A の修正で `TOTAL_AMOUNT` が正しく整数化されている前提** が必要。
B 単独で先行する場合、現行 `TOTAL_AMOUNT` の整合性を信じて `SUM(th.TOTAL_AMOUNT)` のままで良い。

`Account` グルーピングは UNION ALL 構造のまま、明細を見ないので影響なし。

### 5.4 SQLite の丸め関数の確認事項

- `CAST(x AS INTEGER)` は **0 方向への切り捨て**（SQLite 仕様）
  - 正の数なら floor、負の数なら ceil 相当
  - 取引金額は正なのでこれで OK
- `ROUND(x)` は四捨五入（half-away-from-zero）
- 切り上げは `-CAST(-x AS INTEGER)` のイディオムが安全

実装時に SQLite で挙動を再確認すること。

### 5.5 バックエンドの実装変更箇所

- `src/services/aggregation.rs::build_query` を分岐：
  - detail-level（Category2 / Category3 / Product）→ サブクエリ方式の SQL を生成
  - header-level（Category1 / Account / Shop / Date）→ 現状維持
- 必要なら `build_detail_aggregation_query` という別関数に切り出す

### 5.6 テスト方針

| テスト | 検証内容 |
|---|---|
| 単一明細・単一税率 | 単純ケースで `SUM(td.AMOUNT) × 1.08 = total_amount` |
| 同税率の複数明細 | 端数累積なし。`Σtd.AMOUNT × 1.08` ベースで整数化 |
| 複数税率混在（8% + 10%） | 税率ごとに別 SUM → 別丸め → 合算 |
| `TAX_ROUNDING_TYPE` 0/1/2 | 各丸め方向が SQL 上で正しく適用される |
| 1 取引に Cat2 が混在 | 食費 + 日用品の混在レシートで按分が正しい |
| TRANSFER の除外 | 集計に TRANSFER が混入しない |
| ヘッダーレベル集計 | Category1 / Shop / Date の SQL は変更されないことを assert |
| 重複計上の回帰防止 | detail-level grouping で `th.TOTAL_AMOUNT` を直接 SUM していないことを assert |

統合テスト（`#[tokio::test]`）でインメモリ DB に既知の値を入れて、最終 `total_amount` が
期待値と一致することを検証する。`SUM(AMOUNT_INCLUDING_TAX)` 方式と差が出るデータを意図的に
仕込む。

### 5.7 実装手順

1. `build_query` を `build_header_query` と `build_detail_query` に分割
2. 既存テスト（31 件）を実行可能な形に保ったまま、SQL 文字列だけ変える単体テストを先に
3. `#[tokio::test]` で実 SQL の挙動を検証する統合テストを追加
4. 全テスト緑 → 実機（`cargo tauri dev`）でダッシュボード食費が手計算と一致することを確認
5. PR 作成

---

## 6. 進行順序の提案

```
1. 入出金フィルタ修正（既に dev に変更あり、コミット待ち）
   └ コミット & プッシュ → リリース対象に乗せるか別途判断

2. 修正案 B（集計クエリ）  ← 本プランの主軸
   └ 別 PR、別ブランチで作業
   └ 完了後にダッシュボードの食費がユーザー手計算と一致することを確認

3. 修正案 A（フロント自動計算）
   └ さらに別 PR、別ブランチ
   └ 過去データの遡及計算スクリプトを作るかは要相談

4. 口座残高サマリー（既に予定済み）
   └ さらに別 PR
```

---

## 7. リスク・考慮事項

- **B を先行 / A を後続にする場合**、ヘッダーの `TOTAL_AMOUNT` がまだ「明細単位丸めの和」で
  保存されている。Category1 集計値とユーザー認識のズレが残る可能性がある。
  → 当座は許容、A の完了後に過去データを再計算する必要あり
- **過去データの遡及更新** は破壊的なので、必ずバックアップ取得 + 確認画面 + ロールバック手段
  を整備してから実施
- SQLite の `ROUND` の挙動はバージョン差がほぼ無いが、テストで再確認すること
- `CAST(... AS INTEGER)` の切り捨て方向は仕様として SQLite docs で確認すること
- TRANSFER の扱いは現行通り（集計から除外）

---

## 8. 未確定 TODO（着手前に確認）

- [ ] フロント側でヘッダー総額を「明細から自動計算」するトリガーの仕様（手動編集との共存方針）
- [ ] 過去データ遡及計算の実施可否
- [ ] 集計の `count` を「取引数」と「明細数」のどちらで定義するか
  （ユーザー視点では取引数が自然と思うが、現行仕様の確認が必要）
- [ ] `TAX_RATE` が 0 の明細（税なし）の扱い — 単純に `SUM(AMOUNT)` でよいはず
