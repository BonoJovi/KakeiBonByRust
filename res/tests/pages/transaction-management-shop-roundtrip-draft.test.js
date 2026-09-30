// The shop-management round trip keeps the scheduled flag and every header edit (latent-audit scan2-T4)
/**
 * T4  saveModalState never records `is-scheduled`, and the edit-mode branch
 *     of restoreModalState puts back only date, shop, total and a non-empty
 *     memo; category1, the accounts and the tax settings come back from the
 *     DB, and a memo the user cleared comes back too.
 *     (a) New transaction ticked "Scheduled" -> Manage shops -> back: the box
 *         is unticked, so the save books a planned payment as actual.
 *     (b) Edit: rounding floor -> half-up (which rewrites the total), from
 *         account changed, memo cleared -> Manage shops -> back: rounding,
 *         account and memo revert while the draft's total is kept.
 *     Expected: every field the draft form holds survives the round trip.
 *
 * Real page module booted against res/transaction-management.html. The
 * round trip is: click #manage-shops-btn (the draft goes to
 * setSessionModalState), then the page is loaded again with that draft
 * returned by getSessionModalState, as when shop management navigates back.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const HEADER = {
    transaction_id: 1,
    transaction_date: '2026-09-01 10:00:00',
    shop_id: null,
    category1_code: 'EXPENSE',
    from_account_code: 'CASH',
    to_account_code: 'NONE',
    total_amount: 1105,
    tax_rounding_type: 0,
    tax_included_type: 1,
    memo: 'old memo',
    is_scheduled: 0,
};

// 1005 at 10 %: floor -> tax 100 (total 1105), half-up -> tax 101 (total 1106).
const DETAILS = [{
    detail_id: 1, transaction_id: 1, amount: 1005, tax_rate: 10, tax_amount: 100,
    amount_including_tax: 1105,
}];

const { invoke } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return [
                    { category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' }, children: [] },
                    { category1: { category1_code: 'INCOME', category1_name_i18n: 'Income' }, children: [] },
                ];
            case 'get_transactions':
                return {
                    transactions: [{ ...HEADER, category1_name: 'Expense' }],
                    total_count: 1,
                    page: args.page,
                    per_page: 50,
                    total_pages: 1,
                };
            case 'get_accounts':
                return [
                    { account_code: 'CASH', account_name: 'Cash', is_disabled: 0 },
                    { account_code: 'BANK', account_name: 'Bank', is_disabled: 0 },
                ];
            case 'get_shops':
                return [{ shop_id: 1, shop_name: 'Mart', is_disabled: 0 }];
            case 'get_transaction_header':
                return HEADER;
            case 'get_transaction_details':
                return DETAILS;
            default:
                return null;
        }
    },
});

const session = await import('../../js/session.js');

window.confirm = () => false;
window.alert = () => {};

loadPageBody('transaction-management.html');
await import('../../js/transaction-management.js');
await bootPage();

/** Click Manage shops, then come back to a freshly loaded page with the draft. */
async function roundTripThroughShops() {
    session.setSessionModalState.mockClear();
    document.getElementById('manage-shops-btn').click();
    await flush(10);
    expect(session.setSessionModalState).toHaveBeenCalledTimes(1);
    const draft = session.setSessionModalState.mock.calls[0][0];

    session.getSessionModalState.mockImplementationOnce(async () => draft);
    loadPageBody('transaction-management.html');
    await bootPage();
    await new Promise((r) => setTimeout(r, 400));
    expect(document.getElementById('transaction-modal').classList.contains('hidden')).toBe(false);
}

function submitForm() {
    document.getElementById('transaction-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
}

describe('latent-audit scan2 T4 — Manage shops round trip', () => {
    beforeEach(() => {
        invoke.mockClear();
    });

    test('[T4a] a new scheduled transaction is still scheduled after the round trip', async () => {
        document.getElementById('add-transaction-btn').click();
        await flush(10);

        document.getElementById('transaction-date').value = '2026-09-15T10:00';
        const category1 = document.getElementById('category1');
        category1.value = 'EXPENSE';
        category1.dispatchEvent(new Event('change'));
        document.getElementById('from-account').value = 'CASH';
        document.getElementById('total-amount').value = '3000';
        document.getElementById('is-scheduled').checked = true;

        await roundTripThroughShops();

        expect(document.getElementById('is-scheduled').checked).toBe(true);

        submitForm();
        await flush(10);
        const saves = callsOf(invoke, 'save_transaction_header');
        expect(saves).toHaveLength(1);
        expect(saves[0].isScheduled).toBe(1);
    });

    test('[T4b] edit-mode edits to rounding, account and memo survive the round trip', async () => {
        document.getElementById('cancel-transaction-btn').click();
        await flush(5);

        const editBtn = Array.from(document.querySelectorAll('#transaction-list .transaction-item button'))
            .find((b) => b.getAttribute('data-i18n') === 'common.edit');
        editBtn.click();
        await flush(10);
        expect(document.getElementById('tax-rounding').value).toBe('0');

        const rounding = document.getElementById('tax-rounding');
        rounding.value = '1';
        rounding.dispatchEvent(new Event('change'));
        await flush(10);
        // handleTaxSettingChange rewrote the total under half-up.
        expect(document.getElementById('total-amount').value).toBe('1106');
        document.getElementById('from-account').value = 'BANK';
        document.getElementById('transaction-memo').value = '';

        await roundTripThroughShops();

        expect(document.getElementById('total-amount').value).toBe('1106');
        expect(document.getElementById('tax-rounding').value).toBe('1');
        expect(document.getElementById('from-account').value).toBe('BANK');
        expect(document.getElementById('transaction-memo').value).toBe('');
    });
});
