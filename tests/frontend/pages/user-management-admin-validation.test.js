/**
 * User management screen (res/user-management.html, handled by
 * res/js/user-management.js), administrator session — the checks before a
 * user is added or edited.
 *
 * Adding a user checks the user name (blank after trimming →
 * validation.required under the field), then the password: the
 * confirmation (error.password_mismatch), only whitespace
 * (error.password_empty) and fewer than 16 characters counted as code
 * points (error.password_too_short), shown in #form-message. An accepted
 * user is sent to create_general_user with the trimmed name. The password
 * fields are required, so the Save button does not submit without one.
 *
 * Editing another user's row hides the password fields and only renames.
 * Editing the administrator's own row is tested with the shared checks in
 * ./_user-self-edit-checks.js.
 *
 * These replace user-addition.test.js and admin-edit.test.js, which tested
 * a copy of these checks written for the tests (validation-helpers.js).
 * The real page module is booted against res/user-management.html via
 * ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';
import { describeSelfEditChecks } from './_user-self-edit-checks.js';

const ADMIN = { user_id: 1, name: 'admin', role: 0, entry_dt: '2026-01-01 00:00:00', update_dt: null };
const ALICE = { user_id: 2, name: 'alice', role: 1, entry_dt: '2026-01-02 00:00:00', update_dt: null };
const SIXTEEN = '1234567890123456';

const { invoke } = mockPageModules(jest, {
    user: { user_id: ADMIN.user_id, name: ADMIN.name, role: ADMIN.role },
    invoke: (cmd) => {
        switch (cmd) {
            case 'list_users':
                return [ADMIN, ALICE];
            case 'create_general_user':
                return 5;
            default:
                return null;
        }
    },
});

loadPageBody('user-management.html');
await import('../../../res/js/user-management.js');
await bootPage();

const field = (id) => document.getElementById(id);
const formMessage = () => field('form-message').textContent;
const usernameError = () => field('username').parentElement.querySelector('.validation-error')?.textContent ?? '';

async function openAdd() {
    field('cancel-btn').click();
    await flush(3);
    field('add-user-btn').click();
    await flush(3);
    invoke.mockClear();
}

function fill(username, password, confirm = password) {
    field('username').value = username;
    field('password').value = password;
    field('password-confirm').value = confirm;
}

async function submit() {
    field('user-form').dispatchEvent(new Event('submit', { cancelable: true, bubbles: true }));
    await flush(10);
}

describe('adding a user', () => {
    test.each([
        ['the user name is empty', ''],
        ['the user name is only spaces and tabs', ' \t '],
    ])('should show the required message under the user name and not add when %s', async (_label, username) => {
        await openAdd();
        fill(username, SIXTEEN);
        await submit();

        expect(usernameError()).toBe('validation.required');
        expect(callsOf(invoke, 'create_general_user')).toHaveLength(0);
    });

    test.each([
        ['the confirmation differs', SIXTEEN, '1234567890123457', 'error.password_mismatch'],
        ['the confirmation is empty', SIXTEEN, '', 'error.password_mismatch'],
        ['the password has 15 characters', '123456789012345', '123456789012345', 'error.password_too_short'],
        ['the password has 15 characters but 19 UTF-16 units', 'password🔐🔐🔐🔐123', 'password🔐🔐🔐🔐123', 'error.password_too_short'],
        ['the password is short and the confirmation differs', 'short', 'different', 'error.password_mismatch'],
    ])('should refuse with the right message and not add when %s', async (_label, password, confirm, expected) => {
        await openAdd();
        fill('bob', password, confirm);
        await submit();

        expect(formMessage()).toBe(expected);
        expect(callsOf(invoke, 'create_general_user')).toHaveLength(0);
    });

    test('should not submit the form when the Save button is pressed with no password', async () => {
        await openAdd();
        fill('bob', '');
        field('user-form').querySelector('button[type="submit"]').click();
        await flush(10);

        expect(callsOf(invoke, 'create_general_user')).toHaveLength(0);
    });

    test.each([
        ['the password has exactly 16 characters', SIXTEEN],
        ['the password has 16 characters including emoji', 'abcdefghijkl🔐🔐🔐🔐'],
        ['the password is in Japanese', 'パスワードパスワードパスワードパス'],
    ])('should add the user with the password as typed when %s', async (_label, password) => {
        await openAdd();
        fill('bob', password);
        await submit();

        expect(callsOf(invoke, 'create_general_user')).toEqual([{ username: 'bob', password }]);
    });

    test('should add the user name without surrounding spaces when it has them', async () => {
        await openAdd();
        fill('  bob  ', SIXTEEN);
        await submit();

        expect(callsOf(invoke, 'create_general_user')).toEqual([{ username: 'bob', password: SIXTEEN }]);
    });
});

describe('editing another user', () => {
    async function openEditAlice() {
        field('cancel-btn').click();
        await flush(3);
        document.querySelector(`.btn-edit[data-user-id="${ALICE.user_id}"]`).click();
        await flush(3);
        invoke.mockClear();
    }

    test('should hide the password fields when another user\'s row is edited', async () => {
        await openEditAlice();

        for (const id of ['old-password-group', 'password-group', 'password-confirm-group']) {
            expect(field(id).style.display).toBe('none');
        }
    });

    test('should rename that user when the user name is changed', async () => {
        await openEditAlice();
        field('username').value = 'alice2';
        await submit();

        expect(callsOf(invoke, 'update_general_user_info')).toEqual([{ userId: ALICE.user_id, username: 'alice2' }]);
    });
});

describeSelfEditChecks({
    invoke,
    self: ADMIN,
    renameCommand: 'update_admin_user_info',
    reencryptCommand: 'update_admin_user_with_reencryption',
});
