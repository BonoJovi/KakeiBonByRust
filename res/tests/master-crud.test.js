/**
 * master-crud — mapMasterErrorCode + saveMasterEntry tests (Fable-5 #D3/#D4/#23)
 *
 * Covers the classifier that keys off `ApiError.code` returned from the
 * Rust master services and the orchestration wrapper that every screen's
 * save flow now delegates to.
 *
 * The module under test transitively pulls `res/js/i18n.js`, which
 * imports `@tauri-apps/api/core` — a real ESM module that only exists
 * inside a Tauri build. We stub it with `jest.unstable_mockModule` at
 * the top of the file, then dynamic-import the module under test.
 * Same trick works for the i18n singleton (we stub its default export
 * so `i18n.t` returns a predictable "key(params)" string).
 */

import { jest } from '@jest/globals';

jest.unstable_mockModule('@tauri-apps/api/core', () => ({
    invoke: jest.fn(),
}));

// i18n stub — echoes the key with any params serialised, so the tests
// can assert on the exact key that would be looked up. This is much
// easier to read than mocking the full translation table.
jest.unstable_mockModule('../js/i18n.js', () => {
    const t = (key, params) => {
        if (!params) return key;
        const parts = Object.entries(params).map(([k, v]) => `${k}=${v}`).join(',');
        return `${key}(${parts})`;
    };
    return {
        default: {
            t,
            initialized: true,
            getCurrentLanguage: () => 'ja',
        },
    };
});

// Record toast invocations so we can assert on them.
const toastSpy = jest.fn();
jest.unstable_mockModule('../js/toast.js', () => ({
    showToast: toastSpy,
    clearAllToasts: jest.fn(),
}));

// Record validation-display invocations so we can assert on them.
const showValidationErrorSpy = jest.fn();
const showMaxLengthErrorSpy = jest.fn();
const clearValidationErrorSpy = jest.fn();
jest.unstable_mockModule('../js/validation-display.js', () => ({
    showValidationError: showValidationErrorSpy,
    showMaxLengthError: showMaxLengthErrorSpy,
    clearValidationError: clearValidationErrorSpy,
}));

// Import under test AFTER the mocks are set up.
const { mapMasterErrorCode, saveMasterEntry, API_ERROR_CODES } =
    await import('../js/master-crud.js');

beforeEach(() => {
    toastSpy.mockClear();
    showValidationErrorSpy.mockClear();
    showMaxLengthErrorSpy.mockClear();
    clearValidationErrorSpy.mockClear();
});

// ---- mapMasterErrorCode ------------------------------------------------

