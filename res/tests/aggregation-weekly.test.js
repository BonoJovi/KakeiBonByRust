import { describe, it, expect, beforeEach, afterEach } from './node_modules/mocha/index.js';
import {
    createMockInvoke,
    setInputValue,
    clickButton,
    getTableData,
    isVisible,
    waitFor
} from './aggregation-test-helpers.js';

describe('Weekly Aggregation Tests', () => {
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
        it('should display reference date input with default today', () => {
            const dateInput = document.querySelector('#reference-date');
            expect(dateInput).to.exist;
            
            const today = new Date().toISOString().split('T')[0];
            expect(dateInput.value).to.equal(today);
        });

        it('should display week start select with default Monday', () => {
            const weekStartSelect = document.querySelector('#week-start');
            expect(weekStartSelect).to.exist;
            expect(weekStartSelect.value).to.equal('monday');
        });

        it('should display group-by select with default value', () => {
            const groupBySelect = document.querySelector('#group-by');
            expect(groupBySelect).to.exist;
            expect(groupBySelect.value).to.equal('category1');
        });

        it('should display execute button', () => {
            const executeBtn = document.querySelector('#execute-btn');
            expect(executeBtn).to.exist;
        });
    });

    describe('Reference Date Validation', () => {
        it('should accept valid date', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            setInputValue('#reference-date', '2024-11-20');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.not.exist.or.not.be.visible;
        });

        it('should reject future date', async () => {
            const tomorrow = new Date();
            tomorrow.setDate(tomorrow.getDate() + 1);
            const futureDate = tomorrow.toISOString().split('T')[0];
            
            setInputValue('#reference-date', futureDate);
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.exist;
            expect(errorMsg.textContent).to.match(/future/i);
        });

        it('should accept today', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            const today = new Date().toISOString().split('T')[0];
            setInputValue('#reference-date', today);
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.not.exist.or.not.be.visible;
        });

        it('should reject empty date', async () => {
            setInputValue('#reference-date', '');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.exist;
        });
    });

    describe('Week Start Selection', () => {
        it('should accept sunday as week start', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            setInputValue('#week-start', 'sunday');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls[0].args.weekStart).to.equal('sunday');
        });

        it('should accept monday as week start', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            setInputValue('#week-start', 'monday');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls[0].args.weekStart).to.equal('monday');
        });
    });

    describe('Aggregation Execution', () => {
        it('should call backend with correct parameters', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            setInputValue('#reference-date', '2024-11-20');
            setInputValue('#week-start', 'sunday');
            setInputValue('#group-by', 'category2');
            
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls.length).to.equal(1);
            expect(calls[0].cmd).to.equal('get_weekly_aggregation_by_date');
            expect(calls[0].args.referenceDate).to.equal('2024-11-20');
            expect(calls[0].args.weekStart).to.equal('sunday');
            expect(calls[0].args.groupBy).to.equal('category2');
        });

        it('should display results after execution', async () => {
            window.__TAURI__.core.invoke = async () => [
                { group_key: 'EXPENSE', group_name: 'Expense', total_amount: -30000, count: 20, avg_amount: -1500 }
            ];
            
            setInputValue('#reference-date', '2024-11-20');
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const tableData = getTableData('#results-table');
            expect(tableData.length).to.equal(1);
            expect(tableData[0].group_name).to.equal('Expense');
        });

        it('should handle empty results', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const resultsTable = document.querySelector('#results-table');
            const tbody = resultsTable?.querySelector('tbody');
            expect(tbody?.children.length).to.equal(0);
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

    describe('Week Range Calculation', () => {
        it('should calculate week range for Monday start', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                // Backend should handle calculation
                return [];
            };
            
            // Thursday, Nov 20, 2024
            setInputValue('#reference-date', '2024-11-20');
            setInputValue('#week-start', 'monday');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            // Backend receives reference date, not calculated range
            expect(calls[0].args.referenceDate).to.equal('2024-11-20');
            expect(calls[0].args.weekStart).to.equal('monday');
        });

        it('should calculate week range for Sunday start', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            setInputValue('#reference-date', '2024-11-20');
            setInputValue('#week-start', 'sunday');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls[0].args.referenceDate).to.equal('2024-11-20');
            expect(calls[0].args.weekStart).to.equal('sunday');
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

    describe('Different Days of Week', () => {
        it('should handle Monday reference date', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            // Monday, Nov 18, 2024
            setInputValue('#reference-date', '2024-11-18');
            setInputValue('#week-start', 'monday');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.not.exist.or.not.be.visible;
        });

        it('should handle Sunday reference date', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            // Sunday, Nov 17, 2024
            setInputValue('#reference-date', '2024-11-17');
            setInputValue('#week-start', 'sunday');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.not.exist.or.not.be.visible;
        });

        it('should handle Saturday reference date', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            // Saturday, Nov 23, 2024
            setInputValue('#reference-date', '2024-11-23');
            setInputValue('#week-start', 'monday');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.not.exist.or.not.be.visible;
        });
    });
});
