import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';

/**
 * Reconcile a transaction header's cached TOTAL_AMOUNT with what its current
 * details would compute. Prompts the user when the two disagree and, on
 * confirmation, persists the recommended value to the header.
 *
 * Used by every code path that can drift the relationship between the header
 * total and the details — detail add/update/delete, tax setting changes, and
 * manual edits to the total field — so the prompt UX is consistent across
 * the app.
 *
 * @param {number|string} transactionId - The header to reconcile.
 * @param {number} currentTotal - The TOTAL_AMOUNT the caller currently
 *   believes is on the header (typically what was just saved or what the UI
 *   has cached). Compared against the freshly-computed recommended value.
 * @returns {Promise<{applied: boolean, recommended: number|null}>}
 *   `applied` is true when the cached value can be trusted afterwards: the
 *   two were already equal, or the user accepted the overwrite. `recommended`
 *   is the value the backend would prefer (or null if the compute call failed).
 */
export async function applyHeaderRecalculationPrompt(transactionId, currentTotal) {
    let recommended;
    try {
        recommended = await invoke('compute_recommended_transaction_total', {
            transactionId: parseInt(transactionId)
        });
    } catch (error) {
        console.error('Failed to compute recommended total:', error);
        return { applied: false, recommended: null };
    }

    if (recommended === currentTotal) {
        return { applied: true, recommended };
    }

    // The i18n resource stores `\n` as the literal two characters (backslash
    // + n) because SQLite single-quoted strings do not interpret backslash
    // escapes. Convert them to real newlines here so confirm() renders the
    // line breaks the prompt was designed for.
    const message = i18n.t('detail_mgmt.header_recalc_confirm')
        .replace(/\\n/g, '\n')
        .replace('{0}', `¥${currentTotal.toLocaleString()}`)
        .replace('{1}', `¥${recommended.toLocaleString()}`);
    if (!confirm(message)) {
        console.log('[header-recalc] User declined the overwrite');
        return { applied: false, recommended };
    }

    console.log('[header-recalc] Persisting new total:', recommended, 'for txn', transactionId);
    try {
        await invoke('update_transaction_header_total', {
            transactionId: parseInt(transactionId),
            totalAmount: recommended
        });
        console.log('[header-recalc] update_transaction_header_total succeeded');
        return { applied: true, recommended };
    } catch (error) {
        console.error('[header-recalc] Failed to update header total:', error);
        return { applied: false, recommended };
    }
}
