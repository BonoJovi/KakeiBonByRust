// A failed account save keeps the form open with the input (latent-audit scan2-M4)
/**
 * Account master screen (res/js/account-management.js).
 *
 * scan2-M4  `saveAccount()` catches backend errors (and `return`s on
 *           validation failures) without rethrowing, so `onSave` resolves and
 *           `Modal._handleSave` closes the modal, which also runs
 *           `form.reset()`. A duplicate account code shows the error toast and
 *           then discards everything the user typed.
 *
 * Expected: after a failed save (duplicate code from the backend) the modal
 * stays open and keeps the typed values, like the other master screens.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf, isHiddenOrAbsent,
} from './_page-harness.js';

const account = (code, name, order) => ({
    account_id: order,
    user_id: 2,
    account_code: code,
    account_name: name,
    template_code: 'BANK',
    initial_balance: 0,
    display_order: order,
    is_disabled: 0,
});

const { invoke, showToast } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_account_templates':
                return [{
                    template_id: 1, template_code: 'BANK', template_name_ja: '銀行',
                    template_name_en: 'Bank', display_order: 1, entry_dt: '',
                }];
            case 'get_accounts':
                return [account('NONE', '指定なし', 0), account('BANK1', 'Main Bank', 1)];
            case 'add_account':
                // An active BANK1 already exists.
                return Promise.reject({ code: 'duplicate_code', message: 'Account code already exists' });
            default:
                return null;
        }
    },
});

window.confirm = () => true;
window.alert = () => {};

loadPageBody('account-management.html');
await import('../../js/account-management.js');
await bootPage();

describe('scan2-M4 account modal on save error', () => {
    test('should keep the window open and the typed input when add_account fails with duplicate_code', async () => {
        document.getElementById('add-account-btn').click();
        await flush(5);

        document.getElementById('account-code').value = 'BANK1';
        document.getElementById('account-name').value = 'Second bank';
        document.getElementById('template-code').value = 'BANK';
        document.getElementById('initial-balance').value = '1000';
        document.getElementById('account-form').dispatchEvent(
            new Event('submit', { cancelable: true, bubbles: true })
        );
        await flush(10);

        // Precondition: the save really went to the backend and failed.
        expect(callsOf(invoke, 'add_account')).toHaveLength(1);
        expect(showToast).toHaveBeenCalled();

        const modal = document.getElementById('account-modal');
        expect(isHiddenOrAbsent(modal)).toBe(false);
        expect(document.getElementById('account-code').value).toBe('BANK1');
        expect(document.getElementById('account-name').value).toBe('Second bank');
    });

    test('should keep the window open when a whitespace-only name is stopped before add_account', async () => {
        invoke.mockClear();
        const modal = document.getElementById('account-modal');
        if (isHiddenOrAbsent(modal)) {
            document.getElementById('add-account-btn').click();
            await flush(5);
        }

        document.getElementById('account-code').value = 'BANK2';
        document.getElementById('account-name').value = '   ';
        document.getElementById('template-code').value = 'BANK';
        document.getElementById('initial-balance').value = '0';
        document.getElementById('account-form').dispatchEvent(
            new Event('submit', { cancelable: true, bubbles: true })
        );
        await flush(10);

        expect(callsOf(invoke, 'add_account')).toHaveLength(0);
        expect(isHiddenOrAbsent(modal)).toBe(false);
        expect(document.getElementById('account-code').value).toBe('BANK2');
    });
});
