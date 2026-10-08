import {
    createMockInvoke,
    setInputValue,
    clickButton,
    getTableData,
    isVisible,
    waitFor,
    setupWeeklyAggregationDOM
} from './aggregation-test-helpers.js';

describe('Weekly Aggregation Tests', () => {
    let originalInvoke;
    let mockInvoke;

    beforeEach(() => {
        // Setup DOM
        setupWeeklyAggregationDOM();
        
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
        it('should show today in the reference date input when the screen opens', () => {
            const dateInput = document.querySelector('#reference-date');
            expect(dateInput).toBeTruthy();
            
            const today = new Date().toISOString().split('T')[0];
            expect(dateInput.value).toBe(today);
        });

        it('should show the week start select set to Sunday when the screen opens', () => {
            const weekStartSelect = document.querySelector('#week-start');
            expect(weekStartSelect).toBeTruthy();
            expect(weekStartSelect.value).toBe('0'); // Sunday is default (0)
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

    describe('Reference Date Validation', () => {
        it('should show no error when the date is a past date', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            setInputValue('#reference-date', '2024-11-20');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg.style.display).toBe('none');
        });

        it('should show an error when the date is in the future', async () => {
            const tomorrow = new Date();
            tomorrow.setDate(tomorrow.getDate() + 1);
            const futureDate = tomorrow.toISOString().split('T')[0];
            
            setInputValue('#reference-date', futureDate);
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).toBeTruthy();
            expect(errorMsg.textContent).toMatch(/future/i);
        });

        it('should show no error when the date is today', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            const today = new Date().toISOString().split('T')[0];
            setInputValue('#reference-date', today);
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg.style.display).toBe('none');
        });

        it('should show an error when the date is empty', async () => {
            setInputValue('#reference-date', '');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).toBeTruthy();
        });
    });

    describe('Week Start Selection', () => {
        it('should send weekStart 0 when the week starts on Sunday', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            setInputValue('#week-start', '0');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls[0].args.weekStart).toBe(0);
        });

        it('should send weekStart 1 when the week starts on Monday', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            setInputValue('#week-start', '1');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls[0].args.weekStart).toBe(1);
        });
    });

    describe('Aggregation Execution', () => {
        it('should send the form values to the backend when Execute is pressed', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            setInputValue('#reference-date', '2024-11-20');
            setInputValue('#week-start', '0');
            setInputValue('#group-by', 'category2');
            
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls.length).toBe(1);
            expect(calls[0].cmd).toBe('get_weekly_aggregation');
            expect(calls[0].args.referenceDate).toBe('2024-11-20');
            expect(calls[0].args.weekStart).toBe(0);
            expect(calls[0].args.groupBy).toBe('category2');
        });

        it('should show the result rows when the aggregation returns data', async () => {
            window.__TAURI__.core.invoke = async () => [
                { group_key: 'EXPENSE', group_name: 'Expense', total_amount: -30000, count: 20, avg_amount: -1500 }
            ];
            
            setInputValue('#reference-date', '2024-11-20');
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const tableData = getTableData('#results-table');
            expect(tableData.length).toBe(1);
            expect(tableData[0][0]).toBe('Expense');
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

    describe('Week Range Calculation', () => {
        it('should send the reference date and weekStart 1 when a Monday-start week is aggregated', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                // Backend should handle calculation
                return [];
            };
            
            // Thursday, Nov 20, 2024
            setInputValue('#reference-date', '2024-11-20');
            setInputValue('#week-start', '1');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            // Backend receives reference date, not calculated range
            expect(calls[0].args.referenceDate).toBe('2024-11-20');
            expect(calls[0].args.weekStart).toBe(1);
        });

        it('should send the reference date and weekStart 0 when a Sunday-start week is aggregated', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            setInputValue('#reference-date', '2024-11-20');
            setInputValue('#week-start', '0');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls[0].args.referenceDate).toBe('2024-11-20');
            expect(calls[0].args.weekStart).toBe(0);
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

    describe('Different Days of Week', () => {
        it('should show no error when the reference date is a Monday', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            // Monday, Nov 18, 2024
            setInputValue('#reference-date', '2024-11-18');
            setInputValue('#week-start', '1');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg.style.display).toBe('none');
        });

        it('should show no error when the reference date is a Sunday', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            // Sunday, Nov 17, 2024
            setInputValue('#reference-date', '2024-11-17');
            setInputValue('#week-start', '0');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg.style.display).toBe('none');
        });

        it('should show no error when the reference date is a Saturday', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            // Saturday, Nov 23, 2024
            setInputValue('#reference-date', '2024-11-23');
            setInputValue('#week-start', '1');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg.style.display).toBe('none');
        });
    });
});
