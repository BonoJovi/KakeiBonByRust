import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { setupIndicators } from './indicators.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers, adjustWindowSize } from './font-size.js';
import { Modal } from './modal.js';

// Category level constants
const LEVEL_CATEGORY1 = 1;
const LEVEL_CATEGORY2 = 2;
const LEVEL_CATEGORY3 = 3;

let categories = [];
let expandedCategories = new Set();
const currentUserId = 1; // TODO: Get from session/auth

// Modal instances
let category2Modal;
let category3Modal;

console.log('category-management.js loaded');

document.addEventListener('DOMContentLoaded', async function() {
    console.log('[DOMContentLoaded] DOM loaded');
    
    await i18n.init();
    i18n.updateUI();
    
    // Setup menu handlers
    console.log('[DOMContentLoaded] Setting up menu handlers');
    setupMenuHandlers();
    
    // Setup language and font size menus
    console.log('[DOMContentLoaded] Setting up language menu');
    await setupLanguageMenu();
    setupLanguageMenuHandlers();
    
    console.log('[DOMContentLoaded] Setting up font size menu');
    setupFontSizeMenuHandlers();
    await setupFontSizeMenu();
    setupFontSizeModalHandlers();
    await applyFontSize();
    
    // Setup modal handlers
    console.log('[DOMContentLoaded] Setting up modal handlers');
    initModals();
    setupModalHandlers();
    
    // Setup form indicators
    setupIndicators();
    
    // Load categories
    console.log('[DOMContentLoaded] Loading categories');
    await loadCategories();
    
    console.log('[DOMContentLoaded] Adjusting window size for modals');
    await adjustWindowSize();
    
    console.log('[DOMContentLoaded] Initialization complete');
});

function initModals() {
    // Initialize Category2 Modal
    category2Modal = new Modal('category2-modal', {
        formId: 'category2-form',
        closeButtonId: 'category2-modal-close',
        cancelButtonId: 'category2-cancel',
        saveButtonId: 'category2-save',
        onOpen: (mode, data) => {
            const title = document.getElementById('category2-modal-title');
            const parentNameField = document.getElementById('category2-parent-name');
            const nameJaField = document.getElementById('category2-name-ja');
            const nameEnField = document.getElementById('category2-name-en');
            
            // Set title
            title.textContent = mode === 'add' ? 
                i18n.t('category_mgmt.add_category2') : 
                i18n.t('category_mgmt.edit_category2');
            
            // Find parent category name
            const parentCategory = categories.find(cat => cat.category1.category1_code === data.category1Code);
            const parentName = parentCategory ? parentCategory.category1.category1_name_i18n : data.category1Code;
            parentNameField.value = parentName;
            
            // Set values for edit mode
            if (mode === 'edit') {
                nameJaField.value = data.nameJa || '';
                nameEnField.value = data.nameEn || '';
            }
        },
        onSave: async (formData) => {
            await handleCategory2Save(formData);
        }
    });
    
    // Initialize Category3 Modal
    category3Modal = new Modal('category3-modal', {
        formId: 'category3-form',
        closeButtonId: 'category3-modal-close',
        cancelButtonId: 'category3-cancel',
        saveButtonId: 'category3-save',
        onOpen: (mode, data) => {
            const title = document.getElementById('category3-modal-title');
            const parentNameField = document.getElementById('category3-parent-name');
            const nameJaField = document.getElementById('category3-name-ja');
            const nameEnField = document.getElementById('category3-name-en');
            
            // Set title
            title.textContent = mode === 'add' ? 
                i18n.t('category_mgmt.add_category3') : 
                i18n.t('category_mgmt.edit_category3');
            
            // Find parent category name
            let parentName = data.category2Code;
            const parentCategory1 = categories.find(cat => cat.category1.category1_code === data.category1Code);
            if (parentCategory1) {
                const parentCategory2 = parentCategory1.children.find(cat => cat.category2.category2_code === data.category2Code);
                if (parentCategory2) {
                    parentName = parentCategory2.category2.category2_name_i18n;
                }
            }
            parentNameField.value = parentName;
            
            // Set values for edit mode
            if (mode === 'edit') {
                nameJaField.value = data.nameJa || '';
                nameEnField.value = data.nameEn || '';
            }
        },
        onSave: async (formData) => {
            await handleCategory3Save(formData);
        }
    });
}


