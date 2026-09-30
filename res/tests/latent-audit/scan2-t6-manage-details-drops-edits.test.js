// latent-audit scan2-T6: "Manage details" in the header edit modal discards unsaved header edits without warning
/**
 * T6  The Manage-details button navigates to the detail screen at once. It
 *     neither saves the header nor stores a draft (unlike Manage shops), so
 *     edited date / total / memo / scheduled are silently dropped.
 *     Expected: before leaving, the edits are kept — either the header is
 *     saved with them (possibly after a confirm, answered "yes" here) or a
 *     draft holding them is stored for the return trip.
 *
 * Real page module booted against res/transaction-management.html.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from '../pages/_page-harness.js';

const HEADER = {
    transaction_id: 1,
    transaction_date: '2026-09-01 10:00:00',
    shop_id: null,
    category1_code: 'EXPENSE',
    from_account_code: 'NONE',
    to_account_code: 'NONE',
    total_amount: 5000,
    tax_rounding_type: 0,
    tax_included_type: 1,
    memo: 'old memo',
    is_scheduled: 0,
};

const { invoke } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return [
                    { category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' }, children: [] },
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
            case 'get_shops':
                return [];
            case 'get_transaction_header':
                return HEADER;
            case 'get_transaction_details':
                return [];
            default:
                return null;
        }
    },
});

const session = await import('../../js/session.js');

// If the fix asks before leaving, the user agrees to keep the edits.
window.confirm = () => true;
window.alert = () => {};

loadPageBody('transaction-management.html');
await import('../../js/transaction-management.js');
await bootPage();

describe('latent-audit scan2 T6 — Manage details from the header edit modal', () => {
    test('[T6] edited header values are saved or kept as a draft before leaving', async () => {
        const editBtn = Array.from(document.querySelectorAll('#transaction-list .transaction-item button'))
            .find((b) => b.getAttribute('data-i18n') === 'common.edit');
        editBtn.click();
        await flush(10);
        expect(document.getElementById('total-amount').value).toBe('5000');

        document.getElementById('transaction-date').value = '2026-09-20T18:00';
        document.getElementById('total-amount').value = '6400';
        document.getElementById('transaction-memo').value = 'new memo';
        document.getElementById('is-scheduled').checked = true;

        invoke.mockClear();
        session.setSessionModalState.mockClear();
        document.getElementById('manage-details-btn').click();
        await flush(10);

        const updates = callsOf(invoke, 'update_transaction_header');
        const drafts = session.setSessionModalState.mock.calls.map(([json]) => JSON.parse(json));

        const savedHeader = updates.some((u) => u.totalAmount === 6400
            && u.transactionDate === '2026-09-20 18:00:00'
            && u.memo === 'new memo'
            && u.isScheduled === 1);
        const keptDraft = drafts.some((d) => String(d.total_amount) === '6400'
            && d.transaction_date === '2026-09-20T18:00'
            && d.memo === 'new memo'
            && (d.is_scheduled === true || d.is_scheduled === 1 || d.is_scheduled === '1'));

        // Either route is an acceptable fix; today neither happens.
        expect(savedHeader || keptDraft).toBe(true);
    });
});
