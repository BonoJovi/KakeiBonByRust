/**
 * User management screen (res/js/user-management.js) — scan2-C5.
 *
 * Deleting the only general user made every later admin login land on the
 * "register user" setup form (check_needs_user_setup is true while no
 * general user exists). The backend now refuses to delete the last general
 * user with the ApiError code `last_general_user`.
 *
 * Expected: the screen shows `user_mgmt.last_general_user` instead of the
 * generic failure message with the backend's English text.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const ADMIN = { user_id: 1, name: 'admin', role: 0 };
const LAST_GENERAL_USER = {
    code: 'last_general_user',
    message: 'The last general user cannot be deleted',
};

const { invoke, showToast } = mockPageModules(jest, {
    user: ADMIN,
    invoke: (cmd) => {
        switch (cmd) {
            case 'list_users':
                return [
                    { user_id: 1, name: 'admin', role: 0, entry_dt: '2026-01-01 00:00:00', update_dt: null },
                    { user_id: 2, name: 'alice', role: 1, entry_dt: '2026-01-02 00:00:00', update_dt: null },
                ];
            case 'delete_general_user_info':
                return Promise.reject(LAST_GENERAL_USER);
            default:
                return null;
        }
    },
});

loadPageBody('user-management.html');
await import('../../js/user-management.js');
await bootPage();

describe('user management: deleting the last general user (scan2-C5)', () => {
    test('[scan2-C5] shows the dedicated message when the backend refuses', async () => {
        document.querySelector('.btn-delete[data-user-id="2"]').click();
        await flush(3);
        document.getElementById('confirm-delete').click();
        await flush(10);

        expect(callsOf(invoke, 'delete_general_user_info')).toHaveLength(1);
        expect(showToast).toHaveBeenCalledWith('user_mgmt.last_general_user', { variant: 'error' });
    });
});
