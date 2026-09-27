/**
 * Account master screen (res/js/account-management.js) — regression tests for
 * latent-audit M7.
 *
 * M7  An account still used by transactions cannot be deleted, and the
 *     in-use toast says "Disable it instead" — but the account screen had no
 *     way to disable an account or to see disabled ones. Pinned: the edit
 *     form has a "disabled" checkbox that is sent on add / update, and the
 *     list can show disabled accounts (marked with the disabled label), like
 *     the other master screens.
 *
 * The real page module is booted against res/account-management.html via
 * ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, deferred, callsOf,
} from './_page-harness.js';

const account = (code, name, isDisabled, order) => ({
    account_id: order,
    user_id: 2,
    account_code: code,
    account_name: name,
    template_code: 'BANK',
    initial_balance: 0,
    display_order: order,
    is_disabled: isDisabled,
});

const NONE_ACCOUNT = account('NONE', '指定なし', 0, 0);
const ACTIVE_ACCOUNT = account('BANK', 'Main Bank', 0, 1);
const DISABLED_ACCOUNT = account('OLDCARD', 'Old Card', 1, 2);

// When set, get_accounts with includeDisabled parks its response here.
let pendingIncludeDisabled = null;

const { invoke } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_account_templates':
                return [{
                    template_id: 1, template_code: 'BANK', template_name_ja: '銀行',
                    template_name_en: 'Bank', display_order: 1, entry_dt: '',
                }];
            case 'get_accounts':
                if (args && args.includeDisabled && pendingIncludeDisabled) {
                    return pendingIncludeDisabled.promise;
                }
                return args && args.includeDisabled
                    ? [NONE_ACCOUNT, ACTIVE_ACCOUNT, DISABLED_ACCOUNT]
                    : [NONE_ACCOUNT, ACTIVE_ACCOUNT];
            default:
                return null;
        }
    },
});

loadPageBody('account-management.html');
await import('../../js/account-management.js');
await bootPage();

const rows = () => Array.from(document.querySelectorAll('#accounts-tbody tr'));

function submitAccountForm() {
    document.getElementById('account-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
}

describe('account master screen — disable (regression, latent audit 2026-09)', () => {
    beforeEach(() => {
        invoke.mockClear();
    });

    test('[M7] should list disabled accounts, marked, only while "show disabled" is on', async () => {
        expect(rows()).toHaveLength(1); // NONE is never listed

        document.getElementById('toggle-disabled-btn').click();
        await flush(5);

        expect(callsOf(invoke, 'get_accounts')[0]).toEqual({ includeDisabled: true });
        expect(rows()).toHaveLength(2);
        expect(rows()[1].cells[1].textContent).toBe('Old Card[common.disabled_label]');

        document.getElementById('toggle-disabled-btn').click();
        await flush(5);
        expect(rows()).toHaveLength(1);
    });

    test('[M7] should not let a late "show disabled" response overwrite a newer list', async () => {
        pendingIncludeDisabled = deferred();
        document.getElementById('toggle-disabled-btn').click(); // on: response parked
        await flush(2);
        document.getElementById('toggle-disabled-btn').click(); // off: answers at once
        await flush(5);
        expect(rows()).toHaveLength(1);

        pendingIncludeDisabled.resolve([NONE_ACCOUNT, ACTIVE_ACCOUNT, DISABLED_ACCOUNT]);
        pendingIncludeDisabled = null;
        await flush(5);

        expect(rows()).toHaveLength(1);
    });

    test('[M7] should send the disabled checkbox when adding an account', async () => {
        document.getElementById('add-account-btn').click();
        await flush(5);
        expect(document.getElementById('account-is-disabled').checked).toBe(false);

        document.getElementById('account-code').value = 'WALLET';
        document.getElementById('account-name').value = 'Wallet';
        document.getElementById('template-code').value = 'BANK';
        document.getElementById('initial-balance').value = '0';
        document.getElementById('account-is-disabled').checked = true;
        submitAccountForm();
        await flush(10);

        const adds = callsOf(invoke, 'add_account');
        expect(adds).toHaveLength(1);
        expect(adds[0]).toMatchObject({ accountCode: 'WALLET', isDisabled: 1 });
    });

    test('[M7] should show and send the disabled state when editing an account', async () => {
        document.getElementById('toggle-disabled-btn').click();
        await flush(5);

        rows()[1].querySelector('.btn-edit').click();
        await flush(5);
        expect(document.getElementById('account-is-disabled').checked).toBe(true);

        // Enable it again.
        document.getElementById('account-is-disabled').checked = false;
        submitAccountForm();
        await flush(10);

        const updates = callsOf(invoke, 'update_account');
        expect(updates).toHaveLength(1);
        expect(updates[0]).toMatchObject({ accountCode: 'OLDCARD', isDisabled: 0 });

        document.getElementById('toggle-disabled-btn').click();
        await flush(5);
    });
});
