/**
 * Shared harness for the 2026-09 latent-audit frontend tests.
 *
 * Most screens under res/js/ are page scripts: they register a
 * `DOMContentLoaded` listener on import and read/write the DOM of their
 * matching res/*.html page. To exercise the real production code (instead
 * of a mirrored copy of the logic) each test file:
 *
 *   1. calls `mockPageModules(jest, { invoke, user })` BEFORE importing the
 *      page module, which stubs Tauri `invoke` plus the chrome-only modules
 *      (menu bar, font size, language menu, indicators, window fit, session,
 *      toast, i18n) with `jest.unstable_mockModule`;
 *   2. calls `loadPageBody('<page>.html')` to put the real page markup into
 *      jsdom;
 *   3. dynamic-imports the page module and calls `bootPage()` to fire
 *      `DOMContentLoaded` and wait for the async initialiser to settle.
 *
 * Not a test file (no `.test.js` suffix), so Jest never collects it.
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const HERE = path.dirname(fileURLToPath(import.meta.url));
export const RES_DIR = path.resolve(HERE, '../..');

/** Put the <body> of res/<htmlFile> into the jsdom document (scripts stripped). */
export function loadPageBody(htmlFile) {
    const html = fs.readFileSync(path.join(RES_DIR, htmlFile), 'utf8');
    const m = html.match(/<body[^>]*>([\s\S]*)<\/body>/i);
    if (!m) throw new Error(`no <body> in ${htmlFile}`);
    document.body.innerHTML = m[1].replace(/<script[\s\S]*?<\/script>/gi, '');
}

/** All RESOURCE_KEYs defined in res/sql/dbaccess.sql (used to spot raw keys). */
export function definedI18nKeys() {
    const sql = fs.readFileSync(path.join(RES_DIR, 'sql', 'dbaccess.sql'), 'utf8');
    const keys = new Set();
    const re = /\(\s*\d+\s*,\s*'([a-z0-9_.]+)'\s*,\s*'(?:ja|en)'/gi;
    let m;
    while ((m = re.exec(sql)) !== null) keys.add(m[1]);
    return keys;
}

/**
 * i18n stub: `t(key)` echoes the key (plus serialised params) so assertions
 * can check which resource key was looked up.
 */
export function makeI18nStub() {
    const t = (key, params) => {
        if (!params) return key;
        const parts = Object.entries(params).map(([k, v]) => `${k}=${v}`).join(',');
        return `${key}(${parts})`;
    };
    return {
        t,
        init: async () => {},
        updateUI: () => {},
        setLanguage: async () => {},
        getCurrentLanguage: () => 'ja',
        currentLanguage: 'ja',
        initialized: true,
    };
}

/**
 * Register module mocks. Must run before the page module is imported.
 *
 * @param {object} jest   - the `jest` object from '@jest/globals'
 * @param {object} opts
 * @param {Function} opts.invoke   - (cmd, args) => value | Promise
 * @param {object}   [opts.user]   - session user returned by getCurrentSessionUser
 * @param {boolean}  [opts.keepMenu] - true to NOT mock menu.js (menu.js under test)
 * @returns {{ invoke: jest.Mock, showToast: jest.Mock, i18n: object }}
 */
export function mockPageModules(jest, opts) {
    const invoke = jest.fn((cmd, args) => Promise.resolve(opts.invoke(cmd, args)));
    const showToast = jest.fn();
    const i18n = makeI18nStub();
    const user = opts.user ?? { user_id: 2, name: 'alice', role: 1 };

    jest.unstable_mockModule('@tauri-apps/api/core', () => ({ invoke }));
    jest.unstable_mockModule('../../js/i18n.js', () => ({ default: i18n }));
    jest.unstable_mockModule('../../js/toast.js', () => ({
        showToast,
        clearAllToasts: jest.fn(),
    }));
    jest.unstable_mockModule('../../js/indicators.js', () => ({
        setupIndicators: jest.fn(),
        wrapInputFields: jest.fn(),
        setupInputIndicators: jest.fn(),
        setupButtonIndicators: jest.fn(),
    }));
    jest.unstable_mockModule('../../js/font-size.js', () => ({
        setupFontSizeMenuHandlers: jest.fn(),
        setupFontSizeMenu: jest.fn(async () => {}),
        applyFontSize: jest.fn(async () => {}),
        setupFontSizeModalHandlers: jest.fn(),
    }));
    jest.unstable_mockModule('../../js/window-fit.js', () => ({
        fitWindowToScreen: jest.fn(async () => {}),
    }));
    jest.unstable_mockModule('../../js/language-menu.js', () => ({
        setupLanguageMenuHandlers: jest.fn(),
        setupLanguageMenu: jest.fn(async () => {}),
    }));
    jest.unstable_mockModule('../../js/session.js', () => ({
        getCurrentSessionUser: jest.fn(async () => user),
        isSessionAuthenticated: jest.fn(async () => true),
        setSessionSourceScreen: jest.fn(async () => {}),
        getSessionSourceScreen: jest.fn(async () => null),
        clearSessionSourceScreen: jest.fn(async () => {}),
        setSessionCategory1Code: jest.fn(async () => {}),
        getSessionCategory1Code: jest.fn(async () => null),
        clearSessionCategory1Code: jest.fn(async () => {}),
        setSessionModalState: jest.fn(async () => {}),
        getSessionModalState: jest.fn(async () => null),
        clearSessionModalState: jest.fn(async () => {}),
        clearSession: jest.fn(async () => {}),
    }));
    if (!opts.keepMenu) {
        jest.unstable_mockModule('../../js/menu.js', () => ({
            createMenuBar: jest.fn(),
            setupFileMenuHandlers: jest.fn(),
            setupLanguageMenuHandlers: jest.fn(),
            setupLanguageMenu: jest.fn(async () => {}),
            handleLogout: jest.fn(),
            handleQuit: jest.fn(),
            setupCustomValidationMessages: jest.fn(),
        }));
    }
    return { invoke, showToast, i18n };
}

/** Let pending promise callbacks and 0ms timers run. */
export async function flush(times = 5) {
    for (let i = 0; i < times; i++) {
        await new Promise((r) => setTimeout(r, 0));
    }
}

/** Fire DOMContentLoaded and wait for the page's async initialiser. */
export async function bootPage() {
    document.dispatchEvent(new Event('DOMContentLoaded'));
    await flush(10);
}

/** Resolve/reject a promise from the outside. */
export function deferred() {
    let resolve;
    let reject;
    const promise = new Promise((res, rej) => {
        resolve = res;
        reject = rej;
    });
    return { promise, resolve, reject };
}

/** Invocations of `cmd` recorded on the invoke mock. */
export function callsOf(invokeMock, cmd) {
    return invokeMock.mock.calls.filter(([c]) => c === cmd).map(([, args]) => args);
}

/** True when the element is absent or hidden by any of the usual means. */
export function isHiddenOrAbsent(el) {
    for (let node = el; node && node !== document.body; node = node.parentElement) {
        if (node.hidden) return true;
        if (node.classList?.contains('hidden')) return true;
        if (node.style?.display === 'none') return true;
        if (node.style?.visibility === 'hidden') return true;
    }
    return !el || !el.isConnected;
}
