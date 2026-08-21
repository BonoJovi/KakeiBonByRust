/**
 * Shared save flow for master-CRUD screens (shop, manufacturer, product).
 *
 * Before this module every screen carried its own ~120-line `saveXxx()`
 * function whose bodies drifted whenever one screen was patched and the
 * other two were not — most recently around the not_found handling and
 * the substring-based error classifier. Fable-5 review items #D3, #D4
 * and #23 all ask for the same thing: one place that
 *
 *   (1) runs the defense-in-depth client validation for name/memo
 *   (2) resolves the edit target from the local cache before invoke
 *   (3) invokes the backend command the caller specifies
 *   (4) maps the returned `ApiError { code, message, entity? }` back
 *       to localised inline errors / toasts via `mapMasterErrorCode`
 *   (5) leaves post-save side effects (list reload, draft link,
 *       side-trip navigation) to the caller
 *
 * The classifier keys off the stable `err.code` returned by the Rust
 * services (`src/api_error.rs`) — never the English `err.message`.
 * That decoupling is the whole point of #23; keep it that way when
 * adding new codes.
 */

import i18n from './i18n.js';
import { showValidationError, showMaxLengthError, clearValidationError } from './validation-display.js';
import { showToast } from './toast.js';

/**
 * ApiError codes returned by the Rust master-CRUD services. Kept in
 * lock-step with `src/api_error.rs` (`CODE_*` constants) — if you
 * rename one on either side, rename it here too.
 */
export const API_ERROR_CODES = Object.freeze({
    DUPLICATE_NAME: 'duplicate_name',
    NOT_FOUND: 'not_found',
    MANUFACTURER_NOT_FOUND: 'manufacturer_not_found',
    VALIDATION: 'validation',
    DATABASE: 'database',
});

/**
 * Classify a backend error into inline / toast messages for the master
 * CRUD save flow. Returns `{ nameMessage, memoMessage, toastMessage }`
 * with any subset populated; caller decides where to route each.
 *
 * Accepts either the raw `catch (err)` object (`{ code, message, entity }`
 * shape returned by Tauri after `Result<T, ApiError>` serialises), or a
 * bare string for defensive backwards-compatibility while any callers
 * still get plain-string errors from unmigrated commands.
 *
 * @param {object|string} err
 * @param {object} ctx
 * @param {string} ctx.i18nPrefix          e.g. 'shop_mgmt'
 * @param {string} ctx.nameFieldI18nKey    e.g. 'shop_mgmt.shop_name'
 * @param {string} ctx.memoFieldI18nKey    e.g. 'shop_mgmt.memo'
 * @param {number} ctx.nameMaxLen
 * @param {number} ctx.memoMaxLen
 * @param {number} ctx.actualNameLen       code-point count of the submitted name
 * @param {number} ctx.actualMemoLen       code-point count of the submitted memo
 */
