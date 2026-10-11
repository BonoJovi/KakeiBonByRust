/**
 * User management screen (res/user-management.html, handled by
 * res/js/user-management.js), general-user session — the checks when the
 * user edits their own row. The checks are shared with the administrator
 * session in ./_user-self-edit-checks.js; a general user's own row is
 * renamed with update_general_user_info and its password changed with
 * update_general_user_with_reencryption.
 *
 * This replaces general-user-edit.test.js, which tested a copy of these
 * checks written for the tests (validation-helpers.js). The real page
 * module is booted against res/user-management.html via ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import { mockPageModules, loadPageBody, bootPage } from './_page-harness.js';
import { describeSelfEditChecks } from './_user-self-edit-checks.js';

const ADMIN = { user_id: 1, name: 'admin', role: 0, entry_dt: '2026-01-01 00:00:00', update_dt: null };
const ALICE = { user_id: 2, name: 'alice', role: 1, entry_dt: '2026-01-02 00:00:00', update_dt: null };

const { invoke } = mockPageModules(jest, {
    user: { user_id: ALICE.user_id, name: ALICE.name, role: ALICE.role },
    invoke: (cmd) => (cmd === 'list_users' ? [ADMIN, ALICE] : null),
});

loadPageBody('user-management.html');
await import('../../../res/js/user-management.js');
await bootPage();

describeSelfEditChecks({
    invoke,
    self: ALICE,
    renameCommand: 'update_general_user_info',
    reencryptCommand: 'update_general_user_with_reencryption',
});
