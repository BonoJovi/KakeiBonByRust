/**
 * Shared checks for editing one's own user on the user management screen
 * (res/js/user-management.js). Used by
 * ./user-management-admin-validation.test.js (the administrator editing
 * their own row) and ./user-management-self-edit-validation.test.js (a
 * general user editing their own row); the two differ only in the commands
 * the page calls.
 *
 * Editing one's own row shows the current password, password and
 * confirmation fields, none of them required. With no password the page
 * renames through `renameCommand` (username null when unchanged). With a
 * password it checks, in this order: the confirmation (error.password_mismatch),
 * only whitespace (error.password_empty), fewer than 16 characters counted
 * as code points (error.password_too_short) and the current password
 * (error.current_password_required); then it calls `reencryptCommand`.
 * Every message goes to #form-message.
 */

import { flush, callsOf } from './_page-harness.js';

const SIXTEEN = '1234567890123456';

/**
 * @param {object} opts
 * @param {import('@jest/globals').jest.Mock} opts.invoke
 * @param {object} opts.self            the session user's list row
 * @param {string} opts.renameCommand
 * @param {string} opts.reencryptCommand
 */
export function describeSelfEditChecks({ invoke, self, renameCommand, reencryptCommand }) {
    const field = (id) => document.getElementById(id);
    const formMessage = () => field('form-message').textContent;
    const sent = () => [...callsOf(invoke, renameCommand), ...callsOf(invoke, reencryptCommand)];

    async function openSelfEdit() {
        field('cancel-btn')?.click();
        await flush(3);
        document.querySelector(`.btn-edit[data-user-id="${self.user_id}"]`).click();
        await flush(3);
        invoke.mockClear();
    }

    async function save({ username = self.name, oldPassword = '', password = '', confirm = password } = {}) {
        field('username').value = username;
        field('old-password').value = oldPassword;
        field('password').value = password;
        field('password-confirm').value = confirm;
        field('user-form').dispatchEvent(new Event('submit', { cancelable: true, bubbles: true }));
        await flush(10);
    }

    describe('editing one\'s own user', () => {
        test('should show the current password, password and confirmation fields when the own row is edited', async () => {
            await openSelfEdit();

            for (const id of ['old-password-group', 'password-group', 'password-confirm-group']) {
                expect(field(id).style.display).toBe('block');
            }
        });

        test('should rename without a password when only the user name is changed', async () => {
            await openSelfEdit();
            await save({ username: '  renamed  ' });

            expect(callsOf(invoke, renameCommand)).toEqual([{ userId: self.user_id, username: 'renamed' }]);
            expect(callsOf(invoke, reencryptCommand)).toHaveLength(0);
        });

        test('should send no new user name when the user name is unchanged', async () => {
            await openSelfEdit();
            await save();

            expect(callsOf(invoke, renameCommand)).toEqual([{ userId: self.user_id, username: null }]);
        });

        test('should change the password with the current password when all three are entered', async () => {
            await openSelfEdit();
            await save({ oldPassword: 'current-password-1', password: SIXTEEN });

            expect(callsOf(invoke, reencryptCommand)).toEqual([
                { oldPassword: 'current-password-1', username: null, newPassword: SIXTEEN },
            ]);
            expect(callsOf(invoke, renameCommand)).toHaveLength(0);
        });

        test('should send the new user name with the password change when both are changed', async () => {
            await openSelfEdit();
            await save({ username: 'renamed', oldPassword: 'current-password-1', password: SIXTEEN });

            expect(callsOf(invoke, reencryptCommand)[0].username).toBe('renamed');
        });

        const CURRENT = 'current-password-1';
        test.each([
            ['the confirmation differs', { oldPassword: CURRENT, password: SIXTEEN, confirm: '6543210987654321' }, 'error.password_mismatch'],
            ['the confirmation is empty', { oldPassword: CURRENT, password: SIXTEEN, confirm: '' }, 'error.password_mismatch'],
            ['the password is 16 spaces', { oldPassword: CURRENT, password: ' '.repeat(16) }, 'error.password_empty'],
            ['the password has 15 characters', { oldPassword: CURRENT, password: '123456789012345' }, 'error.password_too_short'],
            ['the password has 15 characters but 19 UTF-16 units', { oldPassword: CURRENT, password: 'password🔐🔐🔐🔐123' }, 'error.password_too_short'],
            ['the password is short and the confirmation differs', { oldPassword: CURRENT, password: 'short', confirm: 'different' }, 'error.password_mismatch'],
            ['the current password is not entered', { password: SIXTEEN }, 'error.current_password_required'],
        ])('should refuse with the right message and send nothing when %s', async (_label, values, expected) => {
            await openSelfEdit();
            await save(values);

            expect(formMessage()).toBe(expected);
            expect(sent()).toHaveLength(0);
        });

        test('should refuse an empty user name under the field and send nothing even when the password is valid', async () => {
            await openSelfEdit();
            await save({ username: '   ', oldPassword: 'current-password-1', password: SIXTEEN });

            expect(field('username').parentElement.querySelector('.validation-error').textContent).toBe('validation.required');
            expect(sent()).toHaveLength(0);
        });
    });
}
