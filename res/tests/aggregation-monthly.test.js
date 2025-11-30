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
        it('should call backend with correct parameters', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            setInputValue('#year', '2024');
            setInputValue('#month', '11');
            setInputValue('#group-by', 'category2');
            
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls.length).to.equal(1);
            expect(calls[0].cmd).to.equal('get_monthly_aggregation');
            expect(calls[0].args.year).to.equal(2024);
            expect(calls[0].args.month).to.equal(11);
            expect(calls[0].args.groupBy).to.equal('category2');
        });

        it('should display results after execution', async () => {
            window.__TAURI__.core.invoke = async () => [
                { group_key: 'EXPENSE', group_name: 'Expense', total_amount: -50000, count: 10, avg_amount: -5000 },
                { group_key: 'INCOME', group_name: 'Income', total_amount: 100000, count: 2, avg_amount: 50000 }
            ];
            
            setInputValue('#year', '2024');
            setInputValue('#month', '11');
            setInputValue('#group-by', 'category1');
            
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const resultsTable = document.querySelector('#results-table');
            expect(resultsTable).to.exist;
            
            const tableData = getTableData('#results-table');
            expect(tableData.length).to.equal(2);
            expect(tableData[0].group_name).to.equal('Expense');
            expect(tableData[1].group_name).to.equal('Income');
        });

        it('should display result count', async () => {
            window.__TAURI__.core.invoke = async () => [
                { group_key: 'CAT1', group_name: 'Category 1', total_amount: -10000, count: 5, avg_amount: -2000 },
                { group_key: 'CAT2', group_name: 'Category 2', total_amount: -20000, count: 10, avg_amount: -2000 }
            ];
            
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const resultCount = document.querySelector('#result-count');
            expect(resultCount).to.exist;
            expect(resultCount.textContent).to.match(/2/);
        });

        it('should handle empty results', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const resultsTable = document.querySelector('#results-table');
            const tbody = resultsTable?.querySelector('tbody');
            expect(tbody?.children.length).to.equal(0);
        });

        it('should display loading state during execution', async () => {
            window.__TAURI__.core.invoke = async () => {
                await new Promise(resolve => setTimeout(resolve, 100));
                return [];
            };
            
            clickButton('#execute-btn');
            
            const loadingMsg = document.querySelector('.message.info');
            expect(loadingMsg).to.exist;
            expect(loadingMsg.textContent).to.include('Loading');
            
            await waitFor(200);
        });

        it('should handle backend errors', async () => {
            window.__TAURI__.core.invoke = async () => {
                throw new Error('Database connection failed');
            };
            
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.exist;
            expect(errorMsg.textContent).to.include('error');
        });

        it('should format amounts correctly', async () => {
            window.__TAURI__.core.invoke = async () => [
                { group_key: 'TEST', group_name: 'Test', total_amount: -123456, count: 1, avg_amount: -123456 }
            ];
            
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const tableData = getTableData('#results-table');
            expect(tableData[0].total_amount).to.match(/-123,456/);
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

    describe('Grouping Axis Changes', () => {
        it('should aggregate by category1', async () => {
            window.__TAURI__.core.invoke = async (cmd, args) => {
                expect(args.groupBy).to.equal('category1');
                return [{ group_key: 'EXPENSE', group_name: 'Expense', total_amount: -50000, count: 10, avg_amount: -5000 }];
            };
            
            setInputValue('#group-by', 'category1');
            clickButton('#execute-btn');
            await waitFor(100);
        });

        it('should aggregate by category2', async () => {
            window.__TAURI__.core.invoke = async (cmd, args) => {
                expect(args.groupBy).to.equal('category2');
                return [{ group_key: 'FOOD', group_name: 'Food', total_amount: -30000, count: 15, avg_amount: -2000 }];
            };
            
            setInputValue('#group-by', 'category2');
            clickButton('#execute-btn');
            await waitFor(100);
        });

        it('should aggregate by category3', async () => {
            window.__TAURI__.core.invoke = async (cmd, args) => {
                expect(args.groupBy).to.equal('category3');
                return [{ group_key: 'DINING', group_name: 'Dining Out', total_amount: -15000, count: 5, avg_amount: -3000 }];
            };
            
            setInputValue('#group-by', 'category3');
            clickButton('#execute-btn');
            await waitFor(100);
        });

        it('should aggregate by account', async () => {
            window.__TAURI__.core.invoke = async (cmd, args) => {
                expect(args.groupBy).to.equal('account');
                return [{ group_key: 'CASH', group_name: 'Cash', total_amount: -25000, count: 20, avg_amount: -1250 }];
            };
            
            setInputValue('#group-by', 'account');
            clickButton('#execute-btn');
            await waitFor(100);
        });

        it('should aggregate by shop', async () => {
            window.__TAURI__.core.invoke = async (cmd, args) => {
                expect(args.groupBy).to.equal('shop');
                return [{ group_key: '1', group_name: 'Convenience Store', total_amount: -10000, count: 8, avg_amount: -1250 }];
            };
            
            setInputValue('#group-by', 'shop');
            clickButton('#execute-btn');
            await waitFor(100);
        });
    });

    describe('Future Date Validation', () => {
        it('should reject future month', async () => {
            const now = new Date();
            const futureYear = now.getFullYear() + 1;
            
            setInputValue('#year', futureYear.toString());
            setInputValue('#month', '1');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.exist;
            expect(errorMsg.textContent).to.match(/future/i);
        });

        it('should accept current month', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            const now = new Date();
            setInputValue('#year', now.getFullYear().toString());
            setInputValue('#month', (now.getMonth() + 1).toString());
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.not.exist.or.not.be.visible;
        });
    });
});
