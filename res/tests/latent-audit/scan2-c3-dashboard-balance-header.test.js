// latent-audit scan2-C3: `dashboard.balance` is seeded twice with different meanings; the Account Balances header shows 「収支」
/**
 * res/sql/dbaccess.sql seeds `dashboard.balance` first as 収支 (income/expense
 * balance, used as the dashboard chart label) and later as 残高 (the Account
 * Balances column header). INSERT OR IGNORE on UNIQUE(RESOURCE_KEY, LANG_CODE)
 * keeps the first row, so the column header in res/dashboard.html reads 収支.
 * Expected: the key used by the Account Balances column header resolves to
 * 残高 in a DB initialised from dbaccess.sql.
 */

import fs from 'fs';
import path from 'path';
import { loadPageBody, RES_DIR } from '../pages/_page-harness.js';

/**
 * Effective I18N_RESOURCES rows after running dbaccess.sql top to bottom:
 * INSERT OR IGNORE keeps the first row per RESOURCE_ID and per
 * (RESOURCE_KEY, LANG_CODE).
 */
function effectiveTranslations() {
    const sql = fs.readFileSync(path.join(RES_DIR, 'sql', 'dbaccess.sql'), 'utf8');
    const re = /\(\s*(\d+)\s*,\s*'([a-z0-9_.]+)'\s*,\s*'(ja|en)'\s*,\s*'((?:[^']|'')*)'/gi;
    const ids = new Set();
    const values = new Map();
    let m;
    while ((m = re.exec(sql)) !== null) {
        const [, id, key, lang, value] = m;
        const k = `${key}|${lang}`;
        if (ids.has(id) || values.has(k)) continue;
        ids.add(id);
        values.set(k, value.replace(/''/g, "'"));
    }
    return values;
}

describe('scan2-C3 — Account Balances column header', () => {
    test('the balance column header resolves to 残高 (ja) / Balance (en)', () => {
        loadPageBody('dashboard.html');
        const th = document.querySelector('.account-balances-table th.balance-col');
        expect(th).not.toBeNull();
        const key = th.getAttribute('data-i18n');

        const values = effectiveTranslations();
        expect(values.get(`${key}|ja`)).toBe('残高');
        expect(values.get(`${key}|en`)).toBe('Balance');
    });
});
