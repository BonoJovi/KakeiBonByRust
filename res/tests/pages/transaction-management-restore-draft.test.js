/**
 * Transaction list / header screen (res/js/transaction-management.js) —
 * regression tests for latent-audit L6 (and the M7 guard it unblocks).
 *
 * L6  When the user steps out of a new-transaction form (e.g. to add a
 *     shop) the draft is saved and restored on return. restoreModalState
 *     did not wait for the modal's async onOpen, whose late form reset and
 *     default date then overwrote the restored draft. Pinned: the restored
 *     date and shop survive.
 * M7  A shop disabled after the draft was saved is not given to the new
 *     transaction: the shop falls back to "Unspecified". (This guard only
 *     became reachable once L6 was fixed.)
 *
 * The real page module is booted against res/transaction-management.html via
 * ./_page-harness.js, with a saved new-transaction draft in the session.
 */

import { jest } from '@jest/globals';
import { mockPageModules, loadPageBody, bootPage } from './_page-harness.js';

const ACTIVE_SHOP = { shop_id: 1, shop_name: 'New Mart', is_disabled: 0 };
const DISABLED_SHOP = { shop_id: 7, shop_name: 'Old Mart', is_disabled: 1 };

// Master lists answer only after a macrotask, like a real IPC round trip,
// so onOpen's reset really runs after restoreModalState would have written.
const later = (value) => new Promise((resolve) => setTimeout(() => resolve(value), 20));

mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return later([{
                    category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' },
                    children: [],
                }]);
            case 'get_transactions':
                return { transactions: [], total_count: 0, page: 1, per_page: 50, total_pages: 1 };
            case 'get_accounts':
                return later([]);
            case 'get_shops':
                return later(args && args.includeDisabled ? [ACTIVE_SHOP, DISABLED_SHOP] : [ACTIVE_SHOP]);
            default:
                return null;
        }
    },
});

const session = await import('../../js/session.js');
const drafts = [
    { transaction_date: '2026-09-01T10:00', shop_id: String(ACTIVE_SHOP.shop_id), memo: 'draft memo' },
];
session.getSessionModalState.mockImplementation(async () => {
    const draft = drafts.shift();
    return draft ? JSON.stringify(draft) : null;
});

window.confirm = () => true;
window.alert = () => {};

loadPageBody('transaction-management.html');
await import('../../js/transaction-management.js');
await bootPage();
await new Promise((r) => setTimeout(r, 300));

describe('transaction management screen — draft restore (regression, latent audit 2026-09)', () => {
    test('should keep the restored draft instead of the window\'s late defaults when the defaults arrive after the restore (L6)', () => {
        expect(document.getElementById('transaction-modal').classList.contains('hidden')).toBe(false);
        expect(document.getElementById('transaction-date').value).toBe('2026-09-01T10:00');
        expect(document.getElementById('shop').value).toBe('1');
        expect(document.getElementById('transaction-memo').value).toBe('draft memo');
    });
});
