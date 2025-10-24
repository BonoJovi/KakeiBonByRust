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
        try {
            const translations = await invoke('get_translations', { 
                language: this.currentLanguage 
            });
            this.translations = translations;
            console.log('Loaded translations for', this.currentLanguage, ':', Object.keys(this.translations).length, 'keys');
        } catch (error) {
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
