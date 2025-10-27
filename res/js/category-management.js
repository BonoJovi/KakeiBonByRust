import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { setupIndicators } from './indicators.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers } from './font-size.js';

let categories = [];
let expandedCategories = new Set();

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
    setupModalHandlers();
    
    // Setup form indicators
    setupIndicators();
    
    // Load categories
    console.log('[DOMContentLoaded] Loading categories');
    await loadCategories();
    
    console.log('[DOMContentLoaded] Initialization complete');
});

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
    // Category1 modal handlers
    setupCategory1ModalHandlers();
    
    // Category2 modal handlers
    setupCategory2ModalHandlers();
    
    // Category3 modal handlers
    setupCategory3ModalHandlers();
    
    // Add category1 button
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

function setupCategory2ModalHandlers() {
    const modal = document.getElementById('category2-modal');
    const closeBtn = document.getElementById('category2-modal-close');
    const cancelBtn = document.getElementById('category2-cancel');
    const saveBtn = document.getElementById('category2-save');
    
    const closeModal = () => {
        modal.classList.add('hidden');
    };
    
    if (closeBtn) closeBtn.addEventListener('click', closeModal);
    if (cancelBtn) cancelBtn.addEventListener('click', closeModal);
    if (saveBtn) saveBtn.addEventListener('click', handleCategory2Save);
}

function setupCategory3ModalHandlers() {
    const modal = document.getElementById('category3-modal');
    const closeBtn = document.getElementById('category3-modal-close');
    const cancelBtn = document.getElementById('category3-cancel');
    const saveBtn = document.getElementById('category3-save');
    
    const closeModal = () => {
        modal.classList.add('hidden');
    };
    
    if (closeBtn) closeBtn.addEventListener('click', closeModal);
    if (cancelBtn) cancelBtn.addEventListener('click', closeModal);
    if (saveBtn) saveBtn.addEventListener('click', handleCategory3Save);
}

async function loadCategories() {
    try {
        const treeContainer = document.getElementById('category-tree');
        treeContainer.innerHTML = '<div class="loading" data-i18n="common.loading">Loading...</div>';
        i18n.updateUI();
        
        // TODO: Fetch categories from backend
        // For now, use mock data
        categories = getMockCategories();
        
        renderCategoryTree();
    } catch (error) {
        console.error('Failed to load categories:', error);
        const treeContainer = document.getElementById('category-tree');
        treeContainer.innerHTML = '<div class="error">Failed to load categories</div>';
    }
}

