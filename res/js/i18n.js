import { invoke } from '@tauri-apps/api/core';

class I18n {
    constructor() {
        this.currentLanguage = 'ja'; // Default to Japanese
        this.translations = {};
        this.initialized = false;
    }

    async init() {
        try {
            // Get current language from backend
            const settings = await invoke('get_user_settings');
            console.log('Settings from backend:', settings);
            this.currentLanguage = settings.language || 'ja';
            console.log('Current language set to:', this.currentLanguage);
            
            // Load translations
            await this.loadTranslations();
            
            this.initialized = true;
            console.log('i18n initialized with language:', this.currentLanguage);
        } catch (error) {
            console.error('Failed to initialize i18n:', error);
            // Fallback to Japanese
            this.currentLanguage = 'ja';
            await this.loadTranslations();
            this.initialized = true;
        }
    }

    async loadTranslations() {
        console.log('[DEBUG] loadTranslations() called');
        try {
            console.log('[DEBUG] Calling invoke get_translations with language:', this.currentLanguage);
            const translations = await invoke('get_translations', { 
                language: this.currentLanguage 
            });
            console.log('[DEBUG] invoke returned:', typeof translations, 'with', Object.keys(translations).length, 'keys');
            
            // Log the entire translations object structure
            console.log('[DEBUG] Full translations object:', translations);
            console.log('[DEBUG] Object.keys sample (first 10):', Object.keys(translations).slice(0, 10));
            
            // Log all keys that start with __DEBUG
            const debugKeys = Object.keys(translations).filter(k => k.startsWith('__DEBUG'));
            console.log('[DEBUG] Found debug keys:', debugKeys);
            
            // Try different ways to access the properties
            console.log('[DEBUG] Direct access test:');
            console.log('  translations["__DEBUG_ORIGINAL_COUNT__"]:', translations["__DEBUG_ORIGINAL_COUNT__"]);
            console.log('  translations.__DEBUG_ORIGINAL_COUNT__:', translations.__DEBUG_ORIGINAL_COUNT__);
            console.log('  Has property?', Object.prototype.hasOwnProperty.call(translations, '__DEBUG_ORIGINAL_COUNT__'));
            
            // Extract and log debug info
            console.log('=== BACKEND DEBUG INFO (from special keys) ===');
            console.log('Original count (from DB):', translations['__DEBUG_ORIGINAL_COUNT__']);
            console.log('Has menu.admin BEFORE?', translations['__DEBUG_HAS_MENU_ADMIN_BEFORE__']);
            console.log('menu.admin value:', translations['__DEBUG_MENU_ADMIN_VALUE__']);
            console.log('Menu keys count:', translations['__DEBUG_MENU_KEYS_COUNT__']);
            console.log('Menu keys:', translations['__DEBUG_MENU_KEYS__']);
            console.log('Final count (with debug keys):', translations['__DEBUG_FINAL_COUNT__']);
            console.log('Has menu.admin AFTER?', translations['__DEBUG_HAS_MENU_ADMIN_AFTER__']);
            
            // Remove debug keys before using translations
            delete translations['__DEBUG_ORIGINAL_COUNT__'];
            delete translations['__DEBUG_HAS_MENU_ADMIN_BEFORE__'];
            delete translations['__DEBUG_MENU_ADMIN_VALUE__'];
            delete translations['__DEBUG_MENU_KEYS_COUNT__'];
            delete translations['__DEBUG_MENU_KEYS__'];
            delete translations['__DEBUG_FINAL_COUNT__'];
            delete translations['__DEBUG_HAS_MENU_ADMIN_AFTER__'];
            
            this.translations = translations;
            console.log('Loaded translations for', this.currentLanguage, ':', Object.keys(this.translations).length, 'keys');
            console.log('menu.admin =', this.translations['menu.admin']);
            console.log('menu.font_size =', this.translations['menu.font_size']);
        } catch (error) {
            console.error('[DEBUG] Error in loadTranslations:', error);
            console.error('Failed to load translations:', error);
            this.translations = {};
        }
    }

    t(key, params = {}) {
        let text = this.translations[key] || key;
        
        // Replace parameters
        Object.keys(params).forEach(paramKey => {
            text = text.replace(new RegExp(`{${paramKey}}`, 'g'), params[paramKey]);
        });
        
        return text;
    }

    async setLanguage(language) {
        if (this.currentLanguage === language) {
            return;
        }

        try {
            // Update backend settings
            await invoke('update_user_settings', {
                settings: { language: language }
            });
            
            this.currentLanguage = language;
            await this.loadTranslations();
            
            // Update UI
            this.updateUI();
            
            console.log('Language changed to:', language);
        } catch (error) {
            console.error('Failed to change language:', error);
            throw error;
        }
    }

    getCurrentLanguage() {
        return this.currentLanguage;
    }

    updateUI() {
        // Update all elements with data-i18n attribute
        document.querySelectorAll('[data-i18n]').forEach(element => {
            const key = element.getAttribute('data-i18n');
            element.textContent = this.t(key);
        });

        // Update all elements with data-i18n-placeholder attribute
        document.querySelectorAll('[data-i18n-placeholder]').forEach(element => {
            const key = element.getAttribute('data-i18n-placeholder');
            element.placeholder = this.t(key);
        });

        // Update all elements with data-i18n-title attribute
        document.querySelectorAll('[data-i18n-title]').forEach(element => {
            const key = element.getAttribute('data-i18n-title');
            element.title = this.t(key);
        });
    }
}

// Create global i18n instance
const i18n = new I18n();

export default i18n;
