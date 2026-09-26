/**
 * Latent audit 2026-09 — transaction detail screen (res/js/transaction-detail-management.js)
 *
 * IDs covered: H3, M19 (detail form half), L7
 *
 * H3  Bug: openDetailModal() restores `autocompleteState.selectedProductId`
 *     from detail.product_id, then dispatches a synthetic `input` event on
 *     #item-name (to refresh the char counter). The autocomplete `input`
 *     handler treats that as a user keystroke and resets the id to null, so
 *     "open existing product-linked detail → Save without changes" sends
 *     productId: null and the PRODUCT_ID link is silently dropped.
 *     Expected: update_transaction_detail receives the original productId.
 *
 * M19 Bug: handleDetailFormSubmit has no re-entrancy guard; a double click on
 *     Save / double Enter while add_transaction_detail is in flight invokes
 *     the command twice → duplicate detail rows.
 *     Expected: add_transaction_detail is invoked exactly once.
 *
 * L7  Bug: detail list renders `detail.amount_including_tax?.toLocaleString()
 *     || detail.amount...`; for legacy rows where amount_including_tax is 0,
 *     `(0).toLocaleString()` is the truthy string "0", so the row shows ¥0
 *     even though AMOUNT > 0.
 *     Expected: the amount cell never shows ¥0 when AMOUNT > 0 (falls back to
 *     AMOUNT or a computed tax-included value).
 *     Note (spec 2026-09-26): AMOUNT is always tax-excluded; legacy
 *     NULL/0 tax-included rows will be backfilled, but the display fallback
 *     must still be correct.
 *
 * The real page module is booted against res/transaction-detail-management.html
 * via ./_page-harness.js; Tauri invoke is routed per command below.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, deferred, callsOf,
} from './_page-harness.js';

const TRANSACTION_ID = 10;

const HEADER = {
    transaction_id: TRANSACTION_ID,
    transaction_date: '2026-09-01 10:00:00',
    category1_code: 'EXPENSE',
    from_account_code: 'CASH',
    from_account_name: 'Cash',
    shop_name: 'Shop',
    total_amount: 2100,
    tax_rounding_type: 0,
};

const PRODUCT_LINKED_DETAIL = {
    detail_id: 1,
    transaction_id: TRANSACTION_ID,
    category1_code: 'EXPENSE',
    category2_code: 'C2_E_1',
    category2_name: 'Food',
    category3_code: 'C3_1',
    category3_name: 'Veg',
    item_name: 'サバ缶',
    amount: 1000,
    tax_rate: 10,
    tax_amount: 100,
    amount_including_tax: 1100,
    product_id: 7,
    memo_text: null,
};

const LEGACY_ZERO_INCL_DETAIL = {
    detail_id: 2,
    transaction_id: TRANSACTION_ID,
    category1_code: 'EXPENSE',
    category2_code: 'C2_E_1',
    category2_name: 'Food',
    category3_code: 'C3_1',
    category3_name: 'Veg',
    item_name: 'Legacy row',
    amount: 1000,
    tax_rate: 0,
    tax_amount: 0,
    amount_including_tax: 0,
    product_id: null,
    memo_text: null,
};

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
];

let addInflight = null;

const { invoke } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_transaction_header_with_info':
                return HEADER;
            case 'get_transaction_details':
                return [PRODUCT_LINKED_DETAIL, LEGACY_ZERO_INCL_DETAIL];
            case 'get_category_tree_with_lang':
                return CATEGORY_TREE;
            case 'search_products_by_name':
                return [];
            case 'compute_recommended_transaction_total':
                return HEADER.total_amount; // equal → no recalc prompt
            case 'add_transaction_detail':
                return addInflight ? addInflight.promise : null;
            default:
                return null;
        }
    },
});

window.history.replaceState(null, '', `/?transaction_id=${TRANSACTION_ID}`);
loadPageBody('transaction-detail-management.html');
await import('../../js/transaction-detail-management.js');
await bootPage();

function submitDetailForm() {
    document.getElementById('detail-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
}

describe('transaction detail screen — latent audit 2026-09', () => {
    beforeEach(() => {
        invoke.mockClear();
    });

    test('[latent H3] saving a product-linked detail without changes keeps its productId', async () => {
        const editBtn = document.querySelector('.edit-detail-btn[data-detail-id="1"]');
        expect(editBtn).not.toBeNull();
        editBtn.click();
        await flush(10);

        // Sanity: the modal opened in edit mode with the row's values.
        expect(document.getElementById('detail-modal').classList.contains('hidden')).toBe(false);
        expect(document.getElementById('detail-id').value).toBe('1');
        expect(document.getElementById('item-name').value).toBe('サバ缶');

        // No user edit — just Save.
        submitDetailForm();
        await flush(10);

        const updates = callsOf(invoke, 'update_transaction_detail');
        expect(updates).toHaveLength(1);
        expect(updates[0].productId).toBe(7);
    });

    test('[latent L7] legacy row with amount_including_tax = 0 is not rendered as ¥0', () => {
        const row = document.querySelector('#detail-list tr[data-detail-id="2"]');
        expect(row).not.toBeNull();
        const amountCell = row.querySelectorAll('td')[2];
        const text = amountCell.textContent.trim();
        // AMOUNT is 1000 (tax-excluded, 0% tax) — the display must fall
        // back to a non-zero value instead of the stored 0.
        expect(text).not.toBe('¥0');
        expect(text).toMatch(/¥1,\d00/);
    });

    test('[latent M19] double submit of the add-detail form invokes add_transaction_detail once', async () => {
        document.getElementById('add-detail-btn').click();
        await flush(10);
        expect(document.getElementById('detail-modal').classList.contains('hidden')).toBe(false);

        document.getElementById('item-name').value = 'New item';
        document.getElementById('tax-rate').value = '10';
        document.getElementById('amount-excluding-tax').value = '1000';
        document.getElementById('amount-including-tax').value = '1100';
        document.getElementById('tax-amount').value = '100';

        addInflight = deferred();
        submitDetailForm();
        submitDetailForm();
        await flush(5);

        const adds = callsOf(invoke, 'add_transaction_detail');
        addInflight.resolve(null);
        addInflight = null;
        await flush(10);

        expect(adds).toHaveLength(1);
    });
});
