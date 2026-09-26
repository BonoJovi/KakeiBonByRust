/**
 * Latent audit 2026-09 — transaction list / header screen (res/js/transaction-management.js)
 *
 * IDs covered: H4 (frontend half), L5
 *
 * H4  Bug (frontend): after update_transaction_header the edit flow always
 *     calls applyHeaderRecalculationPrompt(). For a header that has NO
 *     details, compute_recommended_transaction_total returns 0 (current
 *     backend contract), which differs from the saved TOTAL_AMOUNT, so every
 *     header save pops "overwrite total with ¥0?" — one wrong click wipes the
 *     total. (The backend half — bulk recalc zeroing detail-less headers —
 *     is covered by the Rust tests.)
 *     Expected: saving a detail-less header does not show the recalc prompt
 *     and never sends update_transaction_header_total.
 *     仕様確認待ち: the fixture pins the current backend contract (0 for a
 *     header without details). If the fix instead changes the backend to
 *     return e.g. null for "nothing to recommend", update the fixture; the
 *     asserted UI behaviour (no prompt, no overwrite) stays the same.
 *
 * L5  Bug 1: deleting the only row on the last page reloads the same page
 *     number; the backend answers page=3 / total_pages=2 with no rows, so the
 *     screen shows an empty "3 / 2" page.
 *     Expected: the list moves back to the last existing page
 *     (current page <= total pages, rows visible).
 *     Bug 2: loadTransactions has no request token, so when page requests
 *     resolve out of order an older response overwrites the newer one.
 *     Expected: the screen reflects the most recently requested page (a fix
 *     that ignores the second click while loading also satisfies this).
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
        from_account_code: 'NONE',
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
    from_account_code: 'NONE',
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
                return [];
            case 'get_shops':
                return [];
            case 'get_transaction_header':
                return DETAILLESS_HEADER;
            case 'get_transaction_details':
                return []; // header without details
            case 'compute_recommended_transaction_total':
                return 0; // current backend contract for a detail-less header
            default:
                return null;
        }
    },
});

window.confirm = () => true;
window.alert = () => {};

loadPageBody('transaction-management.html');
await import('../../js/transaction-management.js');
await bootPage();

const text = (id) => document.getElementById(id).textContent.trim();

function rowButtons(label) {
    return Array.from(document.querySelectorAll('#transaction-list .transaction-item button'))
        .filter((b) => b.getAttribute('data-i18n') === label);
}

describe('transaction management screen — latent audit 2026-09', () => {
    beforeEach(() => {
        invoke.mockClear();
    });

    test('[latent H4] saving a header without details does not prompt to overwrite the total with ¥0', async () => {
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

        expect(callsOf(invoke, 'update_transaction_header')).toHaveLength(1);

        const recalcModal = document.getElementById('header-recalc-modal');
        const promptShown = !!recalcModal && !recalcModal.classList.contains('hidden');

        // Clean up a pending prompt so the save flow can settle.
        if (promptShown) {
            document.getElementById('header-recalc-keep').click();
            await flush(10);
        }

        expect(promptShown).toBe(false);
        expect(callsOf(invoke, 'update_transaction_header_total')).toHaveLength(0);
    });

    test('[latent L5] deleting the only row on the last page moves back to the previous page', async () => {
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
        expect(current).toBeLessThanOrEqual(totalPages);
        expect(document.querySelectorAll('#transaction-list .transaction-item').length).toBeGreaterThan(0);
    });

    test('[latent L5] an older page response resolving late does not overwrite the newer page', async () => {
        seedTransactions(3 * PER_PAGE);
        document.getElementById('clear-filter-btn').click();
        await flush(10);
        expect(text('current-page')).toBe('1');

        pendingPageRequests = [];
        document.getElementById('next-page-btn').click(); // request page 2
        document.getElementById('next-page-btn').click(); // request page 3 (unless ignored while loading)
        await flush(5);
        const requests = pendingPageRequests;
        pendingPageRequests = null;
        expect(requests.length).toBeGreaterThanOrEqual(1);
        const lastPage = requests[requests.length - 1].page;

        // Answer in reverse order: newest first, oldest last.
        for (let i = requests.length - 1; i >= 0; i--) {
            requests[i].d.resolve(pageResponse(requests[i].page));
            await flush(5);
        }
        await flush(10);

        expect(text('current-page')).toBe(String(lastPage));
    });
});
