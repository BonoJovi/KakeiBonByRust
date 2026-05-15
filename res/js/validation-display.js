/**
 * Shared validation-error display helper.
 *
 * Renders a single inline error message immediately after the input
 * element. Designed to be the one place that decides:
 *   - where the message lives in the DOM (next sibling of the input)
 *   - what it looks like (red, 12px, left-aligned, no layout shift)
 *   - the placeholder substitution path (delegated to i18n.t)
 *
 * Phase 1 foundation for issue #37 — unused until services and HTML
 * are wired up in Phase 2.
 */

import i18n from './i18n.js';

const ERROR_CLASS = 'validation-error';
const ERROR_STYLE = 'color:#c0392b;font-size:12px;margin-top:4px;line-height:1.3;';

/**
 * Show or update the inline error message for `inputEl`.
 *
 * @param {HTMLElement} inputEl - <input> or <textarea> element
 * @param {string} message - already-translated, ready-to-display text
 */
export function showValidationError(inputEl, message) {
    if (!inputEl) return;
    let errorEl = findErrorElement(inputEl);
    if (!errorEl) {
        errorEl = document.createElement('div');
        errorEl.className = ERROR_CLASS;
        errorEl.style.cssText = ERROR_STYLE;
        // Insert right after the input so the message tracks the field
        // even when fields are inside grid/flex form rows.
        inputEl.insertAdjacentElement('afterend', errorEl);
    }
    errorEl.textContent = message;
}

/**
 * Remove the inline error message for `inputEl` (no-op if none exists).
 *
 * @param {HTMLElement} inputEl
 */
export function clearValidationError(inputEl) {
    if (!inputEl) return;
    const errorEl = findErrorElement(inputEl);
    if (errorEl) errorEl.remove();
}

/**
 * Convenience wrapper for the most common case: a max-length violation.
 * Looks up the shared `validation.max_length` i18n message and substitutes
 * the field label, max, and actual character count.
 *
 * @param {HTMLElement} inputEl
 * @param {string} fieldLabel - localized name of the field (e.g. i18n.t('common.shop'))
 * @param {number} max - the limit in characters
 * @param {number} [actual] - current character count; defaults to inputEl.value length
 */
export function showMaxLengthError(inputEl, fieldLabel, max, actual) {
    if (!inputEl) return;
    const count = actual !== undefined
        ? actual
        : [...(inputEl.value || '')].length; // codepoint-aware count
    const message = i18n.t('validation.max_length', {
        field: fieldLabel,
        max: max,
        actual: count,
    });
    showValidationError(inputEl, message);
}

function findErrorElement(inputEl) {
    const next = inputEl.nextElementSibling;
    return next && next.classList && next.classList.contains(ERROR_CLASS)
        ? next
        : null;
}
