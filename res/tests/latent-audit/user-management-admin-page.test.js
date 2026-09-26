/**
 * Latent audit 2026-09 — user management screen, admin session (res/js/user-management.js)
 *
 * IDs covered: L24, L31 (user-management half; menu.js half is in
 *              index-setup-password-length.test.js)
 *
 * L24 Bug: a password of 16 spaces passes the frontend check
 *     (`password.length < 16` only). The backend `validate_password` trims and
 *     returns ApiError { code: 'validation', message: 'Password cannot be
 *     empty!' }. handleUserSave routes it through mapMasterErrorCode, whose
 *     `cannot be empty` branch maps to `${prefix}.empty_name` →
 *     `user_mgmt.empty_name`, a key that does not exist in dbaccess.sql, and
 *     shows it inline under the USERNAME field.
 *     Expected: no raw `user_mgmt.empty_name` anywhere, nothing attributed to
 *     the username field, and the error is attributed to the password
 *     (form message or inline error on #password). Rejecting the whitespace
 *     password on the frontend before invoking is an acceptable fix too.
 *
 * L31 Bug: the frontend counts password length in UTF-16 code units
 *     (`password.length`) while the backend counts chars (`chars().count()`).
 *     8 emoji = 16 UTF-16 units passes the frontend check but has only 8
 *     characters, so the backend rejects it after a round-trip.
 *     Expected: the frontend rejects it itself (create_general_user is not
 *     invoked) with the `error.password_too_short` message.
 *
 * The real page module is booted against res/user-management.html via
 * ./_page-harness.js with an admin session.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf, definedI18nKeys,
} from './_page-harness.js';

const ADMIN = { user_id: 1, name: 'admin', role: 0 };

const { invoke } = mockPageModules(jest, {
    user: ADMIN,
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'list_users':
                return [{ user_id: 1, name: 'admin', role: 0, entry_dt: '2026-01-01 00:00:00', update_dt: null }];
            case 'create_general_user': {
                // Mirror src/validation.rs::validate_password.
                const pw = args.password;
                if (pw.trim() === '') {
                    return Promise.reject({ code: 'validation', message: 'Password cannot be empty!' });
                }
                if ([...pw].length < 16) {
                    return Promise.reject({ code: 'validation', message: 'Password must be at least 16 characters long!' });
                }
                return 5;
            }
            default:
                return null;
        }
    },
});

loadPageBody('user-management.html');
await import('../../js/user-management.js');
await bootPage();

async function openAddAndSubmit(username, password) {
    document.getElementById('add-user-btn').click();
    await flush(3);
    document.getElementById('username').value = username;
    document.getElementById('password').value = password;
    document.getElementById('password-confirm').value = password;
    document.getElementById('user-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
    await flush(10);
}

const inlineErrorOf = (id) => {
    const next = document.getElementById(id).nextElementSibling;
    return next && next.classList.contains('validation-error') ? next.textContent : null;
};

describe('user management (admin) — latent audit 2026-09', () => {
    beforeEach(() => {
        invoke.mockClear();
        document.getElementById('cancel-btn')?.click();
    });

    test('[latent L24] 16-space password is reported on the password, not as raw user_mgmt.empty_name on the username', async () => {
        // Premise: the key the classifier produces is not a defined resource.
        expect(definedI18nKeys().has('user_mgmt.empty_name')).toBe(false);

        await openAddAndSubmit('bob', ' '.repeat(16));

        expect(document.body.textContent).not.toContain('user_mgmt.empty_name');
        expect(inlineErrorOf('username')).toBeNull();

        const formMessage = document.getElementById('form-message');
        const passwordErrorShown =
            inlineErrorOf('password') !== null
            || (formMessage.classList.contains('error') && formMessage.textContent.trim() !== '');
        expect(passwordErrorShown).toBe(true);
    });

    test('[latent L31] 8 emoji (16 UTF-16 units, 8 chars) is rejected by the frontend length check', async () => {
        const eightEmoji = '😀'.repeat(8);
        expect(eightEmoji.length).toBe(16);
        expect([...eightEmoji].length).toBe(8);

        await openAddAndSubmit('carol', eightEmoji);

        expect(callsOf(invoke, 'create_general_user')).toHaveLength(0);
        expect(document.getElementById('form-message').textContent).toContain('error.password_too_short');
    });
});
