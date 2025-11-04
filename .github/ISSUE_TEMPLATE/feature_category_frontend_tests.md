---
name: Category Management Frontend Tests / 費目管理画面フロントエンドテスト
about: Implement comprehensive frontend tests for category management screen / 費目管理画面の包括的なフロントエンドテストを実装
title: 'feat(test): add frontend tests for category management'
labels: enhancement, testing, frontend
assignees: ''

---

## Description / 概要

Implement comprehensive frontend tests for the category management screen (Phase 5-2).

費目管理画面の包括的なフロントエンドテストを実装します（Phase 5-2）。

## Background / 背景

- Backend tests are complete (125 tests passing)
  - バックエンドテストは完了（125テスト成功）
- Frontend testing framework (Jest or similar) needs to be set up
  - フロントエンドテストフレームワーク（Jest等）のセットアップが必要
- Manual testing is currently in place but automated tests are needed
  - 現在は手動テストのみ、自動テストが必要

## Tasks / タスク

### Test Infrastructure / テストインフラ
- [ ] Set up frontend testing framework (Jest + JSDOM or similar)
  - フロントエンドテストフレームワークのセットアップ（Jest + JSDOM等）
- [ ] Configure test environment for ES modules
  - ESモジュール用のテスト環境設定
- [ ] Set up mock for Tauri invoke API
  - Tauri invoke APIのモック実装

### Tree Display Tests / ツリー表示テスト
- [ ] Test category tree rendering
  - カテゴリツリーの描画テスト
- [ ] Test expand/collapse functionality
  - 展開/折りたたみ機能のテスト
- [ ] Test language switching in tree view
  - ツリービューでの言語切り替えテスト
- [ ] Test empty state display
  - 空状態の表示テスト

### Modal Operation Tests (Common Modal Class) / モーダル操作テスト（共通Modalクラス）
- [ ] Test ESC key to close modal
  - ESCキーでモーダルを閉じるテスト
- [ ] Test focus trap (TAB/SHIFT+TAB navigation)
  - フォーカストラップのテスト（TAB/SHIFT+TABナビゲーション）
- [ ] Test aria attribute changes on open/close
  - 開閉時のaria属性変更のテスト
- [ ] Test backdrop click to close
  - バックドロップクリックで閉じるテスト
- [ ] Test modal behavior on all screens (user-management, category-management)
  - 全画面でのモーダル動作確認（ユーザー管理、費目管理）

### CRUD Operation Tests / CRUD操作テスト
- [ ] Test add category2 form validation
  - 中分類追加フォームのバリデーションテスト
- [ ] Test add category3 form validation
  - 小分類追加フォームのバリデーションテスト
- [ ] Test edit category2 with existing data
  - 既存データを使用した中分類編集テスト
- [ ] Test edit category3 with existing data
  - 既存データを使用した小分類編集テスト
- [ ] Test duplicate name validation
  - 重複名のバリデーションテスト
- [ ] Test bilingual name input
  - 日英名入力のテスト

### Display Order Tests / 並び順変更テスト
- [ ] Test move up functionality
  - 上へ移動機能のテスト
- [ ] Test move down functionality
  - 下へ移動機能のテスト
- [ ] Test boundary conditions (first/last item)
  - 境界条件のテスト（最初/最後の項目）
- [ ] Test order persistence after reload
  - 再読み込み後の並び順永続化テスト

### Error Handling Tests / エラーハンドリングテスト
- [ ] Test network error handling
  - ネットワークエラー処理のテスト
- [ ] Test validation error display
  - バリデーションエラー表示のテスト
- [ ] Test i18n error messages
  - 多言語対応エラーメッセージのテスト

## Acceptance Criteria / 受け入れ基準
- [ ] All frontend tests pass
  - 全てのフロントエンドテストが成功
- [ ] Test coverage > 80% for category management code
  - 費目管理コードのテストカバレッジ > 80%
- [ ] CI/CD pipeline includes frontend tests
  - CI/CDパイプラインにフロントエンドテストを含める
- [ ] Test documentation is updated
  - テストドキュメントを更新

## Dependencies / 依存関係
- Jest or similar testing framework
  - Jest等のテストフレームワーク
- @testing-library/dom or similar
  - @testing-library/dom等
- Mock implementation of @tauri-apps/api
  - @tauri-apps/apiのモック実装

## Estimated Effort / 見積もり工数
Medium to Large (requires infrastructure setup + test implementation)

中〜大（インフラセットアップ + テスト実装が必要）

## Priority / 優先度
Medium (automated tests improve maintainability but manual tests are in place)

中（自動テストは保守性を向上させるが、手動テストは実施済み）

## Related / 関連
- Phase 5-2: Frontend Test Implementation (TODO.md)
  - Phase 5-2: フロントエンドテスト実装（TODO.md）
- Backend tests completed in Phase 3.5
  - バックエンドテストはPhase 3.5で完了
- Current test count: 125 backend tests passing
  - 現在のテスト数: バックエンドテスト125件成功
