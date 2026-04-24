# Tauri Development Notes

## Key Points

- KakeiBon is a **Tauri desktop app**, not a browser web app
- No browser reload (F5/Ctrl+R) — tell user to **restart the app**
- Frontend changes require app restart: close window → Ctrl+C → `cargo tauri dev`
- Rust backend changes auto-recompile in dev mode
- DevTools available (F12/right-click→Inspect) but reload button doesn't work
- Rust logs appear in the terminal running `cargo tauri dev`