export function mapMasterErrorCode(err, ctx) {
    const isObject = err !== null && typeof err === 'object';
    const code = isObject ? err.code : undefined;
    const message = isObject ? String(err.message ?? '') : String(err ?? '');

    switch (code) {
        case API_ERROR_CODES.DUPLICATE_NAME:
            return {
                nameMessage: i18n.t(`${ctx.i18nPrefix}.duplicate_error`),
                memoMessage: null,
                toastMessage: null,
            };

        case API_ERROR_CODES.NOT_FOUND:
            return {
                nameMessage: null,
                memoMessage: null,
                toastMessage: i18n.t(`${ctx.i18nPrefix}.not_found`),
            };

        case API_ERROR_CODES.MANUFACTURER_NOT_FOUND:
            return {
                nameMessage: null,
                memoMessage: null,
                toastMessage: i18n.t('product_mgmt.manufacturer_not_found'),
            };

        case API_ERROR_CODES.VALIDATION: {
            // Validation trip is rare — the client-side guard should
            // have caught it. When it does fire, use the message text
            // to pick between the memo-length case and the name-length
            // / empty-name cases so the inline error lands on the
            // right field. This is the ONE place the classifier still
            // looks at message text, and only after `code === 'validation'`
            // has already narrowed the space.
            if (message.startsWith('Memo ')) {
                return {
                    nameMessage: null,
                    memoMessage: i18n.t('validation.max_length', {
                        field: i18n.t(ctx.memoFieldI18nKey),
                        max: ctx.memoMaxLen,
                        actual: ctx.actualMemoLen,
                    }),
                    toastMessage: null,
                };
            }
            if (message.includes('cannot be empty')) {
                return {
                    nameMessage: i18n.t(`${ctx.i18nPrefix}.empty_name`),
                    memoMessage: null,
                    toastMessage: null,
                };
            }
            if (message.includes('characters or less')) {
                return {
                    nameMessage: i18n.t('validation.max_length', {
                        field: i18n.t(ctx.nameFieldI18nKey),
                        max: ctx.nameMaxLen,
                        actual: ctx.actualNameLen,
                    }),
                    memoMessage: null,
                    toastMessage: null,
                };
            }
            // Unknown validation subtype — fall through to the generic
            // failure message rather than the raw English text.
            return {
                nameMessage: i18n.t(`${ctx.i18nPrefix}.failed_to_save`),
                memoMessage: null,
                toastMessage: null,
            };
        }

        // Legacy path — some Tauri commands not yet migrated to ApiError
        // still return a plain string. Do a last-ditch substring match so
        // this file stays interoperable during the rollout. New callers
        // hitting a migrated command will never reach this branch because
        // `code` is always set.
        case undefined: {
            if (message.includes('already exists')) {
                return {
                    nameMessage: i18n.t(`${ctx.i18nPrefix}.duplicate_error`),
                    memoMessage: null,
                    toastMessage: null,
                };
            }
            if (message.includes('cannot be empty')) {
                return {
                    nameMessage: i18n.t(`${ctx.i18nPrefix}.empty_name`),
                    memoMessage: null,
                    toastMessage: null,
                };
            }
            return {
                nameMessage: i18n.t(`${ctx.i18nPrefix}.failed_to_save`),
                memoMessage: null,
                toastMessage: null,
            };
        }

        // Database or unknown code — surface a generic failure inline
        // and rely on console.error at the call site for the details.
        default:
            return {
                nameMessage: i18n.t(`${ctx.i18nPrefix}.failed_to_save`),
                memoMessage: null,
                toastMessage: null,
            };
    }
}

/**
 * Orchestrated master-CRUD save. See file header for the full
 * responsibilities. Screens supply:
 *
 *   - `nameInput`, `memoInput`       — the two HTMLInputElements the
 *                                       client-side validation lives on.
 *   - `editingId`                    — non-null iff we are updating.
 *   - `findInCacheById(id)`          — returns the cached row when the
 *                                       list is loaded, else null. Used to
 *                                       distinguish concurrent-delete
 *                                       from actual-invoke-error paths.
 *   - `invokeAdd(name, memo)`        — closure the caller wraps
 *                                       around invoke('add_...', {...})
 *                                       so it can inject extra fields
 *                                       (manufacturer_id, is_disabled).
 *   - `invokeUpdate(row, name, memo)`— same shape for update.
 *   - `i18nPrefix`, `nameFieldI18nKey`, `memoFieldI18nKey`,
 *     `nameMaxLen`, `memoMaxLen`    — passed through to
 *                                       `mapMasterErrorCode` when
 *                                       classification fires.
 *   - `onNotFoundBeforeInvoke()`    — optional; runs when the edit
 *                                       target vanished from the cache
 *                                       (typically show toast + reload
 *                                       list). If omitted, a generic
 *                                       toast + no-op is used.
 *   - `onSuccess(mode, name)`       — optional; runs after a successful
 *                                       invoke. Screens do their list
 *                                       reload / side-trip navigation
 *                                       here. Mode is 'add' | 'update'.
 *
 * Throws on backend error so the surrounding Modal keeps its inline
 * error visible and stays open (Modal._handleSave does not close on
 * error). Returns `{ mode: 'add' | 'update' | 'skip' }` on success or
 * on a "target vanished" short-circuit.
 */
