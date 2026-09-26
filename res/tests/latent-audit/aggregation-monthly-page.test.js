/**
 * Latent audit 2026-09 — monthly aggregation screen (res/js/aggregation.js displayResults)
 *
 * IDs covered: M11 (monthly screen), M12, L12
 *
 * M12 Bug: the Fable-5 #22 fix (empty group_name → i18n `common.unspecified`)
 *     only landed in aggregation-common.js renderResults. The monthly screen
 *     has its own displayResults() which renders group_name verbatim, so the
 *     "unspecified" row shows an empty first cell.
 *     Expected: an empty group_name renders as i18n.t('common.unspecified').
 *
 * L12 Bug: displayResults formats negatives as '¥' + (-1234).toLocaleString()
 *     → "¥-1,234", inconsistent with the other aggregation screens.
 *     Expected: no "¥-" in the rendered amount (e.g. "-¥1,234" or the shared
 *     "-1,234" formatting).
 *
 * M11 Bug: the total row sums per-row `count`. On the account axis a
 *     TRANSFER appears in both the FROM row and the TO row; on the category2
 *     axis one transaction whose details span two groups is counted in each
 *     group (backend count is COUNT(DISTINCT txn) per group). The total row
 *     therefore shows 2 transactions (and a halved average) for a single
 *     transaction.
 *     Expected (chosen assertion): the total-row count cell is NOT the
 *     inflated naive sum — it must show the distinct transaction count (1)
 *     or be left blank / "-" (count/average not shown for overlapping axes).
 *     The per-row results alone do not carry enough information for the
 *     frontend to compute the distinct total, so a real fix will likely need
 *     a backend-provided total (or hide the cells).
 *     仕様確認待ち: whether the fix supplies a distinct total from the backend
 *     or suppresses count/average on overlapping axes. With a backend-provided
 *     total, extend the fixture accordingly.
 *
 * The real page module is booted against res/aggregation.html via
 * ../pages/_page-harness.js.
 */

import { jest } from '@jest/globals';
import { mockPageModules, loadPageBody, bootPage, flush } from '../pages/_page-harness.js';

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

const ACCEPTABLE_DISTINCT_TOTAL = ['1', '', '-', '—'];

describe('monthly aggregation screen — latent audit 2026-09', () => {
    test('[latent M12] empty group_name renders as common.unspecified', async () => {
        await runAggregation('shop', [
            { group_key: '1', group_name: 'Real Shop', total_amount: -100, count: 1, avg_amount: -100 },
            { group_key: '', group_name: '', total_amount: -50, count: 1, avg_amount: -50 },
        ]);
        expect(bodyCells(0)[0]).toBe('Real Shop');
        expect(bodyCells(1)[0]).toBe('common.unspecified');
    });

    test('[latent L12] negative amounts are not rendered as "¥-1,234"', async () => {
        await runAggregation('category1', [
            { group_key: 'EXPENSE', group_name: 'Expense', total_amount: -1234, count: 1, avg_amount: -1234 },
        ]);
        const [, amount, , avg] = bodyCells(0);
        expect(amount).toContain('1,234');
        expect(amount).not.toContain('¥-');
        expect(avg).not.toContain('¥-');
        const footer = footerCells();
        expect(footer[1]).not.toContain('¥-');
    });

    test('[latent M11] account axis: one transfer (FROM row + TO row) is not counted twice in the total row', async () => {
        await runAggregation('account', [
            { group_key: 'BANK', group_name: 'Bank', total_amount: -10000, count: 1, avg_amount: -10000 },
            { group_key: 'CASH', group_name: 'Cash', total_amount: 10000, count: 1, avg_amount: 10000 },
        ]);
        expect(ACCEPTABLE_DISTINCT_TOTAL).toContain(footerCells()[2]);
    });

    test('[latent M11] category2 axis: one transaction spanning two groups is not counted twice in the total row', async () => {
        // One EXPENSE transaction (total 3,000) with a Food detail (1,000)
        // and a Daily-goods detail (2,000).
        await runAggregation('category2', [
            { group_key: 'C2_E_1', group_name: 'Food', total_amount: -1000, count: 1, avg_amount: -1000 },
            { group_key: 'C2_E_2', group_name: 'Daily', total_amount: -2000, count: 1, avg_amount: -2000 },
        ]);
        expect(ACCEPTABLE_DISTINCT_TOTAL).toContain(footerCells()[2]);
    });
});
