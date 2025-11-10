/**
 * Modal - Reusable modal dialog class
 * 
 * Usage:
 * const modal = new Modal('modal-id', {
 *     onOpen: (mode, data) => { ... },
 *     onSave: async (formData) => { ... },
 *     onClose: () => { ... }
 * });
 * 
 * modal.open('add', { parentId: 123 });
 */
class Modal {
    /**
     * @param {string} modalId - ID of the modal element
     * @param {Object} options - Configuration options
     * @param {Function} options.onOpen - Called when modal opens (mode, data)
     * @param {Function} options.onSave - Called when save button is clicked (formData)
     * @param {Function} options.onClose - Called when modal closes
     * @param {string} options.formId - ID of the form element (optional)
     * @param {string} options.closeButtonId - ID of the close button (optional)
     * @param {string} options.cancelButtonId - ID of the cancel button (optional)
     * @param {string} options.saveButtonId - ID of the save button (optional)
     */
    constructor(modalId, options = {}) {
        this.modal = document.getElementById(modalId);
        if (!this.modal) {
            throw new Error(`Modal element with id "${modalId}" not found`);
        }
        
        this.options = {
            onOpen: options.onOpen || (() => {}),
            onSave: options.onSave || (async () => {}),
            onClose: options.onClose || (() => {}),
            formId: options.formId,
            closeButtonId: options.closeButtonId,
            cancelButtonId: options.cancelButtonId,
            saveButtonId: options.saveButtonId,
            closeOnBackdropClick: options.closeOnBackdropClick !== false, // Default true
            closeOnEscape: options.closeOnEscape !== false, // Default true
            enableFocusTrap: options.enableFocusTrap !== false // Default true
        };
        
        this.form = this.options.formId ? document.getElementById(this.options.formId) : null;
        this.mode = null;
        this.data = null;
        this.previousActiveElement = null;
        this.focusableElements = [];
        
        this._setupEventListeners();
    }
    
    /**
     * Set up event listeners for modal controls
     * @private
     */
    _setupEventListeners() {
        // Close button
        if (this.options.closeButtonId) {
            const closeBtn = document.getElementById(this.options.closeButtonId);
            if (closeBtn) {
                closeBtn.addEventListener('click', () => this.close());
            }
        }
        
        // Cancel button
        if (this.options.cancelButtonId) {
            const cancelBtn = document.getElementById(this.options.cancelButtonId);
            if (cancelBtn) {
                cancelBtn.addEventListener('click', () => this.close());
            }
        }
        
        // Save button
        if (this.options.saveButtonId) {
            const saveBtn = document.getElementById(this.options.saveButtonId);
            if (saveBtn) {
                saveBtn.addEventListener('click', async () => {
                    await this._handleSave();
                });
            }
        }
        
        // Backdrop click
        if (this.options.closeOnBackdropClick) {
            this.modal.addEventListener('click', (e) => {
                if (e.target === this.modal) {
                    this.close();
                }
            });
        }
        
        // Form submit
        if (this.form) {
            this.form.addEventListener('submit', async (e) => {
                e.preventDefault();
                await this._handleSave();
            });
        }
        
        // ESC key handler
        this._escKeyHandler = (e) => {
            if (e.key === 'Escape' && this.options.closeOnEscape) {
                this.close();
            }
        };
        
        // Focus trap handler
        this._focusTrapHandler = (e) => {
            // Check for Tab key (handle both key and code properties)
            const isTab = e.key === 'Tab' || e.code === 'Tab' || 
                         (e.shiftKey && (e.key === undefined || e.code === 'Tab'));
            
            if (!this.options.enableFocusTrap || !isTab) {
                return;
            }
            
            if (this.focusableElements.length === 0) {
                return;
            }
            
            const firstElement = this.focusableElements[0];
            const lastElement = this.focusableElements[this.focusableElements.length - 1];
            
            if (e.shiftKey) {
                // Shift + Tab
                if (document.activeElement === firstElement) {
                    e.preventDefault();
                    lastElement.focus();
                }
            } else {
                // Tab
                if (document.activeElement === lastElement) {
                    e.preventDefault();
                    firstElement.focus();
                }
            }
        };
    }
    
