/**
 * Latent audit 2026-09 — user management screen, general-user session (res/js/user-management.js)
 *
 * IDs covered: L30
 *
 * Bug: createUserRow() only hides the delete button on ADMIN rows and the
 *      "Add User" button is always visible, so a general (non-admin) user
 *      sees an Add User button (backend create is admin-only → error) and a
 *      Delete button on their own row (self-delete).
 * Expected: for a non-admin session the Add User button is not offered and
 *      the user's own row has no delete button.
 *
 * The real page module is booted against res/user-management.html via
 * ../pages/_page-harness.js with a general-user (role 1) session.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, isHiddenOrAbsent,
} from '../pages/_page-harness.js';

const SELF = { user_id: 2, name: 'alice', role: 1 };

mockPageModules(jest, {
    user: SELF,
    invoke: (cmd) => {
        switch (cmd) {
            case 'list_users':
                return [{ user_id: 2, name: 'alice', role: 1, entry_dt: '2026-01-01 00:00:00', update_dt: null }];
            default:
                return null;
        }
    },
});

loadPageBody('user-management.html');
await import('../../js/user-management.js');
await bootPage();

describe('user management (general user) — latent audit 2026-09', () => {
    test('[latent L30] a non-admin user is not offered the Add User button', () => {
        const addBtn = document.getElementById('add-user-btn');
        expect(isHiddenOrAbsent(addBtn) || addBtn.disabled).toBe(true);
    });

    test('[latent L30] a non-admin user has no delete button on their own row', () => {
        const rows = Array.from(document.querySelectorAll('#user-list tr'));
        expect(rows).toHaveLength(1); // sanity: list rendered
        const deleteBtn = rows[0].querySelector('.btn-delete');
        expect(isHiddenOrAbsent(deleteBtn) || deleteBtn.disabled).toBe(true);
    });
});
