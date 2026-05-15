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
 * The dialog defaults to **keep the current total** (Enter / ESC) because the
 * common case is "user is still adding details, mismatch is expected". The
 * caller has to actively pick "overwrite" to apply the computed value.
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

    const choice = await showRecalcDialog(currentTotal, recommended);
    if (choice !== 'apply') {
        // 'keep' or dismissed via ESC / backdrop — leave the header alone.
        return { applied: false, recommended };
    }

    try {
        await invoke('update_transaction_header_total', {
            transactionId: parseInt(transactionId),
            totalAmount: recommended
        });
        return { applied: true, recommended };
    } catch (error) {
        console.error('Failed to update header total:', error);
        return { applied: false, recommended };
    }
}

/**
 * Render the reconciliation dialog and resolve with the user's choice.
 *
 * Lazily creates the modal DOM on first call so the helper works in any
 * host page that already loads modal CSS (`.modal` / `.modal-content` /
 * `.modal-actions` / `.btn-primary` / `.btn-secondary`).
 *
 * @returns {Promise<'keep'|'apply'>}
 */
function showRecalcDialog(currentTotal, recommended) {
    const modal = ensureRecalcModal();
    const titleEl = modal.querySelector('#header-recalc-title');
    const bodyEl = modal.querySelector('#header-recalc-body');
    const keepBtn = modal.querySelector('#header-recalc-keep');
    const applyBtn = modal.querySelector('#header-recalc-apply');

    titleEl.textContent = i18n.t('detail_mgmt.header_recalc_title');
    bodyEl.textContent = i18n.t('detail_mgmt.header_recalc_body')
        .replace('{0}', `¥${currentTotal.toLocaleString()}`)
        .replace('{1}', `¥${recommended.toLocaleString()}`);
    keepBtn.textContent = i18n.t('detail_mgmt.header_recalc_keep');
    applyBtn.textContent = i18n.t('detail_mgmt.header_recalc_apply');

    return new Promise((resolve) => {
        const settle = (choice) => {
            modal.classList.add('hidden');
            keepBtn.removeEventListener('click', onKeep);
            applyBtn.removeEventListener('click', onApply);
            modal.removeEventListener('click', onBackdrop);
            document.removeEventListener('keydown', onKey);
            resolve(choice);
        };
        const onKeep = () => settle('keep');
        const onApply = () => settle('apply');
        const onBackdrop = (e) => { if (e.target === modal) settle('keep'); };
        const onKey = (e) => { if (e.key === 'Escape') settle('keep'); };

        keepBtn.addEventListener('click', onKeep);
        applyBtn.addEventListener('click', onApply);
        modal.addEventListener('click', onBackdrop);
        document.addEventListener('keydown', onKey);

        modal.classList.remove('hidden');
        // Keep the modal content scrolled to the top on every open.
        const modalContent = modal.querySelector('.modal-content');
        if (modalContent) modalContent.scrollTop = 0;
        // Focus the "keep" button so Enter confirms the recommended action.
        keepBtn.focus();
    });
}

function ensureRecalcModal() {
    let modal = document.getElementById('header-recalc-modal');
    if (modal) return modal;

    modal = document.createElement('div');
    modal.id = 'header-recalc-modal';
    modal.className = 'modal hidden';
    modal.innerHTML = `
        <div class="modal-content">
            <div class="modal-header">
                <h2 id="header-recalc-title"></h2>
            </div>
            <div class="modal-body">
                <p id="header-recalc-body" style="white-space:pre-line;"></p>
            </div>
            <div class="modal-actions">
                <button type="button" class="btn btn-secondary" id="header-recalc-apply"></button>
                <button type="button" class="btn btn-primary" id="header-recalc-keep"></button>
            </div>
        </div>
    `;
    document.body.appendChild(modal);
    return modal;
}
