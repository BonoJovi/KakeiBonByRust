/**
 * Shared aggregation renderer (res/js/aggregation-common.js renderResults) —
 * regression test promoted from the 2026-09 latent audit.
 *
 * IDs covered: M11 (aggregation-common half, exercised via the yearly screen)
 *
 * Bug: renderResults sums per-row `count` into the total row. On the account
 *      axis a TRANSFER is counted in both its FROM and TO rows, so one
 *      transaction shows as 2 in the total row (daily / weekly / yearly /
 *      period screens all share this renderer).
 * Fixed by omitting the total row's count and average ("—") on axes where
 *      one transaction can sit in several rows (account, category2/3,
 *      product). Pinned: the count cell is not the inflated sum.
 *
 * The real yearly page module is booted against res/aggregation-yearly.html
 * via ./_page-harness.js so the renderer sees the real group-by context.
 */

import { jest } from '@jest/globals';
import { mockPageModules, loadPageBody, bootPage, flush } from './_page-harness.js';

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

describe('yearly aggregation total row — regression (latent audit 2026-09)', () => {
    test('should not count one transfer twice in the shared total row when the axis is account (M11)', async () => {
        document.getElementById('group-by').value = 'account';
        document.getElementById('execute-btn').click();
        await flush(10);

        const rows = document.querySelectorAll('#results-list tr');
        expect(rows).toHaveLength(2); // sanity
        const footer = Array.from(document.querySelectorAll('#results-footer tr td'))
            .map((td) => td.textContent.trim());
        expect(footer[2]).toBe('—');
        expect(footer[3]).toBe('—');
    });
});
