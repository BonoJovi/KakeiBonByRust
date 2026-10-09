/**
 * Modal.open — waits for an async onOpen (latent-audit L6)
 *
 * `open()` called `onOpen` without waiting for it. The transaction screen's
 * onOpen is async (it loads the master lists, then resets the form and sets
 * today's date), so restoreModalState filled the saved draft in first and
 * onOpen's late reset overwrote it. `open()` now returns a promise that
 * settles once onOpen has finished. Pinned here on the Modal class alone.
 */

import { Modal } from '../../res/js/modal.js';
import { deferred, flush } from './pages/_page-harness.js';

function buildModalDom() {
    document.body.innerHTML = `
        <div id="test-modal" class="modal hidden">
            <div class="modal-content">
                <button id="close-btn" type="button">×</button>
                <form id="test-form">
                    <input type="text" name="field" id="field" />
                    <button id="save-btn" type="submit">Save</button>
                </form>
            </div>
        </div>
    `;
}

describe('Modal.open — async onOpen (latent audit L6)', () => {
    test('should settle only after onOpen has finished when onOpen is async (L6)', async () => {
        buildModalDom();
        const loading = deferred();
        const modal = new Modal('test-modal', {
            formId: 'test-form',
            closeButtonId: 'close-btn',
            onOpen: async () => {
                await loading.promise;
                document.getElementById('field').value = 'initialised';
            },
        });

        let settled = false;
        const opened = modal.open('add', {}).then(() => { settled = true; });
        await flush();
        // Shown immediately, but not settled while onOpen is still loading.
        expect(modal.modal.classList.contains('hidden')).toBe(false);
        expect(settled).toBe(false);

        loading.resolve();
        await opened;
        expect(settled).toBe(true);
        expect(document.getElementById('field').value).toBe('initialised');
    });

    test('should settle at once when onOpen is synchronous (L6)', async () => {
        buildModalDom();
        const modal = new Modal('test-modal', {
            formId: 'test-form',
            closeButtonId: 'close-btn',
            onOpen: () => {},
        });

        await expect(modal.open('add', {})).resolves.toBeUndefined();
    });
});
