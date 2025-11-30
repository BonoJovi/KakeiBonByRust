# 税計算ロジックと端数処理の自動検出 / Tax Calculation Logic and Rounding Detection

**作成日 / Created**: 2025-11-19 08:33 JST  
**最終更新 / Last Updated**: 2025-11-19 08:33 JST

---

## 概要 / Overview

### 日本語

このドキュメントでは、KakeiBonの入出金明細管理機能における税計算ロジックと、端数処理の自動検出機能について詳細に説明します。

### English

This document provides detailed information about the tax calculation logic and automatic rounding detection feature in KakeiBon's transaction detail management.

---

## 税の端数処理の種類 / Tax Rounding Types

### 日本語

KakeiBonは、日本の商習慣で一般的に使用される3種類の端数処理に対応しています：

- **0: 切り捨て (Floor)** - デフォルト設定。最も一般的な処理方法
- **1: 四捨五入 (Round Half-Up)** - 0.5以上を切り上げ
- **2: 切り上げ (Ceil)** - 最も少ない店舗で使用される処理方法

### English

KakeiBon supports three types of tax rounding commonly used in Japanese business practices:

- **0: Floor (切り捨て)** - Default setting. Most commonly used
- **1: Round Half-Up (四捨五入)** - Round up when 0.5 or greater
- **2: Ceil (切り上げ)** - Used by the fewest stores

---

## 税計算の相互入力機能 / Bidirectional Tax Calculation

### 日本語

#### 機能の目的

レシートには税込金額のみが記載されている場合と、税抜金額と税額が記載されている場合があります。KakeiBonでは、どちらの入力パターンにも対応できるよう、相互計算機能を実装しています。

#### 計算の方向性

**税抜金額 → 税込金額の計算：**
```javascript
税額 = applyTaxRounding(税抜金額 × 税率 / 100, 端数処理タイプ)
税込金額 = 税抜金額 + 税額
```

**税込金額 → 税抜金額の計算：**
```javascript
税抜金額 = applyTaxRounding(税込金額 / (1 + 税率 / 100), 端数処理タイプ)
税額 = 税込金額 - 税抜金額
```

#### 最後に編集されたフィールドの追跡

- `lastTaxInputField`変数で、最後に編集されたフィールド（'excluding' または 'including'）を追跡
- 税率変更時は、最後に入力されたフィールドを基準に、反対側のフィールドを再計算
- これにより、ユーザーの入力意図を尊重した自然な挙動を実現

### English

#### Purpose

Receipts may show only the tax-included amount, or both the tax-excluded amount and tax. KakeiBon implements bidirectional calculation to support both input patterns.

#### Calculation Directions

**Tax-Excluded → Tax-Included Calculation:**
```javascript
tax = applyTaxRounding(excludedAmount × taxRate / 100, roundingType)
includedAmount = excludedAmount + tax
```

**Tax-Included → Tax-Excluded Calculation:**
```javascript
excludedAmount = applyTaxRounding(includedAmount / (1 + taxRate / 100), roundingType)
tax = includedAmount - excludedAmount
```

#### Last Edited Field Tracking

- Tracks the last edited field ('excluding' or 'including') via `lastTaxInputField` variable
- When tax rate changes, recalculates the opposite field based on the last input field
- Provides natural behavior that respects user input intent

---

## 丸め誤差の検出と警告 / Rounding Error Detection and Warning

### 日本語

#### 問題の背景

税込金額から税抜金額を計算する際、浮動小数点演算と端数処理により、1円の誤差が発生する可能性があります。

#### 検証ロジック

税込金額から税抜金額を計算した後、逆方向の計算を行って検証します：

```javascript
// 順方向の計算
税抜金額 = applyTaxRounding(税込金額 / (1 + 税率 / 100), 端数処理タイプ)
税額 = 税込金額 - 税抜金額

// 逆方向の検証計算
税額（検証用） = applyTaxRounding(税抜金額 × 税率 / 100, 端数処理タイプ)
税込金額（検証用） = 税抜金額 + 税額（検証用）

// 不一致チェック
if (税込金額（検証用） !== 税込金額（ユーザー入力）) {
    警告を表示();
}
```

#### 警告メッセージ

誤差が検出された場合、ユーザーに以下の情報を提供します：

- ユーザーが入力した税込金額
- 自動計算で算出された税込金額
- 差額（円）

警告例：
```
⚠️ 税込金額から計算した結果、税抜金額に1円の誤差が発生しました。
自動計算した税抜金額から税込金額を再計算すると365円（入力は366円）になります。
正確な金額が必要な場合は、税抜金額を直接入力してください。
```

#### 警告のクリア

- ユーザーが税抜金額を手動で入力した場合、警告は自動的にクリアされる
- 税抜金額から税込金額を計算する方向では、誤差は発生しない

### English

#### Background

When calculating tax-excluded amount from tax-included amount, a 1-yen discrepancy may occur due to floating-point arithmetic and rounding.

#### Verification Logic

After calculating tax-excluded amount from tax-included amount, perform reverse calculation for verification:

```javascript
// Forward calculation
excludedAmount = applyTaxRounding(includedAmount / (1 + taxRate / 100), roundingType)
tax = includedAmount - excludedAmount

// Reverse verification calculation
taxReverse = applyTaxRounding(excludedAmount × taxRate / 100, roundingType)
includedReverse = excludedAmount + taxReverse

// Discrepancy check
if (includedReverse !== userInputIncluded) {
    showWarning();
}
```

#### Warning Message

When discrepancy is detected, provides the following information to the user:

- User-entered tax-included amount
- Auto-calculated tax-included amount
- Difference (yen)

