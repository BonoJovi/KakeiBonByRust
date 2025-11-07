import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { FONT_SIZE_OPTIONS, FONT_SIZE_SMALL, FONT_SIZE_MEDIUM, FONT_SIZE_LARGE, FONT_SIZE_CUSTOM } from './consts.js';
import { Modal } from './modal.js';

let resizeInProgress = false;
let fontSizeModal;

// Setup font size menu handlers
export function setupFontSizeMenuHandlers() {
    console.log('[setupFontSizeMenuHandlers] Starting setup');
    const fontSizeMenu = document.getElementById('font-size-menu');
    const fontSizeDropdown = document.getElementById('font-size-dropdown');
    
    if (!fontSizeMenu || !fontSizeDropdown) {
        console.warn('Font size menu elements not found');
        return;
    }
    
    // Check if already initialized
    if (fontSizeMenu.dataset.initialized === 'true') {
        console.log('[setupFontSizeMenuHandlers] Already initialized');
        return;
    }
    
    fontSizeMenu.addEventListener('click', function(e) {
        e.stopPropagation();
        
        const isShown = fontSizeDropdown.classList.contains('show');
        
        // Close all other dropdowns
        document.querySelectorAll('.dropdown').forEach(d => {
            if (d !== fontSizeDropdown) {
                d.classList.remove('show');
            }
        });
        
        // Toggle this dropdown
        if (!isShown) {
            fontSizeDropdown.classList.add('show');
        }
    });
    
    // Prevent dropdown from closing when clicking inside it
    fontSizeDropdown.addEventListener('click', function(e) {
        e.stopPropagation();
    });
    
    // Mark as initialized
    fontSizeMenu.dataset.initialized = 'true';
    console.log('[setupFontSizeMenuHandlers] Marked as initialized');
}

// Setup font size menu items
export async function setupFontSizeMenu() {
    try {
        // Get current font size
        const currentSize = await invoke('get_font_size');
        
        // Get dropdown container
        const fontSizeDropdown = document.getElementById('font-size-dropdown');
        
        if (!fontSizeDropdown) {
            console.error('Font size dropdown not found');
            return;
        }
        
        // Clear existing items
        fontSizeDropdown.innerHTML = '';
        
        // Add font size items from constants
        for (const size of FONT_SIZE_OPTIONS) {
            const item = document.createElement('div');
            item.className = 'dropdown-item';
            item.textContent = i18n.t(size.key);
            item.dataset.sizeCode = size.code;
            
            // Mark current size with active class
            if (size.code === currentSize) {
                item.classList.add('active');
            }
            
            item.addEventListener('click', async function(e) {
                e.stopPropagation();
                if (size.action === 'modal') {
                    // Open modal for custom settings
                    openFontSizeModal();
                } else {
                    await handleFontSizeChange(size.code);
                }
                fontSizeDropdown.classList.remove('show');
            });
            
            fontSizeDropdown.appendChild(item);
        }
        
    } catch (error) {
        console.error('Failed to setup font size menu:', error);
    }
}

// Handle font size change
async function handleFontSizeChange(sizeCode) {
    try {
        console.log('Changing font size to:', sizeCode);
        await invoke('set_font_size', { fontSize: sizeCode });
        
        // Apply the new font size
        await applyFontSize();
        
        // Reload font size menu to update selection
        await setupFontSizeMenu();
        
        console.log('Font size changed successfully');
    } catch (error) {
        console.error('Failed to change font size:', error);
        alert(i18n.t('error.font_size_change_failed') + ': ' + error);
    }
}

// Apply saved font size
export async function applyFontSize() {
    try {
        const fontSize = await invoke('get_font_size');
        
        // Map size code to CSS variable
        const sizeMap = {
            [FONT_SIZE_SMALL]: 'var(--font-size-small)',
            [FONT_SIZE_MEDIUM]: 'var(--font-size-medium)',
            [FONT_SIZE_LARGE]: 'var(--font-size-large)'
        };
        
        let cssValue;
        
        if (sizeMap[fontSize]) {
            // It's a preset size
            cssValue = sizeMap[fontSize];
        } else {
            // It's a custom percentage value
            const percent = parseInt(fontSize);
            if (!isNaN(percent)) {
                cssValue = percent + '%';
            } else {
                cssValue = sizeMap[FONT_SIZE_MEDIUM]; // fallback
            }
        }
        
        document.documentElement.style.setProperty('--base-font-size', cssValue);
        
        console.log('Applied font size:', fontSize, '→', cssValue);
        
        // Prevent multiple simultaneous resize attempts
        if (resizeInProgress) {
            return;
        }
        
        resizeInProgress = true;
        
        try {
            // Wait for layout to update using requestAnimationFrame
            await new Promise(resolve => {
                requestAnimationFrame(() => {
                    requestAnimationFrame(() => {
                        requestAnimationFrame(resolve);
                    });
                });
            });
            
            // Adjust window size based on content (only once)
            await adjustWindowSize();
        } finally {
            resizeInProgress = false;
        }
    } catch (error) {
        console.error('Failed to apply font size:', error);
        resizeInProgress = false;
    }
}

