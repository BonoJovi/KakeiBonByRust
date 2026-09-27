/**
 * Latent audit 2026-09 — monthly aggregation screen (res/js/aggregation.js displayResults)
 *
 * IDs covered: L12
 *
 * L12 Bug: displayResults formats negatives as '¥' + (-1234).toLocaleString()
 *     → "¥-1,234", inconsistent with the other aggregation screens.
 *     Expected: no "¥-" in the rendered amount (e.g. "-¥1,234" or the shared
 *     "-1,234" formatting).
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
});
