/**
 * Which account a category needs and is left "Unspecified" (NONE).
 *
 * An EXPENSE needs a From account, an INCOME a To account and a TRANSFER
 * both. With that side left as NONE the amount counts as an expense or
 * income but moves no account balance, since the dashboard hides NONE.
 * Mirrors `missing_required_account` in src/services/transaction.rs, which
 * refuses the same with the `account_required` code.
 *
 * @param {string} category1Code
 * @param {string} fromAccountCode
 * @param {string} toAccountCode
 * @returns {{ from: boolean, to: boolean }} the sides that are missing
 */
export function missingRequiredAccounts(category1Code, fromAccountCode, toAccountCode) {
    const needsFrom = category1Code === 'EXPENSE' || category1Code === 'TRANSFER';
    const needsTo = category1Code === 'INCOME' || category1Code === 'TRANSFER';
    return {
        from: needsFrom && fromAccountCode === 'NONE',
        to: needsTo && toAccountCode === 'NONE',
    };
}