function setupMenuHandlers() {
    const fileMenu = document.getElementById('file-menu');
    const fileDropdown = document.getElementById('file-dropdown');
    
    if (fileMenu && fileDropdown) {
        fileMenu.addEventListener('click', function(e) {
            e.stopPropagation();
            const isShown = fileDropdown.classList.contains('show');
            
            document.querySelectorAll('.dropdown').forEach(d => {
                if (d !== fileDropdown) {
                    d.classList.remove('show');
                }
            });
            
            if (!isShown) {
                fileDropdown.classList.add('show');
            }
        });
        
        fileDropdown.addEventListener('click', function(e) {
            e.stopPropagation();
        });
        
        // Back to main
        const backToMainItem = fileDropdown.querySelector('.dropdown-item:nth-child(1)');
        if (backToMainItem) {
            backToMainItem.addEventListener('click', function() {
                window.location.href = 'index.html';
            });
        }
        
        // Logout
        const logoutItem = fileDropdown.querySelector('.dropdown-item:nth-child(3)');
        if (logoutItem) {
            logoutItem.addEventListener('click', function() {
                // TODO: Implement logout
                window.location.href = 'index.html';
            });
        }
        
        // Quit
        const quitItem = fileDropdown.querySelector('.dropdown-item:nth-child(4)');
        if (quitItem) {
            quitItem.addEventListener('click', function() {
                invoke('handle_quit');
            });
        }
    }
    
    // Global click handler to close dropdowns
    if (!document.body.dataset.globalClickHandlerInitialized) {
        document.addEventListener('click', function() {
            document.querySelectorAll('.dropdown').forEach(d => {
                d.classList.remove('show');
            });
        });
        document.body.dataset.globalClickHandlerInitialized = 'true';
    }
}

function setupLanguageMenuHandlers() {
    const languageMenu = document.getElementById('language-menu');
    const languageDropdown = document.getElementById('language-dropdown');
    
    if (!languageMenu || !languageDropdown) {
        return;
    }
    
    if (languageMenu.dataset.initialized === 'true') {
        return;
    }
    
    languageMenu.addEventListener('click', function(e) {
        e.stopPropagation();
        
        const isShown = languageDropdown.classList.contains('show');
        
        document.querySelectorAll('.dropdown').forEach(d => {
            if (d !== languageDropdown) {
                d.classList.remove('show');
            }
        });
        
        if (!isShown) {
            languageDropdown.classList.add('show');
        }
    });
    
    languageDropdown.addEventListener('click', function(e) {
        e.stopPropagation();
    });
    
    languageMenu.dataset.initialized = 'true';
}

async function setupLanguageMenu() {
    try {
        const languages = await invoke('get_available_languages');
        const currentLang = i18n.currentLanguage;
        const languageNames = await invoke('get_language_names', { languages });
        
        const languageDropdown = document.getElementById('language-dropdown');
        if (!languageDropdown) {
            return;
        }
        
        languageDropdown.innerHTML = '';
        
        for (const lang of languages) {
            const item = document.createElement('div');
            item.className = 'dropdown-item';
            item.textContent = languageNames[lang] || lang;
            item.dataset.langCode = lang;
            
            if (lang === currentLang) {
                item.classList.add('active');
            }
            
            item.addEventListener('click', async function(e) {
                e.stopPropagation();
                await handleLanguageChange(lang);
                languageDropdown.classList.remove('show');
            });
            
            languageDropdown.appendChild(item);
        }
    } catch (error) {
        console.error('Failed to setup language menu:', error);
    }
}

async function handleLanguageChange(langCode) {
    try {
        await i18n.setLanguage(langCode);
        await setupLanguageMenu();
        
        // Reload categories to get translated names
        await loadCategories();
    } catch (error) {
        console.error('Failed to change language:', error);
    }
}

function setupModalHandlers() {
    // Category1 modal handlers (not yet migrated to Modal class)
    setupCategory1ModalHandlers();
    
    // Note: Category2 and Category3 modals now use Modal class instances
    // initialized in initModals()
    
    // Add category1 button (if exists)
    const addCategory1Btn = document.getElementById('add-category1-btn');
    if (addCategory1Btn) {
        addCategory1Btn.addEventListener('click', () => openCategory1Modal());
    }
}

