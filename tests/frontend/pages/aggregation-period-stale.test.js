// An older, slower Execute does not overwrite the newer one on the period aggregation screen (latent-audit scan2-A4)
/**
 * Period aggregation screen (res/js/aggregation-period.js). Scenario and
 * expectations: ./_aggregation-stale.js.
 */

import { jest } from '@jest/globals';
import { runStaleAggregationScenario } from './_aggregation-stale.js';

const set = (id, value) => { document.getElementById(id).value = value; };

await runStaleAggregationScenario(jest, {
    screen: 'period',
    html: 'aggregation-period.html',
    script: '../../../res/js/aggregation-period.js',
    command: 'get_period_aggregation',
    fillOld: () => { set('start-date', '2026-09-01'); set('end-date', '2026-09-30'); },
    fillNew: () => { set('start-date', '2026-03-01'); set('end-date', '2026-03-31'); },
    fillInvalid: () => { set('start-date', ''); set('end-date', ''); },
});
