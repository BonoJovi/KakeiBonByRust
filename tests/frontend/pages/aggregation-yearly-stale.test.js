// An older, slower Execute does not overwrite the newer one on the yearly aggregation screen (latent-audit scan2-A4)
/**
 * Yearly aggregation screen (res/js/aggregation-yearly.js). Scenario and
 * expectations: ./_aggregation-stale.js.
 */

import { jest } from '@jest/globals';
import { runStaleAggregationScenario } from './_aggregation-stale.js';

const set = (id, value) => { document.getElementById(id).value = value; };

await runStaleAggregationScenario(jest, {
    screen: 'yearly',
    html: 'aggregation-yearly.html',
    script: '../../../res/js/aggregation-yearly.js',
    command: 'get_yearly_aggregation',
    fillOld: () => set('year', '2026'),
    fillNew: () => set('year', '2025'),
    fillInvalid: () => set('year', ''),
});
