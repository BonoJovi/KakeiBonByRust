/**
 * Indicator module for active element visualization
 * Provides focus/blur indicators for form fields and buttons
 */

/**
 * Wrap input fields in input-wrapper div for proper layout
 * Only wraps inputs in .form-group that are not already wrapped
 */
export function wrapInputFields() {
    document.querySelectorAll('.form-group input').forEach(input => {
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
 */
export function setupInputIndicators() {
    document.querySelectorAll('input, textarea, select').forEach(input => {
        input.addEventListener('focus', function() {
            const formGroup = this.closest('.form-group');
            if (formGroup) {
                formGroup.classList.add('active');
            }
        });
        
        input.addEventListener('blur', function() {
            const formGroup = this.closest('.form-group');
            if (formGroup) {
                formGroup.classList.remove('active');
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
