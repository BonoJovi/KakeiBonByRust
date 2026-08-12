/**
 * Escape a value for safe interpolation into HTML text or attribute values.
 *
 * Quotes are escaped as well so the result is safe inside quoted attributes.
 */
export function escapeHtml(value) {
    return String(value == null ? '' : value)
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#39;');
}