function getMockCategories() {
    // TODO: Remove this when backend is ready
    return [
        {
            id: 1,
            name_ja: '食費',
            name_en: 'Food',
            display_order: 1,
            children: [
                {
                    id: 11,
                    name_ja: '食料品',
                    name_en: 'Groceries',
                    display_order: 1,
                    children: [
                        { id: 111, name_ja: '野菜', name_en: 'Vegetables', display_order: 1 },
                        { id: 112, name_ja: '肉類', name_en: 'Meat', display_order: 2 }
                    ]
                },
                {
                    id: 12,
                    name_ja: '外食',
                    name_en: 'Dining Out',
                    display_order: 2,
                    children: []
                }
            ]
        },
        {
            id: 2,
            name_ja: '交通費',
            name_en: 'Transportation',
            display_order: 2,
            children: []
        }
    ];
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

function renderCategory1(category, index, total) {
    const isExpanded = expandedCategories.has(`cat1-${category.id}`);
    const hasChildren = category.children && category.children.length > 0;
    
    const div = document.createElement('div');
    div.className = 'category-item category-level-1';
    div.dataset.categoryId = category.id;
    div.dataset.level = '1';
    
    const currentLang = i18n.currentLanguage;
    const categoryName = currentLang === 'ja' ? category.name_ja : category.name_en;
    
    div.innerHTML = `
        <div class="category-header">
            <span class="expand-icon ${hasChildren ? (isExpanded ? 'expanded' : 'collapsed') : 'empty'}" data-category-id="${category.id}"></span>
            <span class="category-name">${categoryName}</span>
            <span class="category-order">${i18n.t('category_mgmt.order')}: ${category.display_order}</span>
            <div class="category-actions">
                <button class="btn-icon btn-add" data-action="add-child" data-category-id="${category.id}" data-level="1">
                    ${i18n.t('category_mgmt.add_sub')}
                </button>
                <button class="btn-icon btn-edit" data-action="edit" data-category-id="${category.id}" data-level="1">
                    ${i18n.t('common.edit')}
                </button>
                <button class="btn-icon btn-up" data-action="move-up" data-category-id="${category.id}" data-level="1" ${index === 0 ? 'disabled' : ''}>
                    ↑
                </button>
                <button class="btn-icon btn-down" data-action="move-down" data-category-id="${category.id}" data-level="1" ${index === total - 1 ? 'disabled' : ''}>
                    ↓
                </button>
            </div>
        </div>
    `;
    
    // Add event listeners
    const expandIcon = div.querySelector('.expand-icon');
    if (expandIcon && hasChildren) {
        expandIcon.addEventListener('click', () => toggleCategory(`cat1-${category.id}`));
    }
    
    // Add action button event listeners
    addActionListeners(div);
    
    // Render children
    if (hasChildren) {
        const childrenDiv = document.createElement('div');
        childrenDiv.className = `category-children ${isExpanded ? '' : 'collapsed'}`;
        childrenDiv.id = `cat1-${category.id}-children`;
        
        category.children.forEach((cat2, idx) => {
            const childElement = renderCategory2(cat2, category.id, idx, category.children.length);
            childrenDiv.appendChild(childElement);
        });
        
        div.appendChild(childrenDiv);
    }
    
    return div;
}

function renderCategory2(category, parentId, index, total) {
    const isExpanded = expandedCategories.has(`cat2-${category.id}`);
    const hasChildren = category.children && category.children.length > 0;
    
    const div = document.createElement('div');
    div.className = 'category-item category-level-2';
    div.dataset.categoryId = category.id;
    div.dataset.level = '2';
    div.dataset.parentId = parentId;
    
    const currentLang = i18n.currentLanguage;
    const categoryName = currentLang === 'ja' ? category.name_ja : category.name_en;
    
    div.innerHTML = `
        <div class="category-header">
            <span class="expand-icon ${hasChildren ? (isExpanded ? 'expanded' : 'collapsed') : 'empty'}" data-category-id="${category.id}"></span>
            <span class="category-name">${categoryName}</span>
            <span class="category-order">${i18n.t('category_mgmt.order')}: ${category.display_order}</span>
            <div class="category-actions">
                <button class="btn-icon btn-add" data-action="add-child" data-category-id="${category.id}" data-level="2">
                    ${i18n.t('category_mgmt.add_sub')}
                </button>
                <button class="btn-icon btn-edit" data-action="edit" data-category-id="${category.id}" data-level="2">
                    ${i18n.t('common.edit')}
                </button>
                <button class="btn-icon btn-up" data-action="move-up" data-category-id="${category.id}" data-level="2" ${index === 0 ? 'disabled' : ''}>
                    ↑
                </button>
                <button class="btn-icon btn-down" data-action="move-down" data-category-id="${category.id}" data-level="2" ${index === total - 1 ? 'disabled' : ''}>
                    ↓
                </button>
            </div>
        </div>
    `;
    
    // Add event listeners
    const expandIcon = div.querySelector('.expand-icon');
    if (expandIcon && hasChildren) {
        expandIcon.addEventListener('click', () => toggleCategory(`cat2-${category.id}`));
    }
    
    addActionListeners(div);
    
    // Render children
    if (hasChildren) {
        const childrenDiv = document.createElement('div');
        childrenDiv.className = `category-children ${isExpanded ? '' : 'collapsed'}`;
        childrenDiv.id = `cat2-${category.id}-children`;
        
        category.children.forEach((cat3, idx) => {
            const childElement = renderCategory3(cat3, category.id, idx, category.children.length);
            childrenDiv.appendChild(childElement);
        });
        
        div.appendChild(childrenDiv);
    }
    
    return div;
}

function renderCategory3(category, parentId, index, total) {
    const div = document.createElement('div');
    div.className = 'category-item category-level-3';
    div.dataset.categoryId = category.id;
    div.dataset.level = '3';
    div.dataset.parentId = parentId;
    
    const currentLang = i18n.currentLanguage;
    const categoryName = currentLang === 'ja' ? category.name_ja : category.name_en;
    
    div.innerHTML = `
        <div class="category-header">
            <span class="expand-icon empty"></span>
            <span class="category-name">${categoryName}</span>
            <span class="category-order">${i18n.t('category_mgmt.order')}: ${category.display_order}</span>
            <div class="category-actions">
                <button class="btn-icon btn-edit" data-action="edit" data-category-id="${category.id}" data-level="3">
                    ${i18n.t('common.edit')}
                </button>
                <button class="btn-icon btn-up" data-action="move-up" data-category-id="${category.id}" data-level="3" ${index === 0 ? 'disabled' : ''}>
                    ↑
                </button>
                <button class="btn-icon btn-down" data-action="move-down" data-category-id="${category.id}" data-level="3" ${index === total - 1 ? 'disabled' : ''}>
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
            const categoryId = parseInt(btn.dataset.categoryId);
            const level = parseInt(btn.dataset.level);
            
            switch (action) {
                case 'add-child':
                    openAddChildModal(categoryId, level);
                    break;
                case 'edit':
                    openEditModal(categoryId, level);
                    break;
                case 'move-up':
                    await moveCategoryUp(categoryId, level);
                    break;
                case 'move-down':
                    await moveCategoryDown(categoryId, level);
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

function openAddChildModal(parentId, parentLevel) {
    // TODO: Implement based on parent level
    console.log('Open add child modal for parent:', parentId, 'level:', parentLevel);
}

function openEditModal(categoryId, level) {
    // TODO: Implement based on level
    console.log('Open edit modal for category:', categoryId, 'level:', level);
}

async function handleCategory1Save() {
    // TODO: Implement save logic
    console.log('Save category1');
}

async function handleCategory2Save() {
    // TODO: Implement save logic
    console.log('Save category2');
}

async function handleCategory3Save() {
    // TODO: Implement save logic
    console.log('Save category3');
}

async function moveCategoryUp(categoryId, level) {
    // TODO: Implement move up logic
    console.log('Move up:', categoryId, 'level:', level);
}

async function moveCategoryDown(categoryId, level) {
    // TODO: Implement move down logic
    console.log('Move down:', categoryId, 'level:', level);
}
