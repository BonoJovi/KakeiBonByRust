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
const COUNTER_CLASS = 'char-counter';
const COUNTER_STYLE = 'color:#888;font-size:11px;margin-top:2px;text-align:right;line-height:1.3;';

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

/**
 * Attach a live character counter ("actual / max") to a bounded-length
 * input/textarea. Updates on every `input` event. Counts Unicode code
 * points via `[...str].length` so the displayed count matches the
 * backend's `chars().count()` validation (surrogate pairs count as one).
 * Input beyond `max` code points is truncated, replacing the HTML
 * `maxlength` attribute (which counts UTF-16 code units, not code points).
 *
 * IME composition guard (Fable-5 review #D1): pre-conversion IME input
 * (kana/kanji, hangul, pinyin) fires `input` events for every candidate
 * keystroke. Writing back to `inputEl.value` mid-composition — as the
 * truncation branch does — cancels or corrupts the IME buffer, breaking
 * Japanese input around the boundary. This helper tracks
 * `compositionstart` / `compositionend` and skips truncation while a
 * composition is active; truncation is applied once the composition
 * commits. Before this fix a stray `maxlength=128` attribute masked the
 * bug by keeping input under the boundary; removing the attribute
 * (product-management.html:70, shop-management.html:65,
 * manufacturer-management.html:69) exposed it.
 *
 * Idempotent: calling twice on the same element detaches the prior
 * listeners first.
 *
 * @param {HTMLElement} inputEl - <input> or <textarea> element
 * @param {number} max - limit in characters (same value as backend bound)
 * @returns {() => void} detach — removes the listeners and counter element
 */
export function attachCharCounter(inputEl, max) {
    if (!inputEl) return () => {};

    // Idempotent: detach any prior attachment so we don't stack listeners.
    if (inputEl.__charCounterDetach) {
        inputEl.__charCounterDetach();
    }

    let counterEl = findCounterElement(inputEl);
    if (!counterEl) {
        counterEl = document.createElement('div');
        counterEl.className = COUNTER_CLASS;
        counterEl.style.cssText = COUNTER_STYLE;
        inputEl.insertAdjacentElement('afterend', counterEl);
    }

    let isComposing = false;

    const update = () => {
        const chars = [...(inputEl.value || '')];
        if (!isComposing && chars.length > max) {
            inputEl.value = chars.slice(0, max).join('');
        }
        counterEl.textContent = `${[...(inputEl.value || '')].length} / ${max}`;
    };
    const onCompositionStart = () => {
        isComposing = true;
    };
    const onCompositionEnd = () => {
        // Composition committed: apply any truncation the composing
        // branch skipped and refresh the counter with the final value.
        isComposing = false;
        update();
    };

    inputEl.addEventListener('input', update);
    inputEl.addEventListener('compositionstart', onCompositionStart);
    inputEl.addEventListener('compositionend', onCompositionEnd);
    update();

    const detach = () => {
        inputEl.removeEventListener('input', update);
        inputEl.removeEventListener('compositionstart', onCompositionStart);
        inputEl.removeEventListener('compositionend', onCompositionEnd);
        if (inputEl.__charCounterDetach === detach) {
            delete inputEl.__charCounterDetach;
        }
        counterEl.remove();
    };
    inputEl.__charCounterDetach = detach;

    return detach;
}

function findErrorElement(inputEl) {
    return findSiblingByClass(inputEl, ERROR_CLASS);
}

function findCounterElement(inputEl) {
    return findSiblingByClass(inputEl, COUNTER_CLASS);
}

function findSiblingByClass(inputEl, className) {
    let sibling = inputEl.nextElementSibling;
    while (sibling) {
        if (sibling.classList && sibling.classList.contains(className)) {
            return sibling;
        }
        sibling = sibling.nextElementSibling;
    }
    return null;
}