describe('mapMasterErrorCode — ApiError code → i18n key', () => {
    const shopCtx = {
        i18nPrefix: 'shop_mgmt',
        nameFieldI18nKey: 'shop_mgmt.shop_name',
        memoFieldI18nKey: 'shop_mgmt.memo',
        nameMaxLen: 128,
        memoMaxLen: 500,
        actualNameLen: 130,
        actualMemoLen: 10,
    };

    test('duplicate_name → inline name error, no toast', () => {
        const err = { code: 'duplicate_name', message: 'Shop name already exists', entity: 'shop' };
        const out = mapMasterErrorCode(err, shopCtx);
        expect(out.nameMessage).toBe('shop_mgmt.duplicate_error');
        expect(out.memoMessage).toBeNull();
        expect(out.toastMessage).toBeNull();
    });

    test('not_found → toast, no inline messages', () => {
        const err = { code: 'not_found', message: 'Shop not found', entity: 'shop' };
        const out = mapMasterErrorCode(err, shopCtx);
        expect(out.nameMessage).toBeNull();
        expect(out.memoMessage).toBeNull();
        expect(out.toastMessage).toBe('shop_mgmt.not_found');
    });

    test('manufacturer_not_found → product-scoped toast regardless of the caller prefix', () => {
        const err = { code: 'manufacturer_not_found', message: 'Manufacturer not found', entity: 'manufacturer' };
        // Even if invoked from a shop context, this code is product-scoped.
        const out = mapMasterErrorCode(err, shopCtx);
        expect(out.toastMessage).toBe('product_mgmt.manufacturer_not_found');
    });

    test('validation with "Memo …" message routes to memo inline', () => {
        const err = { code: 'validation', message: 'Memo must be 500 characters or less' };
        const out = mapMasterErrorCode(err, shopCtx);
        expect(out.nameMessage).toBeNull();
        expect(out.memoMessage).toContain('validation.max_length');
        expect(out.memoMessage).toContain('field=shop_mgmt.memo');
        expect(out.memoMessage).toContain('max=500');
        expect(out.memoMessage).toContain('actual=10');
    });

    test('validation with "cannot be empty" routes to name inline via i18n prefix', () => {
        const err = { code: 'validation', message: 'Shop name cannot be empty' };
        const out = mapMasterErrorCode(err, shopCtx);
        expect(out.nameMessage).toBe('shop_mgmt.empty_name');
    });

    test('validation with "characters or less" routes to name-length inline', () => {
        const err = { code: 'validation', message: 'Shop name must be 128 characters or less' };
        const out = mapMasterErrorCode(err, shopCtx);
        expect(out.nameMessage).toContain('validation.max_length');
        expect(out.nameMessage).toContain('field=shop_mgmt.shop_name');
        expect(out.nameMessage).toContain('max=128');
        expect(out.nameMessage).toContain('actual=130');
    });

    test('validation with unknown subtype falls back to generic failure inline', () => {
        const err = { code: 'validation', message: 'Something exotic happened' };
        const out = mapMasterErrorCode(err, shopCtx);
        expect(out.nameMessage).toBe('shop_mgmt.failed_to_save');
    });

    test('database code falls back to generic failure inline', () => {
        const err = { code: 'database', message: 'DB borked' };
        const out = mapMasterErrorCode(err, shopCtx);
        expect(out.nameMessage).toBe('shop_mgmt.failed_to_save');
    });

    test('legacy string error still classified for backward compatibility', () => {
        // Simulates an as-yet-unmigrated command still returning Err(String).
        const out = mapMasterErrorCode('Shop name already exists', shopCtx);
        expect(out.nameMessage).toBe('shop_mgmt.duplicate_error');
    });

    test('legacy string with "cannot be empty" hits the empty-name path', () => {
        const out = mapMasterErrorCode('Shop name cannot be empty', shopCtx);
        expect(out.nameMessage).toBe('shop_mgmt.empty_name');
    });

    test('legacy string that does not match falls back to generic failure', () => {
        const out = mapMasterErrorCode('Something else', shopCtx);
        expect(out.nameMessage).toBe('shop_mgmt.failed_to_save');
    });

    test('API_ERROR_CODES exports the codes the Rust side documents', () => {
        expect(API_ERROR_CODES.DUPLICATE_NAME).toBe('duplicate_name');
        expect(API_ERROR_CODES.NOT_FOUND).toBe('not_found');
        expect(API_ERROR_CODES.MANUFACTURER_NOT_FOUND).toBe('manufacturer_not_found');
        expect(API_ERROR_CODES.VALIDATION).toBe('validation');
        expect(API_ERROR_CODES.DATABASE).toBe('database');
    });
});

// ---- saveMasterEntry ---------------------------------------------------

function makeInput(value = '') {
    // Minimal shape saveMasterEntry expects from an <input>.
    return { value };
}

const commonCtx = {
    i18nPrefix: 'shop_mgmt',
    nameFieldI18nKey: 'shop_mgmt.shop_name',
    memoFieldI18nKey: 'shop_mgmt.memo',
    nameMaxLen: 128,
    memoMaxLen: 500,
};

