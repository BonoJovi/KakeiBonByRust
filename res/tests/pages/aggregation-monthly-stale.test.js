// An older, slower Execute does not overwrite the newer one on the monthly aggregation screen (latent-audit scan2-A4)
/**
 * Monthly aggregation screen (res/js/aggregation.js). Scenario and
 * expectations: ./_aggregation-stale.js.
 */

import { jest } from '@jest/globals';
import { runStaleAggregationScenario } from './_aggregation-stale.js';

const set = (id, value) => { document.getElementById(id).value = value; };

await runStaleAggregationScenario(jest, {
    screen: 'monthly',
    html: 'aggregation.html',
    script: '../../js/aggregation.js',
    command: 'get_monthly_aggregation',
    isOld: (args) => args.month === 9,
    fillOld: () => { set('year', '2026'); set('month', '9'); },
    fillNew: () => { set('year', '2026'); set('month', '3'); },
});
