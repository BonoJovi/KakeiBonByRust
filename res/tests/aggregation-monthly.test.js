import { describe, it, expect, beforeEach, afterEach } from './node_modules/mocha/index.js';
import {
    createMockInvoke,
    createMockSessionUser,
    createMockI18n,
    setInputValue,
    clickButton,
    getTableData,
    isVisible,
    waitFor
} from './aggregation-test-helpers.js';

describe('Monthly Aggregation Tests', () => {
    let originalInvoke;
    let mockInvoke;

    beforeEach(() => {
        // Setup mock
        mockInvoke = createMockInvoke();
        originalInvoke = window.__TAURI__?.core?.invoke;
        
        if (!window.__TAURI__) window.__TAURI__ = {};
        if (!window.__TAURI__.core) window.__TAURI__.core = {};
        window.__TAURI__.core.invoke = mockInvoke;
    });

    afterEach(() => {
        // Restore original
        if (originalInvoke) {
            window.__TAURI__.core.invoke = originalInvoke;
        }
    });

    describe('UI Initialization', () => {
        it('should display year input with default current year', () => {
            const yearInput = document.querySelector('#year');
            expect(yearInput).to.exist;
            
            const currentYear = new Date().getFullYear();
            expect(parseInt(yearInput.value)).to.equal(currentYear);
        });

        it('should display month select with default current month', () => {
            const monthSelect = document.querySelector('#month');
            expect(monthSelect).to.exist;
            
            const currentMonth = new Date().getMonth() + 1;
            expect(parseInt(monthSelect.value)).to.equal(currentMonth);
        });

        it('should display group-by select with default value', () => {
            const groupBySelect = document.querySelector('#group-by');
            expect(groupBySelect).to.exist;
            expect(groupBySelect.value).to.equal('category1');
        });

        it('should display execute button', () => {
            const executeBtn = document.querySelector('#execute-btn');
            expect(executeBtn).to.exist;
            expect(executeBtn.textContent).to.include('Execute');
        });
    });

    describe('Year Input Validation', () => {
        it('should accept valid year (2024)', async () => {
            setInputValue('#year', '2024');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.not.exist.or.not.be.visible;
        });

        it('should reject year below 1900', async () => {
            setInputValue('#year', '1899');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.exist;
            expect(errorMsg.textContent).to.include('year');
        });

        it('should reject year above 2100', async () => {
            setInputValue('#year', '2101');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.exist;
            expect(errorMsg.textContent).to.include('year');
        });

        it('should accept boundary year 1900', async () => {
            setInputValue('#year', '1900');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.not.exist.or.not.be.visible;
        });

        it('should accept boundary year 2100', async () => {
            setInputValue('#year', '2100');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.not.exist.or.not.be.visible;
        });
    });

    describe('Month Selection', () => {
        it('should accept valid month (1-12)', async () => {
            for (let month = 1; month <= 12; month++) {
                setInputValue('#month', month.toString());
                clickButton('#execute-btn');
                
                await waitFor(50);
                
                const errorMsg = document.querySelector('.message.error');
                expect(errorMsg).to.not.exist.or.not.be.visible;
            }
        });
    });

    describe('Year Spinner Buttons', () => {
        it('should increment year with up button', () => {
            setInputValue('#year', '2024');
            const initialYear = parseInt(document.querySelector('#year').value);
            
            clickButton('#year-up');
            
            const newYear = parseInt(document.querySelector('#year').value);
            expect(newYear).to.equal(initialYear + 1);
        });

        it('should decrement year with down button', () => {
            setInputValue('#year', '2024');
            const initialYear = parseInt(document.querySelector('#year').value);
            
            clickButton('#year-down');
            
            const newYear = parseInt(document.querySelector('#year').value);
            expect(newYear).to.equal(initialYear - 1);
        });

        it('should not go below 1900', () => {
            setInputValue('#year', '1900');
            clickButton('#year-down');
            
            const year = parseInt(document.querySelector('#year').value);
            expect(year).to.equal(1900);
        });

        it('should not go above 2100', () => {
            setInputValue('#year', '2100');
            clickButton('#year-up');
            
            const year = parseInt(document.querySelector('#year').value);
            expect(year).to.equal(2100);
        });
    });

    describe('Aggregation Execution', () => {
        it('should display results after execution', async () => {
            setInputValue('#year', '2024');
            setInputValue('#month', '11');
            setInputValue('#group-by', 'category1');
            
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const resultsTable = document.querySelector('#results-table');
            expect(resultsTable).to.exist;
            
            const tableData = getTableData('#results-table');
            expect(tableData.length).to.be.greaterThan(0);
        });

        it('should display result count', async () => {
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const resultCount = document.querySelector('#result-count');
            expect(resultCount).to.exist;
            expect(resultCount.textContent).to.match(/\(\d+ items\)/);
        });

        it('should handle empty results', async () => {
            // Mock empty result
            window.__TAURI__.core.invoke = async () => [];
            
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const emptyState = document.querySelector('.empty-state');
            expect(emptyState).to.exist;
            expect(emptyState.textContent).to.include('No results');
        });
    });

    describe('Account Note Display', () => {
        it('should show note when account grouping is selected', async () => {
            setInputValue('#group-by', 'account');
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const accountNote = document.querySelector('#account-note');
            expect(isVisible('#account-note')).to.be.true;
        });

        it('should hide note when category grouping is selected', async () => {
            setInputValue('#group-by', 'category1');
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            expect(isVisible('#account-note')).to.be.false;
        });

        it('should hide note when shop grouping is selected', async () => {
            setInputValue('#group-by', 'shop');
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            expect(isVisible('#account-note')).to.be.false;
        });
    });

    describe('Filter Toggle', () => {
        it('should toggle filter section when header is clicked', () => {
            const filterForm = document.querySelector('#filter-form-content');
            const initialState = filterForm.classList.contains('collapsed');
            
            clickButton('#filter-header');
            
            const newState = filterForm.classList.contains('collapsed');
            expect(newState).to.not.equal(initialState);
        });

        it('should toggle filter section with toggle button', () => {
            const filterForm = document.querySelector('#filter-form-content');
            const initialState = filterForm.classList.contains('collapsed');
            
            clickButton('#toggle-filter-btn');
            
            const newState = filterForm.classList.contains('collapsed');
            expect(newState).to.not.equal(initialState);
        });
    });
});
