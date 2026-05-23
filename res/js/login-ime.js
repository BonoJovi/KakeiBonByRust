import { invoke } from '@tauri-apps/api/core';

// Deactivate fcitx5 when a login/setup username field gains focus.
//
// ibus already respects inputmode="latin" + lang="en" on the input, but
// fcitx5 ignores those hints and opens the field in compose mode. We call
// the fcitx5 Controller DBus API on the Rust side. The previous attempt
// (readOnly toggle on focus) made the I-beam disappear on fcitx5 + WebKitGTK,
// so that workaround is intentionally avoided here.

let pending = false;

async function deactivateIme(event) {
    console.log('[login-ime] focus -> deactivate fcitx5 IME', event?.target?.id);
    if (pending) {
        console.log('[login-ime] skipped (already in-flight)');
        return;
    }
    pending = true;
    try {
        await invoke('deactivate_fcitx_ime');
        console.log('[login-ime] deactivate_fcitx_ime done');
    } catch (e) {
        console.warn('[login-ime] deactivate_fcitx_ime failed (silent):', e);
    } finally {
        pending = false;
    }
}

function attach(input) {
    if (!input) {
        console.log('[login-ime] input not found, skipping');
        return;
    }
    console.log('[login-ime] attaching focus handler to', input.id);
    input.addEventListener('focus', deactivateIme);
}

document.addEventListener('DOMContentLoaded', () => {
    console.log('[login-ime] DOMContentLoaded, attaching handlers');
    ['admin-username', 'user-username', 'username'].forEach((id) => {
        attach(document.getElementById(id));
    });
});
