/**
 * Transaction edit window (res/transaction-management.html, handled by
 * res/js/transaction-management.js) — what the window shows for a saved
 * header and what it sends back when saved.
 *
 * Opening a saved header fills the form from get_transaction_header: the
 * SQLite date `YYYY-MM-DD HH:MM:SS` becomes the datetime-local value
 * `YYYY-MM-DDTHH:MM` (a date with no time gets 00:00), a null memo shows as
 * an empty field and is_scheduled 1 ticks the box. Saving sends the date
 * back as `YYYY-MM-DD HH:MM:00`, the memo trimmed (blank → null), and the
 * accounts as their codes (`NONE` when no account is chosen) to
 * update_transaction_header, or to save_transaction_header for a new
 * header.
 *
 * These replace part of a former test file that tested copies of these
 * conversions written inside the test. The real page module is booted
 * against res/transaction-management.html via ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const SAVED = {
    transaction_id: 1,
    transaction_date: '2026-09-01 10:30:45',
    shop_id: null,
    category1_code: 'EXPENSE',
    from_account_code: 'CASH',
    to_account_code: 'NONE',
    total_amount: 5000,
    tax_rounding_type: 2,
    tax_included_type: 0,
    memo: null,
    is_scheduled: 0,
};

// What get_transaction_header answers for the header being edited.
let header = SAVED;

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
                    transactions: [{ ...SAVED, category1_name: 'Expense' }],
                    total_count: 1,
                    page: args.page,
                    per_page: 50,
                    total_pages: 1,
                };
            case 'get_accounts':
                return [{ account_code: 'CASH', account_name: 'Cash', is_disabled: 0 }];
            case 'get_shops':
                return [];
            case 'get_transaction_header':
                return header;
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

async function closeWindow() {
    field('cancel-transaction-btn')?.click();
    await flush();
}

// Open the saved header from the list, answering get_transaction_header
// with `saved`.
async function openEdit(saved) {
    header = { ...SAVED, ...saved };
    await closeWindow();
    const editBtn = Array.from(document.querySelectorAll('#transaction-list .transaction-item button'))
        .find((b) => b.getAttribute('data-i18n') === 'common.edit');
    editBtn.click();
    await flush(10);
    invoke.mockClear();
}

async function openAdd() {
    await closeWindow();
    field('add-transaction-btn').click();
    await flush(10);
    invoke.mockClear();
}

async function save() {
    field('transaction-form').dispatchEvent(new Event('submit', { cancelable: true, bubbles: true }));
    await flush(10);
}

const sentUpdate = () => callsOf(invoke, 'update_transaction_header')[0];
const sentNew = () => callsOf(invoke, 'save_transaction_header')[0];

describe('transaction edit window — loading a saved header', () => {
    test('should show the date and time without seconds when the saved date has seconds', async () => {
        await openEdit({ transaction_date: '2026-09-01 10:30:45' });
        expect(field('transaction-date').value).toBe('2026-09-01T10:30');
    });

    test('should show midnight when the saved date has no time', async () => {
        await openEdit({ transaction_date: '2026-09-01' });
        expect(field('transaction-date').value).toBe('2026-09-01T00:00');
    });

    test('should show an empty memo when the saved memo is null', async () => {
        await openEdit({ memo: null });
        expect(field('transaction-memo').value).toBe('');
    });

    test('should show the saved values when a header is opened', async () => {
        await openEdit({ memo: '昼食\nランチ', is_scheduled: 1 });
        expect(field('transaction-memo').value).toBe('昼食\nランチ');
        expect(field('is-scheduled').checked).toBe(true);
        expect(field('category1').value).toBe('EXPENSE');
        expect(field('from-account').value).toBe('CASH');
        expect(field('total-amount').value).toBe('5000');
        expect(field('tax-rounding').value).toBe('2');
        expect(field('tax-included-type').value).toBe('0');
    });
});

describe('transaction edit window — saving', () => {
    test('should send the date with :00 seconds when an edited header is saved', async () => {
        await openEdit({});
        field('transaction-date').value = '2026-09-20T18:05';
        await save();
        expect(sentUpdate().transactionDate).toBe('2026-09-20 18:05:00');
    });

    test('should send the memo trimmed when it has surrounding spaces', async () => {
        await openEdit({});
        field('transaction-memo').value = '  lunch  ';
        await save();
        expect(sentUpdate().memo).toBe('lunch');
    });

    test('should send a null memo when the memo has only spaces', async () => {
        await openEdit({ memo: 'old memo' });
        field('transaction-memo').value = '   ';
        await save();
        expect(sentUpdate().memo).toBeNull();
    });

    test('should send the account codes with NONE when an account is not chosen', async () => {
        await openEdit({ from_account_code: 'CASH', to_account_code: 'NONE' });
        await save();
        expect(sentUpdate()).toMatchObject({ fromAccountCode: 'CASH', toAccountCode: 'NONE' });
    });

    test('should send the saved values back when a header is saved without changes', async () => {
        await openEdit({ transaction_date: '2026-09-01 10:30:00', memo: null, is_scheduled: 1 });
        await save();
        expect(sentUpdate()).toEqual({
            transactionId: 1,
            shopId: null,
            category1Code: 'EXPENSE',
            fromAccountCode: 'CASH',
            toAccountCode: 'NONE',
            transactionDate: '2026-09-01 10:30:00',
            totalAmount: 5000,
            taxRoundingType: 2,
            taxIncludedType: 0,
            memo: null,
            isScheduled: 1,
        });
    });

    test('should send the converted values to save_transaction_header when a new header is saved', async () => {
        await openAdd();
        field('transaction-date').value = '2026-10-01T09:00';
        field('category1').value = 'INCOME';
        field('category1').dispatchEvent(new Event('change'));
        field('total-amount').value = '1200';
        field('transaction-memo').value = '  gift  ';
        await save();
        expect(callsOf(invoke, 'update_transaction_header')).toHaveLength(0);
        expect(sentNew()).toMatchObject({
            category1Code: 'INCOME',
            fromAccountCode: 'NONE',
            transactionDate: '2026-10-01 09:00:00',
            totalAmount: 1200,
            memo: 'gift',
            isScheduled: 0,
        });
    });
});
