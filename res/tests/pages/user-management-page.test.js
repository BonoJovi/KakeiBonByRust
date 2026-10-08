/**
 * User management screen (res/js/user-management.js) — regression tests
 * promoted from the 2026-09 latent audit.
 *
 * M13 A whitespace-only username trimmed to '' and was sent to
 *     create_general_user; the backend only checked the maximum length, so
 *     an account nobody could log in to was created (the login form cannot
 *     submit a blank username). The backend now rejects it too; this pins
 *     the frontend guard: the username field shows the required-field
 *     message and no create command is sent.
 *
 * The real page module is booted against res/user-management.html via
 * ./_page-harness.js with an admin session.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf, isHiddenOrAbsent,
} from './_page-harness.js';

const ADMIN = { user_id: 1, name: 'admin', role: 0 };

const { invoke } = mockPageModules(jest, {
    user: ADMIN,
    invoke: (cmd) => {
        switch (cmd) {
            case 'list_users':
                return [
                    { user_id: 1, name: 'admin', role: 0, entry_dt: '2026-01-01 00:00:00', update_dt: null },
                    { user_id: 2, name: 'alice', role: 1, entry_dt: '2026-04-22 16:43:45', update_dt: '2026-05-26 19:28:14' },
                ];
            case 'create_general_user':
                return 5;
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

describe('user management (admin) — regression (latent audit 2026-09)', () => {
    beforeEach(() => {
        invoke.mockClear();
        document.getElementById('cancel-btn')?.click();
    });

    test('[M13] a whitespace-only username is rejected before create_general_user', async () => {
        await openAddAndSubmit(' 　 ', 'valid_password_123456');

        expect(callsOf(invoke, 'create_general_user')).toHaveLength(0);
        expect(inlineErrorOf('username')).toBe('validation.required');
    });

    test('[M13] a normal username still reaches create_general_user', async () => {
        await openAddAndSubmit('bob', 'valid_password_123456');

        const creates = callsOf(invoke, 'create_general_user');
        expect(creates).toHaveLength(1);
        expect(creates[0].username).toBe('bob');
    });

    test('created and updated times are shown in local time, not as stored UTC', () => {
        // Stored values are UTC; the tests run with TZ=Asia/Tokyo (UTC+9).
        const cells = (id) => {
            const btn = document.querySelector(`#user-list .btn-edit[data-user-id="${id}"]`);
            return [...btn.closest('tr').querySelectorAll('td')].map((td) => td.textContent.trim());
        };
        const admin = cells(1);
        expect(admin[3]).toBe('2026-01-01 09:00:00');
        expect(admin[4]).toBe('-');
        const alice = cells(2);
        expect(alice[3]).toBe('2026-04-23 01:43:45');
        expect(alice[4]).toBe('2026-05-27 04:28:14');
    });

    test('[L30] an admin sees the Add User button and its footer', () => {
        const addBtn = document.getElementById('add-user-btn');
        expect(isHiddenOrAbsent(addBtn)).toBe(false);
        expect(isHiddenOrAbsent(addBtn.closest('.section-footer'))).toBe(false);
    });
});