describe('saveMasterEntry — validation before invoke', () => {
    test('empty name shows inline error, does NOT call invokeAdd/invokeUpdate', async () => {
        const invokeAdd = jest.fn();
        const invokeUpdate = jest.fn();

        await expect(saveMasterEntry({
            nameInput: makeInput('   '),
            memoInput: makeInput(''),
            editingId: null,
            findInCacheById: () => null,
            invokeAdd,
            invokeUpdate,
            ...commonCtx,
        })).rejects.toThrow(/empty/);

        expect(invokeAdd).not.toHaveBeenCalled();
        expect(invokeUpdate).not.toHaveBeenCalled();
        expect(showValidationErrorSpy).toHaveBeenCalledWith(
            expect.anything(),
            'shop_mgmt.empty_name'
        );
    });

    test('name over max-len shows inline error, does NOT invoke', async () => {
        const invokeAdd = jest.fn();
        await expect(saveMasterEntry({
            nameInput: makeInput('a'.repeat(129)),
            memoInput: makeInput(''),
            editingId: null,
            findInCacheById: () => null,
            invokeAdd,
            invokeUpdate: jest.fn(),
            ...commonCtx,
        })).rejects.toThrow(/too long/);
        expect(invokeAdd).not.toHaveBeenCalled();
        expect(showMaxLengthErrorSpy).toHaveBeenCalled();
    });

    test('memo over max-len shows inline error, does NOT invoke', async () => {
        const invokeAdd = jest.fn();
        await expect(saveMasterEntry({
            nameInput: makeInput('ok'),
            memoInput: makeInput('m'.repeat(501)),
            editingId: null,
            findInCacheById: () => null,
            invokeAdd,
            invokeUpdate: jest.fn(),
            ...commonCtx,
        })).rejects.toThrow(/memo too long/);
        expect(invokeAdd).not.toHaveBeenCalled();
    });
});

describe('saveMasterEntry — edit target vanished', () => {
    test('editingId with empty cache short-circuits to onNotFoundBeforeInvoke and returns { mode: "skip" }', async () => {
        const invokeUpdate = jest.fn();
        const onNotFoundBeforeInvoke = jest.fn().mockResolvedValue(undefined);

        const result = await saveMasterEntry({
            nameInput: makeInput('name'),
            memoInput: makeInput(''),
            editingId: 42,
            findInCacheById: () => null,
            invokeAdd: jest.fn(),
            invokeUpdate,
            onNotFoundBeforeInvoke,
            ...commonCtx,
        });

        expect(result).toEqual({ mode: 'skip' });
        expect(invokeUpdate).not.toHaveBeenCalled();
        expect(onNotFoundBeforeInvoke).toHaveBeenCalledTimes(1);
    });

    test('editingId with empty cache and NO onNotFoundBeforeInvoke shows the default not_found toast', async () => {
        const result = await saveMasterEntry({
            nameInput: makeInput('name'),
            memoInput: makeInput(''),
            editingId: 42,
            findInCacheById: () => null,
            invokeAdd: jest.fn(),
            invokeUpdate: jest.fn(),
            ...commonCtx,
        });

        expect(result).toEqual({ mode: 'skip' });
        expect(toastSpy).toHaveBeenCalledWith('shop_mgmt.not_found', { variant: 'error' });
    });
});

describe('saveMasterEntry — happy path', () => {
    test('editingId null → invokeAdd called, onSuccess("add") fires, returns { mode: "add" }', async () => {
        const invokeAdd = jest.fn().mockResolvedValue(undefined);
        const invokeUpdate = jest.fn();
        const onSuccess = jest.fn().mockResolvedValue(undefined);

        const result = await saveMasterEntry({
            nameInput: makeInput('  new shop  '),
            memoInput: makeInput(''),
            editingId: null,
            findInCacheById: () => null,
            invokeAdd,
            invokeUpdate,
            onSuccess,
            ...commonCtx,
        });

        expect(result).toEqual({ mode: 'add' });
        expect(invokeAdd).toHaveBeenCalledWith('new shop', null);
        expect(invokeUpdate).not.toHaveBeenCalled();
        expect(onSuccess).toHaveBeenCalledWith('add', 'new shop');
    });

    test('editingId with cached target → invokeUpdate called, onSuccess("update") fires', async () => {
        const cached = { shop_id: 7, display_order: 3 };
        const invokeAdd = jest.fn();
        const invokeUpdate = jest.fn().mockResolvedValue(undefined);
        const onSuccess = jest.fn().mockResolvedValue(undefined);

        const result = await saveMasterEntry({
            nameInput: makeInput('renamed'),
            memoInput: makeInput('m'),
            editingId: 7,
            findInCacheById: (id) => (id === 7 ? cached : null),
            invokeAdd,
            invokeUpdate,
            onSuccess,
            ...commonCtx,
        });

        expect(result).toEqual({ mode: 'update' });
        expect(invokeUpdate).toHaveBeenCalledWith(cached, 'renamed', 'm');
        expect(onSuccess).toHaveBeenCalledWith('update', 'renamed');
    });
});

