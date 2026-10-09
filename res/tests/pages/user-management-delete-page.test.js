/**
 * User management screen (res/js/user-management.js) — deleting a user.
 *
 * The admin deletes a general user from the user list: the delete button
 * opens the delete window with the user's name, "Delete" calls
 * delete_general_user_info for that user, and on success the screen shows
 * `user_mgmt.user_deleted` and reloads the list. Cancel and × close the
 * window without deleting. An admin row has no delete button. Errors from
 * the backend keep the window open: `admin_protected` has its own message,
 * anything else shows `error.delete_user_failed` with the backend message.
 * (The `last_general_user` message is tested in
 * ./user-management-delete-last-user.test.js.)
 *
 * These replace a former test file that tested copies of this logic. The
 * real page module is booted against res/user-management.html via
 * ./_page-harness.js. Each test boots the page again (page body reloaded,
 * DOMContentLoaded fired again) so it starts from a fresh list.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const ADMIN = { user_id: 1, name: 'admin', role: 0 };

function userRow(user_id, name, role = 1) {
    return { user_id, name, role, entry_dt: '2026-01-01 00:00:00', update_dt: null };
}

// What list_users returns; a successful delete removes the user from it.
let users = [];
// When set, delete_general_user_info rejects with this value.
let deleteError = null;

const { invoke, showToast } = mockPageModules(jest, {
    user: ADMIN,
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'list_users':
                return users.map((u) => ({ ...u }));
            case 'delete_general_user_info':
                if (deleteError) return Promise.reject(deleteError);
                users = users.filter((u) => u.user_id !== args.userId);
                return null;
            default:
                return null;
        }
    },
});

loadPageBody('user-management.html');
await import('../../js/user-management.js');
await bootPage();

async function bootWith(list) {
    users = list;
    loadPageBody('user-management.html');
    await bootPage();
}

const deleteModalOpen = () =>
    !document.getElementById('delete-modal').classList.contains('hidden');

const listedUserIds = () =>
    [...document.querySelectorAll('#user-list tr')].map((tr) => tr.cells[0].textContent);

async function clickDeleteFor(userId) {
    document.querySelector(`.btn-delete[data-user-id="${userId}"]`).click();
    await flush(3);
}

async function confirmDelete() {
    document.getElementById('confirm-delete').click();
    await flush(10);
}

describe('user management screen — deleting a user', () => {
    beforeEach(() => {
        deleteError = null;
        invoke.mockClear();
        showToast.mockClear();
    });

    test('should show a delete button only on general user rows when the admin views the list', async () => {
        await bootWith([userRow(1, 'admin', 0), userRow(2, 'alice'), userRow(3, 'bob')]);

        const buttons = [...document.querySelectorAll('#user-list .btn-delete')];
        expect(buttons.map((b) => b.dataset.userId)).toEqual(['2', '3']);
    });

    test('should open the delete window with the user name as it is when a delete button is clicked', async () => {
        await bootWith([userRow(1, 'admin', 0), userRow(2, '山田 太郎')]);

        await clickDeleteFor(2);

        expect(deleteModalOpen()).toBe(true);
        expect(document.getElementById('delete-username').textContent).toBe('山田 太郎');
        expect(callsOf(invoke, 'delete_general_user_info')).toHaveLength(0);
    });

    test.each([
        ['cancel-delete'],
        ['close-delete-modal'],
    ])('should close the delete window without deleting when #%s is clicked', async (buttonId) => {
        await bootWith([userRow(1, 'admin', 0), userRow(2, 'alice')]);
        await clickDeleteFor(2);

        document.getElementById(buttonId).click();
        await flush(3);

        expect(deleteModalOpen()).toBe(false);
        expect(callsOf(invoke, 'delete_general_user_info')).toHaveLength(0);
    });

    test('should delete the user picked last when the window was cancelled and opened for another user', async () => {
        await bootWith([userRow(1, 'admin', 0), userRow(2, 'alice'), userRow(3, 'bob')]);
        await clickDeleteFor(2);
        document.getElementById('cancel-delete').click();
        await flush(3);

        await clickDeleteFor(3);
        expect(document.getElementById('delete-username').textContent).toBe('bob');
        await confirmDelete();

        expect(callsOf(invoke, 'delete_general_user_info')).toEqual([{ userId: 3 }]);
    });

    test('should show the success message, reload the list and close the window when the delete succeeds', async () => {
        await bootWith([userRow(1, 'admin', 0), userRow(2, 'alice'), userRow(3, 'bob')]);
        invoke.mockClear();

        await clickDeleteFor(2);
        await confirmDelete();

        expect(callsOf(invoke, 'delete_general_user_info')).toEqual([{ userId: 2 }]);
        expect(showToast).toHaveBeenCalledWith('user_mgmt.user_deleted', { variant: 'success' });
        expect(callsOf(invoke, 'list_users')).toHaveLength(1);
        expect(listedUserIds()).toEqual(['1', '3']);
        expect(deleteModalOpen()).toBe(false);
    });

    test('should show the generic failure with the backend message and keep the window open when the user is not found', async () => {
        await bootWith([userRow(1, 'admin', 0), userRow(2, 'alice')]);
        deleteError = { code: 'not_found', message: 'User not found', entity: 'user' };

        await clickDeleteFor(2);
        await confirmDelete();

        expect(showToast).toHaveBeenCalledWith('error.delete_user_failed: User not found', { variant: 'error' });
        expect(deleteModalOpen()).toBe(true);
        expect(listedUserIds()).toEqual(['1', '2']);
    });

    test('should show the admin-protected message when the backend refuses to delete an admin', async () => {
        await bootWith([userRow(1, 'admin', 0), userRow(2, 'alice')]);
        deleteError = { code: 'admin_protected', message: 'Admin user cannot be deleted' };

        await clickDeleteFor(2);
        await confirmDelete();

        expect(showToast).toHaveBeenCalledWith('user_mgmt.admin_protected', { variant: 'error' });
        expect(deleteModalOpen()).toBe(true);
    });

    test('should show a name with HTML markup as plain text when the name contains markup', async () => {
        const name = '<b>x</b>"&';
        await bootWith([userRow(1, 'admin', 0), userRow(2, name)]);

        expect(document.querySelector('#user-list b')).toBeNull();
        expect(document.querySelector('#user-list tr:nth-child(2)').cells[1].textContent).toBe(name);

        await clickDeleteFor(2);
        expect(document.getElementById('delete-username').textContent).toBe(name);
        expect(document.querySelector('#delete-username b')).toBeNull();
    });
});