Warning example:
```
⚠️ A 1-yen discrepancy occurred when calculating from tax-included amount.
Recalculating from the tax-excluded amount gives ¥365 (input was ¥366).
For accurate amounts, please enter the tax-excluded amount directly.
```

#### Warning Clearing

- Warning automatically clears when user manually enters tax-excluded amount
- No discrepancy occurs when calculating tax-included from tax-excluded amount

---

## 実装の詳細 / Implementation Details

### 日本語

#### 端数処理関数

```javascript
function applyTaxRounding(value, roundingType = 0) {
    switch (roundingType) {
        case 0: // 切り捨て
            return Math.floor(value);
        case 1: // 四捨五入
            return Math.round(value);
        case 2: // 切り上げ
            return Math.ceil(value);
        default:
            return Math.floor(value);
    }
}
```

#### グローバル変数

- `taxRoundingType`: 入出金ヘッダから取得した端数処理タイプ（0-2）
- `lastTaxInputField`: 最後に編集されたフィールド（'excluding' | 'including'）

#### フィールドの関連付け

- **税抜金額** (`amount-excluding-tax`) - 入力可能
- **税額** (`amount-tax`) - 読み取り専用（自動計算）
- **税込金額** (`amount-including-tax`) - 入力可能
- **税率** (`tax-rate`) - 選択肢から選択

### English

#### Rounding Function

```javascript
function applyTaxRounding(value, roundingType = 0) {
    switch (roundingType) {
        case 0: // Floor
            return Math.floor(value);
        case 1: // Round Half-Up
            return Math.round(value);
        case 2: // Ceil
            return Math.ceil(value);
        default:
            return Math.floor(value);
    }
}
```

#### Global Variables

- `taxRoundingType`: Rounding type (0-2) retrieved from transaction header
- `lastTaxInputField`: Last edited field ('excluding' | 'including')

#### Field Relationships

- **Tax-Excluded Amount** (`amount-excluding-tax`) - Input enabled
- **Tax Amount** (`amount-tax`) - Read-only (auto-calculated)
- **Tax-Included Amount** (`amount-including-tax`) - Input enabled
- **Tax Rate** (`tax-rate`) - Select from options

---

## 今後の拡張予定 / Future Extensions

### 日本語

現在は手動で端数処理タイプを選択する必要がありますが、将来的には以下の機能を実装予定：

1. **端数処理の自動検出**
   - レシートの税抜金額と税額から、使用されている端数処理タイプを推測
   - 複数の明細から統計的に最も可能性の高い端数処理を判定
   - 自動判定された結果をユーザーに提案

2. **店舗別端数処理の記憶**
   - 店舗マスタに端数処理タイプを保存
   - 同じ店舗での入力時に自動適用

3. **丸め誤差の自動修正提案**
   - 誤差が検出された場合、最適な税抜金額を自動提案
   - ワンクリックで修正を適用

### English

Manual selection of rounding type is currently required, but future implementations include:

1. **Automatic Rounding Detection**
   - Infer rounding type from receipt's tax-excluded amount and tax
   - Statistically determine most likely rounding from multiple details
   - Suggest auto-detected result to user

2. **Store-Specific Rounding Memory**
   - Save rounding type in store master
   - Auto-apply when entering for same store

3. **Automatic Correction Suggestion**
   - Auto-suggest optimal tax-excluded amount when discrepancy detected
   - Apply correction with one click

---

## データベース保存 / Database Storage

### 日本語

#### 保存される値

- **TRANSACTIONS_HEADER.TAX_ROUNDING_TYPE**: 端数処理タイプ（0-2）
- **TRANSACTIONS_HEADER.TAX_INCLUDED_TYPE**: 税区分（0:内税, 1:外税）
- **TRANSACTIONS_DETAIL.AMOUNT**: 金額（税抜金額）
- **TRANSACTIONS_DETAIL.TAX_AMOUNT**: 税額
- **TRANSACTIONS_DETAIL.AMOUNT_INCLUDING_TAX**: 税込金額

#### 重要な設計判断

明細レベルでは税抜金額（AMOUNT）と税込金額（AMOUNT_INCLUDING_TAX）の両方を保存します。これにより：

- 税率変更時の再計算が可能
- 税込/税抜の集計が正確
- 丸め誤差の累積を最小化

### English

#### Saved Values

- **TRANSACTIONS_HEADER.TAX_ROUNDING_TYPE**: Rounding type (0-2)
- **TRANSACTIONS_HEADER.TAX_INCLUDED_TYPE**: Tax division (0: inclusive, 1: exclusive)
- **TRANSACTIONS_DETAIL.AMOUNT**: Amount (tax-excluded amount)
- **TRANSACTIONS_DETAIL.TAX_AMOUNT**: Tax amount
- **TRANSACTIONS_DETAIL.AMOUNT_INCLUDING_TAX**: Tax-included amount

#### Important Design Decision

Detail level stores both tax-excluded amount (AMOUNT) and tax-included amount (AMOUNT_INCLUDING_TAX). This enables:

- Recalculation when tax rate changes
- Accurate tax-included/excluded aggregation
- Minimized accumulation of rounding errors

---

## 関連ファイル / Related Files

- **Frontend**: `res/js/transaction-detail-management.js`
- **Constants**: `res/js/consts.js`, `src/consts.rs`
- **Translation**: `res/locales/*/transaction-detail-management.json`
- **Database**: `sql/schema.sql`

---

## 参考 / References

- 日本の消費税法における端数処理の規定
- IEEE 754浮動小数点演算の精度限界
- 一般的な会計ソフトウェアにおける税計算の実装パターン
