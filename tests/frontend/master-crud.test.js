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
jest.unstable_mockModule('../../res/js/i18n.js', () => {
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
jest.unstable_mockModule('../../res/js/toast.js', () => ({
    showToast: toastSpy,
    clearAllToasts: jest.fn(),
}));

// Record validation-display invocations so we can assert on them.
const showValidationErrorSpy = jest.fn();
const showMaxLengthErrorSpy = jest.fn();
const clearValidationErrorSpy = jest.fn();
jest.unstable_mockModule('../../res/js/validation-display.js', () => ({
    showValidationError: showValidationErrorSpy,
    showMaxLengthError: showMaxLengthErrorSpy,
    clearValidationError: clearValidationErrorSpy,
}));

// Import under test AFTER the mocks are set up.
const { mapMasterErrorCode, saveMasterEntry, formatApiError, API_ERROR_CODES } =
    await import('../../res/js/master-crud.js');

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

    test('should show an inline name error and no toast when the code is duplicate_name', () => {
        const err = { code: 'duplicate_name', message: 'Shop name already exists', entity: 'shop' };
        const out = mapMasterErrorCode(err, shopCtx);
        expect(out.nameMessage).toBe('shop_mgmt.duplicate_error');
        expect(out.memoMessage).toBeNull();
        expect(out.toastMessage).toBeNull();
    });

    test('should show a toast with the ${prefix}.duplicate_error key, not a name error, when the code is duplicate_code', () => {
        // Account is the current caller: the duplicate check is on the
        // account CODE column, so the inline error must NOT land on the
        // name input the saveMasterEntry helper tracks. Routed as a
        // toast so the calling screen can decide where to display it.
        const err = { code: 'duplicate_code', message: 'Account code already exists', entity: 'account' };
        const accountCtx = { ...shopCtx, i18nPrefix: 'account_mgmt', nameFieldI18nKey: 'account_mgmt.account_name' };
        const out = mapMasterErrorCode(err, accountCtx);
        expect(out.nameMessage).toBeNull();
        expect(out.memoMessage).toBeNull();
        expect(out.toastMessage).toBe('account_mgmt.duplicate_error');
    });

    test('should show a toast and no inline messages when the code is not_found', () => {
        const err = { code: 'not_found', message: 'Shop not found', entity: 'shop' };
        const out = mapMasterErrorCode(err, shopCtx);
        expect(out.nameMessage).toBeNull();
        expect(out.memoMessage).toBeNull();
        expect(out.toastMessage).toBe('shop_mgmt.not_found');
    });

    test('should show the per-screen ${prefix}.admin_protected toast when the code is admin_protected', () => {
        // Introduced by the user_management migration: UserManagementError
        // ::AdminUserCannotBeDeleted maps to ApiError::admin_protected("User"),
        // which the classifier routes to `user_mgmt.admin_protected`.
        const err = { code: 'admin_protected', message: 'Admin user cannot be modified this way', entity: 'user' };
        const userCtx = { ...shopCtx, i18nPrefix: 'user_mgmt', nameFieldI18nKey: 'user_mgmt.username' };
        const out = mapMasterErrorCode(err, userCtx);
        expect(out.nameMessage).toBeNull();
        expect(out.memoMessage).toBeNull();
        expect(out.toastMessage).toBe('user_mgmt.admin_protected');
    });

    test('should show the product-scoped toast for any caller prefix when the code is manufacturer_not_found', () => {
        const err = { code: 'manufacturer_not_found', message: 'Manufacturer not found', entity: 'manufacturer' };
        // Even if invoked from a shop context, this code is product-scoped.
        const out = mapMasterErrorCode(err, shopCtx);
        expect(out.toastMessage).toBe('product_mgmt.manufacturer_not_found');
    });

    test('should show a memo inline error when a validation message starts with "Memo …"', () => {
        const err = { code: 'validation', message: 'Memo must be 500 characters or less' };
        const out = mapMasterErrorCode(err, shopCtx);
        expect(out.nameMessage).toBeNull();
        expect(out.memoMessage).toContain('validation.max_length');
        expect(out.memoMessage).toContain('field=shop_mgmt.memo');
        expect(out.memoMessage).toContain('max=500');
        expect(out.memoMessage).toContain('actual=10');
    });

    test('should show the prefixed name inline error when a validation message says "cannot be empty"', () => {
        const err = { code: 'validation', message: 'Shop name cannot be empty' };
        const out = mapMasterErrorCode(err, shopCtx);
        expect(out.nameMessage).toBe('shop_mgmt.empty_name');
    });

    test('should show the name-length inline error when a validation message says "characters or less"', () => {
        const err = { code: 'validation', message: 'Shop name must be 128 characters or less' };
        const out = mapMasterErrorCode(err, shopCtx);
        expect(out.nameMessage).toContain('validation.max_length');
        expect(out.nameMessage).toContain('field=shop_mgmt.shop_name');
        expect(out.nameMessage).toContain('max=128');
        expect(out.nameMessage).toContain('actual=130');
    });

    test('should fall back to the generic failure inline error when the validation subtype is unknown', () => {
        const err = { code: 'validation', message: 'Something exotic happened' };
        const out = mapMasterErrorCode(err, shopCtx);
        expect(out.nameMessage).toBe('shop_mgmt.failed_to_save');
    });

    test('should fall back to the generic failure inline error when the code is database', () => {
        const err = { code: 'database', message: 'DB borked' };
        const out = mapMasterErrorCode(err, shopCtx);
        expect(out.nameMessage).toBe('shop_mgmt.failed_to_save');
    });

    test('should still classify the error when it is a legacy string', () => {
        // Simulates an as-yet-unmigrated command still returning Err(String).
        const out = mapMasterErrorCode('Shop name already exists', shopCtx);
        expect(out.nameMessage).toBe('shop_mgmt.duplicate_error');
    });

    test('should take the empty-name path when a legacy string says "cannot be empty"', () => {
        const out = mapMasterErrorCode('Shop name cannot be empty', shopCtx);
        expect(out.nameMessage).toBe('shop_mgmt.empty_name');
    });

    test('should fall back to the generic failure when a legacy string matches no pattern', () => {
        const out = mapMasterErrorCode('Something else', shopCtx);
        expect(out.nameMessage).toBe('shop_mgmt.failed_to_save');
    });

    // Devin review on #99: string concatenation with an ApiError object
    // renders as "[object Object]" — every unmigrated error-surface site
    // now goes through formatApiError.
    describe('formatApiError', () => {
        test('should return the message string when the error is an ApiError-shaped object', () => {
            expect(formatApiError({ code: 'not_found', message: 'Account not found', entity: 'account' }))
                .toBe('Account not found');
        });

        test('should fall back to String(err) when the error is a plain string (unmigrated command)', () => {
            expect(formatApiError('Legacy error text')).toBe('Legacy error text');
        });

        test('should return .message when the error is an Error instance (same shape as ApiError)', () => {
            // Error instances have a string .message, so the helper
            // returns it directly — a nicer default than String(err)
            // which would prepend "Error:".
            const s = formatApiError(new Error('boom'));
            expect(s).toBe('boom');
        });

        test('should fall back to String(err) when the object has no message field', () => {
            expect(formatApiError({ code: 'weird' })).toBe('[object Object]');
        });

        test('should handle the value when it is null or undefined', () => {
            expect(formatApiError(null)).toBe('null');
            expect(formatApiError(undefined)).toBe('undefined');
        });
    });

    test('should export the codes the Rust side documents when API_ERROR_CODES is read', () => {
        expect(API_ERROR_CODES.DUPLICATE_NAME).toBe('duplicate_name');
        expect(API_ERROR_CODES.DUPLICATE_CODE).toBe('duplicate_code');
        expect(API_ERROR_CODES.NOT_FOUND).toBe('not_found');
        expect(API_ERROR_CODES.MANUFACTURER_NOT_FOUND).toBe('manufacturer_not_found');
        expect(API_ERROR_CODES.ADMIN_PROTECTED).toBe('admin_protected');
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
    test('should show an inline error and not call invokeAdd/invokeUpdate when the name is empty', async () => {
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

    test('should show an inline error and not invoke when the name is over the maximum length', async () => {
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

    test('should show an inline error and not invoke when the memo is over the maximum length', async () => {
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
    test('should call onNotFoundBeforeInvoke and return { mode: "skip" } when editingId is set and the cache is empty', async () => {
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

    test('should show the default not_found toast when editingId is set, the cache is empty and there is no onNotFoundBeforeInvoke', async () => {
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
    test('should call invokeAdd and onSuccess("add") and return { mode: "add" } when editingId is null', async () => {
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

    test('should call invokeUpdate and onSuccess("update") when editingId has a cached target', async () => {
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
    test('should show an inline name error and throw to keep the window open when the backend returns duplicate_name', async () => {
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

    test('should show the product-scoped toast when the backend returns manufacturer_not_found', async () => {
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
    test('should call onNotFoundBeforeInvoke and return { mode: "skip" } when the backend returns not_found', async () => {
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

    test('should show the default not_found toast when the backend returns not_found and there is no onNotFoundBeforeInvoke', async () => {
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
