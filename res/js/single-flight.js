/**
 * Wrap an async form-submit handler so a second submit while the first is
 * still running is ignored.
 *
 * Forms that save through a Tauri command stay on screen until the command
 * (and any follow-up prompt) finishes, so a double click on Save or a
 * repeated Enter used to invoke the command twice and create duplicate rows
 * (latent-audit M19). The shared `Modal` class has its own guard; this one
 * covers the plain forms that do not go through it.
 *
 * The returned listener calls `event.preventDefault()` for every submit,
 * including the ignored ones, so the browser never falls back to a native
 * form submission.
 *
 * @param {(event: Event) => Promise<void>} handler
 * @returns {(event: Event) => Promise<void>}
 */
export function singleFlight(handler) {
    let inFlight = false;
    return async (event) => {
        event?.preventDefault?.();
        if (inFlight) return;
        inFlight = true;
        try {
            await handler(event);
        } finally {
            inFlight = false;
        }
    };
}
