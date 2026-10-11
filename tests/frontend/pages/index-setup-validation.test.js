/**
 * Setup forms on the start screen (res/index.html, handled by
 * res/js/menu.js) — the checks before the first administrator or a user is
 * registered.
 *
 * Both forms check, in this order: the user name (blank after trimming →
 * error.username_required), the password (blank or only whitespace →
 * error.password_empty; fewer than 16 characters, counted as code points
 * like the backend → error.password_too_short) and the confirmation (not
 * exactly the same → error.password_mismatch). A refused form calls
 * neither register_admin nor register_user. An accepted form sends the
 * trimmed user name and the password as typed.
 *
 * These replace admin-setup.test.js and user-addition.test.js, which
 * tested a copy of these checks written for the tests
 * (validation-helpers.js). The copy counted UTF-16 units, so it accepted
 * "password🔐🔐🔐🔐123" (15 characters), which the app refuses. The real
 * menu.js is booted against res/index.html via ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const { invoke } = mockPageModules(jest, {
    keepMenu: true,
    invoke: (cmd) => {
        switch (cmd) {
            case 'check_needs_setup':
                return true;
            case 'register_admin':
            case 'register_user':
                return 'ok';
            default:
                return null;
        }
    },
});

loadPageBody('index.html');
await import('../../../res/js/menu.js');
await bootPage();

const SIXTEEN = '1234567890123456';

const FORMS = [
    {
        name: 'administrator setup',
        formId: 'admin-setup-form',
        prefix: 'admin-',
        messageId: 'setup-message',
        command: 'register_admin',
        success: 'admin.registration_success',
    },
    {
        name: 'user setup',
        formId: 'user-setup-form',
        prefix: 'user-',
        messageId: 'user-setup-message',
        command: 'register_user',
        success: 'user.registration_success',
    },
];

describe.each(FORMS)('$name form', (form) => {
    const field = (name) => document.getElementById(form.prefix + name);
    const message = () => document.getElementById(form.messageId).textContent;

    async function submit(username, password, confirm = password) {
        field('username').value = username;
        field('password').value = password;
        field('password-confirm').value = confirm;
        document.getElementById(form.formId).dispatchEvent(
            new Event('submit', { cancelable: true, bubbles: true })
        );
        await flush(10);
    }

    beforeEach(() => invoke.mockClear());

    test.each([
        ['the user name is empty', '', SIXTEEN, SIXTEEN, 'error.username_required'],
        ['the user name is only spaces and tabs', ' \t ', SIXTEEN, SIXTEEN, 'error.username_required'],
        ['the password is empty', 'alice', '', '', 'error.password_empty'],
        ['the password is 16 whitespace characters', 'alice', ' \t\n '.repeat(4), ' \t\n '.repeat(4), 'error.password_empty'],
        ['the password has 15 characters', 'alice', '123456789012345', '123456789012345', 'error.password_too_short'],
        ['the password has 15 characters but 19 UTF-16 units', 'alice', 'password🔐🔐🔐🔐123', 'password🔐🔐🔐🔐123', 'error.password_too_short'],
        ['the confirmation differs by one character', 'alice', SIXTEEN, '1234567890123457', 'error.password_mismatch'],
        ['the confirmation differs only in letter case', 'alice', 'Passwordpassword', 'passwordpassword', 'error.password_mismatch'],
        ['the confirmation is empty', 'alice', SIXTEEN, '', 'error.password_mismatch'],
        ['the user name is empty and the password is short', '', 'short', 'short', 'error.username_required'],
        ['the password is short and the confirmation differs', 'alice', 'short', 'different', 'error.password_too_short'],
    ])('should refuse with the right message and not register when %s', async (_label, username, password, confirm, expected) => {
        await submit(username, password, confirm);

        expect(message()).toBe(expected);
        expect(callsOf(invoke, form.command)).toHaveLength(0);
    });

    test.each([
        ['the password has exactly 16 characters', SIXTEEN],
        ['the password has 16 characters including emoji', 'abcdefghijkl🔐🔐🔐🔐'],
        ['the password has symbols', 'p@ss!w0rd#123456'],
        ['the password is in Japanese', 'パスワードパスワードパスワードパス'],
        ['the password has surrounding spaces', ' passwordpassword '],
    ])('should register with the password as typed when %s', async (_label, password) => {
        await submit('alice', password);

        expect(callsOf(invoke, form.command)).toEqual([{ username: 'alice', password }]);
        expect(message()).toBe(form.success);
    });

    test('should register the user name without surrounding spaces when it has them', async () => {
        await submit('  alice  ', SIXTEEN);

        expect(callsOf(invoke, form.command)).toEqual([{ username: 'alice', password: SIXTEEN }]);
    });
});
