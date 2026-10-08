// The account code is limited to 50 characters on add; existing longer codes stay editable
/**
 * Account master screen (res/js/account-management.js).
 *
 * The account code had no length limit anywhere: the add form had no
 * counter or check, and the backend only rejected an empty code and NONE,
 * so a 256+ character code could be saved (ACCOUNTS.ACCOUNT_CODE is
 * VARCHAR(50), which SQLite does not enforce).
 *
 * Expected:
 * - Add: the code field shows a "n / 50" counter and is cut at 50
 *   characters; a longer code is stopped before add_account with the
 *   max-length message.
 * - Edit: the code field is read-only, has no counter, and an existing
 *   code longer than 50 characters is sent to update_account unchanged
 *   (the code is only the key of the row being updated).
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf, isHiddenOrAbsent,
} from './_page-harness.js';

const MAX_ACCOUNT_CODE_LEN = 50;
const LONG_CODE = 'A'.repeat(300);

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
                return [account('NONE', '指定なし', 0), account(LONG_CODE, 'Long code bank', 1)];
            case 'add_account':
            case 'update_account':
                return 'ok';
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

const codeInput = () => document.getElementById('account-code');
const counterText = () => {
    const el = codeInput().parentElement.querySelector('.char-counter');
    return el ? el.textContent : null;
};
const errorText = () => {
    const el = codeInput().parentElement.querySelector('.validation-error');
    return el ? el.textContent : '';
};

async function openAdd() {
    document.getElementById('add-account-btn').click();
    await flush(5);
}

async function submitForm(code) {
    codeInput().value = code;
    document.getElementById('account-name').value = 'Bank';
    document.getElementById('template-code').value = 'BANK';
    document.getElementById('initial-balance').value = '0';
    document.getElementById('account-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
    await flush(10);
}

describe('account code length limit', () => {
    beforeEach(() => invoke.mockClear());

    test('should show a 50-character counter and cut typed input at 50 when an account is added', async () => {
        await openAdd();
        expect(counterText()).toBe(`0 / ${MAX_ACCOUNT_CODE_LEN}`);

        codeInput().value = 'b'.repeat(60);
        codeInput().dispatchEvent(new Event('input'));
        expect(codeInput().value).toBe('B'.repeat(MAX_ACCOUNT_CODE_LEN));
        expect(counterText()).toBe(`${MAX_ACCOUNT_CODE_LEN} / ${MAX_ACCOUNT_CODE_LEN}`);
    });

    test('should stop a 51-character code before add_account with the max-length message when an account is added', async () => {
        const modal = document.getElementById('account-modal');
        if (isHiddenOrAbsent(modal)) await openAdd();

        await submitForm('C'.repeat(MAX_ACCOUNT_CODE_LEN + 1));

        expect(callsOf(invoke, 'add_account')).toHaveLength(0);
        expect(isHiddenOrAbsent(modal)).toBe(false);
        expect(errorText()).toContain('validation.max_length');
        expect(errorText()).toContain(`max=${MAX_ACCOUNT_CODE_LEN}`);
    });

    test('should send a 50-character code to add_account when an account is added', async () => {
        const modal = document.getElementById('account-modal');
        if (isHiddenOrAbsent(modal)) await openAdd();

        await submitForm('D'.repeat(MAX_ACCOUNT_CODE_LEN));

        const calls = callsOf(invoke, 'add_account');
        expect(calls).toHaveLength(1);
        expect(calls[0].accountCode).toBe('D'.repeat(MAX_ACCOUNT_CODE_LEN));
    });

    test('should keep an existing code longer than 50 characters and send it to update_account when an account is edited', async () => {
        const editBtn = [...document.querySelectorAll('.btn-edit')]
            .find((b) => b.dataset.code === LONG_CODE);
        expect(editBtn).toBeTruthy();
        editBtn.click();
        await flush(5);

        expect(codeInput().hasAttribute('readonly')).toBe(true);
        expect(codeInput().value).toBe(LONG_CODE);
        expect(counterText()).toBeNull();

        document.getElementById('account-name').value = 'Renamed bank';
        document.getElementById('account-form').dispatchEvent(
            new Event('submit', { cancelable: true, bubbles: true })
        );
        await flush(10);

        const calls = callsOf(invoke, 'update_account');
        expect(calls).toHaveLength(1);
        expect(calls[0].accountCode).toBe(LONG_CODE);
    });
});
