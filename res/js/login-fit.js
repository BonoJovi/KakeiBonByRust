import { fitWindowToScreen } from './window-fit.js';

// Re-fit + recenter the window on login/setup page load.
//
// On Windows, WebView2 auto-resizes the window to each page's intrinsic
// content size on navigation, anchored to the top-left of the previous
// position. Without an explicit re-fit, the login page ends up offset to
// the left after the initial centered display.
document.addEventListener('DOMContentLoaded', async () => {
    await fitWindowToScreen();
});
