import { describe, it, expect, beforeEach, afterEach } from './node_modules/mocha/index.js';
import {
    createMockInvoke,
    setInputValue,
    clickButton,
    getTableData,
    isVisible,
    waitFor
} from './aggregation-test-helpers.js';

describe('Yearly Aggregation Tests', () => {
    let originalInvoke;
    let mockInvoke;

    beforeEach(() => {
        mockInvoke = createMockInvoke();
        originalInvoke = window.__TAURI__?.core?.invoke;
        
        if (!window.__TAURI__) window.__TAURI__ = {};
        if (!window.__TAURI__.core) window.__TAURI__.core = {};
        window.__TAURI__.core.invoke = mockInvoke;
    });

    afterEach(() => {
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

        it('should display year start month select with default January', () => {
            const yearStartSelect = document.querySelector('#year-start-month');
            expect(yearStartSelect).to.exist;
            expect(parseInt(yearStartSelect.value)).to.equal(1);
        });

        it('should display group-by select with default value', () => {
            const groupBySelect = document.querySelector('#group-by');
            expect(groupBySelect).to.exist;
            expect(groupBySelect.value).to.equal('category1');
        });
    });

    describe('Year Input Validation', () => {
        it('should accept valid year', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
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
        });

        it('should reject year above 2100', async () => {
            setInputValue('#year', '2101');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.exist;
        });
    });

    describe('Year Spinner Buttons', () => {
        it('should increment year with up button', () => {
            setInputValue('#year', '2024');
            clickButton('#year-up');
            
            const year = parseInt(document.querySelector('#year').value);
            expect(year).to.equal(2025);
        });

        it('should decrement year with down button', () => {
            setInputValue('#year', '2024');
            clickButton('#year-down');
            
            const year = parseInt(document.querySelector('#year').value);
            expect(year).to.equal(2023);
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

    describe('Year Start Month Selection', () => {
        it('should accept January (calendar year)', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            setInputValue('#year-start-month', '1');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls[0].args.yearStartMonth).to.equal(1);
        });

        it('should accept April (fiscal year)', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            setInputValue('#year-start-month', '4');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls[0].args.yearStartMonth).to.equal(4);
        });
    });

    describe('Aggregation Execution', () => {
        it('should call backend with correct parameters for calendar year', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            setInputValue('#year', '2024');
            setInputValue('#year-start-month', '1');
            setInputValue('#group-by', 'category2');
            
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls[0].cmd).to.equal('get_yearly_aggregation');
            expect(calls[0].args.year).to.equal(2024);
            expect(calls[0].args.yearStartMonth).to.equal(1);
            expect(calls[0].args.groupBy).to.equal('category2');
        });

        it('should call backend with correct parameters for fiscal year', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            setInputValue('#year', '2024');
            setInputValue('#year-start-month', '4');
            setInputValue('#group-by', 'account');
            
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls[0].args.year).to.equal(2024);
            expect(calls[0].args.yearStartMonth).to.equal(4);
        });

        it('should display results after execution', async () => {
            window.__TAURI__.core.invoke = async () => [
                { group_key: 'EXPENSE', group_name: 'Expense', total_amount: -1200000, count: 365, avg_amount: -3288 },
                { group_key: 'INCOME', group_name: 'Income', total_amount: 3600000, count: 12, avg_amount: 300000 }
            ];
            
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const tableData = getTableData('#results-table');
            expect(tableData.length).to.equal(2);
        });

        it('should handle backend errors', async () => {
            window.__TAURI__.core.invoke = async () => {
                throw new Error('Failed to fetch data');
            };
            
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.exist;
        });
    });

    describe('Fiscal Year Periods', () => {
        it('should handle FY2024 (Apr 2024 - Mar 2025)', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                // Backend calculates: 2024-04-01 to 2025-03-31
                return [];
            };
            
            setInputValue('#year', '2024');
            setInputValue('#year-start-month', '4');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls[0].args.year).to.equal(2024);
            expect(calls[0].args.yearStartMonth).to.equal(4);
        });

        it('should handle calendar year 2024 (Jan 2024 - Dec 2024)', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                // Backend calculates: 2024-01-01 to 2024-12-31
                return [];
            };
            
            setInputValue('#year', '2024');
            setInputValue('#year-start-month', '1');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls[0].args.year).to.equal(2024);
            expect(calls[0].args.yearStartMonth).to.equal(1);
        });
    });

    describe('Grouping Axis Changes', () => {
        it('should aggregate by all grouping axes', async () => {
            const groupings = ['category1', 'category2', 'category3', 'account', 'shop'];
            
            for (const grouping of groupings) {
                const calls = [];
                window.__TAURI__.core.invoke = async (cmd, args) => {
                    calls.push({ cmd, args });
                    return [];
                };
                
                setInputValue('#group-by', grouping);
                clickButton('#execute-btn');
                
                await waitFor(50);
                
                expect(calls[0].args.groupBy).to.equal(grouping);
            }
        });
    });

    describe('Account Note Display', () => {
        it('should show note when account grouping is selected', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            setInputValue('#group-by', 'account');
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            expect(isVisible('#account-note')).to.be.true;
        });

        it('should hide note when category grouping is selected', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            setInputValue('#group-by', 'category1');
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            expect(isVisible('#account-note')).to.be.false;
        });
    });
});