function setupCategory1ModalHandlers() {
    const modal = document.getElementById('category1-modal');
    const closeBtn = document.getElementById('category1-modal-close');
    const cancelBtn = document.getElementById('category1-cancel');
    const saveBtn = document.getElementById('category1-save');
    
    const closeModal = () => {
        modal.classList.add('hidden');
    };
    
    if (closeBtn) closeBtn.addEventListener('click', closeModal);
    if (cancelBtn) cancelBtn.addEventListener('click', closeModal);
    if (saveBtn) saveBtn.addEventListener('click', handleCategory1Save);
}

async function loadCategories() {
    try {
        const treeContainer = document.getElementById('category-tree');
        treeContainer.innerHTML = '<div class="loading" data-i18n="common.loading">Loading...</div>';
        i18n.updateUI();
        
        // Get current language
        const currentLang = i18n.getCurrentLanguage();
        
        // Fetch categories from backend
        categories = await invoke('get_category_tree_with_lang', {
            userId: currentUserId,
            langCode: currentLang
        });
        
        console.log('Loaded categories:', categories);
        
        renderCategoryTree();
    } catch (error) {
        console.error('Failed to load categories:', error);
        const treeContainer = document.getElementById('category-tree');
        treeContainer.innerHTML = '<div class="error">Failed to load categories: ' + error + '</div>';
    }
}


function renderCategoryTree() {
    const treeContainer = document.getElementById('category-tree');
    treeContainer.innerHTML = '';
    
    if (categories.length === 0) {
        treeContainer.innerHTML = '<div class="empty-message" data-i18n="category_mgmt.no_categories">No categories found.</div>';
        i18n.updateUI();
        return;
    }
    
    categories.forEach((cat1, index) => {
        const element = renderCategory1(cat1, index, categories.length);
        treeContainer.appendChild(element);
    });
}

function renderCategory1(categoryTree, index, total) {
    const category = categoryTree.category1;
    const isExpanded = expandedCategories.has(`cat1-${category.category1_code}`);
    const hasChildren = categoryTree.children && categoryTree.children.length > 0;
    
    const div = document.createElement('div');
    div.className = 'category-item category-level-1';
    div.dataset.categoryCode = category.category1_code;
    div.dataset.level = '1';
    
    // Use i18n name if available, fallback to base name
    const categoryName = category.category1_name_i18n || category.category1_name;
    
    div.innerHTML = `
        <div class="category-header">
            <span class="expand-icon ${hasChildren ? (isExpanded ? 'expanded expandable' : 'collapsed expandable') : 'empty'}" data-category-code="${category.category1_code}"></span>
            <span class="category-name ${hasChildren ? 'expandable' : ''}">${categoryName}</span>
            <span class="category-order">${i18n.t('category_mgmt.order')}: ${category.display_order}</span>
            <div class="category-actions">
                <button class="btn-icon btn-add" data-action="add-child" data-category-code="${category.category1_code}" data-category1-code="${category.category1_code}" data-level="1">
                    ${i18n.t('category_mgmt.add_sub')}
                </button>
            </div>
        </div>
    `;
    
    // Add event listeners
    const expandIcon = div.querySelector('.expand-icon');
    const categoryNameElem = div.querySelector('.category-name');
    
    if (expandIcon && hasChildren) {
        expandIcon.addEventListener('click', () => toggleCategory(`cat1-${category.category1_code}`));
    }
    
    if (categoryNameElem && hasChildren) {
        categoryNameElem.addEventListener('dblclick', () => toggleCategory(`cat1-${category.category1_code}`));
    }
    
    // Add action button event listeners
    addActionListeners(div);
    
    // Render children
    if (hasChildren) {
        const childrenDiv = document.createElement('div');
        childrenDiv.className = `category-children ${isExpanded ? '' : 'collapsed'}`;
        childrenDiv.id = `cat1-${category.category1_code}-children`;
        
        categoryTree.children.forEach((cat2Tree, idx) => {
            const childElement = renderCategory2(cat2Tree, category.category1_code, idx, categoryTree.children.length);
            childrenDiv.appendChild(childElement);
        });
        
        div.appendChild(childrenDiv);
    }
    
    return div;
}

