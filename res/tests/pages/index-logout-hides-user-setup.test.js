/**
 * First run: the admin logs in, check_needs_user_setup is true, so menu.js
 * hides the login form and shows #user-setup. File > Logout (handleLogout)
 * only toggles #login-form / #app-content, so #user-setup stays on screen
 * next to the login form; submitting it then fails with "User not
 * authenticated".
 * Logging out within 1 s of logging in had the same result: the pending
 * timer showed #user-setup after the logout.
 *
 * Expected: after logout only the login form is shown (#user-setup and
 * #admin-setup are hidden), also once the login's pending timer has run and
 * when the login's check_needs_user_setup answers only after the logout
 * (CodeRabbit CLI pre-review).
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, isHiddenOrAbsent, deferred,
} from './_page-harness.js';

// When set, check_needs_user_setup answers through this deferred.
let heldSetupCheck = null;

mockPageModules(jest, {
    keepMenu: true,
    user: { user_id: 1, name: 'admin', role: 0 },
    invoke: (cmd) => {
        switch (cmd) {
            case 'check_needs_setup':
                return false;
            case 'login_user':
                return { user_id: 1, name: 'admin', role: 0 };
            case 'check_needs_user_setup':
                return heldSetupCheck ? heldSetupCheck.promise : true;
            default:
                return null;
        }
    },
});

// Start logged out, so the index page shows the login form.
const session = await import('../../js/session.js');
session.isSessionAuthenticated.mockResolvedValue(false);

loadPageBody('index.html');
const { handleLogout } = await import('../../js/menu.js');
await bootPage();

const visible = (id) => !isHiddenOrAbsent(document.getElementById(id));

async function submitLogin() {
    document.getElementById('username').value = 'admin';
    document.getElementById('password').value = 'admin_password123456';
    document.getElementById('login-form-element').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
    await flush(10);
}

const waitForLoginTimer = async () => {
    await new Promise((r) => setTimeout(r, 1100));
    await flush();
};

describe('logout from the user-setup step (scan2-C6)', () => {
    test('[scan2-C6] logout hides the user-setup form and shows only the login form', async () => {
        expect(visible('login-form')).toBe(true);

        await submitLogin();
        // menu.js switches to the user-setup step after a 1 s delay.
        await waitForLoginTimer();
        // Precondition: the user-setup step is on screen.
        expect(visible('user-setup')).toBe(true);
        expect(visible('login-form')).toBe(false);

        await handleLogout();
        await flush();

        expect(visible('login-form')).toBe(true);
        expect(visible('user-setup')).toBe(false);
        expect(visible('admin-setup')).toBe(false);
        expect(visible('app-content')).toBe(false);
    });

    test('[scan2-C6] logging out before the login timer runs keeps only the login form', async () => {
        await submitLogin();
        await handleLogout();
        await flush();

        await waitForLoginTimer();

        expect(visible('login-form')).toBe(true);
        expect(visible('user-setup')).toBe(false);
        expect(visible('app-content')).toBe(false);
    });

    test('[scan2-C6] a setup check answering after the logout does not switch screens', async () => {
        heldSetupCheck = deferred();
        try {
            await submitLogin();
            await handleLogout();
            await flush();

            heldSetupCheck.resolve(true);
            await flush();
            await waitForLoginTimer();

            expect(visible('login-form')).toBe(true);
            expect(visible('user-setup')).toBe(false);
            expect(visible('app-content')).toBe(false);
        } finally {
            heldSetupCheck = null;
        }
    });
});
