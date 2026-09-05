/**
 * aggregation-common — `renderResults` unspecified-group i18n swap (Fable-5 #22)
 *
 * Pins the frontend half of the fix that stopped Japanese "指定なし"
 * from leaking into the aggregation banner on the English UI. The
 * backend now returns an empty string for the unspecified-group row
 * (SHOP_ID NULL, PRODUCT_ID NULL, or account_code === 'NONE');
 * `renderResults` swaps that empty string for
 * `i18n.t('common.unspecified')` so English users see "Unspecified"
 * and Japanese users see "指定なし" — both routed through the
 * localised i18n resource `common.unspecified` (RESOURCE_ID 801/802).
 *
 * `aggregation-common.js` transitively imports `res/js/i18n.js`,
 * which pulls `@tauri-apps/api/core` — a real ESM module that only
 * exists in a Tauri build. Stubbed with `jest.unstable_mockModule`,
 * then dynamic-imported (same pattern as
 * `aggregation-error-translate.test.js`).
 */

import { jest } from '@jest/globals';

jest.unstable_mockModule('@tauri-apps/api/core', () => ({
    invoke: jest.fn(),
}));

const translations = {
    'common.unspecified': 'i18n:unspecified',
    'aggregation.total': 'i18n:total',
    'aggregation.no_results': 'i18n:no_results',
};
jest.unstable_mockModule('../js/i18n.js', () => ({
    default: {
        t: (key) => translations[key] || key,
        updateUI: () => {},
        init: async () => {},
    },
}));

const { renderResults } = await import('../js/aggregation-common.js');

// Build a fresh <tbody> / <tfoot> per test. `renderResults` walks
// `results.forEach` and appends `<tr>` elements to `tbody`, so we can
// read them back via `tbody.querySelectorAll('tr td:first-child')`
// to check the group-name cell.
function makeTable() {
    const table = document.createElement('table');
    const tbody = document.createElement('tbody');
    const tfoot = document.createElement('tfoot');
    table.appendChild(tbody);
    table.appendChild(tfoot);
    return { tbody, tfoot };
}

function firstColumnTexts(tbody) {
    return Array.from(tbody.querySelectorAll('tr td:first-child')).map((td) => td.textContent);
}

describe('renderResults — unspecified-group i18n swap (Fable-5 #22)', () => {
    test('empty group_name is swapped for the localised label', () => {
        const { tbody, tfoot } = makeTable();
        renderResults(
            [{ group_key: '0', group_name: '', total_amount: 100, count: 1, avg_amount: 100 }],
            tbody,
            tfoot,
        );
        expect(firstColumnTexts(tbody)).toEqual(['i18n:unspecified']);
    });

    test('non-empty group_name is rendered verbatim', () => {
        const { tbody, tfoot } = makeTable();
        renderResults(
            [{ group_key: '1', group_name: 'Real Shop', total_amount: 100, count: 1, avg_amount: 100 }],
            tbody,
            tfoot,
        );
        expect(firstColumnTexts(tbody)).toEqual(['Real Shop']);
    });

    test('mixed rows — real names stay, empty ones swap to i18n', () => {
        const { tbody, tfoot } = makeTable();
        renderResults(
            [
                { group_key: '1', group_name: 'Real Shop', total_amount: 100, count: 1, avg_amount: 100 },
                { group_key: '0', group_name: '', total_amount: 50, count: 1, avg_amount: 50 },
                { group_key: '2', group_name: 'Another', total_amount: 200, count: 1, avg_amount: 200 },
            ],
            tbody,
            tfoot,
        );
        expect(firstColumnTexts(tbody)).toEqual(['Real Shop', 'i18n:unspecified', 'Another']);
    });

    test('null group_name is treated the same as empty (defensive)', () => {
        // Not the current backend contract but a defensive `||` on the
        // JS side also catches this shape, so pin it too.
        const { tbody, tfoot } = makeTable();
        renderResults(
            [{ group_key: '0', group_name: null, total_amount: 100, count: 1, avg_amount: 100 }],
            tbody,
            tfoot,
        );
        expect(firstColumnTexts(tbody)).toEqual(['i18n:unspecified']);
    });
});

describe('renderResults — regression: no-results path still fires', () => {
    // The unspecified-swap change touches the per-row loop, not the
    // no-results branch. Pin that branch here so a future edit to the
    // helper can't silently break the empty-state UX.
    test('empty results renders the no_results i18n cell', () => {
        const { tbody, tfoot } = makeTable();
        renderResults([], tbody, tfoot);
        // Empty-state row has one <td> that carries the data-i18n
        // attribute and default text; on the mocked i18n.updateUI()
        // call the text is not swapped (mock is a no-op), so check
        // the raw content instead.
        const emptyCell = tbody.querySelector('td.empty-state');
        expect(emptyCell).not.toBeNull();
        expect(emptyCell.textContent.trim()).toBe('No results found');
    });
});
