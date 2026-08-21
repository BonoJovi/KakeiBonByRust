/**
 * Modal — Double-Submit Guard Tests (D2)
 *
 * Regression coverage for the re-entrancy guard in `_handleSave`. Prior to
 * this fix, rapid Save clicks or repeated Enter presses would invoke the
 * `onSave` callback multiple times concurrently. Combined with the
 * SELECT-then-INSERT duplicate check in Rust master services (a TOCTOU
 * window), both invokes could reach the DB and the second failed with the
 * raw "UNIQUE constraint failed" message — no substring branch in the
 * frontend classifier matched, so a generic inline "save failed" error was
 * shown even though the first save had already succeeded.
 *
 * These tests exercise the Modal class in isolation. Backend calls are
 * simulated via a jest.fn() plumbed into `onSave` that resolves on demand.
 */

import { jest } from '@jest/globals';
import { Modal } from '../js/modal.js';

/**
 * Build a minimal DOM that Modal recognises: outer .modal wrapper, an inner
 * form with a submit button (the shop/manufacturer/product shape), close /
 * cancel buttons wired by id.
 */
function buildModalDom() {
    document.body.innerHTML = `
        <div id="test-modal" class="hidden">
            <div class="modal-content">
                <button id="close-btn" type="button">×</button>
                <form id="test-form">
                    <input type="text" name="field" />
                    <button id="cancel-btn" type="button">Cancel</button>
                    <button id="save-btn" type="submit">Save</button>
                </form>
            </div>
        </div>
    `;
}

/**
 * Build a save-button-driven modal (the category/user shape) — no form
 * element, save is wired via `saveButtonId`.
 */
function buildButtonModalDom() {
    document.body.innerHTML = `
        <div id="test-modal" class="hidden">
            <div class="modal-content">
                <button id="close-btn" type="button">×</button>
                <button id="cancel-btn" type="button">Cancel</button>
                <button id="save-btn" type="button">Save</button>
            </div>
        </div>
    `;
}

/**
 * A deferred is a promise you can resolve/reject from the outside — useful
 * for holding an in-flight save open across event dispatches.
 */
function deferred() {
    let resolveFn;
    let rejectFn;
    const promise = new Promise((resolve, reject) => {
        resolveFn = resolve;
        rejectFn = reject;
    });
    return { promise, resolve: resolveFn, reject: rejectFn };
}

/**
 * Wait one microtask tick so promise-then callbacks queued by the code under
 * test run before we assert.
 */
function flush() {
    return new Promise(resolve => setTimeout(resolve, 0));
}

describe('Modal double-submit guard — form submit path', () => {
    beforeEach(() => {
        buildModalDom();
    });

    afterEach(() => {
        document.body.innerHTML = '';
    });

    test('form-submit rapid-fire invokes onSave only once', async () => {
        const inflight = deferred();
        const onSave = jest.fn(() => inflight.promise);

        const modal = new Modal('test-modal', {
            formId: 'test-form',
            closeButtonId: 'close-btn',
            cancelButtonId: 'cancel-btn',
            onSave,
        });
        modal.open('add');

        const form = document.getElementById('test-form');
        form.dispatchEvent(new Event('submit', { cancelable: true }));
        form.dispatchEvent(new Event('submit', { cancelable: true }));
        form.dispatchEvent(new Event('submit', { cancelable: true }));

        await flush();

        expect(onSave).toHaveBeenCalledTimes(1);

        // Release the first save so downstream state settles.
        inflight.resolve();
        await flush();
    });

    test('save button is disabled while onSave is pending', async () => {
        const inflight = deferred();
        const onSave = jest.fn(() => inflight.promise);

        const modal = new Modal('test-modal', {
            formId: 'test-form',
            closeButtonId: 'close-btn',
            cancelButtonId: 'cancel-btn',
            onSave,
        });
        modal.open('add');

        const saveBtn = document.getElementById('save-btn');
        expect(saveBtn.disabled).toBe(false);

        document.getElementById('test-form').dispatchEvent(
            new Event('submit', { cancelable: true })
        );
        await flush();

        expect(saveBtn.disabled).toBe(true);

        inflight.resolve();
        await flush();

        // After close(), modal hides but the DOM node remains; the guard
        // re-enables the button in the finally block.
        expect(saveBtn.disabled).toBe(false);
    });

    test('after a successful save, a second open+submit fires onSave again', async () => {
        const onSave = jest.fn(() => Promise.resolve());
        const modal = new Modal('test-modal', {
            formId: 'test-form',
            closeButtonId: 'close-btn',
            cancelButtonId: 'cancel-btn',
            onSave,
        });

        modal.open('add');
        document.getElementById('test-form').dispatchEvent(
            new Event('submit', { cancelable: true })
        );
        await flush();

        modal.open('add');
        document.getElementById('test-form').dispatchEvent(
            new Event('submit', { cancelable: true })
        );
        await flush();

        expect(onSave).toHaveBeenCalledTimes(2);
    });

    test('after a failed save, the guard resets and retry fires onSave again', async () => {
        const onSave = jest.fn()
            .mockRejectedValueOnce(new Error('boom'))
            .mockResolvedValueOnce(undefined);

        const modal = new Modal('test-modal', {
            formId: 'test-form',
            closeButtonId: 'close-btn',
            cancelButtonId: 'cancel-btn',
            onSave,
        });
        modal.open('add');

        // Call the guarded handler directly so the rejection is under our
        // await — dispatching via form.submit would leak an
        // "unhandledrejection" from jsdom's async event loop into Jest.
        await expect(modal._handleSave()).rejects.toThrow('boom');

        const saveBtn = document.getElementById('save-btn');
        expect(saveBtn.disabled).toBe(false);
        expect(modal.modal.classList.contains('hidden')).toBe(false);
        expect(modal._isSaving).toBe(false);

        // Retry succeeds — guard is not stuck.
        await modal._handleSave();
        expect(onSave).toHaveBeenCalledTimes(2);
        expect(modal.modal.classList.contains('hidden')).toBe(true);
    });
});

describe('Modal double-submit guard — saveButtonId path', () => {
    beforeEach(() => {
        buildButtonModalDom();
    });

    afterEach(() => {
        document.body.innerHTML = '';
    });

    test('rapid save-button clicks invoke onSave only once', async () => {
        const inflight = deferred();
        const onSave = jest.fn(() => inflight.promise);

        const modal = new Modal('test-modal', {
            closeButtonId: 'close-btn',
            cancelButtonId: 'cancel-btn',
            saveButtonId: 'save-btn',
            onSave,
        });
        modal.open('confirm');

        const saveBtn = document.getElementById('save-btn');
        saveBtn.click();
        saveBtn.click();
        saveBtn.click();

        await flush();

        expect(onSave).toHaveBeenCalledTimes(1);
        expect(saveBtn.disabled).toBe(true);

        inflight.resolve();
        await flush();
        expect(saveBtn.disabled).toBe(false);
    });
});
