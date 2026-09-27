/**
 * Monthly aggregation screen (res/js/aggregation.js displayResults) —
 * regression tests promoted from the 2026-09 latent audit.
 *
 * M12 The Fable-5 #22 fix (empty group_name → i18n `common.unspecified`) only
 *     landed in aggregation-common.js; the monthly screen's own
 *     displayResults() rendered group_name verbatim, leaving the
 *     "unspecified" row blank. Pinned: it renders common.unspecified.
 *
 * M11 The total row summed per-row `count`. On axes where one transaction
 *     sits in several rows (account: a TRANSFER is in its FROM and TO rows;
 *     category2/3 / product: details spanning several groups) that
 *     over-counted it and halved the average. Fixed by omitting the total
 *     row's count and average ("—") on those axes. Pinned: the count cell is
 *     not the inflated sum.
 *
 * L12 Negative amounts were formatted as '¥' + (-1234).toLocaleString() →
 *     "¥-1,234". Pinned: "-¥1,234", matching the "+¥1,234" positives.
 *
 * The real page module is booted against res/aggregation.html via
 * ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import { mockPageModules, loadPageBody, bootPage, flush } from './_page-harness.js';

let monthlyResults = [];

mockPageModules(jest, {
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_monthly_aggregation':
                return monthlyResults;
            case 'get_language_names':
                return [];
            case 'get_language':
                return 'ja';
            default:
                return null;
        }
    },
});

loadPageBody('aggregation.html');
await import('../../js/aggregation.js');
await bootPage();

async function runAggregation(groupBy, results) {
    monthlyResults = results;
    document.getElementById('group-by').value = groupBy;
    document.getElementById('execute-btn').click();
    await flush(10);
}

const bodyCells = (rowIdx) =>
    Array.from(document.querySelectorAll('#results-list tr')[rowIdx].querySelectorAll('td'))
        .map((td) => td.textContent.trim());
const footerCells = () =>
    Array.from(document.querySelectorAll('#results-footer tr td')).map((td) => td.textContent.trim());

// Total-row count / average are omitted on overlapping axes (M11).
const NOT_APPLICABLE = '—';

describe('monthly aggregation screen — regression (latent audit 2026-09)', () => {
    test('[M12] empty group_name renders as common.unspecified', async () => {
        await runAggregation('shop', [
            { group_key: '1', group_name: 'Real Shop', total_amount: -100, count: 1, avg_amount: -100 },
            { group_key: '', group_name: '', total_amount: -50, count: 1, avg_amount: -50 },
        ]);
        expect(bodyCells(0)[0]).toBe('Real Shop');
        expect(bodyCells(1)[0]).toBe('common.unspecified');
    });

    test('[M11] account axis: one transfer (FROM row + TO row) is not counted twice in the total row', async () => {
        await runAggregation('account', [
            { group_key: 'BANK', group_name: 'Bank', total_amount: -10000, count: 1, avg_amount: -10000 },
            { group_key: 'CASH', group_name: 'Cash', total_amount: 10000, count: 1, avg_amount: 10000 },
        ]);
        expect(footerCells()[2]).toBe(NOT_APPLICABLE);
        expect(footerCells()[3]).toBe(NOT_APPLICABLE);
    });

    test('[M11] category2 axis: one transaction spanning two groups is not counted twice in the total row', async () => {
        // One EXPENSE transaction (total 3,000) with a Food detail (1,000)
        // and a Daily-goods detail (2,000).
        await runAggregation('category2', [
            { group_key: 'C2_E_1', group_name: 'Food', total_amount: -1000, count: 1, avg_amount: -1000 },
            { group_key: 'C2_E_2', group_name: 'Daily', total_amount: -2000, count: 1, avg_amount: -2000 },
        ]);
        expect(footerCells()[2]).toBe(NOT_APPLICABLE);
        expect(footerCells()[3]).toBe(NOT_APPLICABLE);
    });

    test('[M11] category1 axis still sums the count into the total row', async () => {
        // Each transaction belongs to exactly one category1 row.
        await runAggregation('category1', [
            { group_key: 'EXPENSE', group_name: 'Expense', total_amount: -3000, count: 2, avg_amount: -1500 },
            { group_key: 'INCOME', group_name: 'Income', total_amount: 5000, count: 1, avg_amount: 5000 },
        ]);
        expect(footerCells()[2]).toBe('3');
        expect(footerCells()[3]).not.toBe(NOT_APPLICABLE);
    });

    test('[L12] should put the minus sign before the yen symbol when the amount is negative', async () => {
        await runAggregation('category1', [
            { group_key: 'EXPENSE', group_name: 'Expense', total_amount: -1234, count: 1, avg_amount: -1234 },
        ]);
        const [, amount, , avg] = bodyCells(0);
        expect(amount).toBe('-¥1,234');
        expect(avg).toBe('-¥1,234');
        expect(footerCells()[1]).toBe('-¥1,234');
    });
});
