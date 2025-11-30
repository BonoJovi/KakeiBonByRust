import { describe, it, expect, beforeEach, afterEach } from './node_modules/mocha/index.js';
import {
    createMockInvoke,
    setInputValue,
    clickButton,
    getTableData,
    isVisible,
    waitFor
} from './aggregation-test-helpers.js';

describe('Daily Aggregation Tests', () => {
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
        it('should display date input with default today', () => {
            const dateInput = document.querySelector('#date');
            expect(dateInput).to.exist;
            
            const today = new Date().toISOString().split('T')[0];
            expect(dateInput.value).to.equal(today);
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

    describe('Date Input Validation', () => {
        it('should accept valid date', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            setInputValue('#date', '2024-11-20');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.not.exist.or.not.be.visible;
        });

        it('should reject future date', async () => {
            const tomorrow = new Date();
            tomorrow.setDate(tomorrow.getDate() + 1);
            const futureDate = tomorrow.toISOString().split('T')[0];
            
            setInputValue('#date', futureDate);
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.exist;
            expect(errorMsg.textContent).to.match(/future/i);
        });

        it('should accept today', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            const today = new Date().toISOString().split('T')[0];
            setInputValue('#date', today);
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.not.exist.or.not.be.visible;
        });

        it('should reject invalid date format', async () => {
            setInputValue('#date', '2024/11/20');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.exist;
        });

        it('should reject empty date', async () => {
            setInputValue('#date', '');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).to.exist;
        });
    });

    describe('Aggregation Execution', () => {
        it('should call backend with correct parameters', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            setInputValue('#date', '2024-11-20');
            setInputValue('#group-by', 'category2');
            
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls.length).to.equal(1);
            expect(calls[0].cmd).to.equal('get_daily_aggregation');
            expect(calls[0].args.date).to.equal('2024-11-20');
            expect(calls[0].args.groupBy).to.equal('category2');
        });

        it('should display results after execution', async () => {
            window.__TAURI__.core.invoke = async () => [
                { group_key: 'EXPENSE', group_name: 'Expense', total_amount: -5000, count: 3, avg_amount: -1667 }
            ];
            
            setInputValue('#date', '2024-11-20');
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

    describe('Enter Key Execution', () => {
        it('should execute on Enter key in date field', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            const dateInput = document.querySelector('#date');
            setInputValue('#date', '2024-11-20');
            
            const enterEvent = new KeyboardEvent('keypress', { key: 'Enter', keyCode: 13 });
            dateInput.dispatchEvent(enterEvent);
            
            await waitFor(100);
            
            expect(calls.length).to.equal(1);
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
