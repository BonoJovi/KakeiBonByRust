import { invoke } from '@tauri-apps/api/core';

// Resize the window to the current monitor and center it.
//
// Linux: calls fit_window_to_monitor. The Rust side runs as a single
// synchronous Tauri command — splitting set_size and set_position across
// two commands raced the X11/XCB sequence counter across Tokio worker
// threads and crashed the app with `xcb_xlib_threads_sequence_lost`.
// Keep this thin: one invoke, no setTimeout between phases.
//
// Windows: calls fit_window_to_monitor_windows. WebView2 auto-resizes
// the window to each page's intrinsic content size on navigation, so we
// re-fit + recenter on every page load. Larger height margin (200) keeps
// the window clear of the taskbar.
export async function fitWindowToScreen() {
    const isWindows = /Windows/i.test(navigator.userAgent);

    console.log('[fitWindowToScreen] start', {
        platform: isWindows ? 'windows' : 'linux',
        availWidth: window.screen.availWidth,
        availHeight: window.screen.availHeight,
    });
    try {
        // Width: pass tauri.conf.json's minWidth (1100). On Linux WebKitGTK
        // expands the window to its content's natural width (~1140px) on its
        // own — we *want* that. On Windows WebView2 honors set_size directly.
        // Height: fill most of the screen, minus a margin for WM panels
        // (top bar / dock / taskbar) and the title bar. Windows needs the
        // larger margin to clear the taskbar.
        const targetWidth = 1100;
        const heightMargin = isWindows ? 200 : 80;
        const targetHeight = Math.max(window.screen.availHeight - heightMargin, 600);
        const command = isWindows ? 'fit_window_to_monitor_windows' : 'fit_window_to_monitor';

        console.log('[fitWindowToScreen] invoking', command, {
            targetWidth,
            targetHeight,
        });
        await invoke(command, {
            width: targetWidth,
            height: targetHeight,
        });
        console.log('[fitWindowToScreen] done');
    } catch (error) {
        console.error('[fitWindowToScreen] FAILED:', error);
    }
}
