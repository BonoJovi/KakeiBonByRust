import {
    createMockInvoke,
    setInputValue,
    clickButton,
    getTableData,
    isVisible,
    waitFor,
    setupDailyAggregationDOM
} from './aggregation-test-helpers.js';

describe('Daily Aggregation Tests', () => {
    let originalInvoke;
    let mockInvoke;

    beforeEach(() => {
        // Setup DOM
        setupDailyAggregationDOM();
        
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
        it('should show today in the date input when the screen opens', () => {
            const dateInput = document.querySelector('#date');
            expect(dateInput).toBeTruthy();
            
            const today = new Date().toISOString().split('T')[0];
            expect(dateInput.value).toBe(today);
        });

        it('should show the group-by select set to category1 when the screen opens', () => {
            const groupBySelect = document.querySelector('#group-by');
            expect(groupBySelect).toBeTruthy();
            expect(groupBySelect.value).toBe('category1');
        });

        it('should show the Execute button when the screen opens', () => {
            const executeBtn = document.querySelector('#execute-btn');
            expect(executeBtn).toBeTruthy();
        });
    });

    describe('Date Input Validation', () => {
        it('should show no error when the date is a past date', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            setInputValue('#date', '2024-11-20');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg.style.display).toBe('none');
        });

        it('should show an error when the date is in the future', async () => {
            const tomorrow = new Date();
            tomorrow.setDate(tomorrow.getDate() + 1);
            const futureDate = tomorrow.toISOString().split('T')[0];
            
            setInputValue('#date', futureDate);
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).toBeTruthy();
            expect(errorMsg.textContent).toMatch(/future/i);
        });

        it('should show no error when the date is today', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            const today = new Date().toISOString().split('T')[0];
            setInputValue('#date', today);
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg.style.display).toBe('none');
        });

        it('should show an error when the date uses slashes', async () => {
            setInputValue('#date', '2024/11/20');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).toBeTruthy();
        });

        it('should show an error when the date is empty', async () => {
            setInputValue('#date', '');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).toBeTruthy();
        });
    });

    describe('Aggregation Execution', () => {
        it('should send the form values to the backend when Execute is pressed', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            setInputValue('#date', '2024-11-20');
            setInputValue('#group-by', 'category2');
            
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls.length).toBe(1);
            expect(calls[0].cmd).toBe('get_daily_aggregation');
            expect(calls[0].args.date).toBe('2024-11-20');
            expect(calls[0].args.groupBy).toBe('category2');
        });

        it('should show the result rows when the aggregation returns data', async () => {
            window.__TAURI__.core.invoke = async () => [
                { group_key: 'EXPENSE', group_name: 'Expense', total_amount: -5000, count: 3, avg_amount: -1667 }
            ];
            
            setInputValue('#date', '2024-11-20');
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const tableData = getTableData('#results-table');
            expect(tableData.length).toBe(1);
            expect(tableData[0][0]).toBe('Expense'); // First column of first row
        });

        it('should show an empty table when the aggregation returns no rows', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const resultsTable = document.querySelector('#results-table');
            const tbody = resultsTable?.querySelector('tbody');
            expect(tbody?.children.length).toBe(0);
        });

        it('should show an error when the backend fails', async () => {
            window.__TAURI__.core.invoke = async () => {
                throw new Error('Failed to fetch data');
            };
            
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).toBeTruthy();
        });
    });

    describe('Enter Key Execution', () => {
        it('should run the aggregation when Enter is pressed in the date field', async () => {
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
            
            expect(calls.length).toBe(1);
        });
    });

    describe('Grouping Axis Changes', () => {
        it('should send each grouping axis when it is selected', async () => {
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
                
                expect(calls[0].args.groupBy).toBe(grouping);
            }
        });
    });

    describe('Account Note Display', () => {
        it('should show note when account grouping is selected', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            setInputValue('#group-by', 'account');
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            expect(isVisible('#account-note')).toBe(true);
        });

        it('should hide note when category grouping is selected', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            setInputValue('#group-by', 'category1');
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            expect(isVisible('#account-note')).toBe(false);
        });
    });
});
