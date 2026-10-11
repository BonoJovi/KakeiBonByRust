/**
 * Transaction edit window (res/transaction-management.html, handled by
 * res/js/transaction-management.js) — which header the window works on,
 * and the messages shown when loading or saving fails.
 *
 * Opening a transaction from the list shows the edit title and loads that
 * header with get_transaction_header; the add window shows the add title
 * and loads nothing. Closing the window forgets the transaction, so the
 * next add saves a new header with save_transaction_header, and the next
 * edit updates the transaction that was opened.
 *
 * A failed save shows "failed_to_save: <backend message>" and keeps the
 * window open; a backend memo limit error is shown under the memo instead.
 * A failed load shows "failed_to_load: <backend message>". Errors with
 * their own message (account_required, transfer_same_account,
 * category1_has_details) are tested in the other
 * ./transaction-management-*.test.js files.
 *
 * These replace part of a former test file that tested copies of these
 * rules written inside the test. The real page module is booted against
 * res/transaction-management.html via ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const SAVED = {
    transaction_date: '2026-09-01 10:30:00',
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

const HEADERS = [
    { ...SAVED, transaction_id: 11 },
    { ...SAVED, transaction_id: 22, total_amount: 800 },
];

// What get_transaction_header and update_transaction_header fail with,
// or null to answer normally.
let loadError = null;
let saveError = null;

const { invoke, showToast } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return [
                    { category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' }, children: [] },
                ];
            case 'get_transactions':
                return {
                    transactions: HEADERS.map((h) => ({ ...h, category1_name: 'Expense' })),
                    total_count: HEADERS.length,
                    page: args.page,
                    per_page: 50,
                    total_pages: 1,
                };
            case 'get_accounts':
                return [{ account_code: 'CASH', account_name: 'Cash', is_disabled: 0 }];
            case 'get_shops':
                return [];
            case 'get_transaction_header':
                if (loadError) return Promise.reject(loadError);
                return HEADERS.find((h) => h.transaction_id === args.transactionId);
            case 'update_transaction_header':
                return saveError ? Promise.reject(saveError) : null;
            case 'save_transaction_header':
                return 99;
            case 'get_transaction_details':
                return [];
            default:
                return null;
        }
    },
});

window.alert = () => {};

loadPageBody('transaction-management.html');
await import('../../../res/js/transaction-management.js');
await bootPage();

const field = (id) => document.getElementById(id);
const isOpen = () => !field('transaction-modal').classList.contains('hidden');
const title = () => field('transaction-modal-title').textContent;
const memoError = () => field('transaction-memo').parentElement.querySelector('.validation-error');

async function closeWindow() {
    field('cancel-transaction-btn')?.click();
    await flush();
}

// Open the index-th transaction of the list.
async function openEdit(index) {
    await closeWindow();
    const editBtns = Array.from(document.querySelectorAll('#transaction-list .transaction-item button'))
        .filter((b) => b.getAttribute('data-i18n') === 'common.edit');
    editBtns[index].click();
    await flush(10);
}

async function openAdd() {
    await closeWindow();
    field('add-transaction-btn').click();
    await flush(10);
}

async function save() {
    field('transaction-form').dispatchEvent(new Event('submit', { cancelable: true, bubbles: true }));
    await flush(10);
}

function lastToast() {
    const calls = showToast.mock.calls;
    return calls[calls.length - 1];
}

beforeEach(() => {
    loadError = null;
    saveError = null;
    invoke.mockClear();
    showToast.mockClear();
});

describe('transaction edit window — which header it works on', () => {
    test('should show the edit title and load that header when a transaction is opened from the list', async () => {
        await openEdit(1);

        expect(title()).toBe('transaction_mgmt.edit_transaction');
        expect(callsOf(invoke, 'get_transaction_header')).toEqual([{ transactionId: 22 }]);
        expect(field('total-amount').value).toBe('800');
    });

    test('should show the add title and load no header when the add window is opened', async () => {
        await openAdd();

        expect(title()).toBe('transaction_mgmt.add_transaction');
        expect(callsOf(invoke, 'get_transaction_header')).toHaveLength(0);
    });

    test('should save a new header when the add window is opened after an edit window is closed', async () => {
        await openEdit(0);
        await openAdd();
        field('total-amount').value = '1200';
        invoke.mockClear();
        await save();

        expect(callsOf(invoke, 'update_transaction_header')).toHaveLength(0);
        expect(callsOf(invoke, 'save_transaction_header')).toHaveLength(1);
    });

    test('should update the second transaction when it is opened after the first is closed', async () => {
        await openEdit(0);
        await openEdit(1);
        invoke.mockClear();
        await save();

        const sent = callsOf(invoke, 'update_transaction_header');
        expect(sent).toHaveLength(1);
        expect(sent[0].transactionId).toBe(22);
    });
});

describe('transaction edit window — messages when saving fails', () => {
    test('should show the save-failed message with the backend message and keep the window open when the backend refuses the save', async () => {
        await openEdit(0);
        saveError = { code: 'not_found', message: 'Transaction not found' };
        await save();

        expect(lastToast()).toEqual([
            'transaction_mgmt.failed_to_save: Transaction not found',
            { variant: 'error' },
        ]);
        expect(isOpen()).toBe(true);
    });

    test('should show the save-failed message with the text when the backend answers a plain string', async () => {
        await openEdit(0);
        saveError = 'database is locked';
        await save();

        expect(lastToast()).toEqual([
            'transaction_mgmt.failed_to_save: database is locked',
            { variant: 'error' },
        ]);
    });

    test('should show the memo limit under the memo and no toast when the backend refuses the memo', async () => {
        await openEdit(0);
        field('transaction-memo').value = 'memo';
        saveError = { code: 'validation', message: 'Memo must be at most 1000 characters' };
        await save();

        expect(showToast).not.toHaveBeenCalled();
        expect(memoError().textContent).toMatch(/^validation\.max_length\(field=transaction_mgmt\.memo,max=\d+,actual=4\)$/);
        expect(isOpen()).toBe(true);
    });

    test('should show the save-failed message when a validation error is not about the memo', async () => {
        await openEdit(0);
        saveError = { code: 'validation', message: 'Total must be a whole number' };
        await save();

        expect(lastToast()).toEqual([
            'transaction_mgmt.failed_to_save: Total must be a whole number',
            { variant: 'error' },
        ]);
        expect(memoError()).toBeNull();
    });

    test('should show no memo message when the window is opened again after a memo error', async () => {
        await openEdit(0);
        field('transaction-memo').value = 'memo';
        saveError = { code: 'validation', message: 'Memo must be at most 1000 characters' };
        await save();
        expect(memoError().textContent).not.toBe('');

        await openEdit(0);

        expect(memoError()?.textContent ?? '').toBe('');
    });
});

describe('transaction edit window — messages when loading fails', () => {
    test('should show the load-failed message with the backend message when the header cannot be loaded', async () => {
        loadError = { code: 'not_found', message: 'Transaction not found' };
        await openEdit(0);

        expect(lastToast()).toEqual([
            'transaction_mgmt.failed_to_load: Transaction not found',
            { variant: 'error' },
        ]);
    });

    test('should show the load-failed message with the text when loading answers a plain string', async () => {
        loadError = 'database is locked';
        await openEdit(0);

        expect(lastToast()).toEqual([
            'transaction_mgmt.failed_to_load: database is locked',
            { variant: 'error' },
        ]);
    });
});
