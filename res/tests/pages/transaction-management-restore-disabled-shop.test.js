/**
 * Transaction list / header screen (res/js/transaction-management.js) —
 * regression test for latent-audit M7, reachable since L6.
 *
 * M7  A shop disabled after a new-transaction draft was saved is not given
 *     to the transaction when the draft is restored: the shop falls back to
 *     "Unspecified". (Before L6 the modal's late reset wiped the restored
 *     draft anyway, so this guard could not be exercised.)
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
    { transaction_date: '2026-09-01T10:00', shop_id: String(DISABLED_SHOP.shop_id), memo: 'draft memo' },
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

describe('transaction management screen — draft restore with a disabled shop (regression, latent audit 2026-09)', () => {
    test('[M7] should not give a restored new transaction a shop disabled since the draft was saved', () => {
        expect(document.getElementById('transaction-date').value).toBe('2026-09-01T10:00');
        const shopSelect = document.getElementById('shop');
        expect(Array.from(shopSelect.options).map((o) => o.value)).toEqual(['', '1']);
        expect(shopSelect.value).toBe('');
    });
});
