# 集計APIリファレンス

**最終更新**: 2025-11-21 05:10 JST

## 目次

1. [概要](#概要)
2. [アーキテクチャ](#アーキテクチャ)
3. [共通データ構造](#共通データ構造)
4. [Tauriコマンド](#tauriコマンド)
5. [Rustバックエンド関数](#rustバックエンド関数)
6. [使用例](#使用例)
7. [エラーハンドリング](#エラーハンドリング)

---

## 概要

集計APIは、入出金データを様々な条件で集計し、分析結果を返すAPIです。

### 主な機能

- **時間軸別集計**: 月次・日次・週次・年次・期間別
- **グルーピング**: カテゴリ・口座・店舗別
- **動的SQL生成**: 条件に応じた最適なクエリを自動生成
- **型安全設計**: Enumで不正な条件指定を防止

### アーキテクチャの特徴

- **3階層設計**: コア関数 → ラッパー関数 → Tauriコマンド
- **責務分離**: コア関数はSQL生成、ラッパー関数はバリデーション
- **拡張性**: 新しい集計タイプの追加が容易

---

## アーキテクチャ

### 3階層アーキテクチャ

```
┌─────────────────────────────────────┐
│  フロントエンド (JavaScript)        │
│  - UI操作                          │
│  - パラメータ収集                   │
│  - 結果表示                        │
└────────────┬────────────────────────┘
             │ Tauri invoke()
             ↓
┌─────────────────────────────────────┐
│  Tauriコマンド層                    │
│  - フロントエンドからの呼び出し受付 │
│  - パラメータのパース                │
│  - ラッパー関数呼び出し              │
└────────────┬────────────────────────┘
             │
             ↓
┌─────────────────────────────────────┐
│  ラッパー関数層                     │
│  - 期間の妥当性検証                 │
│  - ビジネスロジック                 │
│  - コア関数呼び出し                 │
└────────────┬────────────────────────┘
             │
             ↓
┌─────────────────────────────────────┐
│  コア関数層 (SQLジェネレータ)       │
│  - 動的SQL生成                     │
│  - クエリ実行                       │
│  - 結果の整形                       │
└─────────────────────────────────────┘
```

### 責務の分離

| 層 | 責務 | 例 |
|----|------|---|
| **フロントエンド** | UI操作、表示 | ボタンクリック、結果テーブル表示 |
| **Tauriコマンド** | パラメータ変換 | 文字列→Enum変換 |
| **ラッパー関数** | バリデーション | 未来日付チェック、範囲チェック |
| **コア関数** | SQL生成・実行 | WHERE句生成、GROUP BY句生成 |

---

## 共通データ構造

### Enum定義

#### DateFilter

日付フィルタを表現するEnum。

```rust
pub enum DateFilter {
    Exact(NaiveDate),               // 特定日
    Between(NaiveDate, NaiveDate),  // 期間範囲
}
```

**使用例**:
- 日次集計: `DateFilter::Exact(date)`
- 月次集計: `DateFilter::Between(month_start, month_end)`

#### GroupBy

集計軸を表現するEnum。

```rust
pub enum GroupBy {
    Category1,  // 大分類
    Category2,  // 中分類
    Category3,  // 小分類
    Account,    // 口座
    Shop,       // 店舗
}
```

#### OrderField

ソート対象フィールド。

```rust
pub enum OrderField {
    Amount,  // 金額
}
```

#### SortOrder

ソート順。

```rust
pub enum SortOrder {
    Asc,   // 昇順
    Desc,  // 降順
}
```

### 構造体定義

#### AggregationFilter

フィルタ条件をまとめる構造体。

```rust
pub struct AggregationFilter {
    date: DateFilter,                    // 日付フィルタ（必須）
    amount: Option<AmountFilter>,        // 金額フィルタ（将来拡張用）
    category: Option<CategoryFilter>,    // カテゴリフィルタ（将来拡張用）
    shop_id: Option<i64>,                // 店舗ID（将来拡張用）
}
```

#### AggregationRequest

集計リクエスト全体を表す構造体。

```rust
pub struct AggregationRequest {
    user_id: i64,                  // ユーザーID
    filter: AggregationFilter,     // フィルタ条件
    group_by: GroupBy,             // 集計軸
    order_by: OrderField,          // ソート対象
    sort_order: SortOrder,         // ソート順
}
```

#### AggregationResult

集計結果を表す構造体。

```rust
#[derive(Debug, Serialize)]
pub struct AggregationResult {
    pub group_key: String,      // グループキー（カテゴリ名等）
    pub group_name: String,     // 表示名
    pub total_amount: i64,      // 合計金額（純増減）
    pub count: i64,             // 件数
    pub avg_amount: i64,        // 平均金額
}
```

---

## Tauriコマンド

フロントエンドから呼び出すTauriコマンド。

### get_monthly_aggregation

月次集計を実行。

#### シグネチャ

```rust
#[tauri::command]
async fn get_monthly_aggregation(
    user_id: i64,
    year: i32,
    month: u32,
    group_by: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<AggregationResult>, String>
```

#### パラメータ

| 名前 | 型 | 説明 |
|------|---|------|
| `user_id` | `i64` | ユーザーID |
| `year` | `i32` | 年（例: 2025） |
| `month` | `u32` | 月（1-12） |
| `group_by` | `String` | 集計軸（"category1", "category2", "category3", "account", "shop"） |
| `state` | `tauri::State<AppState>` | アプリケーション状態 |

#### 戻り値

成功時: `Vec<AggregationResult>`  
失敗時: `String`（エラーメッセージ）

#### フロントエンドからの呼び出し例

```javascript
import { invoke } from '@tauri-apps/api/core';

const results = await invoke('get_monthly_aggregation', {
    userId: 1,
    year: 2025,
    month: 11,
    groupBy: 'category1'
});
```

---

### get_daily_aggregation

日次集計を実行。

#### シグネチャ

```rust
#[tauri::command]
async fn get_daily_aggregation(
    user_id: i64,
    date: String,           // Format: "YYYY-MM-DD"
    group_by: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<AggregationResult>, String>
```

#### パラメータ

| 名前 | 型 | 説明 |
|------|---|------|
| `user_id` | `i64` | ユーザーID |
| `date` | `String` | 日付（"YYYY-MM-DD"形式） |
| `group_by` | `String` | 集計軸 |
| `state` | `tauri::State<AppState>` | アプリケーション状態 |

#### フロントエンドからの呼び出し例

```javascript
const results = await invoke('get_daily_aggregation', {
    userId: 1,
    date: '2025-11-20',
    groupBy: 'category2'
});
```

---

### get_weekly_aggregation_by_date

週次集計を実行（基準日ベース）。

#### シグネチャ

```rust
#[tauri::command]
async fn get_weekly_aggregation_by_date(
    user_id: i64,
    reference_date: String,  // Format: "YYYY-MM-DD"
    week_start: String,      // "sunday" or "monday"
    group_by: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<AggregationResult>, String>
```

#### パラメータ

| 名前 | 型 | 説明 |
|------|---|------|
| `user_id` | `i64` | ユーザーID |
| `reference_date` | `String` | 基準日（"YYYY-MM-DD"形式） |
| `week_start` | `String` | 週の開始曜日（"sunday" or "monday"） |
| `group_by` | `String` | 集計軸 |
| `state` | `tauri::State<AppState>` | アプリケーション状態 |

#### フロントエンドからの呼び出し例

```javascript
const results = await invoke('get_weekly_aggregation_by_date', {
    userId: 1,
    referenceDate: '2025-11-20',
    weekStart: 'monday',
    groupBy: 'account'
});
```

---

### get_yearly_aggregation

年次集計を実行。

#### シグネチャ

```rust
#[tauri::command]
async fn get_yearly_aggregation(
    user_id: i64,
    year: i32,
    year_start_month: u32,  // 1 for January, 4 for April
    group_by: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<AggregationResult>, String>
```

#### パラメータ

| 名前 | 型 | 説明 |
|------|---|------|
| `user_id` | `i64` | ユーザーID |
| `year` | `i32` | 年 |
| `year_start_month` | `u32` | 年度開始月（1: 暦年、4: 会計年度） |
| `group_by` | `String` | 集計軸 |
| `state` | `tauri::State<AppState>` | アプリケーション状態 |

#### フロントエンドからの呼び出し例

```javascript
// 暦年（1月開始）
const results1 = await invoke('get_yearly_aggregation', {
    userId: 1,
    year: 2025,
    yearStartMonth: 1,
    groupBy: 'shop'
});

// 会計年度（4月開始）
const results2 = await invoke('get_yearly_aggregation', {
    userId: 1,
    year: 2025,
    yearStartMonth: 4,
    groupBy: 'shop'
});
```

---

### get_period_aggregation

期間別集計を実行。

#### シグネチャ

```rust
#[tauri::command]
async fn get_period_aggregation(
    user_id: i64,
    start_date: String,  // Format: "YYYY-MM-DD"
    end_date: String,    // Format: "YYYY-MM-DD"
    group_by: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<AggregationResult>, String>
```

#### パラメータ

| 名前 | 型 | 説明 |
|------|---|------|
| `user_id` | `i64` | ユーザーID |
| `start_date` | `String` | 開始日（"YYYY-MM-DD"形式） |
| `end_date` | `String` | 終了日（"YYYY-MM-DD"形式） |
| `group_by` | `String` | 集計軸 |
| `state` | `tauri::State<AppState>` | アプリケーション状態 |

#### フロントエンドからの呼び出し例

```javascript
const results = await invoke('get_period_aggregation', {
    userId: 1,
    startDate: '2025-10-01',
    endDate: '2025-11-20',
    groupBy: 'category3'
});
```

---

## Rustバックエンド関数

### コア関数

#### monthly_aggregation

月次集計リクエストを生成。

```rust
pub fn monthly_aggregation(
    user_id: i64,
    year: i32,
    month: u32,
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError>
```

**機能**:
- 年月から開始日・終了日を計算
- `DateFilter::Between`でフィルタを作成
- バリデーション（未来日付チェック等）

---

#### daily_aggregation

日次集計リクエストを生成。

```rust
pub fn daily_aggregation(
    user_id: i64,
    date: NaiveDate,
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError>
```

**機能**:
- `DateFilter::Exact`でフィルタを作成
- 未来日付のバリデーション

---

#### weekly_aggregation_by_date

週次集計リクエストを生成（基準日ベース）。

```rust
pub fn weekly_aggregation_by_date(
    user_id: i64,
    reference_date: NaiveDate,
    week_start: Weekday,
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError>
```

**機能**:
- 基準日と週の開始曜日から週の範囲を計算
- O(1)計算で高速
- 週番号不要で直感的

**計算ロジック**:
```rust
// 基準日から週の開始日を見つける
let days_from_week_start = (reference_date.weekday().num_days_from(week_start) as i64);
let week_start_date = reference_date - Duration::days(days_from_week_start);
let week_end_date = week_start_date + Duration::days(6);
```

---

#### yearly_aggregation

年次集計リクエストを生成。

```rust
pub fn yearly_aggregation(
    user_id: i64,
    year: i32,
    year_start_month: u32,
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError>
```

**機能**:
- 年度開始月から年度範囲を計算
- 暦年（1月開始）と会計年度（4月開始）対応

**計算ロジック**:
```rust
let start_date = NaiveDate::from_ymd_opt(year, year_start_month, 1)?;

let end_year = if year_start_month == 1 { year } else { year + 1 };
let end_month = if year_start_month == 1 { 12 } else { year_start_month - 1 };
let end_date = last_day_of_month(end_year, end_month)?;
```

---

#### period_aggregation

期間別集計リクエストを生成。

```rust
pub fn period_aggregation(
    user_id: i64,
    start_date: NaiveDate,
    end_date: NaiveDate,
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError>
```

**機能**:
- 開始日・終了日のバリデーション
- `start_date <= end_date`チェック
- 未来日付チェック

---

### ラッパー関数

各コア関数に対応するラッパー関数が存在します。

#### execute_monthly_aggregation

```rust
pub async fn execute_monthly_aggregation(
    pool: &SqlitePool,
    user_id: i64,
    year: i32,
    month: u32,
    group_by: GroupBy,
    lang: &str,
) -> Result<Vec<AggregationResult>, String>
```

**機能**:
- コア関数を呼び出してリクエスト生成
- `execute_aggregation`を呼び出してSQL実行
- エラーを文字列に変換

**内部処理**:
```rust
let request = monthly_aggregation(user_id, year, month, group_by)
    .map_err(|e| e.to_string())?;

execute_aggregation(pool, &request, lang).await
```

同様に、`execute_daily_aggregation`, `execute_weekly_aggregation_by_date`, `execute_yearly_aggregation`, `execute_period_aggregation`が存在します。

---

### 共通実行関数

#### execute_aggregation

すべての集計リクエストを実行する共通関数。

```rust
pub async fn execute_aggregation(
    pool: &SqlitePool,
    request: &AggregationRequest,
    lang: &str,
) -> Result<Vec<AggregationResult>, String>
```

**機能**:
- リクエストから動的にSQLを生成
- SQLを実行
- 結果を整形して返す

**動的SQL生成例**:

カテゴリ（大分類）でグルーピング:
```sql
SELECT 
    h.CATEGORY1_CODE as group_key,
    COALESCE(i.CATEGORY1_NAME_I18N, c.CATEGORY1_NAME) as group_name,
    SUM(CASE 
        WHEN h.CATEGORY1_CODE = 'INCOME' THEN h.AMOUNT
        WHEN h.CATEGORY1_CODE = 'EXPENSE' THEN -h.AMOUNT
        ELSE 0
    END) as total_amount,
    COUNT(*) as count
FROM TRANSACTION_HEADERS h
LEFT JOIN CATEGORY1 c ON h.CATEGORY1_CODE = c.CATEGORY1_CODE
LEFT JOIN CATEGORY1_I18N i ON c.CATEGORY1_CODE = i.CATEGORY1_CODE AND i.LANG_CODE = ?
WHERE h.USER_ID = ? AND h.TRANSACTION_DATE >= ? AND h.TRANSACTION_DATE <= ?
GROUP BY h.CATEGORY1_CODE
ORDER BY total_amount DESC
```

---

## 使用例

### 例1: 月次集計（カテゴリ大分類別）

```javascript
// フロントエンド
const results = await invoke('get_monthly_aggregation', {
    userId: 1,
    year: 2025,
    month: 11,
    groupBy: 'category1'
});

// 結果
[
    {
        group_key: "EXPENSE",
        group_name: "支出",
        total_amount: -150000,
        count: 45,
        avg_amount: -3333
    },
    {
        group_key: "INCOME",
        group_name: "収入",
        total_amount: 300000,
        count: 1,
        avg_amount: 300000
    }
]
```

### 例2: 週次集計（店舗別）

```javascript
const results = await invoke('get_weekly_aggregation_by_date', {
    userId: 1,
    referenceDate: '2025-11-20',
    weekStart: 'monday',
    groupBy: 'shop'
});

// 結果
[
    {
        group_key: "1",
        group_name: "コンビニA",
        total_amount: -15000,
        count: 10,
        avg_amount: -1500
    },
    {
        group_key: "2",
        group_name: "スーパーB",
        total_amount: -30000,
        count: 3,
        avg_amount: -10000
    }
]
```

### 例3: 年次集計（会計年度、口座別）

```javascript
const results = await invoke('get_yearly_aggregation', {
    userId: 1,
    year: 2025,
    yearStartMonth: 4,  // 会計年度
    groupBy: 'account'
});

// 結果
[
    {
        group_key: "CASH",
        group_name: "現金",
        total_amount: -50000,  // 振替含む
        count: 100,
        avg_amount: -500
    },
    {
        group_key: "BANK001",
        group_name: "みずほ銀行",
        total_amount: 200000,  // 振替含む
        count: 30,
        avg_amount: 6667
    }
]
```

---

## エラーハンドリング

### エラーの種類

#### AggregationError

```rust
pub enum AggregationError {
    InvalidMonth { year: i32, month: u32 },
    InvalidDateRange { start: NaiveDate, end: NaiveDate },
    FutureDate { year: i32, month: u32 },
    DatabaseError(String),
}
```

### エラーメッセージ例

```javascript
try {
    const results = await invoke('get_monthly_aggregation', {
        userId: 1,
        year: 2025,
        month: 13,  // 不正な月
        groupBy: 'category1'
    });
} catch (error) {
    console.error('集計エラー:', error);
    // "Invalid month: 2025-13"
}
```

### バリデーション

各関数で以下のバリデーションが実行されます：

1. **月のバリデーション** (1-12)
2. **日付範囲のバリデーション** (start_date <= end_date)
3. **未来日付のチェック** (日付 <= 今日)
4. **週の開始曜日のチェック** ("sunday" or "monday")

---

## パフォーマンス最適化

### インデックスの活用

集計クエリは以下のインデックスを使用します：

```sql
CREATE INDEX idx_transaction_headers_user_date 
ON TRANSACTION_HEADERS(USER_ID, TRANSACTION_DATE);

CREATE INDEX idx_transaction_headers_category 
ON TRANSACTION_HEADERS(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE);
```

### クエリ最適化

- **LEFT JOIN**: 多言語名が存在しない場合のフォールバック
- **COALESCE**: NULL値の安全な処理
- **GROUP BY**: 集計軸に応じた動的なグルーピング
- **ORDER BY**: デフォルトは金額降順（支出の多い順）

---

## 今後の拡張予定

- [ ] `AmountFilter`の実装（金額範囲フィルタ）
- [ ] `CategoryFilter`の実装（カテゴリフィルタ）
- [ ] 商品別・メーカー別集計
- [ ] ページネーション対応
- [ ] CSVエクスポート

---

## 関連ドキュメント

- [集計機能ユーザーガイド](AGGREGATION_USER_GUIDE.md) - ユーザー向け使い方ガイド
- [入出金管理API](API_TRANSACTION.md) - 入出金データの登録・取得API
- [カテゴリ管理API](API_CATEGORY.md) - 費目管理API

---

**Last Updated**: 2025-11-21 05:10 JST
