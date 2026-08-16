---
name: testing-kakeibon-gui
description: How to build, launch and GUI-test the KakeiBon Tauri desktop app locally (login, menus, settings persistence, DevTools).
---

# GUI testing the KakeiBon Tauri app

## Launching
- The repo documents `cargo tauri dev`, but the Cargo subcommand may not be installed
  (`error: no such command: tauri`). In that case run `npm install` in the repo root once and
  launch with `npx tauri dev` (first build takes several minutes; later launches ~40s).
- The window title is `KakeiBon`. It un-maximizes on almost every page navigation, and other
  windows may steal focus, so after each navigation run:
  `wmctrl -a KakeiBon; wmctrl -r KakeiBon -b add,maximized_vert,maximized_horz`
- Data lives in `~/.kakeibon/`: `KakeiBonDB.sqlite3` (delete to reset first-run setup) and
  `KakeiBon.json` (UI settings — useful to assert language/font_size persistence from the shell).

## Accounts / navigation
- First run requires admin setup; passwords must be >= 16 chars (e.g. `TestPassword12345!`).
- Master pages: 管理 → マスタ管理 → 費目/口座/店舗/メーカー/商品/ユーザー管理.
  Submenus close on diagonal mouse movement — move horizontally into the submenu, then vertically.
- The dashboard (管理 → 集計 → ダッシュボード) is rejected for admin accounts with a JS alert;
  log in as a general user instead.

## Console evidence
- DevTools: right-click → "要素の詳細を表示" (Inspect). F12 does not work.
- The WebKit inspector console is cleared on every page navigation and there is no working
  "preserve log" option, so logs emitted right before a `window.location.href` navigation
  (e.g. `Session cleared` in `res/js/menu.js` handleLogout) cannot be captured on management
  pages. Workaround: trigger the same shared handler from the index page, where logout does not
  navigate, and the log stays in the console.

## Known pre-existing blockers to watch for
- `res/js/account-management.js` may both import and re-declare `escapeHtml`
  (`node --check res/js/account-management.js` reports "Identifier 'escapeHtml' has already been
  declared"). The module then never loads: the account page renders untranslated English text and
  has no menu bar, so any menu-related behaviour there is untestable until it is fixed.

## Menu behaviour gotchas
- Page-specific scripts (`dashboard.js`, `*-management.js`) install their own dropdown handlers
  guarded by `dataset.initialized`, competing with `res/js/menu.js`. Which one wins depends on
  script ordering, so verify open/close AND mutual exclusion (File / 管理 / 言語 / フォントサイズ)
  separately on each page type — dashboard behaviour can differ from management pages.
