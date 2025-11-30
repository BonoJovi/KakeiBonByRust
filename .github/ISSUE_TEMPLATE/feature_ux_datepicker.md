---
name: UX Improvement - Date Picker Enhancement
about: Consider custom date picker implementation for better UX
title: '[UX検討] 日付ピッカーのカスタマイズ / Date Picker Customization'
labels: enhancement, ux, consideration
assignees: ''

---

## 概要 / Overview

**日本語:**
現在、日付入力にHTML標準の`<input type="date">`を使用していますが、カレンダーを閉じるにはESCキーが必要です。エリア外クリックで閉じられるようにすることで、UXが向上する可能性があります。

**English:**
Currently using HTML native `<input type="date">` for date input. The calendar requires ESC key to close. Consider implementing custom date picker that closes when clicking outside the calendar area for better UX.

---

## 現状の動作 / Current Behavior

- 日付フィールドをクリック → カレンダーが表示
- カレンダーを閉じる方法: ESCキーのみ
- エリア外クリックでは閉じない（ブラウザ標準動作）

---

## 改善案 / Improvement Proposal

### Option 1: カスタム日付ピッカーライブラリ導入
**候補:**
- Flatpickr
- Pikaday  
- react-datepicker (if migrating to React)

**メリット:**
- エリア外クリックで閉じる
- カスタマイズ性が高い
- 多言語対応

**デメリット:**
- 依存関係が増える
- フォントサイズ対応の追加実装が必要
- メンテナンスコスト

### Option 2: 現状維持
**理由:**
- HTML標準なので動作が安定
- ユーザーはOS標準動作に慣れている
- プロトタイプ段階では十分

---

## 優先度 / Priority

**低 / Low**

- 現状でも使用に支障はない
- リリースプロダクトで検討
- ユーザーフィードバック後に判断

---

## 関連情報 / Related Information

- 発見日: 2025-11-05
- 発見者: User testing
- 影響範囲: 入出金管理画面のフィルター機能

---

## 実装検討タイミング / Implementation Timing

- [ ] Phase 1（プロトタイプ）: 実装しない
- [ ] Phase 2（ベータ版）: ユーザーフィードバック収集
- [ ] Phase 3（リリース版）: 必要に応じて実装検討
