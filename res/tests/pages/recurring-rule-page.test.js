/**
 * Recurring rule screen (res/js/recurring-rule.js) — regression tests
 * promoted from the 2026-09 latent audit.
 *
 * M16 Creating a recurring rule skipped the write validation a normal
 *     transaction gets. Reachable from the UI: a TRANSFER template whose
 *     from/to accounts are the same (both default to NONE) generated N
 *     self-transfers that the regular edit path then refused to save. The
 *     backend now rejects it (transfer_same_account); this pins the
 *     frontend guard: the transaction screen's message is shown and
 *     create_recurring_rule is never sent.
 *
 * The real page module is booted against res/recurring-rule.html via
 * ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const CATEGORY_TREE = [
    {
        category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' },
        children: [
            {
                category2: { category2_code: 'C2_E_1', category2_name_i18n: 'Food' },
                children: [{ category3_code: 'C3_1', category3_name_i18n: 'Veg' }],
            },
        ],
    },
    {
        category1: { category1_code: 'TRANSFER', category1_name_i18n: 'Transfer' },
        children: [],
    },
];

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
                return { rule_id: 1, generated_count: 12 };
            default:
                return null;
        }
    },
});

loadPageBody('recurring-rule.html');
await import('../../js/recurring-rule.js');
await bootPage();

async function fillTransferForm(fromAccount, toAccount) {
    document.getElementById('rule-name').value = 'Savings';
    const category1 = document.getElementById('category1');
    category1.value = 'TRANSFER';
    category1.dispatchEvent(new Event('change'));
    await flush();
    document.getElementById('from-account').value = fromAccount;
    document.getElementById('to-account').value = toAccount;
    document.getElementById('item-name').value = 'Savings';
    document.getElementById('total-amount').value = '10000';
    document.getElementById('tax-rate').value = '0';
    document.getElementById('amount-excluding-tax').value = '10000';
    document.getElementById('amount-including-tax').value = '10000';
    document.getElementById('tax-amount').value = '0';
}

async function submitForm() {
    document.getElementById('recurring-rule-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
    await flush(10);
}

describe('recurring rule form — regression (latent audit 2026-09)', () => {
    beforeEach(() => {
        invoke.mockClear();
    });

    test('[M16] a TRANSFER from an account to itself is rejected before create_recurring_rule', async () => {
        await fillTransferForm('CASH', 'CASH');
        // Sanity: the options exist, so the selection really is CASH → CASH.
        expect(document.getElementById('from-account').value).toBe('CASH');
        expect(document.getElementById('to-account').value).toBe('CASH');

        await submitForm();

        expect(callsOf(invoke, 'create_recurring_rule')).toHaveLength(0);
        const box = document.getElementById('result-box');
        expect(box.classList.contains('error')).toBe(true);
        expect(box.textContent).toBe('transaction_mgmt.transfer_same_account');
    });

    test('[M16] a TRANSFER between two different accounts still reaches create_recurring_rule', async () => {
        await fillTransferForm('CASH', 'BANK');

        await submitForm();

        const creates = callsOf(invoke, 'create_recurring_rule');
        expect(creates).toHaveLength(1);
        expect(creates[0].request.from_account_code).toBe('CASH');
        expect(creates[0].request.to_account_code).toBe('BANK');
    });
});
