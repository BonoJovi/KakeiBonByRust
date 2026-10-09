/**
 * Login screen (res/index.html, handled by res/js/menu.js) — logging in and
 * out.
 *
 * With no active session the index page shows the login form and focuses the
 * user name. Submitting the form sends the user name and password as typed
 * (no trimming, no checks in JavaScript) to login_user. On success the
 * screen shows the success and welcome messages and, one second later,
 * switches to the app (or to the user-setup form, tested in
 * ./index-logout-hides-user-setup.test.js). On failure it shows
 * `error.invalid_credentials` for wrong credentials, otherwise
 * `error.login_failed` with the backend message. Logging out clears the
 * session, empties the form and shows it again; when the session cannot be
 * cleared, the app stays on screen and the failure is shown.
 *
 * These replace a former test file that tested values written inside the
 * test. The real menu.js is booted against res/index.html via
 * ./_page-harness.js (keepMenu). The module boots once for this file, so
 * each test after the first starts by logging out.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf, isHiddenOrAbsent,
} from './_page-harness.js';

const ADMIN = { user_id: 1, name: 'admin', role: 0 };

// What login_user answers: a user object, or a rejection when loginError is set.
let loginUser = ADMIN;
let loginError = null;

const { invoke, showToast } = mockPageModules(jest, {
    keepMenu: true,
    user: ADMIN,
    invoke: (cmd) => {
        switch (cmd) {
            case 'check_needs_setup':
                return false;
            case 'login_user':
                return loginError ? Promise.reject(loginError) : loginUser;
            case 'check_needs_user_setup':
                return false;
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
const loginMessage = () => document.getElementById('login-message');

function fillLogin(username, password) {
    document.getElementById('username').value = username;
    document.getElementById('password').value = password;
}

async function submitLogin() {
    const ev = new Event('submit', { cancelable: true, bubbles: true });
    document.getElementById('login-form-element').dispatchEvent(ev);
    await flush(10);
    return ev;
}

// Log out the way the user does: click Logout in the File menu.
async function clickLogout() {
    document.querySelector('#file-dropdown [data-i18n="menu.logout"]')
        .dispatchEvent(new MouseEvent('click', { bubbles: true, cancelable: true }));
    await flush();
}

const waitForLoginTimer = async () => {
    await new Promise((r) => setTimeout(r, 1100));
    await flush();
};

describe('login screen — at start', () => {
    test('should show the login form and focus the user name when no session is active at start', () => {
        expect(visible('login-form')).toBe(true);
        expect(visible('app-content')).toBe(false);
        expect(visible('admin-setup')).toBe(false);
        expect(document.activeElement).toBe(document.getElementById('username'));
    });

    test('should mask the password when the login form is shown', () => {
        expect(document.getElementById('password').type).toBe('password');
    });
});

describe('login screen — logging in and out', () => {
    beforeEach(async () => {
        loginUser = ADMIN;
        loginError = null;
        await handleLogout();
        await flush();
        invoke.mockClear();
        showToast.mockClear();
        session.clearSession.mockClear();
    });

    test('should cancel the browser form submission when the login form is submitted', async () => {
        fillLogin('admin', 'admin_password123456');
        const ev = await submitLogin();
        expect(ev.defaultPrevented).toBe(true);
    });

    test('should send the user name and password as typed when they have surrounding spaces', async () => {
        fillLogin('  admin  ', '  pass word  ');
        await submitLogin();
        expect(callsOf(invoke, 'login_user')).toEqual([
            { username: '  admin  ', password: '  pass word  ' },
        ]);
    });

    test('should show the success and welcome messages when the login succeeds', async () => {
        loginUser = { user_id: 2, name: 'alice', role: 1 };
        fillLogin('alice', 'alice_password123456');
        await submitLogin();

        expect(loginMessage().textContent).toBe('login.success login.welcome(name=alice)');
        expect(loginMessage().className).toBe('message success');
    });

    test('should show the user name as text when the name in the welcome message contains HTML', async () => {
        loginUser = { user_id: 2, name: '<img src=x onerror=alert(1)>', role: 1 };
        fillLogin('x', 'x_password1234567890');
        await submitLogin();

        expect(loginMessage().querySelector('img')).toBeNull();
        expect(loginMessage().textContent).toContain('<img src=x onerror=alert(1)>');
    });

    test('should show the invalid-credentials message when the user name or password is wrong', async () => {
        loginError = { code: 'auth_invalid_credentials', message: 'Invalid username or password' };
        fillLogin('admin', 'wrong_password1234567');
        await submitLogin();

        expect(loginMessage().textContent).toBe('error.invalid_credentials');
        expect(loginMessage().className).toBe('message error');
        expect(visible('login-form')).toBe(true);
    });

    test.each([
        ['an ApiError from the database', { code: 'database', message: 'disk I/O error' }, 'error.login_failed: disk I/O error'],
        ['a plain string', 'boom', 'error.login_failed: boom'],
    ])('should show the login failure with the backend message when the error is %s', async (_label, error, expected) => {
        loginError = error;
        fillLogin('admin', 'admin_password123456');
        await submitLogin();

        expect(loginMessage().textContent).toBe(expected);
        expect(loginMessage().className).toBe('message error');
    });

    test('should show the app and hide the login form one second after login when no user setup is needed', async () => {
        fillLogin('admin', 'admin_password123456');
        await submitLogin();

        expect(callsOf(invoke, 'check_needs_user_setup')).toHaveLength(1);
        // Before the 1 s switch the login form is still shown.
        expect(visible('login-form')).toBe(true);

        await waitForLoginTimer();

        expect(visible('app-content')).toBe(true);
        expect(visible('login-form')).toBe(false);
        expect(visible('user-setup')).toBe(false);
    });

    test('should clear the session, the user name, the password and the message when the user logs out', async () => {
        loginError = { code: 'auth_invalid_credentials', message: 'Invalid username or password' };
        fillLogin('admin', 'wrong_password1234567');
        await submitLogin();
        expect(loginMessage().textContent).not.toBe('');

        await clickLogout();

        expect(session.clearSession).toHaveBeenCalledTimes(1);
        expect(document.getElementById('username').value).toBe('');
        expect(document.getElementById('password').value).toBe('');
        expect(loginMessage().textContent).toBe('');
        expect(visible('login-form')).toBe(true);
    });

    test('should keep the app on screen and show the failure when the session cannot be cleared at logout', async () => {
        fillLogin('admin', 'admin_password123456');
        await submitLogin();
        await waitForLoginTimer();
        expect(visible('app-content')).toBe(true);

        session.clearSession.mockRejectedValueOnce('session busy');
        await clickLogout();

        expect(showToast).toHaveBeenCalledWith('error.logout_failed: session busy', { variant: 'error' });
        expect(visible('app-content')).toBe(true);
        expect(visible('login-form')).toBe(false);
    });
});
