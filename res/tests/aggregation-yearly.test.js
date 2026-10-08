import {
    createMockInvoke,
    setInputValue,
    clickButton,
    getTableData,
    isVisible,
    waitFor,
    setupYearlyAggregationDOM
} from './aggregation-test-helpers.js';

describe('Yearly Aggregation Tests', () => {
    let originalInvoke;
    let mockInvoke;

    beforeEach(() => {
        // Setup DOM
        setupYearlyAggregationDOM();
        
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
        it('should show the current year in the year input when the screen opens', () => {
            const yearInput = document.querySelector('#year');
            expect(yearInput).toBeTruthy();
            
            const currentYear = new Date().getFullYear();
            expect(parseInt(yearInput.value)).toBe(currentYear);
        });

        it('should show January in the year start month select when the screen opens', () => {
            const yearStartSelect = document.querySelector('#year-start');
            expect(yearStartSelect).toBeTruthy();
            expect(parseInt(yearStartSelect.value)).toBe(1);
        });

        it('should show the group-by select set to category1 when the screen opens', () => {
            const groupBySelect = document.querySelector('#group-by');
            expect(groupBySelect).toBeTruthy();
            expect(groupBySelect.value).toBe('category1');
        });
    });

    describe('Year Input Validation', () => {
        it('should show no error when the year is 2024', async () => {
            window.__TAURI__.core.invoke = async () => [];
            
            setInputValue('#year', '2024');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg.style.display).toBe('none');
        });

        it('should show an error when the year is below 1900', async () => {
            setInputValue('#year', '1899');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).toBeTruthy();
        });

        it('should show an error when the year is above 2100', async () => {
            setInputValue('#year', '2101');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            const errorMsg = document.querySelector('.message.error');
            expect(errorMsg).toBeTruthy();
        });
    });

    describe('Year Spinner Buttons', () => {
        it('should add one year when the up button is clicked', () => {
            setInputValue('#year', '2024');
            clickButton('#year-up');
            
            const year = parseInt(document.querySelector('#year').value);
            expect(year).toBe(2025);
        });

        it('should subtract one year when the down button is clicked', () => {
            setInputValue('#year', '2024');
            clickButton('#year-down');
            
            const year = parseInt(document.querySelector('#year').value);
            expect(year).toBe(2023);
        });

        it('should stay at 1900 when the down button is clicked at 1900', () => {
            setInputValue('#year', '1900');
            clickButton('#year-down');
            
            const year = parseInt(document.querySelector('#year').value);
            expect(year).toBe(1900);
        });

        it('should stay at 2100 when the up button is clicked at 2100', () => {
            setInputValue('#year', '2100');
            clickButton('#year-up');
            
            const year = parseInt(document.querySelector('#year').value);
            expect(year).toBe(2100);
        });
    });

    describe('Year Start Month Selection', () => {
        it('should send yearStartMonth 1 when the year starts in January (calendar year)', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };

            // Use a past year to avoid "future date" validation errors
            setInputValue('#year', '2024');
            setInputValue('#year-start', '1');
            clickButton('#execute-btn');

            await waitFor(100);

            expect(calls[0].args.yearStartMonth).toBe(1);
        });

        it('should send yearStartMonth 4 when the year starts in April (fiscal year)', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };

            // Use a past year to avoid "future date" validation errors
            setInputValue('#year', '2024');
            setInputValue('#year-start', '4');
            clickButton('#execute-btn');

            await waitFor(100);

            expect(calls[0].args.yearStartMonth).toBe(4);
        });
    });

    describe('Aggregation Execution', () => {
        it('should send the form values to the backend when a calendar year is aggregated', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            setInputValue('#year', '2024');
            setInputValue('#year-start', '1');
            setInputValue('#group-by', 'category2');
            
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls[0].cmd).toBe('get_yearly_aggregation');
            expect(calls[0].args.year).toBe(2024);
            expect(calls[0].args.yearStartMonth).toBe(1);
            expect(calls[0].args.groupBy).toBe('category2');
        });

        it('should send the form values to the backend when a fiscal year is aggregated', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                return [];
            };
            
            setInputValue('#year', '2024');
            setInputValue('#year-start', '4');
            setInputValue('#group-by', 'account');
            
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls[0].args.year).toBe(2024);
            expect(calls[0].args.yearStartMonth).toBe(4);
        });

        it('should show the result rows when the aggregation returns data', async () => {
            window.__TAURI__.core.invoke = async () => [
                { group_key: 'EXPENSE', group_name: 'Expense', total_amount: -1200000, count: 365, avg_amount: -3288 },
                { group_key: 'INCOME', group_name: 'Income', total_amount: 3600000, count: 12, avg_amount: 300000 }
            ];
            
            clickButton('#execute-btn');
            
            await waitFor(200);
            
            const tableData = getTableData('#results-table');
            expect(tableData.length).toBe(2);
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

    describe('Fiscal Year Periods', () => {
        it('should send year 2024 and yearStartMonth 4 when FY2024 (Apr 2024 - Mar 2025) is aggregated', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                // Backend calculates: 2024-04-01 to 2025-03-31
                return [];
            };
            
            setInputValue('#year', '2024');
            setInputValue('#year-start', '4');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls[0].args.year).toBe(2024);
            expect(calls[0].args.yearStartMonth).toBe(4);
        });

        it('should send year 2024 and yearStartMonth 1 when calendar year 2024 is aggregated', async () => {
            const calls = [];
            window.__TAURI__.core.invoke = async (cmd, args) => {
                calls.push({ cmd, args });
                // Backend calculates: 2024-01-01 to 2024-12-31
                return [];
            };
            
            setInputValue('#year', '2024');
            setInputValue('#year-start', '1');
            clickButton('#execute-btn');
            
            await waitFor(100);
            
            expect(calls[0].args.year).toBe(2024);
            expect(calls[0].args.yearStartMonth).toBe(1);
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