function renderCategory2(cat2Tree, parent1Code, index, total) {
    const category = cat2Tree.category2;
    const isExpanded = expandedCategories.has(`cat2-${category.category2_code}`);
    const hasChildren = cat2Tree.children && cat2Tree.children.length > 0;
    
    const div = document.createElement('div');
    div.className = 'category-item category-level-2';
    div.dataset.categoryCode = category.category2_code;
    div.dataset.category1Code = category.category1_code;
    div.dataset.level = '2';
    
    const categoryName = category.category2_name_i18n || category.category2_name;
    
    div.innerHTML = `
        <div class="category-header">
            <span class="expand-icon ${hasChildren ? (isExpanded ? 'expanded expandable' : 'collapsed expandable') : 'empty'}" data-category-code="${category.category2_code}"></span>
            <span class="category-name ${hasChildren ? 'expandable' : ''}">${categoryName}</span>
            <span class="category-order">${i18n.t('category_mgmt.order')}: ${category.display_order}</span>
            <div class="category-actions">
                <button class="btn-icon btn-add" data-action="add-child" data-category-code="${category.category2_code}" data-category1-code="${parent1Code}" data-category2-code="${category.category2_code}" data-level="2">
                    ${i18n.t('category_mgmt.add_sub')}
                </button>
                <button class="btn-icon btn-edit" data-action="edit" data-category-code="${category.category2_code}" data-category1-code="${parent1Code}" data-category2-code="${category.category2_code}" data-level="2">
                    ${i18n.t('common.edit')}
                </button>
                <button class="btn-icon btn-up" data-action="move-up" data-category-code="${category.category2_code}" data-category1-code="${parent1Code}" data-category2-code="${category.category2_code}" data-level="2" ${index === 0 ? 'disabled' : ''}>
                    ↑
                </button>
                <button class="btn-icon btn-down" data-action="move-down" data-category-code="${category.category2_code}" data-category1-code="${parent1Code}" data-category2-code="${category.category2_code}" data-level="2" ${index === total - 1 ? 'disabled' : ''}>
                    ↓
                </button>
            </div>
        </div>
    `;
    
    // Add event listeners
    const expandIcon = div.querySelector('.expand-icon');
    const categoryNameElem = div.querySelector('.category-name');
    
    if (expandIcon && hasChildren) {
        expandIcon.addEventListener('click', () => toggleCategory(`cat2-${category.category2_code}`));
    }
    
    if (categoryNameElem && hasChildren) {
        categoryNameElem.addEventListener('dblclick', () => toggleCategory(`cat2-${category.category2_code}`));
    }
    
    addActionListeners(div);
    
    // Render children
    if (hasChildren) {
        const childrenDiv = document.createElement('div');
        childrenDiv.className = `category-children ${isExpanded ? '' : 'collapsed'}`;
        childrenDiv.id = `cat2-${category.category2_code}-children`;
        
        cat2Tree.children.forEach((cat3, idx) => {
            const childElement = renderCategory3(cat3, parent1Code, category.category2_code, idx, cat2Tree.children.length);
            childrenDiv.appendChild(childElement);
        });
        
        div.appendChild(childrenDiv);
    }
    
    return div;
}

function renderCategory3(category, parent1Code, parent2Code, index, total) {
    const div = document.createElement('div');
    div.className = 'category-item category-level-3';
    div.dataset.categoryCode = category.category3_code;
    div.dataset.category1Code = parent1Code;
    div.dataset.category2Code = parent2Code;
    div.dataset.level = '3';
    
    const categoryName = category.category3_name_i18n || category.category3_name;
    
    div.innerHTML = `
        <div class="category-header">
            <span class="expand-icon empty"></span>
            <span class="category-name">${categoryName}</span>
            <span class="category-order">${i18n.t('category_mgmt.order')}: ${category.display_order}</span>
            <div class="category-actions">
                <button class="btn-icon btn-edit" data-action="edit" data-category-code="${category.category3_code}" data-category1-code="${parent1Code}" data-category2-code="${parent2Code}" data-category3-code="${category.category3_code}" data-level="3">
                    ${i18n.t('common.edit')}
                </button>
                <button class="btn-icon btn-up" data-action="move-up" data-category-code="${category.category3_code}" data-category1-code="${parent1Code}" data-category2-code="${parent2Code}" data-category3-code="${category.category3_code}" data-level="3" ${index === 0 ? 'disabled' : ''}>
                    ↑
                </button>
                <button class="btn-icon btn-down" data-action="move-down" data-category-code="${category.category3_code}" data-category1-code="${parent1Code}" data-category2-code="${parent2Code}" data-category3-code="${category.category3_code}" data-level="3" ${index === total - 1 ? 'disabled' : ''}>
                    ↓
                </button>
            </div>
        </div>
    `;
    
    addActionListeners(div);
    
    return div;
}

