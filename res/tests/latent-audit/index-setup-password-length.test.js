/**
 * Latent audit 2026-09 — initial admin / user setup forms (res/js/menu.js)
 *
 * IDs covered: L31 (menu.js half; user-management half is in
 *              user-management-admin-page.test.js)
 *
 * Bug: handleAdminSetup / handleUserSetup check `password.length < 16`, i.e.
 *      UTF-16 code units, while the backend validate_password counts chars
 *      (`chars().count()`). 8 emoji = 16 UTF-16 units passes the frontend and
 *      is rejected only after the register_* round-trip.
 * Expected: the frontend counts code points like the backend, rejects the
 *      password itself (register_admin / register_user not invoked) and shows
 *      `error.password_too_short`.
 *
 * The real menu.js is booted against res/index.html via ../pages/_page-harness.js
 * (menu.js itself is NOT mocked here).
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from '../pages/_page-harness.js';

const { invoke } = mockPageModules(jest, {
    keepMenu: true,
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'check_needs_setup':
                return true;
            case 'register_admin':
            case 'register_user':
                if ([...args.password].length < 16) {
                    return Promise.reject({ code: 'validation', message: 'Password must be at least 16 characters long!' });
                }
                return 'ok';
            default:
                return null;
        }
    },
});

loadPageBody('index.html');
await import('../../js/menu.js');
await bootPage();

const EIGHT_EMOJI = '😀'.repeat(8); // 16 UTF-16 units, 8 chars

function submit(formId) {
    document.getElementById(formId).dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
}

describe('setup forms password length — latent audit 2026-09', () => {
    beforeEach(() => invoke.mockClear());

    test('[latent L31] admin setup rejects 8 emoji (8 chars) on the frontend', async () => {
        expect(EIGHT_EMOJI.length).toBe(16);
        document.getElementById('admin-username').value = 'admin';
        document.getElementById('admin-password').value = EIGHT_EMOJI;
        document.getElementById('admin-password-confirm').value = EIGHT_EMOJI;
        submit('admin-setup-form');
        await flush(10);

        expect(callsOf(invoke, 'register_admin')).toHaveLength(0);
        expect(document.getElementById('setup-message').textContent).toBe('error.password_too_short');
    });

    test('[latent L31] user setup rejects 8 emoji (8 chars) on the frontend', async () => {
        document.getElementById('user-username').value = 'alice';
        document.getElementById('user-password').value = EIGHT_EMOJI;
        document.getElementById('user-password-confirm').value = EIGHT_EMOJI;
        submit('user-setup-form');
        await flush(10);

        expect(callsOf(invoke, 'register_user')).toHaveLength(0);
        expect(document.getElementById('user-setup-message').textContent).toBe('error.password_too_short');
    });
});
