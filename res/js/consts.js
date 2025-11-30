// User role constants
export const ROLE_ADMIN = 0;
export const ROLE_USER = 1;
export const ROLE_VISIT = 999;

// Language constants
export const LANG_ENGLISH = 'en';
export const LANG_JAPANESE = 'ja';
export const LANG_DEFAULT = LANG_JAPANESE;

// Font size constants
export const FONT_SIZE_SMALL = 'small';
export const FONT_SIZE_MEDIUM = 'medium';
export const FONT_SIZE_LARGE = 'large';
export const FONT_SIZE_CUSTOM = 'custom';
export const FONT_SIZE_DEFAULT = FONT_SIZE_MEDIUM;

// Font size i18n keys
export const I18N_FONT_SIZE_SMALL = 'font_size.small';
export const I18N_FONT_SIZE_MEDIUM = 'font_size.medium';
export const I18N_FONT_SIZE_LARGE = 'font_size.large';
export const I18N_FONT_SIZE_CUSTOM = 'font_size.custom';

// Font size menu configuration
export const FONT_SIZE_OPTIONS = [
    { code: FONT_SIZE_SMALL, key: I18N_FONT_SIZE_SMALL },
    { code: FONT_SIZE_MEDIUM, key: I18N_FONT_SIZE_MEDIUM },
    { code: FONT_SIZE_LARGE, key: I18N_FONT_SIZE_LARGE },
    { code: FONT_SIZE_CUSTOM, key: I18N_FONT_SIZE_CUSTOM, action: 'modal' }
];

// Tax rounding mode constants (must match src/consts.rs)
export const TAX_ROUND_DOWN = 0;
export const TAX_ROUND_HALF_UP = 1;
export const TAX_ROUND_UP = 2;

// Tax inclusion type constants (must match src/consts.rs)
export const TAX_INCLUDED = 0;  // 内税 - tax is included in prices
export const TAX_EXCLUDED = 1;  // 外税 - tax is calculated separately