function addActionListeners(element) {
    const buttons = element.querySelectorAll('[data-action]');
    buttons.forEach(btn => {
        btn.addEventListener('click', async (e) => {
            e.stopPropagation();
            const action = btn.dataset.action;
            const level = parseInt(btn.dataset.level);
            const category1Code = btn.dataset.category1Code;
            const category2Code = btn.dataset.category2Code;
            const category3Code = btn.dataset.category3Code;
            
            // Get the appropriate category code based on level
            const categoryCode = level === LEVEL_CATEGORY3 ? category3Code : (level === LEVEL_CATEGORY2 ? category2Code : btn.dataset.categoryCode);
            
            switch (action) {
                case 'add-child':
                    openAddChildModal(categoryCode, category1Code, category2Code, level);
                    break;
                case 'edit':
                    openEditModal(categoryCode, category1Code, category2Code, level);
                    break;
                case 'move-up':
                    await moveCategoryUp(categoryCode, category1Code, category2Code, level);
                    break;
                case 'move-down':
                    await moveCategoryDown(categoryCode, category1Code, category2Code, level);
                    break;
            }
        });
    });
}

function toggleCategory(categoryKey) {
    if (expandedCategories.has(categoryKey)) {
        expandedCategories.delete(categoryKey);
    } else {
        expandedCategories.add(categoryKey);
    }
    renderCategoryTree();
}

function openCategory1Modal(category = null) {
    const modal = document.getElementById('category1-modal');
    const title = document.getElementById('category1-modal-title');
    const form = document.getElementById('category1-form');
    
    if (category) {
        title.textContent = i18n.t('category_mgmt.edit_category1');
        document.getElementById('category1-id').value = category.id;
        document.getElementById('category1-name-ja').value = category.name_ja;
        document.getElementById('category1-name-en').value = category.name_en;
        document.getElementById('category1-order').value = category.display_order;
    } else {
        title.textContent = i18n.t('category_mgmt.add_category1');
        form.reset();
        document.getElementById('category1-id').value = '';
        document.getElementById('category1-order').value = categories.length + 1;
    }
    
    modal.classList.remove('hidden');
}

function openAddChildModal(parentCategoryCode, category1Code, category2Code, parentLevel) {
    console.log('Open add child modal', {parentCategoryCode, category1Code, category2Code, parentLevel});
    
    if (parentLevel === LEVEL_CATEGORY1) {
        // Add category2 under category1
        openCategory2Modal('add', category1Code || parentCategoryCode);
    } else if (parentLevel === LEVEL_CATEGORY2) {
        // Add category3 under category2
        openCategory3Modal('add', category1Code, category2Code || parentCategoryCode);
    }
}

function openCategory2Modal(mode, category1Code) {
    category2Modal.open(mode, { category1Code });
}

function openCategory3Modal(mode, category1Code, category2Code) {
    category3Modal.open(mode, { category1Code, category2Code });
}

async function openEditModal(categoryCode, category1Code, category2Code, level) {
    console.log('Open edit modal for category:', categoryCode, 'level:', level);
    
    try {
        if (level === LEVEL_CATEGORY2) {
            // Fetch category2 data from backend
            const categoryData = await invoke('get_category2_for_edit', {
                userId: currentUserId,
                category1Code: category1Code,
                category2Code: categoryCode
            });
            
            console.log('Category2 data from backend:', categoryData);
            
            // Open modal with data
            category2Modal.open('edit', {
                category1Code: category1Code,
                category2Code: categoryCode,
                nameJa: categoryData.name_ja || '',
                nameEn: categoryData.name_en || ''
            });
        } else if (level === LEVEL_CATEGORY3) {
            // Fetch category3 data from backend
            const categoryData = await invoke('get_category3_for_edit', {
                userId: currentUserId,
                category1Code: category1Code,
                category2Code: category2Code,
                category3Code: categoryCode
            });
            
            console.log('Category3 data from backend:', categoryData);
            
            // Open modal with data
            category3Modal.open('edit', {
                category1Code: category1Code,
                category2Code: category2Code,
                category3Code: categoryCode,
                nameJa: categoryData.name_ja || '',
                nameEn: categoryData.name_en || ''
            });
        }
    } catch (error) {
        console.error('Failed to load category data for edit:', error);
        const errorMsg = i18n.t('category_mgmt.error_load_category');
        const errorElement = document.getElementById('error-message');
        if (errorElement) {
            errorElement.textContent = errorMsg + ': ' + error;
            errorElement.style.display = 'block';
        }
    }
}