describe('saveMasterEntry — backend error re-throws and classifies', () => {
    test('duplicate_name → inline name error, modal-open path (throw)', async () => {
        const err = { code: 'duplicate_name', message: 'Shop name already exists', entity: 'shop' };
        const invokeAdd = jest.fn().mockRejectedValue(err);

        await expect(saveMasterEntry({
            nameInput: makeInput('dupname'),
            memoInput: makeInput(''),
            editingId: null,
            findInCacheById: () => null,
            invokeAdd,
            invokeUpdate: jest.fn(),
            ...commonCtx,
        })).rejects.toBe(err);

        expect(showValidationErrorSpy).toHaveBeenCalledWith(
            expect.anything(),
            'shop_mgmt.duplicate_error'
        );
    });

    test('manufacturer_not_found → product-scoped toast', async () => {
        const err = { code: 'manufacturer_not_found', message: 'Manufacturer not found', entity: 'manufacturer' };
        const invokeAdd = jest.fn().mockRejectedValue(err);

        await expect(saveMasterEntry({
            nameInput: makeInput('newprod'),
            memoInput: makeInput(''),
            editingId: null,
            findInCacheById: () => null,
            invokeAdd,
            invokeUpdate: jest.fn(),
            ...commonCtx,
            i18nPrefix: 'product_mgmt',
            nameFieldI18nKey: 'product_mgmt.name',
            memoFieldI18nKey: 'product_mgmt.memo',
        })).rejects.toBe(err);

        expect(toastSpy).toHaveBeenCalledWith('product_mgmt.manufacturer_not_found', { variant: 'error' });
    });

    // Devin review on #97 flagged that the toast wording says "the list has
    // been reloaded" but the invoke-time not_found path did not actually
    // reload. saveMasterEntry now routes backend-not_found through the
    // same onNotFoundBeforeInvoke hook the cache-miss path uses, so the
    // list is refreshed AND the modal closes (mode: skip).
    test('backend not_found routes through onNotFoundBeforeInvoke and returns { mode: "skip" }', async () => {
        const err = { code: 'not_found', message: 'Shop not found', entity: 'shop' };
        const cached = { shop_id: 7, display_order: 3 };
        const invokeUpdate = jest.fn().mockRejectedValue(err);
        const onNotFoundBeforeInvoke = jest.fn().mockResolvedValue(undefined);

        const result = await saveMasterEntry({
            nameInput: makeInput('renamed'),
            memoInput: makeInput(''),
            editingId: 7,
            findInCacheById: (id) => (id === 7 ? cached : null),
            invokeAdd: jest.fn(),
            invokeUpdate,
            onNotFoundBeforeInvoke,
            ...commonCtx,
        });

        expect(result).toEqual({ mode: 'skip' });
        expect(invokeUpdate).toHaveBeenCalledTimes(1);
        expect(onNotFoundBeforeInvoke).toHaveBeenCalledTimes(1);
        // No inline error fires — the classifier's toast path is bypassed
        // in favour of the caller's reload hook.
        expect(showValidationErrorSpy).not.toHaveBeenCalled();
    });

    test('backend not_found with NO onNotFoundBeforeInvoke shows the default not_found toast', async () => {
        const err = { code: 'not_found', message: 'Shop not found', entity: 'shop' };
        const cached = { shop_id: 7 };
        const invokeUpdate = jest.fn().mockRejectedValue(err);

        const result = await saveMasterEntry({
            nameInput: makeInput('renamed'),
            memoInput: makeInput(''),
            editingId: 7,
            findInCacheById: (id) => (id === 7 ? cached : null),
            invokeAdd: jest.fn(),
            invokeUpdate,
            ...commonCtx,
        });

        expect(result).toEqual({ mode: 'skip' });
        expect(toastSpy).toHaveBeenCalledWith('shop_mgmt.not_found', { variant: 'error' });
    });
});
