/**
 * Language Menu
 * Shared menu-bar language dropdown used by the management screens.
 *
 * Each screen only differs in what has to be reloaded after the language
 * changes, so that part is passed in as a callback.
 */

import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { setupFontSizeMenu } from './font-size.js';

// The screen's reload callback. Persisted so a later callback-less call
// (menu.js re-renders the items on every page) cannot drop it.
let lastOnLanguageChanged = null;

/**
 * Wire the dropdown open/close behaviour. Idempotent: repeated calls on the
 * same page are ignored.
 */
export function setupLanguageMenuHandlers() {
    const languageMenu = document.getElementById('language-menu');
    const languageDropdown = document.getElementById('language-dropdown');

    if (!languageMenu || !languageDropdown) return;
    if (languageMenu.dataset.initialized === 'true') return;

    languageMenu.addEventListener('click', function(e) {
        e.stopPropagation();
        const isShown = languageDropdown.classList.contains('show');
        document.querySelectorAll('.dropdown').forEach(d => {
            if (d !== languageDropdown) d.classList.remove('show');
        });
        if (!isShown) languageDropdown.classList.add('show');
    });

    languageDropdown.addEventListener('click', function(e) {
        e.stopPropagation();
    });

    languageMenu.dataset.initialized = 'true';
}

/**
 * Render the dropdown items. Each entry is shown in its own native script
 * (English / 日本語 / ...) regardless of the current UI language, so users can
 * always recognize the language they want.
 *
 * @param {Function} [onLanguageChanged] - Called after the language changed,
 *   e.g. to reload screen data whose labels come from the backend.
 */
export async function setupLanguageMenu(onLanguageChanged) {
    if (onLanguageChanged !== undefined) {
        lastOnLanguageChanged = onLanguageChanged;
    }
    const callback = lastOnLanguageChanged;
    try {
        const languageNames = await invoke('get_language_names');
        const currentLang = i18n.getCurrentLanguage();
        const languageDropdown = document.getElementById('language-dropdown');

        if (!languageDropdown) return;

        languageDropdown.innerHTML = '';

        for (const [langCode, langName] of languageNames) {
            const item = document.createElement('div');
            item.className = 'dropdown-item';
            item.textContent = langName;
            item.dataset.langCode = langCode;

            if (langCode === currentLang) {
                item.classList.add('active');
            }

            item.addEventListener('click', async function(e) {
                e.stopPropagation();
                await handleLanguageChange(langCode, callback);
                languageDropdown.classList.remove('show');
            });

            languageDropdown.appendChild(item);
        }
    } catch (error) {
        console.error('Failed to setup language menu:', error);
    }
}

async function handleLanguageChange(langCode, onLanguageChanged) {
    try {
        await i18n.setLanguage(langCode);
        await setupLanguageMenu(onLanguageChanged);
        // Font Size submenu items are built via textContent (no data-i18n),
        // so an explicit redraw is needed after language change.
        await setupFontSizeMenu();

        if (onLanguageChanged) await onLanguageChanged();
    } catch (error) {
        console.error('Failed to change language:', error);
    }
}
