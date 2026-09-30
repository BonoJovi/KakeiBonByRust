// latent-audit scan2-C6: logging out from the index page while the user-setup form is shown leaves that form visible next to the login form
/**
 * First run: the admin logs in, check_needs_user_setup is true, so menu.js
 * hides the login form and shows #user-setup. File > Logout (handleLogout)
 * only toggles #login-form / #app-content, so #user-setup stays on screen
 * next to the login form; submitting it then fails with "User not
 * authenticated".
 * Expected: after logout only the login form is shown (#user-setup and
 * #admin-setup are hidden).
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, isHiddenOrAbsent,
} from '../pages/_page-harness.js';

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
                return true;
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

describe('scan2-C6 — logout from the user-setup step', () => {
    test('logout hides the user-setup form and shows only the login form', async () => {
        expect(visible('login-form')).toBe(true);

        document.getElementById('username').value = 'admin';
        document.getElementById('password').value = 'admin_password123456';
        document.getElementById('login-form-element').dispatchEvent(
            new Event('submit', { cancelable: true, bubbles: true })
        );
        await flush(10);
        // menu.js switches to the user-setup step after a 1 s delay.
        await new Promise((r) => setTimeout(r, 1100));
        await flush();
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
});
