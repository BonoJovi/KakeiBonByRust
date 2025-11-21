/**
 * Session Management Module
 * 
 * This module provides functions to manage session state in memory.
 * Session data is NOT persisted and will be cleared when the application exits.
 * 
 * Session data includes:
 * - user_id: User identifier
 * - name: User name
 * - role: User role (0: admin, 1: user)
 * - source_screen: Source screen name (translation resource key prefix)
 * - category1_code: Category 1 code for transaction context
 */

import { invoke } from '@tauri-apps/api/core';

/**
 * Get current session user
 * @returns {Promise<{user_id: number, name: string, role: number} | null>}
 */
export async function getCurrentSessionUser() {
    try {
        return await invoke('get_current_session_user');
    } catch (error) {
        console.error('Failed to get current session user:', error);
        throw error;
    }
}

/**
 * Check if user is authenticated
 * @returns {Promise<boolean>}
 */
export async function isSessionAuthenticated() {
    try {
        return await invoke('is_session_authenticated');
    } catch (error) {
        console.error('Failed to check authentication:', error);
        throw error;
    }
}

/**
 * Set source screen name
 * @param {string} sourceScreen - Source screen name (e.g., "shop_mgmt")
 * @returns {Promise<void>}
 */
export async function setSessionSourceScreen(sourceScreen) {
    try {
        await invoke('set_session_source_screen', { sourceScreen });
    } catch (error) {
        console.error('Failed to set source screen:', error);
        throw error;
    }
}

/**
 * Get source screen name
 * @returns {Promise<string | null>}
 */
export async function getSessionSourceScreen() {
    try {
        return await invoke('get_session_source_screen');
    } catch (error) {
        console.error('Failed to get source screen:', error);
        throw error;
    }
}

/**
 * Clear source screen name
 * @returns {Promise<void>}
 */
export async function clearSessionSourceScreen() {
    try {
        await invoke('clear_session_source_screen');
    } catch (error) {
        console.error('Failed to clear source screen:', error);
        throw error;
    }
}

/**
 * Set category1 code
 * @param {string} category1Code - Category 1 code (e.g., "INCOME", "EXPENSE")
 * @returns {Promise<void>}
 */
export async function setSessionCategory1Code(category1Code) {
    try {
        await invoke('set_session_category1_code', { category1Code });
    } catch (error) {
        console.error('Failed to set category1 code:', error);
        throw error;
    }
}

/**
 * Get category1 code
 * @returns {Promise<string | null>}
 */
export async function getSessionCategory1Code() {
    try {
        return await invoke('get_session_category1_code');
    } catch (error) {
        console.error('Failed to get category1 code:', error);
        throw error;
    }
}

/**
 * Clear category1 code
 * @returns {Promise<void>}
 */
export async function clearSessionCategory1Code() {
    try {
        await invoke('clear_session_category1_code');
    } catch (error) {
        console.error('Failed to clear category1 code:', error);
        throw error;
    }
}

/**
 * Set modal state (serialized JSON)
 * @param {string} modalState - Modal state as JSON string
 * @returns {Promise<void>}
 */
export async function setSessionModalState(modalState) {
    try {
        await invoke('set_session_modal_state', { modalState });
    } catch (error) {
        console.error('Failed to set modal state:', error);
        throw error;
    }
}

/**
 * Get modal state
 * @returns {Promise<string | null>}
 */
export async function getSessionModalState() {
    try {
        return await invoke('get_session_modal_state');
    } catch (error) {
        console.error('Failed to get modal state:', error);
        throw error;
    }
}

/**
 * Clear modal state
 * @returns {Promise<void>}
 */
export async function clearSessionModalState() {
    try {
        await invoke('clear_session_modal_state');
    } catch (error) {
        console.error('Failed to clear modal state:', error);
        throw error;
    }
}

/**
 * Clear all session data (user, source_screen, category1_code)
 * @returns {Promise<void>}
 */
export async function clearSession() {
    try {
        await invoke('clear_session');
    } catch (error) {
        console.error('Failed to clear session:', error);
        throw error;
    }
}
