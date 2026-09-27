/**
 * Latent audit 2026-09 — shared aggregation renderer (res/js/aggregation-common.js renderResults)
 *
 * IDs covered: M11 (aggregation-common half, exercised via the yearly screen)
 *
 * Bug: renderResults sums per-row `count` into the total row. On the account
 *      axis a TRANSFER is counted in both its FROM and TO rows, so one
 *      transaction shows as 2 in the total row (daily / weekly / yearly /
 *      period screens all share this renderer).
 * Expected (chosen assertion): the total-row count is the distinct
 *      transaction count (1) or is left blank / "-". The rows alone do not
 *      carry enough information to compute it, so a real fix likely needs a
 *      backend-provided total or must suppress the cell for this axis.
 *      仕様確認待ち (see aggregation-monthly-page.test.js for details).
 *
 * The real yearly page module is booted against res/aggregation-yearly.html
 * via ../pages/_page-harness.js so the renderer sees the real group-by context.
 */

import { jest } from '@jest/globals';
import { mockPageModules, loadPageBody, bootPage, flush } from '../pages/_page-harness.js';

mockPageModules(jest, {
    invoke: (cmd) => {
        if (cmd === 'get_yearly_aggregation') {
            return [
                { group_key: 'BANK', group_name: 'Bank', total_amount: -10000, count: 1, avg_amount: -10000 },
                { group_key: 'CASH', group_name: 'Cash', total_amount: 10000, count: 1, avg_amount: 10000 },
            ];
        }
        return null;
    },
});

loadPageBody('aggregation-yearly.html');
await import('../../js/aggregation-yearly.js');
await bootPage();

describe('yearly aggregation total row — latent audit 2026-09', () => {
    test('[latent M11] account axis: one transfer is not counted twice in the shared total row', async () => {
        document.getElementById('group-by').value = 'account';
        document.getElementById('execute-btn').click();
        await flush(10);

        const rows = document.querySelectorAll('#results-list tr');
        expect(rows).toHaveLength(2); // sanity
        const footer = Array.from(document.querySelectorAll('#results-footer tr td'))
            .map((td) => td.textContent.trim());
        expect(['1', '', '-', '—']).toContain(footer[2]);
    });
});
