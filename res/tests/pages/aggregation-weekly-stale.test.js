// An older, slower Execute does not overwrite the newer one on the weekly aggregation screen (latent-audit scan2-A4)
/**
 * Weekly aggregation screen (res/js/aggregation-weekly.js). Scenario and
 * expectations: ./_aggregation-stale.js.
 */

import { jest } from '@jest/globals';
import { runStaleAggregationScenario } from './_aggregation-stale.js';

const set = (id, value) => { document.getElementById(id).value = value; };

await runStaleAggregationScenario(jest, {
    screen: 'weekly',
    html: 'aggregation-weekly.html',
    script: '../../js/aggregation-weekly.js',
    command: 'get_weekly_aggregation_by_date',
    fillOld: () => set('reference-date', '2026-09-10'),
    fillNew: () => set('reference-date', '2026-03-10'),
    fillInvalid: () => set('reference-date', ''),
});
