/**
 * Latent audit 2026-09 — shared Modal class (res/js/modal.js)
 *
 * IDs covered: L22
 *
 * Bug: Modal._handleSave() awaits onSave() and then unconditionally calls
 *      this.close(). If the user closes the modal while a save is in flight
 *      and opens it again (e.g. to edit another row), the completion of the
 *      FIRST save closes — and form.reset()s — the NEW session, discarding
 *      whatever the user is typing.
 * Expected: completion of a save that belongs to an earlier open/close cycle
 *      must not close or reset the modal that was re-opened afterwards.
 *
 * Exercises the real Modal class in isolation (same pattern as
 * res/tests/modal-double-submit.test.js).
 */

import { Modal } from '../../js/modal.js';
import { deferred, flush } from '../pages/_page-harness.js';

function buildModalDom() {
    document.body.innerHTML = `
        <div id="test-modal" class="modal hidden">
            <div class="modal-content">
                <button id="close-btn" type="button">×</button>
                <form id="test-form">
                    <input type="text" name="field" id="field" />
                    <button id="cancel-btn" type="button">Cancel</button>
                    <button id="save-btn" type="submit">Save</button>
                </form>
            </div>
        </div>
    `;
}

describe('Modal — stale save completion (latent audit 2026-09)', () => {
    test('[latent L22] a save that finishes after close + re-open does not close/reset the new session', async () => {
        buildModalDom();
        const inflight = deferred();
        let calls = 0;
        const modal = new Modal('test-modal', {
            formId: 'test-form',
            closeButtonId: 'close-btn',
            cancelButtonId: 'cancel-btn',
            onSave: () => {
                calls++;
                return calls === 1 ? inflight.promise : Promise.resolve();
            },
        });

        // Session 1: edit row 1, press Save (backend slow).
        modal.open('edit', { rowId: 1 });
        document.getElementById('field').value = 'row 1 edit';
        document.getElementById('test-form').dispatchEvent(new Event('submit', { cancelable: true }));
        await flush();

        // User closes the modal while the save is pending...
        document.getElementById('close-btn').click();
        expect(modal.modal.classList.contains('hidden')).toBe(true);

        // ...and opens another row, starting to type.
        modal.open('edit', { rowId: 2 });
        document.getElementById('field').value = 'row 2 typing';

        // The first save now completes.
        inflight.resolve();
        await flush();

        expect(modal.modal.classList.contains('hidden')).toBe(false);
        expect(modal.getData()).toEqual({ rowId: 2 });
        expect(document.getElementById('field').value).toBe('row 2 typing');
    });
});