// Adjust window size to fit content
export async function adjustWindowSize() {
    try {
        console.log('[adjustWindowSize] Starting window size adjustment');
        
        // Calculate total content height by getting bounding rectangles
        let maxWidth = 0;
        let maxHeight = 0;
        
        // Check if we are in the main app (not login/setup screen)
        const appContent = document.getElementById('app-content');
        const isMainApp = appContent && !appContent.classList.contains('hidden');
        
        // Only measure modals if we're in the main app
        if (isMainApp) {
            const modals = document.querySelectorAll('.modal');
            console.log('[adjustWindowSize] Found', modals.length, 'modals');
            
            for (const modal of modals) {
                // Temporarily show modal to measure its size
                const wasHidden = modal.classList.contains('hidden');
                if (wasHidden) {
                    modal.classList.remove('hidden');
                    modal.style.visibility = 'hidden'; // Make invisible but still measurable
                }
                
                const modalContent = modal.querySelector('.modal-content');
                if (modalContent) {
                    const rect = modalContent.getBoundingClientRect();
                    console.log('[adjustWindowSize] Modal:', modal.id, 'Content size:', rect.width, 'x', rect.height);
                    // Modal is centered, so we need to account for centering space
                    const modalWidth = rect.width + 80; // Extra space for centering
                    const modalHeight = rect.height + 80; // Extra space for centering
                    maxWidth = Math.max(maxWidth, modalWidth);
                    maxHeight = Math.max(maxHeight, modalHeight);
                }
                
                // Restore hidden state
                if (wasHidden) {
                    modal.classList.add('hidden');
                    modal.style.visibility = '';
                }
            }
            
            console.log('[adjustWindowSize] Modal max size:', maxWidth, 'x', maxHeight);
        } else {
            console.log('[adjustWindowSize] Skipping modal measurement (not in main app)');
        }
        
        // Now shrink window to minimum size to get natural content size
        const minWidth = 400;
        const minHeight = 300;
        
        await invoke('adjust_window_size', { 
            width: minWidth, 
            height: minHeight 
        });
        
        // Wait for layout to update after resize
        await new Promise(resolve => {
            requestAnimationFrame(() => {
                requestAnimationFrame(resolve);
            });
        });
        
        // Measure the natural content size
        const mainContent = document.getElementById('main-content');
        const menuBar = document.getElementById('menu-bar');
        
        // Check all visible elements
        const elements = [menuBar, mainContent];
        for (const el of elements) {
            if (el && !el.classList.contains('hidden')) {
                const rect = el.getBoundingClientRect();
                // Use scrollHeight instead of rect.height to get full content height
                const actualHeight = Math.max(rect.height, el.scrollHeight);
                console.log('[adjustWindowSize] Element:', el.id, 'Size:', rect.width, 'x', actualHeight, 'Bottom:', rect.bottom, 'ScrollHeight:', el.scrollHeight);
                maxWidth = Math.max(maxWidth, rect.right);
                // Calculate height from top position + actual content height
                maxHeight = Math.max(maxHeight, rect.top + actualHeight);
            }
        }
        
        console.log('[adjustWindowSize] Final max size (content + modals):', maxWidth, 'x', maxHeight);
        
        // Add padding
        const padding = 40;
        const targetWidth = Math.max(minWidth, Math.ceil(maxWidth + padding));
        const targetHeight = Math.max(minHeight, Math.ceil(maxHeight + padding));
        
        console.log('Target window size:', targetWidth, 'x', targetHeight);
        
        // Resize to fit content
        await invoke('adjust_window_size', { 
            width: targetWidth, 
            height: targetHeight 
        });
        
        console.log('[adjustWindowSize] Window size adjustment complete');
        
    } catch (error) {
        console.error('Failed to adjust window size:', error);
    }
}

