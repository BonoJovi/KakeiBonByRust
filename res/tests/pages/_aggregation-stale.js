/**
 * Shared scenario for the aggregation screens' stale-load tests
 * (latent-audit scan2-A4).
 *
 * Each screen runs one invoke per Execute. Nothing dropped the result of an
 * older Execute, so a slower, older request could overwrite the newer
 * request's table, and an older request that failed afterwards showed its
 * error and cleared the newer table. The loading class was also removed by
 * whichever request finished first.
 *
 * Expected: only the latest Execute updates the table, the message and the
 * loading state.
 *
 * Usage (one screen per test file, since each page module boots on import):
 *
 *   await runStaleAggregationScenario(jest, {
 *       screen: 'monthly',
 *       html: 'aggregation.html',
 *       script: '../../js/aggregation.js',
 *       command: 'get_monthly_aggregation',
 *       fillOld: () => { ... },   // set the form to the older request
 *       fillNew: () => { ... },   // set the form to the newer request
 *   });
 */

import { mockPageModules, loadPageBody, bootPage, flush, deferred } from './_page-harness.js';

const row = (name, amount) => ({
    group_key: name, group_name: name, total_amount: amount, count: 1, avg_amount: amount,
});

export async function runStaleAggregationScenario(jest, cfg) {
    // Responses to hold, in request order; once empty, requests answer at once.
    let held = [];

    mockPageModules(jest, {
        invoke: (cmd) => {
            if (cmd === cfg.command) {
                if (held.length > 0) return held.shift().promise;
                return [row('NewerGroup', -3000)];
            }
            switch (cmd) {
                case 'get_language_names':
                    return [];
                case 'get_language':
                    return 'ja';
                default:
                    return null;
            }
        },
    });

    loadPageBody(cfg.html);
    await import(cfg.script);
    await bootPage();

    const container = () => document.getElementById('results-container');
    const firstCells = () =>
        Array.from(document.querySelectorAll('#results-list tr'))
            .map((tr) => tr.querySelector('td')?.textContent.trim());
    const message = () => document.getElementById('results-message')?.textContent ?? '';

    const execute = async (fill, ticks) => {
        fill();
        document.getElementById('execute-btn').click();
        await flush(ticks);
    };

    // The older request is held; the newer one answers at once.
    async function runOldThenNew() {
        const older = deferred();
        held = [older];
        await execute(cfg.fillOld, 10);
        await execute(cfg.fillNew, 20);
        expect(firstCells()).toContain('NewerGroup'); // sanity: the newer table is drawn
        return older;
    }

    describe(`${cfg.screen} aggregation: an older Execute does not overwrite a newer one (scan2-A4)`, () => {
        afterEach(() => {
            held = [];
        });

        test('[scan2-A4] a slower, older result arriving later is dropped', async () => {
            const older = await runOldThenNew();
            expect(container().classList.contains('loading')).toBe(false);

            older.resolve([row('OlderGroup', -9000)]);
            await flush(20);

            expect(firstCells()).toContain('NewerGroup');
            expect(firstCells()).not.toContain('OlderGroup');
        });

        test('[scan2-A4] an older request failing later neither shows its error nor clears the table', async () => {
            const older = await runOldThenNew();

            older.reject(new Error('older request failed'));
            await flush(20);

            expect(message()).not.toContain('older request failed');
            expect(firstCells()).toContain('NewerGroup');
        });

        test('[scan2-A4] the loading state stays until the latest request finishes', async () => {
            // Hold both: the older one finishing first must not clear the
            // loading state of the newer one.
            const older = deferred();
            const newer = deferred();
            held = [older, newer];
            await execute(cfg.fillOld, 10);
            await execute(cfg.fillNew, 10);

            older.resolve([row('OlderGroup', -9000)]);
            await flush(20);
            expect(container().classList.contains('loading')).toBe(true);
            expect(firstCells()).not.toContain('OlderGroup');

            newer.resolve([row('NewerGroup', -3000)]);
            await flush(20);
            expect(container().classList.contains('loading')).toBe(false);
            expect(firstCells()).toContain('NewerGroup');
        });
    });
}
