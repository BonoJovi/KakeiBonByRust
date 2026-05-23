import { invoke } from '@tauri-apps/api/core';

// Resize the window to the current monitor and center it.
//
// The Rust side runs as a single synchronous Tauri command
// (`fit_window_to_monitor`) — splitting set_size and set_position across
// two commands raced the X11/XCB sequence counter across Tokio worker
// threads and crashed the app with `xcb_xlib_threads_sequence_lost`.
// Keep this thin: one invoke, no setTimeout between phases.
export async function fitWindowToScreen() {
    console.log('[fitWindowToScreen] start', {
        availWidth: window.screen.availWidth,
        availHeight: window.screen.availHeight,
    });
    try {
        // Width: pass tauri.conf.json's minWidth (1100). WebKitGTK then
        // expands the window to its content's natural width (~1140px on
        // these screens) on its own — we *want* that, so we don't force a
        // larger min-width on the body.
        // Height: fill most of the screen, minus a small margin for WM
        // panels (top bar / dock) and the title bar.
        const targetWidth = 1100;
        const targetHeight = Math.max(window.screen.availHeight - 80, 600);

        console.log('[fitWindowToScreen] invoking fit_window_to_monitor', {
            targetWidth,
            targetHeight,
        });
        await invoke('fit_window_to_monitor', {
            width: targetWidth,
            height: targetHeight,
        });
        console.log('[fitWindowToScreen] done');
    } catch (error) {
        console.error('[fitWindowToScreen] FAILED:', error);
    }
}