export async function saveMasterEntry({
    nameInput,
    memoInput,
    editingId,
    findInCacheById,
    invokeAdd,
    invokeUpdate,
    i18nPrefix,
    nameFieldI18nKey,
    memoFieldI18nKey,
    nameMaxLen,
    memoMaxLen,
    onNotFoundBeforeInvoke,
    onSuccess,
}) {
    const name = nameInput.value.trim();
    const memo = memoInput.value.trim();

    clearValidationError(nameInput);
    clearValidationError(memoInput);

    // Client-side validation mirrors Rust `validate_master_name`.
    if (!name) {
        showValidationError(nameInput, i18n.t(`${i18nPrefix}.empty_name`));
        throw new Error(`Validation error: empty ${i18nPrefix} name`);
    }
    if ([...name].length > nameMaxLen) {
        showMaxLengthError(nameInput, i18n.t(nameFieldI18nKey), nameMaxLen);
        throw new Error(`Validation error: ${i18nPrefix} name too long`);
    }
    if (memo && [...memo].length > memoMaxLen) {
        showMaxLengthError(memoInput, i18n.t(memoFieldI18nKey), memoMaxLen);
        throw new Error(`Validation error: ${i18nPrefix} memo too long`);
    }

    // Resolve edit target BEFORE the invoke try/catch so a concurrent
    // delete lands on the dedicated not_found path — not layered under
    // a generic save-error inline message.
    let target = null;
    if (editingId !== null && editingId !== undefined) {
        target = findInCacheById(editingId);
        if (!target) {
            if (onNotFoundBeforeInvoke) {
                await onNotFoundBeforeInvoke();
            } else {
                showToast(i18n.t(`${i18nPrefix}.not_found`), { variant: 'error' });
            }
            return { mode: 'skip' };
        }
    }

    try {
        if (target) {
            await invokeUpdate(target, name, memo || null);
        } else {
            await invokeAdd(name, memo || null);
        }
    } catch (err) {
        console.error(`Failed to save (${i18nPrefix}):`, err);

        // Backend-reported not_found means the target vanished BETWEEN
        // the cache read and the invoke — behave like the cache-miss
        // path above (reload the list so the toast's "the list has been
        // reloaded" wording matches reality, close the modal because
        // there is nothing left to edit). Without this branch the
        // classifier's toast fired without any actual reload, so the
        // list kept showing the stale row (Devin review on #97).
        const isBackendNotFound = err !== null
            && typeof err === 'object'
            && err.code === API_ERROR_CODES.NOT_FOUND;
        if (isBackendNotFound) {
            if (onNotFoundBeforeInvoke) {
                await onNotFoundBeforeInvoke();
            } else {
                showToast(i18n.t(`${i18nPrefix}.not_found`), { variant: 'error' });
            }
            return { mode: 'skip' };
        }

        const mapped = mapMasterErrorCode(err, {
            i18nPrefix,
            nameFieldI18nKey,
            memoFieldI18nKey,
            nameMaxLen,
            memoMaxLen,
            actualNameLen: [...name].length,
            actualMemoLen: [...memo].length,
        });

        if (mapped.toastMessage) {
            showToast(mapped.toastMessage, { variant: 'error' });
        }
        if (mapped.nameMessage) showValidationError(nameInput, mapped.nameMessage);
        if (mapped.memoMessage) showValidationError(memoInput, mapped.memoMessage);

        // Re-throw so Modal stays open (its _handleSave never closes on error).
        throw err;
    }

    if (onSuccess) {
        await onSuccess(target ? 'update' : 'add', name);
    }
    return { mode: target ? 'update' : 'add' };
}
