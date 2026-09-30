// latent-audit scan2-T5: a save rejected in the frontend, or failing with a generic backend error, closes the header modal and wipes the input
/**
 * T5  Modal treats a resolved onSave as success and calls close(), which
 *     also form.reset()s. handleTransactionSubmit resolves (instead of
 *     throwing) on three failure paths:
 *       - the TRANSFER same-account guard (both accounts "Unspecified"),
 *       - a total that parseAmountStrict rejects (e.g. "1e3"),
 *       - any backend error other than transfer_same_account /
 *         category1_has_details / memo too long.
 *     Expected: on each path nothing is saved, and the modal stays open with
 *     what the user typed.
 *
 * Real page module booted against res/transaction-management.html.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from '../pages/_page-harness.js';

let saveError = null;

const { invoke } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return [
                    { category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' }, children: [] },
                    { category1: { category1_code: 'TRANSFER', category1_name_i18n: 'Transfer' }, children: [] },
                ];
            case 'get_transactions':
                return { transactions: [], total_count: 0, page: 1, per_page: 50, total_pages: 1 };
            case 'get_accounts':
                return [
                    { account_code: 'CASH', account_name: 'Cash', is_disabled: 0 },
                    { account_code: 'BANK', account_name: 'Bank', is_disabled: 0 },
                ];
            case 'get_shops':
                return [];
            case 'save_transaction_header':
                return saveError ? Promise.reject(saveError) : 1;
            default:
                return null;
        }
    },
});

window.confirm = () => false;
window.alert = () => {};

loadPageBody('transaction-management.html');
await import('../../js/transaction-management.js');
await bootPage();

const modal = () => document.getElementById('transaction-modal');

async function openNewAndFill({ category1, from, to, total }) {
    document.getElementById('add-transaction-btn').click();
    await flush(10);
    expect(modal().classList.contains('hidden')).toBe(false);

    document.getElementById('transaction-date').value = '2026-09-15T10:00';
    const cat = document.getElementById('category1');
    cat.value = category1;
    cat.dispatchEvent(new Event('change'));
    document.getElementById('from-account').value = from;
    document.getElementById('to-account').value = to;
    document.getElementById('total-amount').value = total;
    document.getElementById('transaction-memo').value = 'typed memo';
}

async function pressSave() {
    document.getElementById('transaction-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
    await flush(10);
}

function expectFormKept(total) {
    expect(modal().classList.contains('hidden')).toBe(false);
    expect(document.getElementById('transaction-date').value).toBe('2026-09-15T10:00');
    expect(document.getElementById('total-amount').value).toBe(total);
    expect(document.getElementById('transaction-memo').value).toBe('typed memo');
}

describe('latent-audit scan2 T5 — failed header save keeps the modal open', () => {
    beforeEach(async () => {
        invoke.mockClear();
        saveError = null;
        // Start every case from a closed modal.
        if (!modal().classList.contains('hidden')) {
            document.getElementById('cancel-transaction-btn').click();
            await flush(5);
        }
    });

    test('[T5] TRANSFER with both accounts "Unspecified" keeps the form open', async () => {
        await openNewAndFill({ category1: 'TRANSFER', from: 'NONE', to: 'NONE', total: '5000' });
        await pressSave();

        expect(callsOf(invoke, 'save_transaction_header')).toHaveLength(0);
        expectFormKept('5000');
    });

    test('[T5] a total rejected by parseAmountStrict ("1e3") keeps the form open', async () => {
        await openNewAndFill({ category1: 'EXPENSE', from: 'CASH', to: 'NONE', total: '1e3' });
        expect(document.getElementById('total-amount').value).toBe('1e3'); // jsdom keeps it, like a browser
        await pressSave();

        expect(callsOf(invoke, 'save_transaction_header')).toHaveLength(0);
        expectFormKept('1e3');
    });

    test('[T5] a generic backend error keeps the form open', async () => {
        saveError = { code: 'database', message: 'database is locked' };
        await openNewAndFill({ category1: 'EXPENSE', from: 'CASH', to: 'NONE', total: '5000' });
        await pressSave();

        expect(callsOf(invoke, 'save_transaction_header')).toHaveLength(1);
        expectFormKept('5000');
    });
});
