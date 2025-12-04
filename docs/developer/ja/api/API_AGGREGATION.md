# 集計API リファレンス

**最終更新**: 2025-12-05 02:16 JST

## 概要

本ドキュメントは、集計画面で使用されるAPIの仕様を定義します。入出金データを様々な条件で集計し、分析結果を返します。

---

## 目次

1. [集計API一覧](#集計api一覧)
2. [データ構造](#データ構造)
3. [エラーハンドリング](#エラーハンドリング)
4. [使用例](#使用例)

---

## 集計API一覧

### get_monthly_aggregation

月次集計を実行します。

**パラメータ:**
- `year` (i32): 年（例: 2025）
- `month` (u32): 月（1-12）
- `group_by` (String): 集計軸（"category1", "category2", "category3", "account", "shop"）

**戻り値:**
- `Vec<AggregationResult>`: 集計結果の配列

**使用例:**
```javascript
const results = await invoke('get_monthly_aggregation', {
    year: 2025,
    month: 12,
    groupBy: 'category1'
});

results.forEach(r => {
    console.log(`${r.name}: ${r.total_amount}円`);
});
```

**注意:**
- セッションユーザーIDを自動取得
- 言語設定に応じて名称を取得（I18Nテーブル）

---

### get_daily_aggregation

日次集計を実行します。

**パラメータ:**
- `date` (String): 日付（"YYYY-MM-DD"形式）
- `group_by` (String): 集計軸

**戻り値:**
- `Vec<AggregationResult>`: 集計結果の配列

**使用例:**
```javascript
const results = await invoke('get_daily_aggregation', {
    date: '2025-12-05',
    groupBy: 'category2'
});
```

---

### get_period_aggregation

期間集計を実行します。

**パラメータ:**
- `start_date` (String): 開始日（"YYYY-MM-DD"形式）
- `end_date` (String): 終了日（"YYYY-MM-DD"形式）
- `group_by` (String): 集計軸

**戻り値:**
- `Vec<AggregationResult>`: 集計結果の配列

**使用例:**
```javascript
const results = await invoke('get_period_aggregation', {
    startDate: '2025-12-01',
    endDate: '2025-12-31',
    groupBy: 'category3'
});
```

**バリデーション:**
- `start_date <= end_date`
- 未来日付は不可

---

### get_weekly_aggregation

週次集計を実行します（年・週番号指定）。

**パラメータ:**
- `year` (i32): 年
- `week` (u32): 週番号（1-53）
- `week_start` (String): 週の開始曜日（"sunday" or "monday"）
- `group_by` (String): 集計軸

**戻り値:**
- `Vec<AggregationResult>`: 集計結果の配列

**使用例:**
```javascript
const results = await invoke('get_weekly_aggregation', {
    year: 2025,
    week: 49,
    weekStart: 'monday',
    groupBy: 'account'
});
```

---

### get_weekly_aggregation_by_date

週次集計を実行します（基準日指定）。

**パラメータ:**
- `reference_date` (String): 基準日（"YYYY-MM-DD"形式）
- `week_start` (String): 週の開始曜日（"sunday" or "monday"）
- `group_by` (String): 集計軸

**戻り値:**
- `Vec<AggregationResult>`: 集計結果の配列

**使用例:**
```javascript
const results = await invoke('get_weekly_aggregation_by_date', {
    referenceDate: '2025-12-05',
    weekStart: 'sunday',
    groupBy: 'shop'
});
```

**動作:**
- 指定された日付が含まれる週を自動計算
- 週の開始曜日に応じて期間を決定

---

### get_yearly_aggregation

年次集計を実行します。

**パラメータ:**
- `year` (i32): 年
- `year_start` (String): 年度の開始月（"january" or "april"）
- `group_by` (String): 集計軸

**戻り値:**
- `Vec<AggregationResult>`: 集計結果の配列

**使用例:**
```javascript
// 1月～12月（暦年）
const results = await invoke('get_yearly_aggregation', {
    year: 2025,
    yearStart: 'january',
    groupBy: 'category1'
});

// 4月～3月（年度）
const resultsApr = await invoke('get_yearly_aggregation', {
    year: 2025,
    yearStart: 'april',
    groupBy: 'category1'
});
```

---

### get_monthly_aggregation_by_category

カテゴリフィルタ付きの月次集計を実行します。

**パラメータ:**
- `year` (i32): 年
- `month` (u32): 月
- `group_by` (String): 集計軸
- `category1_code` (String): 大分類コード
- `category2_code` (Option<String>): 中分類コード（フィルタ）
- `category3_code` (Option<String>): 小分類コード（フィルタ）

**戻り値:**
- `Vec<AggregationResult>`: 集計結果の配列

**使用例:**
```javascript
// 支出全体を中分類で集計
const results = await invoke('get_monthly_aggregation_by_category', {
    year: 2025,
    month: 12,
    groupBy: 'category2',
    category1Code: 'EXPENSE',
    category2Code: null,
    category3Code: null
});

// 食費を小分類で集計
const foodResults = await invoke('get_monthly_aggregation_by_category', {
    year: 2025,
    month: 12,
    groupBy: 'category3',
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',  // 食費
    category3Code: null
});
```

---

## データ構造

### AggregationResult

```rust
pub struct AggregationResult {
    pub code: Option<String>,       // カテゴリコード、口座コード等
    pub name: String,                // 名称（多言語対応）
    pub total_amount: i64,           // 合計金額
    pub transaction_count: i64,      // トランザクション件数
}
```

**フロントエンドでの受け取り:**
```javascript
{
    code: "EXPENSE",
    name: "支出",
    total_amount: 150000,
    transaction_count: 25
}
```

---

## 集計軸（group_by）

| 値 | 説明 | 例 |
|----|------|---|
| `"category1"` | 大分類別 | 支出、収入、振替 |
| `"category2"` | 中分類別 | 食費、交通費、娯楽費 |
| `"category3"` | 小分類別 | 食料品、外食、飲料 |
| `"account"` | 口座別 | 現金、銀行口座、クレジットカード |
| `"shop"` | 店舗別 | イオン、セブンイレブン |

---

## エラーハンドリング

### 共通エラーパターン

| エラーメッセージ | 原因 | 対処方法 |
|----------------|------|---------|
| `"User not authenticated"` | セッション未認証 | ログインが必要 |
| `"Invalid month: ..."` | 月が1-12の範囲外 | 正しい月を指定 |
| `"Invalid date format: ..."` | 日付形式が不正 | "YYYY-MM-DD"形式に修正 |
| `"start_date must be <= end_date"` | 期間が逆転 | 開始日≦終了日に修正 |
| `"Invalid week_start: ..."` | 週開始曜日が不正 | "sunday"または"monday"を指定 |
| `"Invalid year_start: ..."` | 年度開始月が不正 | "january"または"april"を指定 |

### フロントエンドでのエラーハンドリング例

```javascript
async function loadMonthlyAggregation(year, month, groupBy) {
    try {
        const results = await invoke('get_monthly_aggregation', {
            year,
            month,
            groupBy
        });
        
        return results;
    } catch (error) {
        if (error.includes('Invalid month')) {
            alert('月の指定が不正です（1-12）');
        } else if (error.includes('not authenticated')) {
            window.location.href = 'index.html';
        } else {
            alert(`集計エラー: ${error}`);
        }
        return [];
    }
}
```

---

## 使用例：集計画面の実装

### 月次集計の表示

```javascript
async function displayMonthlyAggregation() {
    const year = parseInt(document.getElementById('year-select').value);
    const month = parseInt(document.getElementById('month-select').value);
    const groupBy = document.getElementById('groupby-select').value;
    
    try {
        const results = await invoke('get_monthly_aggregation', {
            year,
            month,
            groupBy
        });
        
        const tbody = document.getElementById('aggregation-table-body');
        tbody.innerHTML = '';
        
        // 金額合計を計算
        const totalAmount = results.reduce((sum, r) => sum + r.total_amount, 0);
        
        results.forEach(result => {
            const row = document.createElement('tr');
            const percentage = ((result.total_amount / totalAmount) * 100).toFixed(1);
            
            row.innerHTML = `
                <td>${result.name}</td>
                <td>${result.total_amount.toLocaleString()}円</td>
                <td>${result.transaction_count}件</td>
                <td>${percentage}%</td>
            `;
            tbody.appendChild(row);
        });
        
        // 合計行を追加
        const totalRow = document.createElement('tr');
        totalRow.innerHTML = `
            <td><strong>合計</strong></td>
            <td><strong>${totalAmount.toLocaleString()}円</strong></td>
            <td><strong>${results.reduce((sum, r) => sum + r.transaction_count, 0)}件</strong></td>
            <td><strong>100%</strong></td>
        `;
        tbody.appendChild(totalRow);
        
    } catch (error) {
        alert(`集計エラー: ${error}`);
    }
}
```

### 期間集計の実装

```javascript
async function displayPeriodAggregation() {
    const startDate = document.getElementById('start-date').value;
    const endDate = document.getElementById('end-date').value;
    const groupBy = document.getElementById('groupby-select').value;
    
    // 日付バリデーション
    if (new Date(startDate) > new Date(endDate)) {
        alert('開始日は終了日より前である必要があります');
        return;
    }
    
    try {
        const results = await invoke('get_period_aggregation', {
            startDate,
            endDate,
            groupBy
        });
        
        renderAggregationResults(results);
    } catch (error) {
        alert(`集計エラー: ${error}`);
    }
}
```

### カテゴリ別集計の実装

```javascript
async function displayCategoryAggregation() {
    const year = parseInt(document.getElementById('year-select').value);
    const month = parseInt(document.getElementById('month-select').value);
    const category1Code = document.getElementById('category1-select').value;
    const category2Code = document.getElementById('category2-select').value || null;
    
    try {
        const results = await invoke('get_monthly_aggregation_by_category', {
            year,
            month,
            groupBy: 'category3',
            category1Code,
            category2Code,
            category3Code: null
        });
        
        renderAggregationResults(results);
    } catch (error) {
        alert(`集計エラー: ${error}`);
    }
}
```

---

## 多言語対応

集計APIは設定された言語に応じて名称を返します。

**仕組み:**
1. `settings.get_string("language")`で言語を取得
2. I18Nテーブルから該当言語の名称を取得
3. LEFT JOINで存在しない場合はデフォルト名をフォールバック

**例:**
```javascript
// 日本語設定の場合
{
    code: "EXPENSE",
    name: "支出",  // I18Nテーブルから取得
    total_amount: 150000,
    transaction_count: 25
}

// 英語設定の場合
{
    code: "EXPENSE",
    name: "Expense",  // I18Nテーブルから取得
    total_amount: 150000,
    transaction_count: 25
}
```

---

## パフォーマンス最適化

### インデックスの活用

集計クエリは以下のインデックスを使用します：

```sql
CREATE INDEX idx_transaction_headers_user_date 
ON TRANSACTION_HEADERS(USER_ID, TRANSACTION_DATE);

CREATE INDEX idx_transaction_headers_category 
ON TRANSACTION_HEADERS(USER_ID, CATEGORY1_CODE);
```

### 大量データの処理

- **GROUP BY**: データベース側で集計
- **ORDER BY**: 金額降順（支出の多い順）
- **LEFT JOIN**: 多言語名の効率的な取得

---

## 今後の拡張予定

- [ ] 金額範囲フィルタ
- [ ] 商品別・メーカー別集計
- [ ] ページネーション対応
- [ ] CSVエクスポート
- [ ] グラフ表示用データ整形

---

## テストカバレッジ

**AggregationService:**
- ✅ 月次集計テスト
- ✅ 日次集計テスト
- ✅ 期間集計テスト
- ✅ 週次集計テスト
- ✅ 年次集計テスト
- ✅ カテゴリフィルタ付き集計テスト
- ✅ 日付バリデーションテスト

---

## 関連ドキュメント

### 実装ファイル

- 集計サービス: `src/services/aggregation.rs`
- SQL定義: `src/sql_queries.rs`
- Tauri Commands: `src/lib.rs`

### その他のAPIリファレンス

- [共通API](./API_COMMON.md) - セッション管理
- [入出金管理API](./API_TRANSACTION.md) - 入出金データ
- [費目管理API](./API_CATEGORY.md) - カテゴリ情報

---

**変更履歴:**
- 2025-11-21: 初版作成
- 2025-12-05: 実装コードに基づいて改訂
  - user_idパラメータを削除（セッションから自動取得）
  - 使用例を実装に合わせて修正
  - 新しいテンプレートに統一
