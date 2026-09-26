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
                return null; // detail-less header: nothing to recommend
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

describe('transaction management screen — regression (latent audit 2026-09)', () => {
    beforeEach(() => {
        invoke.mockClear();
    });

    test('[H4] saving a header without details does not prompt to overwrite the total with ¥0', async () => {
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
        // The save flow must run to completion (reloading the list): a null
        // recommendation must not throw on the way — the pre-fix prompt
        // crashed formatting it and aborted the save flow.
        expect(callsOf(invoke, 'get_transactions').length).toBeGreaterThan(0);
    });
});
