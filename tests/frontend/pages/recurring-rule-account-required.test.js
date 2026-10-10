/**
 * Recurring rule screen (res/js/recurring-rule.js) — the account a category
 * needs.
 *
 * An expense needs a From account, an income a To account and a transfer
 * both. A rule with that side left "Unspecified" (NONE) used to be created,
 * and every generated transaction was counted as an expense or income on no
 * account. The screen now refuses it before create_recurring_rule with the
 * same messages as the transaction screen, and shows the backend's
 * `account_required` refusal with its own message.
 *
 * The real page module is booted against res/recurring-rule.html via
 * ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const CATEGORY_TREE = ['EXPENSE', 'INCOME', 'TRANSFER'].map((code) => ({
    category1: { category1_code: code, category1_name_i18n: code },
    children: [],
}));

// When set, create_recurring_rule rejects with this error once.
let createRejection = null;

const { invoke } = mockPageModules(jest, {
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return CATEGORY_TREE;
            case 'get_accounts':
                return [
                    { account_code: 'CASH', account_name: 'Cash' },
                    { account_code: 'BANK', account_name: 'Bank' },
                ];
            case 'get_shops':
                return [];
            case 'list_recurring_rules':
                return [];
            case 'create_recurring_rule':
                if (createRejection) {
                    const err = createRejection;
                    createRejection = null;
                    return Promise.reject(err);
                }
                return { rule_id: 1, generated_count: 3 };
            default:
                return null;
        }
    },
});

loadPageBody('recurring-rule.html');
await import('../../../res/js/recurring-rule.js');
await bootPage();

async function fillForm(category1Code, fromAccount, toAccount) {
    document.getElementById('rule-name').value = 'Rule';
    const category1 = document.getElementById('category1');
    category1.value = category1Code;
    category1.dispatchEvent(new Event('change'));
    await flush();
    document.getElementById('from-account').value = fromAccount;
    document.getElementById('to-account').value = toAccount;
    document.getElementById('item-name').value = 'Item';
    document.getElementById('total-amount').value = '1000';
    document.getElementById('tax-rate').value = '0';
    document.getElementById('amount-excluding-tax').value = '1000';
    document.getElementById('amount-including-tax').value = '1000';
    document.getElementById('tax-amount').value = '0';
}

async function submitForm() {
    document.getElementById('recurring-rule-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
    await flush(10);
}

const creates = () => callsOf(invoke, 'create_recurring_rule');
const errorShown = () => {
    const box = document.getElementById('result-box');
    return box.classList.contains('error') ? box.textContent : null;
};

describe('recurring rule form — the account a category needs', () => {
    beforeEach(() => {
        invoke.mockClear();
    });

    test('should refuse an expense rule when its From account is unspecified', async () => {
        await fillForm('EXPENSE', 'NONE', 'NONE');
        await submitForm();
        expect(creates()).toHaveLength(0);
        expect(errorShown()).toBe('transaction_mgmt.from_account_required');
    });

    test('should refuse an income rule when its To account is unspecified', async () => {
        await fillForm('INCOME', 'CASH', 'NONE');
        await submitForm();
        expect(creates()).toHaveLength(0);
        expect(errorShown()).toBe('transaction_mgmt.to_account_required');
    });

    test('should refuse a transfer rule when its From account is unspecified', async () => {
        await fillForm('TRANSFER', 'NONE', 'BANK');
        await submitForm();
        expect(creates()).toHaveLength(0);
        expect(errorShown()).toBe('transaction_mgmt.from_account_required');
    });

    test('should refuse a transfer rule when its To account is unspecified', async () => {
        await fillForm('TRANSFER', 'CASH', 'NONE');
        await submitForm();
        expect(creates()).toHaveLength(0);
        expect(errorShown()).toBe('transaction_mgmt.to_account_required');
    });

    test('should create an income rule when only its To account is chosen', async () => {
        await fillForm('INCOME', 'NONE', 'BANK');
        await submitForm();
        expect(creates()).toHaveLength(1);
        expect(creates()[0].request).toMatchObject({ from_account_code: 'NONE', to_account_code: 'BANK' });
    });

    test('should show the account-required message when the backend refuses with account_required', async () => {
        await fillForm('EXPENSE', 'CASH', 'NONE');
        createRejection = { code: 'account_required', message: 'An account is required for this category' };
        await submitForm();
        expect(creates()).toHaveLength(1);
        expect(errorShown()).toBe('transaction_mgmt.account_required');
    });
});