    /**
     * Open the modal
     * @param {string} mode - Mode of the modal (e.g., 'add', 'edit')
     * @param {Object} data - Data to pass to the modal
     */
    open(mode = 'add', data = {}) {
        this.mode = mode;
        this.data = data || {};

        // Store mode and data in form dataset if form exists
        if (this.form) {
            this.form.dataset.mode = mode;
            Object.keys(this.data).forEach(key => {
                this.form.dataset[key] = this.data[key];
            });
        }
        
        // Save currently focused element
        this.previousActiveElement = document.activeElement;
        
        // Call onOpen callback
        this.options.onOpen(mode, data);
        
        // Show modal
        this.modal.classList.remove('hidden');
        
        // Setup focus trap
        this._setupFocusTrap();
        
        // Add keyboard event listeners
        document.addEventListener('keydown', this._escKeyHandler);
        if (this.options.enableFocusTrap) {
            document.addEventListener('keydown', this._focusTrapHandler);
        }
        
        // Focus first focusable element
        if (this.focusableElements.length > 0) {
            this.focusableElements[0].focus();
        }
    }
    
    /**
     * Close the modal
     */
    close() {
        this.modal.classList.add('hidden');
        
        // Remove keyboard event listeners
        document.removeEventListener('keydown', this._escKeyHandler);
        document.removeEventListener('keydown', this._focusTrapHandler);
        
        // Reset form if exists
        if (this.form) {
            this.form.reset();
            // Clear dataset
            delete this.form.dataset.mode;
            if (this.data) {
                Object.keys(this.data).forEach(key => {
                    delete this.form.dataset[key];
                });
            }
        }
        
        // Restore focus to previous element
        if (this.previousActiveElement && this.previousActiveElement.focus) {
            this.previousActiveElement.focus();
        }
        this.previousActiveElement = null;
        
        this.mode = null;
        this.data = null;
        this.focusableElements = [];
        
        // Call onClose callback
        this.options.onClose();
    }
    
    /**
     * Setup focus trap for the modal
     * @private
     */
    _setupFocusTrap() {
        // Get all focusable elements within the modal
        const focusableSelectors = [
            'a[href]',
            'area[href]',
            'input:not([disabled]):not([type="hidden"])',
            'select:not([disabled])',
            'textarea:not([disabled])',
            'button:not([disabled])',
            '[tabindex]:not([tabindex="-1"])'
        ].join(',');
        
        const modalContent = this.modal.querySelector('.modal-content');
        const container = modalContent || this.modal;
        
        this.focusableElements = Array.from(
            container.querySelectorAll(focusableSelectors)
        ).filter(el => {
            // Filter out hidden elements
            return el.offsetParent !== null;
        });
    }
    
    /**
     * Handle save button click
     * @private
     */
    async _handleSave() {
        if (!this.form) {
            console.warn('No form associated with this modal');
            return;
        }
        
        // Collect form data
        const formData = new FormData(this.form);
        const data = Object.fromEntries(formData.entries());

        // Add mode and stored data
        data.mode = this.mode;
        if (this.data) {
            Object.assign(data, this.data);
        }
        
        try {
            // Call onSave callback
            await this.options.onSave(data);
            
            // Close modal on success
            this.close();
        } catch (error) {
            console.error('Error saving:', error);
            // Don't close modal on error
            throw error;
        }
    }
    
    /**
     * Get form data as object
     * @returns {Object} Form data
     */
    getFormData() {
        if (!this.form) {
            return {};
        }

        const formData = new FormData(this.form);
        const data = Object.fromEntries(formData.entries());
        data.mode = this.mode;
        if (this.data) {
            Object.assign(data, this.data);
        }
        return data;
    }
    
    /**
     * Set form values
     * @param {Object} values - Values to set
     */
    setFormValues(values) {
        if (!this.form) {
            return;
        }
        
        Object.keys(values).forEach(key => {
            const input = this.form.querySelector(`[name="${key}"]`);
            if (input) {
                input.value = values[key];
            }
        });
    }
    
    /**
     * Get current mode
     * @returns {string} Current mode
     */
    getMode() {
        return this.mode;
    }
    
    /**
     * Get stored data
     * @returns {Object} Stored data
     */
    getData() {
        return this.data;
    }
    
    /**
     * Show loading state
     */
    showLoading() {
        if (this.options.saveButtonId) {
            const saveBtn = document.getElementById(this.options.saveButtonId);
            if (saveBtn) {
                saveBtn.disabled = true;
                saveBtn.dataset.originalText = saveBtn.textContent;
                saveBtn.textContent = 'Saving...';
            }
        }
    }
    
    /**
     * Hide loading state
     */
    hideLoading() {
        if (this.options.saveButtonId) {
            const saveBtn = document.getElementById(this.options.saveButtonId);
            if (saveBtn) {
                saveBtn.disabled = false;
                if (saveBtn.dataset.originalText) {
                    saveBtn.textContent = saveBtn.dataset.originalText;
                    delete saveBtn.dataset.originalText;
                }
            }
        }
    }
}

// Make Modal available globally and as ES module
window.Modal = Modal;
export { Modal };