// Open font size modal
async function openFontSizeModal() {
    if (fontSizeModal) {
        await loadFontSizeModalSettings();
        fontSizeModal.open();
        await adjustWindowSize();
    } else {
        console.error('Font size modal not initialized');
    }
}

// Load font size modal settings
async function loadFontSizeModalSettings() {
    try {
        const fontSize = await invoke('get_font_size');
        
        const presetSelect = document.getElementById('font-size-preset');
        const percentInput = document.getElementById('font-size-percent');
        
        if (!presetSelect || !percentInput) {
            return;
        }
        
        // Check if it's a preset or custom value
        if ([FONT_SIZE_SMALL, FONT_SIZE_MEDIUM, FONT_SIZE_LARGE].includes(fontSize)) {
            presetSelect.value = fontSize;
            
            // Set corresponding percentage
            const presetMap = {
                [FONT_SIZE_SMALL]: 85,
                [FONT_SIZE_MEDIUM]: 100,
                [FONT_SIZE_LARGE]: 115
            };
            percentInput.value = presetMap[fontSize];
            percentInput.disabled = true;
        } else {
            presetSelect.value = FONT_SIZE_CUSTOM;
            const percent = parseInt(fontSize);
            percentInput.value = isNaN(percent) ? 100 : percent;
            percentInput.disabled = false;
        }
        
    } catch (error) {
        console.error('Failed to load font size modal settings:', error);
    }
}

// Setup modal event handlers
export function setupFontSizeModalHandlers() {
    const modal = document.getElementById('font-size-modal');
    if (!modal) {
        return;
    }
    
    fontSizeModal = new Modal('font-size-modal', {
        onOpen: () => {
            loadFontSizeModalSettings();
        }
    });
    
    const cancelBtn = document.getElementById('font-size-cancel');
    const applyBtn = document.getElementById('font-size-apply');
    const presetSelect = document.getElementById('font-size-preset');
    const percentInput = document.getElementById('font-size-percent');
    const spinnerUp = modal.querySelector('.spinner-up');
    const spinnerDown = modal.querySelector('.spinner-down');
    
    // Cancel button handler
    if (cancelBtn) {
        cancelBtn.addEventListener('click', () => {
            fontSizeModal.close();
        });
    }
    
    // Apply button handler
    if (applyBtn) {
        applyBtn.addEventListener('click', async () => {
            try {
                const preset = presetSelect.value;
                let sizeValue;
                
                if (preset === FONT_SIZE_CUSTOM) {
                    sizeValue = percentInput.value;
                } else {
                    sizeValue = preset;
                }
                
                await invoke('set_font_size', { fontSize: sizeValue });
                await applyFontSize();
                await setupFontSizeMenu();
                
                fontSizeModal.close();
            } catch (error) {
                console.error('Failed to apply font size:', error);
                alert(i18n.t('error.font_size_apply_failed') + ': ' + error);
            }
        });
    }
    
    // Preset select handler
    if (presetSelect) {
        presetSelect.addEventListener('change', () => {
            if (presetSelect.value === FONT_SIZE_CUSTOM) {
                percentInput.disabled = false;
            } else {
                percentInput.disabled = true;
                
                // Update percentage to match preset
                const presetMap = {
                    [FONT_SIZE_SMALL]: 85,
                    [FONT_SIZE_MEDIUM]: 100,
                    [FONT_SIZE_LARGE]: 115
                };
                percentInput.value = presetMap[presetSelect.value];
            }
        });
    }
    
    // Spinner button handlers
    if (spinnerUp && percentInput) {
        spinnerUp.addEventListener('click', () => {
            const current = parseInt(percentInput.value) || 100;
            const step = parseInt(percentInput.step) || 5;
            const max = parseInt(percentInput.max) || 200;
            percentInput.value = Math.min(current + step, max);
            presetSelect.value = FONT_SIZE_CUSTOM;
            percentInput.disabled = false;
        });
    }
    
    if (spinnerDown && percentInput) {
        spinnerDown.addEventListener('click', () => {
            const current = parseInt(percentInput.value) || 100;
            const step = parseInt(percentInput.step) || 5;
            const min = parseInt(percentInput.min) || 50;
            percentInput.value = Math.max(current - step, min);
            presetSelect.value = FONT_SIZE_CUSTOM;
            percentInput.disabled = false;
        });
    }
}
