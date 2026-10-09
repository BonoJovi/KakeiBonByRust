import {
    createMockInvoke,
    setInputValue,
    clickButton,
    getTableData,
    isVisible,
    waitFor,
    setupPeriodAggregationDOM
} from './aggregation-test-helpers.js';

describe('Period Aggregation Tests', () => {
    let originalInvoke;
    let mockInvoke;

    beforeEach(() => {
        // Setup DOM
        setupPeriodAggregationDOM();
        
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
        it('should show the first day of the current month in the start date when the screen opens', () => {
            const startDateInput = document.querySelector('#start-date');
            expect(startDateInput).toBeTruthy();
            
            const now = new Date();
            const firstDay = new Date(now.getFullYear(), now.getMonth(), 1);
            const expectedDate = firstDay.toISOString().split('T')[0];
            expect(startDateInput.value).toBe(expectedDate);
        });

        it('should show today in the end date when the screen opens', () => {
            const endDateInput = document.querySelector('#end-date');
            expect(endDateInput).toBeTruthy();
            
            const today = new Date().toISOString().split('T')[0];
            expect(endDateInput.value).toBe(today);
        });

        it('should show the group-by select set to category1 when the screen opens', () => {
            const groupBySelect = document.querySelector('#group-by');
            expect(groupBySelect).toBeTruthy();
            expect(groupBySelect.value).toBe('category1');
        });
    });

    describe('Date Range Validation', () => {
        it('should show no error when the start date is before the end date', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            setInputValue('#start-date', '2024-11-01');
            setInputValue('#end-date', '2024-11-20');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg.style.display).toBe('none');
        });

        it('should reject when start date is after end date', async () => {
            setInputValue('#start-date', '2024-11-20');
            setInputValue('#end-date', '2024-11-01');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).toBeTruthy();
            expect(errorMsg.textContent).toMatch(/start.*end|range/i);
        });

        it('should show an error when the start date is in the future', async () => {
            const tomorrow = new Date();
            tomorrow.setDate(tomorrow.getDate() + 1);
            const futureDate = tomorrow.toISOString().split('T')[0];
            
            setInputValue('#start-date', futureDate);
            setInputValue('#end-date', futureDate);
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).toBeTruthy();
            expect(errorMsg.textContent).toMatch(/future/i);
        });

        it('should show an error when the end date is in the future', async () => {
            const tomorrow = new Date();
            tomorrow.setDate(tomorrow.getDate() + 1);
            const futureDate = tomorrow.toISOString().split('T')[0];
            
            setInputValue('#start-date', '2024-11-01');
            setInputValue('#end-date', futureDate);
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).toBeTruthy();
            expect(errorMsg.textContent).toMatch(/future/i);
        });

        it('should show no error when the start and end dates are the same', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            const today = new Date().toISOString().split('T')[0];
            setInputValue('#start-date', today);
            setInputValue('#end-date', today);
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg.style.display).toBe('none');
        });

        it('should show an error when the start date is empty', async () => {
            setInputValue('#start-date', '');
            setInputValue('#end-date', '2024-11-20');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).toBeTruthy();
        });

        it('should show an error when the end date is empty', async () => {
            setInputValue('#start-date', '2024-11-01');
            setInputValue('#end-date', '');
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
            
            setInputValue('#start-date', '2024-10-01');
            setInputValue('#end-date', '2024-11-20');
            setInputValue('#group-by', 'category2');
            
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls.length).toBe(1);
            expect(calls[0].cmd).toBe('get_period_aggregation');
            expect(calls[0].args.startDate).toBe('2024-10-01');
            expect(calls[0].args.endDate).toBe('2024-11-20');
            expect(calls[0].args.groupBy).toBe('category2');
        });

        it('should show the result rows when the aggregation returns data', async () => {
            window.__TAURI__.core.invoke = async () => [
                { group_key: 'EXPENSE', group_name: 'Expense', total_amount: -80000, count: 50, avg_amount: -1600 }
            ];
            
            setInputValue('#start-date', '2024-10-01');
            setInputValue('#end-date', '2024-11-20');
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

    describe('Common Use Cases', () => {
        it('should show no error when the period is one week', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            setInputValue('#start-date', '2024-11-14');
            setInputValue('#end-date', '2024-11-20');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg.style.display).toBe('none');
        });

        it('should show no error when the period is one month', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            setInputValue('#start-date', '2024-10-01');
            setInputValue('#end-date', '2024-10-31');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg.style.display).toBe('none');
        });

        it('should show no error when the period is a quarter', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            setInputValue('#start-date', '2024-10-01');
            setInputValue('#end-date', '2024-12-31');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg.style.display).toBe('none');
        });

        it('should show no error when the period is 5 days', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            setInputValue('#start-date', '2024-11-01');
            setInputValue('#end-date', '2024-11-05');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg.style.display).toBe('none');
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

    describe('Boundary Cases', () => {
        it('should show no error when the period crosses a year boundary', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            setInputValue('#start-date', '2023-12-20');
            setInputValue('#end-date', '2024-01-10');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg.style.display).toBe('none');
        });

        it('should show no error when the period is February of a leap year', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            setInputValue('#start-date', '2024-02-01');
            setInputValue('#end-date', '2024-02-29');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg.style.display).toBe('none');
        });
    });
});
