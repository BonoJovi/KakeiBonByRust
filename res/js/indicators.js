/**
 * Indicator module for active element visualization
 * Provides focus/blur indicators for form fields and buttons
 */

/**
 * Wrap input fields in input-wrapper div for proper layout
 * Only wraps inputs/textareas/selects in .form-group that are not already wrapped
 * Excludes checkboxes and radio buttons as they have different styling
 */
export function wrapInputFields() {
    document.querySelectorAll('.form-group input:not([type="checkbox"]):not([type="radio"]), .form-group textarea, .form-group select').forEach(input => {
        // Check if already wrapped
        if (!input.parentElement.classList.contains('input-wrapper')) {
            const wrapper = document.createElement('div');
            wrapper.className = 'input-wrapper';
            input.parentNode.insertBefore(wrapper, input);
            wrapper.appendChild(input);
        }
    });
}

/**
 * Setup focus indicators for all input fields
 * Adds 'active' class to parent .form-group when input is focused
 * For checkboxes/radios, adds 'active' class to .checkbox-label
 */
export function setupInputIndicators() {
    document.querySelectorAll('input, textarea, select').forEach(input => {
        input.addEventListener('focus', function() {
            // For checkboxes and radio buttons, highlight the label
            if (this.type === 'checkbox' || this.type === 'radio') {
                const label = this.closest('.checkbox-label');
                if (label) {
                    label.classList.add('active');
                }
            } else {
                // For other inputs, highlight the form-group
                const formGroup = this.closest('.form-group');
                if (formGroup) {
                    formGroup.classList.add('active');
                }
            }
        });
        
        input.addEventListener('blur', function() {
            // For checkboxes and radio buttons, remove highlight from label
            if (this.type === 'checkbox' || this.type === 'radio') {
                const label = this.closest('.checkbox-label');
                if (label) {
                    label.classList.remove('active');
                }
            } else {
                // For other inputs, remove highlight from form-group
                const formGroup = this.closest('.form-group');
                if (formGroup) {
                    formGroup.classList.remove('active');
                }
            }
        });
    });
}

/**
 * Setup focus indicators for all buttons
 * Adds 'focus-indicator' class to enable visual focus feedback
 */
export function setupButtonIndicators() {
    document.querySelectorAll('button, .btn-primary').forEach(button => {
        button.classList.add('focus-indicator');
    });
}

/**
 * Setup all indicators (convenience function)
 * Wraps inputs and sets up focus indicators for inputs and buttons
 */
export function setupIndicators() {
    wrapInputFields();
    setupInputIndicators();
    setupButtonIndicators();
}
