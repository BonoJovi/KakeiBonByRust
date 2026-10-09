// The account master shows its input checks and load errors through i18n
/**
 * Account master screen (res/js/account-management.js).
 *
 * `saveAccount()` showed hard-coded English for its input checks
 * ('Account code is required', 'Account name is required', 'Template is
 * required', 'Initial balance must be a number'), and `loadAccounts()`
 * showed 'Error loading accounts: …' — English text on the JA screen.
 * The code / template / balance messages also targeted `*-error` elements
 * that the page does not have, so they never appeared at all.
 *
 * Expected: each message comes from i18n (the harness stub echoes the key)
 * and is shown next to its input.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

let failLoad = false;

const { invoke } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_account_templates':
                return [{
                    template_id: 1, template_code: 'BANK', template_name_ja: '銀行',
                    template_name_en: 'Bank', display_order: 1, entry_dt: '',
                }];
            case 'get_accounts':
                if (failLoad) return Promise.reject({ code: 'database', message: 'disk I/O error' });
                return [];
            default:
                return null;
        }
    },
});

window.confirm = () => true;
window.alert = () => {};

loadPageBody('account-management.html');
await import('../../../res/js/account-management.js');
await bootPage();

async function submitAccount({ code, name, template, balance }) {
    invoke.mockClear();
    document.getElementById('add-account-btn').click();
    await flush(5);
    document.getElementById('account-code').value = code;
    document.getElementById('account-name').value = name;
    document.getElementById('template-code').value = template;
    document.getElementById('initial-balance').value = balance;
    document.getElementById('account-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
    await flush(10);
    expect(callsOf(invoke, 'add_account')).toHaveLength(0);
}

// The inline message next to an input (showValidationError).
function inlineError(id) {
    const el = document.getElementById(id).parentElement.querySelector('.validation-error');
    return el ? el.textContent : '';
}

describe('account master input checks use i18n', () => {
    test('should show the i18n required message when the account code is empty', async () => {
        await submitAccount({ code: '', name: 'Main Bank', template: 'BANK', balance: '0' });
        expect(inlineError('account-code')).toBe('validation.required');
    });

    test('should show the i18n required message when the account name is empty', async () => {
        await submitAccount({ code: 'BANK1', name: '  ', template: 'BANK', balance: '0' });
        expect(inlineError('account-name')).toBe('validation.required');
    });

    test('should show the i18n required message when no template is selected', async () => {
        await submitAccount({ code: 'BANK1', name: 'Main Bank', template: '', balance: '0' });
        expect(inlineError('template-code')).toBe('validation.required');
    });

    test('should show the i18n amount message when the initial balance is empty', async () => {
        await submitAccount({ code: 'BANK1', name: 'Main Bank', template: 'BANK', balance: '' });
        expect(inlineError('initial-balance'))
            .toBe('common.error_amount_not_integer');
    });
});

describe('account list load error uses i18n', () => {
    test('should show only the localized message, not the backend detail, when the list fails to load', async () => {
        failLoad = true;
        document.getElementById('toggle-disabled-btn').click();
        await flush(10);
        const text = document.getElementById('accounts-tbody').textContent;
        expect(text).not.toContain('Error loading accounts');
        expect(text).not.toContain('disk I/O error');
        expect(text.trim()).toBe('account_mgmt.failed_to_load');
    });
});
