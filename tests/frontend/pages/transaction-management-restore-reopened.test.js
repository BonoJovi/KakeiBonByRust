/**
 * Transaction list / header screen (res/js/transaction-management.js) —
 * regression test for latent-audit L6.
 *
 * L6  restoreModalState now waits for the modal's initialisation before
 *     writing the saved draft. If the user closes the modal and opens a new
 *     one while that wait is still running, the draft belongs to a form that
 *     is gone: it must not be written into the reopened form. Pinned: the
 *     reopened form keeps its own defaults.
 *
 * The real page module is booted against res/transaction-management.html via
 * ./_page-harness.js, with a saved new-transaction draft in the session.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, deferred,
} from './_page-harness.js';

const ACTIVE_SHOP = { shop_id: 1, shop_name: 'New Mart', is_disabled: 0 };
const DISABLED_SHOP = { shop_id: 7, shop_name: 'Old Mart', is_disabled: 1 };

// The restore modal's first master list (the 2nd category-tree call; the
// 1st is the page's own filter) is held until the test releases it; every
// other call answers after one macrotask. So the reopened modal finishes its
// initialisation first and the interrupted restore resumes afterwards — the
// case the guard must stop.
const restoreGate = deferred();
let categoryTreeCalls = 0;
const soon = (value) => new Promise((resolve) => setTimeout(() => resolve(value), 0));

mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_category_tree_with_lang': {
                const tree = [{
                    category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' },
                    children: [],
                }];
                categoryTreeCalls += 1;
                return categoryTreeCalls === 2 ? restoreGate.promise.then(() => tree) : soon(tree);
            }
            case 'get_transactions':
                return { transactions: [], total_count: 0, page: 1, per_page: 50, total_pages: 1 };
            case 'get_accounts':
                return soon([]);
            case 'get_shops':
                return soon(args && args.includeDisabled ? [ACTIVE_SHOP, DISABLED_SHOP] : [ACTIVE_SHOP]);
            default:
                return null;
        }
    },
});

const session = await import('../../../res/js/session.js');
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
await import('../../../res/js/transaction-management.js');
await bootPage();
// Wait until the restore has opened the modal (its onOpen is now waiting for
// the master lists), then close it and open a new one before they arrive.
const modalEl = document.getElementById('transaction-modal');
for (let i = 0; i < 200 && modalEl.classList.contains('hidden'); i++) {
    await new Promise((r) => setTimeout(r, 0));
}
if (modalEl.classList.contains('hidden')) throw new Error('restore never opened the modal');
document.getElementById('cancel-transaction-btn').click();
document.getElementById('add-transaction-btn').click();
// Let the reopened modal finish its initialisation, then release the
// restore's held lists so its onOpen — and the restore after it — run to the
// end. From here every step answers within a macrotask.
await flush(20);
restoreGate.resolve();
await flush(20);


describe('transaction management screen — draft restore interrupted (regression, latent audit 2026-09)', () => {
    test('should not write the draft into a window when it was reopened while the restore was waiting (L6)', () => {
        expect(document.getElementById('transaction-modal').classList.contains('hidden')).toBe(false);
        expect(document.getElementById('transaction-date').value).not.toBe('2026-09-01T10:00');
        expect(document.getElementById('transaction-memo').value).toBe('');
    });
});
