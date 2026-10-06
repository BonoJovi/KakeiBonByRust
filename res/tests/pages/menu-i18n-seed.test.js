// Every menu bar label is seeded for ja and en (latent-audit scan2-C2)
/**
 * Menu bar (res/js/menu.js).
 *
 * scan2-C2  menu.js renders `data-i18n="menu.back_to_transactions"` in the
 *           File menu of the transaction detail screen, and
 *           i18n.updateUI() overwrites the English fallback text with
 *           t(key). The key was defined only in the legacy
 *           sql/add_detail_mgmt_i18n.sql, so a DB initialised from
 *           res/sql/dbaccess.sql had no row for it and the menu item showed
 *           "menu.back_to_transactions".
 *
 * Expected: every data-i18n key rendered by createMenuBar() is seeded in
 * dbaccess.sql for both ja and en.
 */

import fs from 'fs';
import path from 'path';
import { jest } from '@jest/globals';
import { mockPageModules, RES_DIR } from '../pages/_page-harness.js';

mockPageModules(jest, { keepMenu: true, invoke: () => null });
const { createMenuBar } = await import('../../js/menu.js');

/** RESOURCE_KEY -> Set of LANG_CODEs seeded in res/sql/dbaccess.sql. */
function seededLanguages() {
    const sql = fs.readFileSync(path.join(RES_DIR, 'sql', 'dbaccess.sql'), 'utf8');
    const map = new Map();
    const re = /\(\s*\d+\s*,\s*'([a-z0-9_.]+)'\s*,\s*'(ja|en)'/gi;
    let m;
    while ((m = re.exec(sql)) !== null) {
        if (!map.has(m[1])) map.set(m[1], new Set());
        map.get(m[1]).add(m[2]);
    }
    return map;
}

describe('menu bar i18n keys are seeded (scan2-C2)', () => {
    test('menu.back_to_transactions is seeded for ja and en', () => {
        const seeded = seededLanguages();
        expect([...(seeded.get('menu.back_to_transactions') ?? [])].sort()).toEqual(['en', 'ja']);
    });

    test('every data-i18n key in the menu bar is seeded for ja and en', () => {
        document.body.innerHTML = '<div id="menu-bar"></div>';
        createMenuBar('transaction-detail');
        const keys = [...document.querySelectorAll('[data-i18n]')]
            .map((el) => el.getAttribute('data-i18n'));
        expect(keys).toContain('menu.back_to_transactions');

        const seeded = seededLanguages();
        const missing = keys.filter((k) => {
            const langs = seeded.get(k);
            return !langs || !langs.has('ja') || !langs.has('en');
        });
        expect(missing).toEqual([]);
    });
});
