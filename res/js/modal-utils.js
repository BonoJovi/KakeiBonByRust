/**
 * Modal Utilities
 * Reusable functions for modal handling across the application
 */

/**
 * Setup ESC key handler for closing modals
 * @param {Object} modalConfig - Configuration object with modal IDs and their close functions
 * @example
 * setupEscapeKeyHandler({
 *   'user-modal': closeUserModal,
 *   'delete-modal': closeDeleteModal
 * });
 */
export function setupEscapeKeyHandler(modalConfig) {
    document.addEventListener('keydown', function(e) {
        if (e.key === 'Escape') {
            for (const [modalId, closeFunction] of Object.entries(modalConfig)) {
                const modal = document.getElementById(modalId);
                if (modal && !modal.classList.contains('hidden')) {
                    closeFunction();
                    break;
                }
            }
        }
    });
}

/**
 * Setup click-outside-to-close handler for a modal
 * @param {string} modalId - The ID of the modal element
 * @param {Function} closeFunction - Function to call when closing the modal
 */
export function setupModalClickOutside(modalId, closeFunction) {
    const modal = document.getElementById(modalId);
    if (modal) {
        modal.addEventListener('click', function(e) {
            if (e.target === this) {
                closeFunction();
            }
        });
    }
}

/**
 * Setup focus trap for a modal
 * Keeps focus within the modal when Tab/Shift+Tab is pressed
 * @param {string} modalId - The ID of the modal element
 */
export function setupFocusTrap(modalId) {
    const modal = document.getElementById(modalId);
    if (!modal) return;

    const handleFocusTrap = function(e) {
        // Handle Tab key (with or without Shift modifier)
        // Also handle "Unidentified" key when Shift is pressed (Tauri-specific issue)
        const isTab = e.key === 'Tab' || (e.shiftKey && (e.key === 'Unidentified' || e.code === 'Tab'));
        
        if (!isTab) return;
        
        // Only trap focus when this modal is visible
        if (modal.classList.contains('hidden')) return;

        // Get all focusable elements within the modal
        const focusableElements = modal.querySelectorAll(
            'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
        );
        
        if (focusableElements.length === 0) return;

        const firstElement = focusableElements[0];
        const lastElement = focusableElements[focusableElements.length - 1];
        
        if (e.shiftKey) {
            // Shift + Tab: moving backwards
            // If at first element or focus is outside modal, wrap to last element
            if (document.activeElement === firstElement || !modal.contains(document.activeElement)) {
                e.preventDefault();
                lastElement.focus();
            }
        } else {
            // Tab: moving forwards
            // If at last element or focus is outside modal, wrap to first element
            if (document.activeElement === lastElement || !modal.contains(document.activeElement)) {
                e.preventDefault();
                firstElement.focus();
            }
        }
    };

    // Use capture phase to intercept Tab events before they reach elements
    // This is crucial for catching SHIFT+TAB on the first element
    document.addEventListener('keydown', handleFocusTrap, true);
}

/**
 * Setup all standard modal handlers (ESC key, click outside, and focus trap)
 * @param {Object} modalConfig - Configuration object with modal IDs and their close functions
 * @example
 * setupModalHandlers({
 *   'user-modal': closeUserModal,
 *   'delete-modal': closeDeleteModal
 * });
 */
export function setupModalHandlers(modalConfig) {
    setupEscapeKeyHandler(modalConfig);
    
    for (const [modalId, closeFunction] of Object.entries(modalConfig)) {
        setupModalClickOutside(modalId, closeFunction);
        setupFocusTrap(modalId);
    }
}
