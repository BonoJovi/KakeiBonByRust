/**
 * Transaction list / header screen (res/js/transaction-management.js) —
 * regression tests promoted from the 2026-09 latent audit.
 *
 * H4  After update_transaction_header the edit flow runs
 *     applyHeaderRecalculationPrompt(). For a header without details the
 *     backend used to recommend 0, so every save of such a header asked to
 *     overwrite the total with ¥0. compute_recommended_transaction_total now
 *     returns null ("nothing to recommend") for a detail-less header.
 *     Pinned: saving it shows no recalc prompt and never sends
 *     update_transaction_header_total.
 *
 * L8  A blank transaction date used to be sent as ':00' and surfaced the
 *     backend's raw English format error. Pinned: the date field shows
 *     validation.required, nothing is sent, and the modal stays open.
 *
 * L5  Deleting the only row on the last page reloaded the same page number,
 *     so the screen showed an empty "3 / 2" page; and without a request
 *     token an older page response resolving late overwrote a newer one.
 *     loadTransactions now moves back to the last existing page and drops
 *     superseded responses. Pinned: both.
 *
 * The real page module is booted against res/transaction-management.html via
 * ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, deferred, callsOf,
} from './_page-harness.js';

const PER_PAGE = 50;

const CATEGORY_TREE = [
    {
        category1: { category1_code: 'EXPENSE', category1_name_i18n: '支出 / Expense' },
        children: [],
    },
];

// --- fake backend state -----------------------------------------------------
let serverTransactions = [];
function seedTransactions(n) {
    serverTransactions = Array.from({ length: n }, (_, i) => ({
        transaction_id: i + 1,
        transaction_date: '2026-09-01 10:00:00',
        category1_code: 'EXPENSE',
        category1_name: 'Expense',
        from_account_code: 'CASH',
        to_account_code: 'NONE',
        total_amount: 100,
        memo: null,
        is_scheduled: 0,
    }));
}
seedTransactions(3);

function pageResponse(page) {
    const total = serverTransactions.length;
    const totalPages = Math.max(1, Math.ceil(total / PER_PAGE));
    return {
        transactions: serverTransactions.slice((page - 1) * PER_PAGE, page * PER_PAGE),
        total_count: total,
        page,
        per_page: PER_PAGE,
        total_pages: totalPages,
    };
}

// When set, get_transactions parks each request in this queue.
let pendingPageRequests = null;

const DETAILLESS_HEADER = {
    transaction_id: 1,
    transaction_date: '2026-09-01 10:00:00',
    shop_id: null,
    category1_code: 'EXPENSE',
    from_account_code: 'CASH',
    to_account_code: 'NONE',
    total_amount: 5000,
    tax_rounding_type: 0,
    tax_included_type: 1,
    memo: null,
    is_scheduled: 0,
};

const { invoke } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return CATEGORY_TREE;
            case 'get_transactions':
                if (pendingPageRequests) {
                    const d = deferred();
                    pendingPageRequests.push({ page: args.page, d });
                    return d.promise;
                }
                return pageResponse(args.page);
            case 'delete_transaction':
                serverTransactions = serverTransactions.filter(
                    (t) => t.transaction_id !== args.transactionId
                );
                return null;
            case 'get_accounts':
                return [{ account_code: 'CASH', account_name: 'Cash', is_disabled: 0 }];
            case 'get_shops':
                return [];
            case 'get_transaction_header':
                return DETAILLESS_HEADER;
            case 'get_transaction_details':
                return []; // header without details
            case 'compute_recommended_transaction_total':
                return null; // detail-less header: nothing to recommend
            default:
                return null;
        }
    },
});

window.confirm = () => true;
window.alert = () => {};

loadPageBody('transaction-management.html');
await import('../../../res/js/transaction-management.js');
await bootPage();

const text = (id) => document.getElementById(id).textContent.trim();

function rowButtons(label) {
    return Array.from(document.querySelectorAll('#transaction-list .transaction-item button'))
        .filter((b) => b.getAttribute('data-i18n') === label);
}

describe('transaction management screen — regression (latent audit 2026-09)', () => {
    beforeEach(() => {
        invoke.mockClear();
    });

    test('should not prompt to overwrite the total with ¥0 when a header without details is saved (H4)', async () => {
        const editBtn = rowButtons('common.edit')[0];
        expect(editBtn).toBeDefined();
        editBtn.click();
        await flush(10);

        const modal = document.getElementById('transaction-modal');
        expect(modal.classList.contains('hidden')).toBe(false);
        expect(document.getElementById('total-amount').value).toBe('5000');

        // Save without changes.
        document.getElementById('transaction-form').dispatchEvent(
            new Event('submit', { cancelable: true, bubbles: true })
        );
        await flush(10);

        const headerUpdates = callsOf(invoke, 'update_transaction_header');
        expect(headerUpdates).toHaveLength(1);
        // The header save itself must carry the unchanged total, not 0.
        expect(headerUpdates[0].totalAmount).toBe(5000);

        const recalcModal = document.getElementById('header-recalc-modal');
        const promptShown = !!recalcModal && !recalcModal.classList.contains('hidden');

        // Clean up a pending prompt so the save flow can settle.
        if (promptShown) {
            document.getElementById('header-recalc-keep').click();
            await flush(10);
        }

        expect(promptShown).toBe(false);
        expect(callsOf(invoke, 'update_transaction_header_total')).toHaveLength(0);
        // The save flow must run to completion (reloading the list): a null
        // recommendation must not throw on the way — the pre-fix prompt
        // crashed formatting it and aborted the save flow.
        expect(callsOf(invoke, 'get_transactions').length).toBeGreaterThan(0);
    });

    test('should reject the save without calling update_transaction_header when the transaction date is blank (L8)', async () => {
        const editBtn = rowButtons('common.edit')[0];
        editBtn.click();
        await flush(10);

        const dateInput = document.getElementById('transaction-date');
        dateInput.value = '';
        document.getElementById('transaction-form').dispatchEvent(
            new Event('submit', { cancelable: true, bubbles: true })
        );
        await flush(10);

        expect(callsOf(invoke, 'update_transaction_header')).toHaveLength(0);
        const next = dateInput.nextElementSibling;
        expect(next && next.classList.contains('validation-error') ? next.textContent : null)
            .toBe('validation.required');
        // The modal stays open for the user to fill the date in.
        expect(document.getElementById('transaction-modal').classList.contains('hidden')).toBe(false);

        document.getElementById('cancel-btn')?.click();
        await flush(5);
    });

    test('should move back to the last page when its only row is deleted (L5)', async () => {
        // 101 rows → pages of 50/50/1.
        seedTransactions(2 * PER_PAGE + 1);
        document.getElementById('clear-filter-btn').click();
        await flush(10);
        document.getElementById('next-page-btn').click();
        await flush(10);
        document.getElementById('next-page-btn').click();
        await flush(10);
        expect(text('current-page')).toBe('3');
        expect(text('total-pages')).toBe('3');

        const deleteBtns = rowButtons('common.delete');
        expect(deleteBtns).toHaveLength(1);
        deleteBtns[0].click();
        await flush(15);

        const current = parseInt(text('current-page'), 10);
        const totalPages = parseInt(text('total-pages'), 10);
        expect(totalPages).toBe(2);
        // The last existing page — not an arbitrary one such as page 1.
        expect(current).toBe(2);
        expect(document.querySelectorAll('#transaction-list .transaction-item').length).toBeGreaterThan(0);
    });

    test('should keep the newer page when an older page response resolves late (L5)', async () => {
        seedTransactions(3 * PER_PAGE);
        document.getElementById('clear-filter-btn').click();
        await flush(10);
        expect(text('current-page')).toBe('1');

        pendingPageRequests = [];
        document.getElementById('next-page-btn').click(); // request page 2
        document.getElementById('next-page-btn').click(); // request page 3
        await flush(5);
        const requests = pendingPageRequests;
        pendingPageRequests = null;
        // Both clicks must reach the backend, so the out-of-order race is
        // really exercised.
        expect(requests.map((request) => request.page)).toEqual([2, 3]);
        const lastPage = 3;

        // Answer in reverse order: newest first, oldest last.
        for (let i = requests.length - 1; i >= 0; i--) {
            requests[i].d.resolve(pageResponse(requests[i].page));
            await flush(5);
        }
        await flush(10);

        expect(text('current-page')).toBe(String(lastPage));
    });
});