async function handleCategory1Save() {
    // TODO: Implement save logic
    console.log('Save category1');
}

async function handleCategory2Save(formData) {
    const mode = formData.mode;
    const category1Code = formData.category1Code;
    const category2Code = formData.category2Code;
    
    let nameJa = document.getElementById('category2-name-ja').value.trim();
    let nameEn = document.getElementById('category2-name-en').value.trim();
    
    // If one is empty, copy from the other
    if (!nameJa && !nameEn) {
        alert('Please enter at least one name (Japanese or English)');
        throw new Error('Name is required');
    }
    if (!nameJa) nameJa = nameEn;
    if (!nameEn) nameEn = nameJa;
    
    try {
        if (mode === 'add') {
            await invoke('add_category2', {
                userId: currentUserId,
                category1Code: category1Code,
                nameJa: nameJa,
                nameEn: nameEn
            });
        } else if (mode === 'edit') {
            await invoke('update_category2', {
                userId: currentUserId,
                category1Code: category1Code,
                category2Code: category2Code,
                nameJa: nameJa,
                nameEn: nameEn
            });
        }
        
        // Reload categories
        await loadCategories();
    } catch (error) {
        console.error('Failed to save category2:', error);
        
        // Check if it's a duplicate name error
        if (error.includes('already exists')) {
            const match = error.match(/Category name '(.+)' already exists/);
            const duplicateName = match ? match[1] : '';
            const errorMsg = i18n.t('error.category_duplicate_name').replace('{0}', duplicateName);
            alert(errorMsg);
        } else {
            alert('Failed to save: ' + error);
        }
        throw error; // Re-throw to prevent modal from closing
    }
}

async function handleCategory3Save(formData) {
    const mode = formData.mode;
    const category1Code = formData.category1Code;
    const category2Code = formData.category2Code;
    const category3Code = formData.category3Code;
    
    let nameJa = document.getElementById('category3-name-ja').value.trim();
    let nameEn = document.getElementById('category3-name-en').value.trim();
    
    // If one is empty, copy from the other
    if (!nameJa && !nameEn) {
        alert('Please enter at least one name (Japanese or English)');
        throw new Error('Name is required');
    }
    if (!nameJa) nameJa = nameEn;
    if (!nameEn) nameEn = nameJa;
    
    try {
        if (mode === 'add') {
            await invoke('add_category3', {
                userId: currentUserId,
                category1Code: category1Code,
                category2Code: category2Code,
                nameJa: nameJa,
                nameEn: nameEn
            });
        } else if (mode === 'edit') {
            await invoke('update_category3', {
                userId: currentUserId,
                category1Code: category1Code,
                category2Code: category2Code,
                category3Code: category3Code,
                nameJa: nameJa,
                nameEn: nameEn
            });
        }
        
        // Reload categories
        await loadCategories();
    } catch (error) {
        console.error('Failed to save category3:', error);
        
        // Check if it's a duplicate name error
        if (error.includes('already exists')) {
            const match = error.match(/Category name '(.+)' already exists/);
            const duplicateName = match ? match[1] : '';
            const errorMsg = i18n.t('error.category_duplicate_name').replace('{0}', duplicateName);
            alert(errorMsg);
        } else {
            alert('Failed to save: ' + error);
        }
        throw error; // Re-throw to prevent modal from closing
    }
}

async function moveCategoryUp(categoryId, level) {
    // TODO: Implement move up logic
    console.log('Move up:', categoryId, 'level:', level);
}

async function moveCategoryDown(categoryId, level) {
    // TODO: Implement move down logic
    console.log('Move down:', categoryId, 'level:', level);
}
