import { jest } from '@jest/globals';
import { showToast, clearAllToasts } from '../js/toast.js';

// Toast module is self-mounting (injects its own <style> and container on
// first call). Each test starts from a clean DOM so we observe the
// mount-on-first-call behavior independently.

const CONTAINER_ID = 'kb-toast-container';
const STYLE_ID = 'kb-toast-style';
const TOAST_CLASS = 'kb-toast';

function getContainer() {
    return document.getElementById(CONTAINER_ID);
}

function getToasts() {
    const container = getContainer();
    return container ? Array.from(container.querySelectorAll(`.${TOAST_CLASS}`)) : [];
}

beforeEach(() => {
    jest.useFakeTimers();
    document.body.innerHTML = '';
    document.head.innerHTML = '';
});

afterEach(() => {
    clearAllToasts();
    jest.useRealTimers();
});

describe('showToast — mounting', () => {
    test('injects style and container on first call', () => {
        expect(document.getElementById(STYLE_ID)).toBeNull();
        expect(getContainer()).toBeNull();

        showToast('hello');

        expect(document.getElementById(STYLE_ID)).not.toBeNull();
        expect(getContainer()).not.toBeNull();
    });

    test('reuses the same container across multiple calls', () => {
        showToast('first');
        const containerAfterFirst = getContainer();
        showToast('second');
        const containerAfterSecond = getContainer();

        expect(containerAfterFirst).toBe(containerAfterSecond);
        expect(document.querySelectorAll(`#${CONTAINER_ID}`).length).toBe(1);
        expect(document.querySelectorAll(`#${STYLE_ID}`).length).toBe(1);
    });
});

describe('showToast — rendering', () => {
    test('renders the message text verbatim (caller is responsible for i18n)', () => {
        showToast('保存しました');
        const toasts = getToasts();
        expect(toasts).toHaveLength(1);
        expect(toasts[0].querySelector(`.${TOAST_CLASS}-text`).textContent).toBe('保存しました');
    });

    test('defaults to the info variant when none is given', () => {
        showToast('hi');
        const toast = getToasts()[0];
        expect(toast.classList.contains(`${TOAST_CLASS}-info`)).toBe(true);
    });

    test('applies the requested variant class', () => {
        for (const variant of ['info', 'success', 'warning', 'error']) {
            clearAllToasts();
            showToast('msg', { variant });
            const toast = getToasts()[0];
            expect(toast.classList.contains(`${TOAST_CLASS}-${variant}`)).toBe(true);
        }
    });

    test('falls back to info for an unknown variant rather than rendering nothing', () => {
        showToast('msg', { variant: 'bogus' });
        const toast = getToasts()[0];
        expect(toast.classList.contains(`${TOAST_CLASS}-info`)).toBe(true);
    });

    test('error variant uses role=alert and aria-live=assertive for screen readers', () => {
        showToast('boom', { variant: 'error' });
        const toast = getToasts()[0];
        expect(toast.getAttribute('role')).toBe('alert');
        expect(toast.getAttribute('aria-live')).toBe('assertive');
    });

    test('non-error variants use role=status / aria-live=polite', () => {
        showToast('ok', { variant: 'success' });
        const toast = getToasts()[0];
        expect(toast.getAttribute('role')).toBe('status');
        expect(toast.getAttribute('aria-live')).toBe('polite');
    });
});

describe('showToast — dismissal', () => {
    test('auto-dismisses after the configured duration', () => {
        showToast('bye', { duration: 1000 });
        expect(getToasts()).toHaveLength(1);

        // Trigger the auto-dismiss timer and the fallback removal timer.
        jest.advanceTimersByTime(1000);
        jest.advanceTimersByTime(500);

        expect(getToasts()).toHaveLength(0);
    });

    test('duration=0 keeps the toast visible (no auto-dismiss timer fires)', () => {
        showToast('sticky', { duration: 0 });
        jest.advanceTimersByTime(10000);
        expect(getToasts()).toHaveLength(1);
    });

    test('returned dismiss function removes the toast immediately', () => {
        const dismiss = showToast('manual', { duration: 0 });
        expect(getToasts()).toHaveLength(1);

        dismiss();
        // Drain the fallback removal timer.
        jest.advanceTimersByTime(500);
        expect(getToasts()).toHaveLength(0);
    });

    test('clicking the close button dismisses the toast', () => {
        showToast('closable', { duration: 0 });
        const toast = getToasts()[0];
        const closeBtn = toast.querySelector(`.${TOAST_CLASS}-close`);
        expect(closeBtn).not.toBeNull();

        closeBtn.click();
        jest.advanceTimersByTime(500);
        expect(getToasts()).toHaveLength(0);
    });

    test('dismiss is idempotent — calling it twice does not throw', () => {
        const dismiss = showToast('once', { duration: 0 });
        dismiss();
        expect(() => dismiss()).not.toThrow();
        jest.advanceTimersByTime(500);
        expect(getToasts()).toHaveLength(0);
    });
});

describe('showToast — stacking', () => {
    test('multiple toasts stack inside the same container', () => {
        showToast('first', { duration: 0 });
        showToast('second', { duration: 0 });
        showToast('third', { duration: 0 });

        const toasts = getToasts();
        expect(toasts).toHaveLength(3);
        // Insertion order is preserved at the DOM level; the visual
        // bottom-up stacking is handled by `flex-direction: column-reverse`
        // in the CSS, which we don't assert in jsdom.
        expect(toasts[0].textContent).toContain('first');
        expect(toasts[2].textContent).toContain('third');
    });
});
