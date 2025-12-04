# Testing Strategy / テスト戦略

## Overview / 概要

KakeiBonByRust employs a comprehensive three-tier testing approach:

KakeiBonByRustは包括的な3層テストアプローチを採用しています：

1. **Backend Unit Tests** - Database layer and API logic / データベース層とAPIロジック
2. **Frontend Unit Tests** - UI components and business logic / UIコンポーネントとビジネスロジック  
3. **Integration Tests** - End-to-end testing through Tauri / Tauriを通じたエンドツーエンドテスト

---

## Phase 1: Backend Unit Tests ✅

### Location / 場所
- `src-tauri/src/db/category.rs` - Category CRUD tests

### Test Coverage / テストカバレッジ

#### Category Management Tests (11 tests)

**CRUD Operations / CRUD操作:**
- ✅ `test_add_category1` - Add major category
- ✅ `test_update_category1` - Update major category
- ✅ `test_delete_category1_cascade` - Delete major category with CASCADE
- ✅ `test_add_category2` - Add medium category
- ✅ `test_update_category2` - Update medium category
- ✅ `test_delete_category2_cascade` - Delete medium category with CASCADE
- ✅ `test_add_category3` - Add minor category
- ✅ `test_update_category3` - Update minor category
- ✅ `test_delete_category3` - Delete minor category

**Business Logic / ビジネスロジック:**
- ✅ `test_display_order_management` - Display order functionality
- ✅ `test_user_isolation` - User data isolation

### Running Backend Tests / バックエンドテストの実行

```bash
# Run all tests
cd src-tauri
cargo test

# Run specific test module
cargo test db::category::tests

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_add_category1
```

### Test Database / テストデータベース

Tests use in-memory SQLite databases to ensure:
- テストは以下を保証するためにインメモリSQLiteデータベースを使用します：
  - No side effects on production database / 本番データベースへの副作用なし
  - Fast execution / 高速実行
  - Isolated test environment / 隔離されたテスト環境

---

## Phase 2: Frontend Unit Tests ⏳ (Planned)

### Planned Location / 予定場所
- `res/tests/` - Frontend test files

### Planned Test Coverage / 計画中のテストカバレッジ

**Category Management UI:**
- [ ] Category tree rendering
- [ ] Add category modal
- [ ] Edit category modal  
- [ ] Delete confirmation dialog
- [ ] Order change functionality
- [ ] Error handling and validation
- [ ] i18n resource loading

**User Management UI:**
- [ ] User list rendering
- [ ] Add user modal
- [ ] Edit user modal
- [ ] Password validation
- [ ] Role management

### Testing Framework / テストフレームワーク (TBD)
- Jest / Vitest (to be decided)
- DOM testing library

---

## Phase 3: Integration Tests ⏳ (Planned)

### Scope / 範囲

End-to-end tests covering:
- Frontend ↔ Tauri ↔ Backend interaction
- フロントエンド ↔ Tauri ↔ バックエンドの相互作用

**Test Scenarios:**
- [ ] Complete category management workflow
- [ ] User management workflow
- [ ] Authentication flow
- [ ] Language switching
- [ ] Error recovery

### Testing Framework / テストフレームワーク (TBD)
- Tauri's WebDriver (to be evaluated)
- Playwright / Cypress (to be evaluated)

---

## Test Execution Summary / テスト実行サマリー

### Current Status / 現在の状態

**Backend Tests:**
- Total: 11 tests
- Passed: 11 ✅
- Failed: 0
- Coverage: Category CRUD operations

**Frontend Tests:**
- Status: Not yet implemented / 未実装

**Integration Tests:**
- Status: Not yet implemented / 未実装

---

## Testing Best Practices / テストのベストプラクティス

### Backend Tests / バックエンドテスト

1. **Use in-memory databases** for unit tests
   - ユニットテストにはインメモリデータベースを使用

2. **Test user isolation** to ensure data security
   - データセキュリティを保証するためにユーザー分離をテスト

3. **Test CASCADE behavior** for related data deletion
   - 関連データ削除のCASCADE動作をテスト

4. **Verify display order** management
   - 表示順管理を検証

### Frontend Tests (Planned) / フロントエンドテスト（計画中）

1. **Test user interactions** (clicks, form inputs)
   - ユーザーインタラクション（クリック、フォーム入力）をテスト

2. **Test error states** and recovery
   - エラー状態と復旧をテスト

3. **Test i18n** resource loading
   - i18nリソース読み込みをテスト

4. **Mock Tauri APIs** for isolation
   - 分離のためにTauri APIをモック

### Integration Tests (Planned) / 統合テスト（計画中）

1. **Test complete workflows** from UI to database
   - UIからデータベースまでの完全なワークフローをテスト

2. **Test error propagation** across layers
   - 層を越えたエラー伝播をテスト

3. **Test concurrent operations** if applicable
   - 該当する場合は同時操作をテスト

---

## Continuous Integration / 継続的インテグレーション

### GitHub Actions (Planned) / GitHub Actions（計画中）

- [ ] Run backend tests on every PR
- [ ] Run frontend tests on every PR
- [ ] Run integration tests before merge
- [ ] Generate test coverage reports
- [ ] Automated test status reporting

---

## Test Maintenance / テストメンテナンス

### When to Update Tests / テストを更新するタイミング

1. **When adding new features** - Add corresponding tests
   - 新機能追加時 - 対応するテストを追加

2. **When fixing bugs** - Add regression tests
   - バグ修正時 - リグレッションテストを追加

3. **When refactoring** - Ensure existing tests still pass
   - リファクタリング時 - 既存のテストが通ることを確認

4. **When schema changes** - Update test database setup
   - スキーマ変更時 - テストデータベースセットアップを更新

---

Last Updated: 2025-10-28
