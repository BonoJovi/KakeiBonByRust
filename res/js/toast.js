/**
 * Shared transient-notification helper (toast).
 *
 * Renders short-lived messages in a fixed-position container at the
 * bottom-right of the viewport. Designed to replace scattered
 * `alert()` calls and surface success / error / warning / info feedback
 * without blocking the user.
 *
 * Issue #45 — independent from but parallel to `validation-display.js`:
 *   - validation-display: inline, persistent until cleared (next to field)
 *   - toast: transient, auto-dismissing, viewport-anchored
 *
 * Self-mounting: the container DOM and styles are injected on first
 * call. Callers only need `import { showToast } from './toast.js'`.
 *
 * i18n contract: callers pass already-translated text, same as
 * `validation-display.js`. This module does not call `i18n.t`.
 */

const CONTAINER_ID = 'kb-toast-container';
const STYLE_ID = 'kb-toast-style';
const TOAST_CLASS = 'kb-toast';
const VARIANTS = new Set(['info', 'success', 'warning', 'error']);
const DEFAULT_DURATION_MS = 3000;
const ENTER_TRANSITION_MS = 200;
const EXIT_TRANSITION_MS = 200;

const STYLE_CSS = `
#${CONTAINER_ID} {
    position: fixed;
    bottom: 16px;
    right: 16px;
    display: flex;
    flex-direction: column-reverse;
    gap: 8px;
    z-index: 9999;
    pointer-events: none;
    max-width: calc(100vw - 32px);
}
.${TOAST_CLASS} {
    pointer-events: auto;
    min-width: 240px;
    max-width: 420px;
    padding: 12px 16px;
    border-radius: 6px;
    color: #fff;
    background: #3498db;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.18);
    display: flex;
    align-items: flex-start;
    gap: 12px;
    font-size: 14px;
    line-height: 1.4;
    opacity: 0;
    transform: translateX(20px);
    transition: opacity ${ENTER_TRANSITION_MS}ms ease-out, transform ${ENTER_TRANSITION_MS}ms ease-out;
}
.${TOAST_CLASS}.show {
    opacity: 1;
    transform: translateX(0);
}
.${TOAST_CLASS}.hide {
    opacity: 0;
    transform: translateX(20px);
    transition: opacity ${EXIT_TRANSITION_MS}ms ease-in, transform ${EXIT_TRANSITION_MS}ms ease-in;
}
.${TOAST_CLASS}-info { background: #3498db; }
.${TOAST_CLASS}-success { background: #27ae60; }
.${TOAST_CLASS}-warning { background: #d68910; }
.${TOAST_CLASS}-error { background: #c0392b; }
.${TOAST_CLASS}-text {
    flex: 1;
    word-break: break-word;
    white-space: pre-line;
}
.${TOAST_CLASS}-close {
    background: transparent;
    border: 0;
    color: inherit;
    font-size: 18px;
    line-height: 1;
    cursor: pointer;
    padding: 0 4px;
    opacity: 0.85;
}
.${TOAST_CLASS}-close:hover {
    opacity: 1;
}
`;

function ensureMounted() {
    if (typeof document === 'undefined') return null;
    if (!document.getElementById(STYLE_ID)) {
        const style = document.createElement('style');
        style.id = STYLE_ID;
        style.textContent = STYLE_CSS;
        document.head.appendChild(style);
    }
    let container = document.getElementById(CONTAINER_ID);
    if (!container) {
        container = document.createElement('div');
        container.id = CONTAINER_ID;
        container.setAttribute('aria-live', 'polite');
        document.body.appendChild(container);
    }
    return container;
}

/**
 * Show a transient toast notification.
 *
 * @param {string} message - already-translated, ready-to-display text
 * @param {object} [options]
 * @param {'info'|'success'|'warning'|'error'} [options.variant='info']
 * @param {number} [options.duration=3000] - auto-dismiss delay in ms; pass 0 to disable auto-dismiss
 * @returns {() => void} dismiss — call to remove the toast programmatically (no-op if already gone)
 */
export function showToast(message, options = {}) {
    const container = ensureMounted();
    if (!container) return () => {};

    const variant = VARIANTS.has(options.variant) ? options.variant : 'info';
    const duration = Number.isFinite(options.duration) ? options.duration : DEFAULT_DURATION_MS;

    const toast = document.createElement('div');
    toast.className = `${TOAST_CLASS} ${TOAST_CLASS}-${variant}`;
    // Errors interrupt screen readers; the rest defer to the next idle moment.
    toast.setAttribute('role', variant === 'error' ? 'alert' : 'status');
    toast.setAttribute('aria-live', variant === 'error' ? 'assertive' : 'polite');

    const text = document.createElement('span');
    text.className = `${TOAST_CLASS}-text`;
    text.textContent = String(message ?? '');
    toast.appendChild(text);

    const closeBtn = document.createElement('button');
    closeBtn.type = 'button';
    closeBtn.className = `${TOAST_CLASS}-close`;
    closeBtn.setAttribute('aria-label', 'Dismiss');
    closeBtn.textContent = '×';
    toast.appendChild(closeBtn);

    container.appendChild(toast);

    // Defer adding `show` to the next frame so the browser observes the
    // initial (hidden) state and animates into the visible one.
    const raf = (typeof requestAnimationFrame === 'function')
        ? requestAnimationFrame
        : (cb) => setTimeout(cb, 16);
    raf(() => toast.classList.add('show'));

    let exitTimer = null;
    let removeTimer = null;
    let dismissed = false;

    const dismiss = () => {
        if (dismissed) return;
        dismissed = true;
        if (exitTimer) clearTimeout(exitTimer);
        exitTimer = null;
        toast.classList.remove('show');
        toast.classList.add('hide');
        // Fallback removal in case `transitionend` never fires (e.g. jsdom).
        removeTimer = setTimeout(() => {
            if (toast.parentNode) toast.parentNode.removeChild(toast);
        }, EXIT_TRANSITION_MS + 50);
        toast.addEventListener('transitionend', () => {
            if (removeTimer) clearTimeout(removeTimer);
            if (toast.parentNode) toast.parentNode.removeChild(toast);
        }, { once: true });
    };

    closeBtn.addEventListener('click', dismiss);

    if (duration > 0) {
        exitTimer = setTimeout(dismiss, duration);
    }

    return dismiss;
}

/**
 * Remove all currently-mounted toasts and the container. Mostly useful
 * for tests; production code should let toasts auto-dismiss.
 */
export function clearAllToasts() {
    if (typeof document === 'undefined') return;
    const container = document.getElementById(CONTAINER_ID);
    if (container && container.parentNode) {
        container.parentNode.removeChild(container);
    }
    const style = document.getElementById(STYLE_ID);
    if (style && style.parentNode) {
        style.parentNode.removeChild(style);
    }
}
