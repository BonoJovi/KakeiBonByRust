/**
 * First-run setup forms (res/js/menu.js on res/index.html) — regression tests
 * for latent-audit L25.
 *
 * L25 register_admin / register_user did not validate the username and a
 *     duplicate name surfaced the raw UNIQUE constraint error. The backend
 *     now rejects blank / over-long names ("Username cannot be empty") and
 *     maps a duplicate to `duplicate_name`. Pinned here:
 *     - a whitespace-only username is stopped before any command is sent
 *       (error.username_required)
 *     - the classifier reports a backend "Username cannot be empty" as the
 *       username error — it used to match only 'cannot be empty' and show
 *       the password message
 *     - a duplicate_name rejection shows error.username_duplicate
 *
 * The real menu.js is booted against res/index.html via ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const VALID_PASSWORD = 'valid_password_123456';

const { invoke } = mockPageModules(jest, {
    keepMenu: true,
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'check_needs_setup':
                return true;
            case 'register_admin':
            case 'register_user':
                // Stand-ins for the backend rejections this suite pins.
                if (args.username === 'taken') {
                    return Promise.reject({ code: 'duplicate_name', message: 'User already exists', entity: 'User' });
                }
                if (args.username === 'backend-blank') {
                    return Promise.reject({ code: 'validation', message: 'Username cannot be empty' });
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

async function submitAdminSetup(username) {
    document.getElementById('admin-username').value = username;
    document.getElementById('admin-password').value = VALID_PASSWORD;
    document.getElementById('admin-password-confirm').value = VALID_PASSWORD;
    document.getElementById('admin-setup-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
    await flush(10);
}

const setupMessage = () => document.getElementById('setup-message').textContent;

describe('setup forms — regression (latent audit 2026-09)', () => {
    beforeEach(() => invoke.mockClear());

    test('should reject the admin setup without calling register_admin when the username is blank (L25)', async () => {
        await submitAdminSetup(' 　 ');

        expect(callsOf(invoke, 'register_admin')).toHaveLength(0);
        expect(setupMessage()).toBe('error.username_required');
    });

    test('should report the username, not the password, when the backend rejects a blank username (L25)', async () => {
        await submitAdminSetup('backend-blank');

        expect(callsOf(invoke, 'register_admin')).toHaveLength(1);
        expect(setupMessage()).toBe('error.username_required');
    });

    test('should show the duplicate-username message when the backend reports duplicate_name (L25)', async () => {
        await submitAdminSetup('taken');

        expect(callsOf(invoke, 'register_admin')).toHaveLength(1);
        expect(setupMessage()).toBe('error.username_duplicate');
    });
});
