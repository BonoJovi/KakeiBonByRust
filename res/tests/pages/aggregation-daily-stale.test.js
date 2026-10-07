// An older, slower Execute does not overwrite the newer one on the daily aggregation screen (latent-audit scan2-A4)
/**
 * Daily aggregation screen (res/js/aggregation-daily.js). Scenario and
 * expectations: ./_aggregation-stale.js.
 */

import { jest } from '@jest/globals';
import { runStaleAggregationScenario } from './_aggregation-stale.js';

const set = (id, value) => { document.getElementById(id).value = value; };

await runStaleAggregationScenario(jest, {
    screen: 'daily',
    html: 'aggregation-daily.html',
    script: '../../js/aggregation-daily.js',
    command: 'get_daily_aggregation',
    fillOld: () => set('date', '2026-09-10'),
    fillNew: () => set('date', '2026-03-10'),
});
