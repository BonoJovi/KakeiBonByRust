# Frontend Test Index

This document provides a complete index of all frontend tests implemented in JavaScript.

**Last Updated**: 2026-10-09 JST  
**Total Tests**: 958 (jest suites; 93 test files, per `npm test`)

---

For detailed Japanese version with all test cases, see [Japanese Frontend Test Index](../ja/FRONTEND_TEST_INDEX.md).

## Quick Reference

### Common Test Suites (56 tests — helper libraries)

Helper functions invoked from screen tests. Their assertions are counted in
the Screen-Specific totals below; they are listed here for discoverability
and are **not** added again to the grand total.

- **password-validation-tests.js** - Password validation tests (26)
- **username-validation-tests.js** - Username validation tests (20)
- **user-edit-validation-tests.js** - User edit validation tests (23)
- **validation-helpers.js** - Common validation functions

### Screen-Specific Tests (305 tests)
- **admin-setup.test.js** - Admin setup tests (32)
- **user-addition.test.js** - User addition tests (46)
- **admin-edit.test.js** - Admin edit tests (62)
- **general-user-edit.test.js** - General user edit tests (62)
- **login.test.js** - Login tests (58)
- **user-deletion.test.js** - User deletion tests (45)

### Feature-Specific Tests (538 tests)

- **transaction-edit.test.js** - Transaction edit tests (111)
- **transaction-detail-management.test.js** - Transaction detail management tests (51)
- **transaction-detail-tax-calculation.test.js** - Tax calculation tests; a typed tax-included price is always kept, with the tax carved out of it when no exact tax-excluded split exists (latent-audit scan2-T2) (30)
- **toast.test.js** - Toast notification tests (14)
- **tax-calc.test.js** - Tax calculation utility tests (12)
- **pages/transaction-detail-draft-storage.test.js** - Imports the real detail page module and calls its exported `persistDraft` / `consumeDraft` / `clearDraft` (the detail draft kept in sessionStorage for the detail → product master round trip): a stored draft reads back unchanged, nothing stored reads as null, malformed JSON reads as null and is removed, clear removes the draft, a second persist overwrites the first, and an edit-mode draft keeps `detail_id` and `selected_product_id` (6)
- **pages/transaction-detail-product-link.test.js** - Boots the real detail page: picking a product suggestion sets the hidden product id and picking another replaces it; typing after a pick drops the id, so the detail is saved with `productId` null; editing a product-linked detail restores its id, a free-text detail has none, and a row without a `product_id` field is saved with `productId` null; reopening the window clears the id and the list; the list shows the newest answer with no item active (Enter picks nothing); an answer that arrives after the window was reopened is not shown (9)
- **pages/product-management-product-draft.test.js** - Boots the real product master page: "Open in manufacturer master" saves the window inputs as the product draft, with the transaction to return to from `?return_to=` (null when opened from the menu), and a second jump overwrites the draft; coming back with `?restore_product=1` restores the inputs, removes the draft and shows "Back to detail entry" only when the draft has a transaction to return to; a malformed draft is discarded and no draft opens no window (7)
- **pages/manufacturer-management-product-draft.test.js** - Boots the real manufacturer master page with `?return_to_product=1`: after adding a manufacturer, its id from the reloaded list is written into the product draft as a string, replacing an earlier pick and keeping every other field; without a draft, or when the name is not in the reloaded list, the draft is left as it is; opened from the menu, the draft is not touched (6)
- **modal-double-submit.test.js** - Shared `Modal._handleSave` re-entrancy guard + unhandled-rejection swallow tests (6)
- **modal-stale-save-close.test.js** - Shared `Modal`: a save that finishes after the modal was closed and re-opened does not close or reset the re-opened session, and the re-opened modal can save while the earlier save is still pending (latent-audit L22) (2)
- **master-crud.test.js** - Shared `saveMasterEntry` + `mapMasterErrorCode` + `formatApiError` (Fable-5 #D3/#D4/#23) tests (30)
- **attach-char-counter-ime.test.js** - `attachCharCounter` baseline + IME composition guard (Fable-5 #D1) tests (8)
- **aggregation-error-translate.test.js** - `translateAggregationError` shape-guard tests (Fable-5 #9): coerces `Err(String)` / `ApiError { code, message }` / `Error` to a substring-matchable string before routing to i18n keys, and swaps unusable coerced values (`"[object Object]"`, `"null"`, `"undefined"`, `""`) for the localised generic banner, so the aggregation banner never renders those literals (13)
- **aggregation-latest-request.test.js** - `createLatestRequestGuard` (aggregation-common.js): only the most recent request is latest, and each guard counts its own requests (latent-audit scan2-A4) (2)
- **parse-amount-strict.test.js** - `parseAmountStrict` money-field strict integer parser (Fable-5 #10): replaces `parseInt(el.value) || 0` across the detail / transaction / recurring-rule submit paths, rejecting decimals, locale commas, scientific notation, sign prefixes, full-width digits, trailing garbage, and precision-losing integers past `Number.MAX_SAFE_INTEGER` (24)
- **format-local-date.test.js** - `formatLocalDate` timezone-safe `YYYY-MM-DD` formatter (Fable-5 #13): replaces `new Date().toISOString().slice(0, 10)` in the recurring-rule modal so JST users opening the form before 09:00 no longer see yesterday's date in start-date / end-date / anchor-date defaults; test file pins `TZ=Asia/Tokyo` so a UTC regression is actually caught; `formatUtcAsLocalDateTime` shows a stored UTC timestamp (`YYYY-MM-DD HH:MM:SS`) in local time, with the `T` separator, text after the timestamp, non-timestamp and empty cases (22)
- **period-end-date.test.js** - `fetchMonthlyPeriodEndDate` (period.js): the dashboard account balances are taken as of the last day of the user's monthly period (custom start day / holiday shift from `get_monthly_period_bounds`), falling back to the calendar month end if the backend fails (latent-audit L14) (2)
- **period-containing.test.js** - `findMonthlyPeriodContaining` / `findYearlyPeriodContaining` (period.js). Yearly: the year containing a date for starts 01-01, 04-01, 12-31 and 02-31 (month end), on both sides of the start. Monthly: returns the calendar month when its period contains the date, the previous month when that period starts later (start day 25), the next month when it ended earlier (holiday shift back over the month end), keeps stepping when the period is two months away (start day 31 + next business day), wraps the year both ways, and falls back to the calendar month when the backend fails or no period ever matches (latent-audit scan2-A3) (16)
- **aggregation-render-unspecified.test.js** - `renderResults` unspecified-group i18n swap (Fable-5 #22): backend now returns an empty `group_name` string when the SHOP / PRODUCT / ACCOUNT reference is unspecified, and the renderer swaps it for `i18n.t('common.unspecified')` so English users don't see Japanese "指定なし" leaking through (5)
- **pages/transaction-detail-page.test.js** - Boots the real detail page against `transaction-detail-management.html`: opening a product-linked detail and saving without changes keeps its `productId` (latent-audit H3), a double submit adds the detail only once (latent-audit M19), and a legacy row with `amount_including_tax = 0` shows AMOUNT + TAX_AMOUNT instead of ¥0 (latent-audit L7) (3)
- **pages/transaction-detail-included-typing.test.js** - Boots the real detail page: typing a tax-included amount digit by digit keeps every digit (the field is no longer rewritten mid-typing) and saves the typed price (latent-audit scan2-T1) (1)
- **pages/transaction-detail-unreachable-included-price.test.js** - Boots the real detail page: a tax-included price with no exact tax-excluded split (1000 at 10 % floor) is kept as typed and saved as 909 + 91 (latent-audit scan2-T2) (1)
- **pages/transaction-detail-hidden-category.test.js** - Boots the real detail page: editing a detail whose category2 (with its category3) or category3 alone is hidden keeps the category on a memo-only save, the hidden entry being offered with `common.disabled_label` (latent-audit scan2-T3) (2)
- **pages/transaction-detail-product-suggest.test.js** - Boots the real detail page: the medium and minor categories come before the item name; the item name field has a short placeholder and a tooltip hint; focusing the empty field lists product suggestions with the detail's categories (`category1Code` / `category2Code` / `category3Code`); no list appears when focus has already left the field; typing while the focus search is pending does not show its stale answer; picking a suggestion closes the list for good (6)
- **pages/transaction-management-page.test.js** - Boots the real transaction page against `transaction-management.html`: saving a header without details shows no ¥0 recalc prompt and completes the save flow (latent-audit H4); a blank transaction date is rejected with `validation.required` before sending (latent-audit L8); deleting the only row on the last page moves back to the last page, and a late older page response does not overwrite a newer one (latent-audit L5) (4)
- **pages/user-management-page.test.js** - Boots the real user management page (admin session): a whitespace-only username is rejected with the required-field message before `create_general_user`, while a normal name still reaches it (latent-audit M13); an admin sees the Add User button and its footer (latent-audit L30); created and updated times are shown in local time, not as stored UTC (4)
- **pages/user-management-delete-last-user.test.js** - Boots the real user management page (admin session): when the backend refuses to delete the last general user (`last_general_user`), the screen shows `user_mgmt.last_general_user` instead of the generic failure with the English backend text (latent-audit scan2-C5) (1)
- **pages/index-logout-hides-user-setup.test.js** - Boots the real index page (menu.js): logging out while the user-setup form is shown, before the 1 s switch after login has run, or before the login's setup check has answered, leaves only the login form on screen (`#user-setup`, `#admin-setup` and `#app-content` hidden) (latent-audit scan2-C6) (3)
- **pages/recurring-rule-page.test.js** - Boots the real recurring rule page: a TRANSFER template from an account to itself is rejected with `transaction_mgmt.transfer_same_account` before `create_recurring_rule`, while two different accounts still go through, and a backend `transfer_same_account` rejection shows the same message (latent-audit M16) ; a backend `recurring_holiday_shift_too_long` rejection shows `recurring_rule.holiday_shift_too_long` (4)
- **pages/recurring-rule-double-submit.test.js** - Boots the real recurring rule page: a double submit while `create_recurring_rule` is in flight invokes it only once (latent-audit M19) (1)
- **pages/recurring-rule-period-range.test.js** - Boots the real recurring rule page: the start / end date pickers are bounded to the seeded holiday years, an out-of-range period is stopped before `create_recurring_rule` with `recurring_rule.period_out_of_range`, a backend `recurring_period_out_of_range` rejection shows the same message, and bounds that changed since the page loaded are applied to the pickers on submit (latent-audit M15 / M18) (5)
- **pages/recurring-rule-anchor-follows-start.test.js** - Boots the real recurring rule page: the daily anchor starts as the start date and follows it until the user edits the anchor, and follows again after Reset (latent-audit scan2-R2) (2)
- **pages/product-management-edit-manufacturer-roundtrip.test.js** - Boots the real product page: editing a product, jumping to the manufacturer master and coming back reopens the same product in edit mode, and saving calls `update_product` for it, never `add_product` (latent-audit scan2-M2) (1)
- **pages/recurring-rule-derived-total.test.js** - Boots the real recurring rule page: the total field is read-only and follows the detail and the header's rounding / tax-included settings, and `create_recurring_rule` receives no typed total (latent-audit M17) (2)
- **pages/recurring-rule-cycle-options.test.js** - Boots the real recurring rule page: the day-of-month mode sends `DAY_OR_END` and the new end-of-month mode sends `END` (latent-audit M14); choosing "daily" resets the holiday shift to "no shift" and disables it (latent-audit L13) (3)
- **pages/recurring-rule-date-order.test.js** - Boots the real recurring rule page: an empty start / end date, a start date after the end date and a daily anchor after the end date are each stopped by the form with a localized message instead of the backend's English text, and `create_recurring_rule` is not called; dates in order still create the rule (latent-audit scan2-R8) (5)
- **pages/recurring-rule-reset.test.js** - Boots the real recurring rule page: after choosing Monthly and pressing Reset, the cycle fields match the checked Daily radio again (anchor shown, Monthly fields hidden, holiday shift "no shift" and disabled) and the default dates (today / one year later) are re-applied (latent-audit scan2-R4) (1)
- **pages/recurring-rule-rounding-recalc.test.js** - Boots the real recurring rule page: changing the tax rounding after the amount recomputes the detail's tax fields (tax excluded: 105 at 10 % becomes 11 / 116 under ceil; tax included: the typed 116 stays and the tax-excluded amount becomes 105), and the request sent to `create_recurring_rule` carries them (latent-audit scan2-R5) (2)
- **pages/menu-i18n-seed.test.js** - `menu.back_to_transactions` and every other `data-i18n` key rendered by the menu bar are seeded in `res/sql/dbaccess.sql` for ja and en, so the detail screen's File menu no longer shows the raw key (latent-audit scan2-C2) (2)
- **pages/i18n-literal-user-text.test.js** - User text is inserted literally: `i18n.t()` keeps `$&`, `$'`, `` $` `` and `$$` in a user name, fills every placeholder in one pass (a value containing `{b}` is not filled again), leaves a placeholder with no param as it is, and the recurring-rule delete confirmation keeps a rule name containing `$'` and `{1}` (latent-audit scan2-C4) (7)
- **pages/dashboard-balance-header.test.js** - The dashboard's Account Balances column header uses its own key that resolves to 残高 / Balance after `dbaccess.sql` is applied, instead of `dashboard.balance` (収支, the chart label) (latent-audit scan2-C3) (1)
- **pages/transaction-list-none-account-label.test.js** - Boots the real transaction list: a row whose account is NONE shows `common.unspecified` instead of the stored name 指定なし (latent-audit scan2-M8) (1)
- **pages/transaction-detail-none-account-label.test.js** - Boots the real transaction detail screen: the header's account shows `common.unspecified` for the NONE account instead of the stored name 指定なし (latent-audit scan2-M8) (1)
- **pages/dashboard-balance-sign.test.js** - Boots the real dashboard: the trend chart's Balance tooltip keeps the minus sign ("-¥30,000"), the axis ticks show the full signed amount without K / M abbreviations, and the account balances read "-¥1,234" (latent-audit scan2-A2) (3)
- **pages/dashboard-default-period.test.js** - Boots the real dashboard with start day 25 on 2026-09-10: it opens on the August period that contains today, not the future September period, and loads that month (latent-audit scan2-A3) (1)
- **pages/dashboard-stale-reload.test.js** - Boots the real dashboard: when a slow September load finishes after a newer March load, the charts and titles stay on March (latent-audit scan2-A4) (1)
- **single-flight.test.js** - `singleFlight` submit guard (latent-audit M19): ignores re-entrant calls, calls `preventDefault` on every submit, releases after resolve and after throw (4)
- **pages/product-management-page.test.js** - Boots the real product master page: editing a product whose manufacturer is disabled keeps `manufacturer_id` on save (latent-audit M5) (1)
- **pages/product-management-link-draft.test.js** - Boots the real product master page from the detail → product-master jump (`?return_to=`): after adding a product, the detail draft is linked only to the product whose name matches exactly, never to another search candidate (latent-audit L17); without a detail draft none is created, an empty search leaves the draft alone, and linking keeps every non-product field (5)
- **pages/shop-management-disabled.test.js** - Boots the real shop master page: "show disabled" lists disabled shops with the disabled label, a late response from a quick double toggle does not overwrite the newer list, and the add / edit form's "disabled" checkbox is shown and sent as `isDisabled` (latent-audit M7) (4)
- **pages/transaction-management-disabled-shop.test.js** - Boots the real transaction page: editing a transaction whose shop is disabled keeps that shop selected (shown with the disabled label) and saves it, while a new transaction is not offered the disabled shop (latent-audit M7) (2)
- **pages/account-management-disabled.test.js** - Boots the real account master page: "show disabled" lists disabled accounts with the disabled label (NONE never listed), a late response from a quick double toggle does not overwrite the newer list, and the add / edit form's "disabled" checkbox is shown and sent as `isDisabled` (latent-audit M7) (4)
- **pages/account-management-save-error-keeps-form.test.js** - Boots the real account master page: a save rejected by the backend (`duplicate_code`) or stopped by the form checks (whitespace-only name) keeps the modal open with the typed input (latent-audit scan2-M4) (2)
- **pages/account-management-validation-i18n.test.js** - Boots the real account master page: the input checks (empty code / name / template / initial balance) show localized messages next to their inputs, and a failed account list load shows only `account_mgmt.failed_to_load`, without "Error loading accounts" or the backend's English detail (latent-audit scan2-R8 follow-up) (5)
- **pages/account-management-code-max-length.test.js** - Boots the real account master page: on add, the account code shows a "n / 50" counter and is cut at 50 characters, a 51-character code is stopped before `add_account` with `validation.max_length`, and a 50-character code is sent; on edit, the read-only code has no counter and an existing code longer than 50 characters is sent to `update_account` unchanged (4)
- **pages/transaction-management-disabled-account.test.js** - Boots the real transaction page: editing a transaction whose account is disabled keeps that account selected (shown with the disabled label) and saves it, while a new transaction is not offered the disabled account (latent-audit M7) (2)
- **pages/transaction-management-category1-has-details.test.js** - Boots the real transaction page: when the backend refuses to change the category1 of a transaction with details (`category1_has_details`), the screen shows `transaction_mgmt.category1_has_details` and keeps the modal open (latent-audit M2) (1)
- **modal-open-awaits-onopen.test.js** - `Modal.open()` returns a promise that settles only after an async `onOpen` has finished (and at once for a synchronous one), so callers can fill the form in afterwards (latent-audit L6) (2)
- **pages/transaction-management-restore-draft.test.js** - Boots the real transaction page with a saved new-transaction draft: the restored date, shop and memo survive the modal's own initialisation (latent-audit L6) (1)
- **pages/transaction-management-shop-roundtrip-draft.test.js** - Boots the real transaction page through the Manage-shops round trip: a new transaction keeps its scheduled flag, an edited one keeps its rounding, account and cleared memo, and a category1 cleared in edit mode stays cleared (latent-audit scan2-T4) (3)
- **pages/transaction-management-rejected-save-keeps-form.test.js** - Boots the real transaction page: a TRANSFER between two "Unspecified" accounts, a total rejected by `parseAmountStrict` (`1e3`) and a generic backend error each keep the modal open with the date, total and memo (latent-audit scan2-T5) (3)
- **pages/transaction-management-restore-disabled-shop.test.js** - Boots the real transaction page with a saved draft whose shop was disabled since: the restored new transaction falls back to "Unspecified" (latent-audit M7, reachable since L6) (1)
- **pages/transaction-management-restore-reopened.test.js** - Boots the real transaction page with a saved draft, then closes and reopens the modal while the restore is still waiting: the draft is not written into the reopened form (latent-audit L6) (1)
- **pages/aggregation-monthly-page.test.js** - Boots the real monthly aggregation page: an empty `group_name` renders as `common.unspecified` (latent-audit M12); the total row shows "—" for count / average on the account and category2 axes, and still sums the count on category1 (latent-audit M11); negative amounts render as "-¥1,234" (latent-audit L12) (5)
- **pages/aggregation-yearly-total-count.test.js** - Boots the real yearly aggregation page: the shared renderer's total row shows "—" for count / average on the account axis (latent-audit M11) (1)
- **pages/aggregation-default-period-monthly.test.js** - Boots the real monthly aggregation page with start day 25 on 2026-09-10: it opens on the August period that contains today, not the future September period (latent-audit scan2-A3) (1)
- **pages/aggregation-default-period-yearly.test.js** - Boots the real yearly aggregation page with the year starting 04-01 on 2026-02-10: it opens on 2025, the period that contains today, not the future 2026 period (latent-audit scan2-A3) (1)
- **pages/aggregation-monthly-stale.test.js** - Boots the real monthly aggregation page (shared scenario in `pages/_aggregation-stale.js`): a slower, older Execute neither overwrites the newer table nor shows its error, the loading state stays until the latest request finishes, and an Execute stopped by the input checks does not leave the running request's loading state on (latent-audit scan2-A4) (4)
- **pages/aggregation-daily-stale.test.js** - Boots the real daily aggregation page (shared scenario in `pages/_aggregation-stale.js`): a slower, older Execute neither overwrites the newer table nor shows its error, the loading state stays until the latest request finishes, and an Execute stopped by the input checks does not leave the running request's loading state on (latent-audit scan2-A4) (4)
- **pages/aggregation-weekly-stale.test.js** - Boots the real weekly aggregation page (shared scenario in `pages/_aggregation-stale.js`): a slower, older Execute neither overwrites the newer table nor shows its error, the loading state stays until the latest request finishes, and an Execute stopped by the input checks does not leave the running request's loading state on (latent-audit scan2-A4) (4)
- **pages/aggregation-period-stale.test.js** - Boots the real period aggregation page (shared scenario in `pages/_aggregation-stale.js`): a slower, older Execute neither overwrites the newer table nor shows its error, the loading state stays until the latest request finishes, and an Execute stopped by the input checks does not leave the running request's loading state on (latent-audit scan2-A4) (4)
- **pages/aggregation-yearly-stale.test.js** - Boots the real yearly aggregation page (shared scenario in `pages/_aggregation-stale.js`): a slower, older Execute neither overwrites the newer table nor shows its error, the loading state stays until the latest request finishes, and an Execute stopped by the input checks does not leave the running request's loading state on (latent-audit scan2-A4) (4)
- **pages/dashboard-bar-top10.test.js** - Boots the real dashboard: the category bar chart lists the largest expenses first (by magnitude, since expense totals are negative) and its top 10 keeps the largest one (latent-audit scan2-A1) (1)
- **pages/index-setup-page.test.js** - Boots the real setup forms (menu.js on index.html): a blank username is stopped before `register_admin`, a backend "Username cannot be empty" is reported as the username error (not the password one), and `duplicate_name` shows `error.username_duplicate` (latent-audit L25) (3)
- **pages/category-management-page.test.js** - Boots the real category management page: moving or showing a category that no longer exists shows `category_mgmt.not_found` and reloads the tree (latent-audit L19) (2)
- **pages/category-management-move-buttons.test.js** - Boots the real category management page: the first / last *visible* CATEGORY2 and CATEGORY3 have their ↑ / ↓ disabled, ignoring hidden siblings listed after them; hidden rows have no ↑/↓ (latent-audit scan2-M5) (2)
- **pages/transaction-management-filter-hidden-category.test.js** - Boots the real transaction list: the category filter also offers a hidden CATEGORY2 and its CATEGORY3, labelled with `common.disabled_label`, so their past transactions can still be searched (latent-audit scan2-M7) (1)
- **pages/transaction-management-save-before-details.test.js** - Boots the real transaction list: "Manage details" with unsaved header edits asks in an in-app dialog (`#save-before-details-modal`, `transaction_mgmt.save_before_details_confirm`, no native `confirm()`), saves the header through the normal save and moves on, and a second click while saving does not save twice; Cancel or Esc closes only the dialog and keeps the edit modal open without saving; without changes it moves on at once (latent-audit scan2-T6) (5)
- **pages/user-management-password-page.test.js** - Admin session: a 16-space password is reported as the password error, not as the raw `user_mgmt.empty_name` key on the username (latent-audit L24); 8 emoji (16 UTF-16 units, 8 characters) are rejected by the frontend length check (latent-audit L31) (2)
- **pages/user-management-nonadmin-page.test.js** - General-user session: no Add User button, no empty footer line where the button was, and no delete button on the user's own row (latent-audit L30) (3)
- **pages/index-setup-password-length.test.js** - Setup forms count password characters, not UTF-16 units: 8 emoji are rejected for admin and user setup (latent-audit L31) (2)

### Aggregation Tests (115 tests)
- **aggregation-daily.test.js** - Daily aggregation (16)
- **aggregation-weekly.test.js** - Weekly aggregation (22)
- **aggregation-monthly.test.js** - Monthly aggregation (33)
- **aggregation-yearly.test.js** - Yearly aggregation (21)
- **aggregation-period.test.js** - Period aggregation (23)

### Browser / Standalone (not counted in the jest total)
- **category-management-ui-tests.js** - DOM-based tests, run in a browser session against a rendered category page
- **tax-rounding-tests.js** - Companion to `tax-rounding-tests.html`; pure-function harness, run via the HTML page
- **backend-validation-standalone.js** - Node-standalone runner (`node backend-validation-standalone.js`)
- **login-test-standalone.js** - Node-standalone runner (`node login-test-standalone.js`)
- **aggregation-test-helpers.js** - Shared mock/fixture helpers imported by the aggregation `.test.js` files
- **pages/_page-harness.js** - Shared jsdom page harness (module mocks, page body loading, boot) used by `pages/*.test.js`, `modal-stale-save-close.test.js`, and the isolated `latent-audit/*.test.js`

---

## Test Statistics Summary

| Category | Test Count |
|----------|------------|
| **Common Test Suites** (helpers — counted inside Screen-Specific) | 56 |
| password-validation-tests.js | 26 |
| username-validation-tests.js | 20 |
| user-edit-validation-tests.js | 23 |
| **Screen-Specific Tests** | **305** |
| admin-setup.test.js | 32 |
| user-addition.test.js | 46 |
| admin-edit.test.js | 62 |
| general-user-edit.test.js | 62 |
| login.test.js | 58 |
| user-deletion.test.js | 45 |
| **Feature-Specific Tests** | **538** |
| transaction-edit.test.js | 111 |
| transaction-detail-management.test.js | 51 |
| transaction-detail-tax-calculation.test.js | 30 |
| toast.test.js | 14 |
| tax-calc.test.js | 12 |
| pages/transaction-detail-draft-storage.test.js | 6 |
| pages/transaction-detail-product-link.test.js | 9 |
| pages/product-management-product-draft.test.js | 7 |
| pages/manufacturer-management-product-draft.test.js | 6 |
| modal-double-submit.test.js | 6 |
| modal-stale-save-close.test.js | 2 |
| master-crud.test.js | 30 |
| attach-char-counter-ime.test.js | 8 |
| aggregation-error-translate.test.js | 13 |
| aggregation-latest-request.test.js | 2 |
| parse-amount-strict.test.js | 24 |
| format-local-date.test.js | 22 |
| period-end-date.test.js | 2 |
| period-containing.test.js | 16 |
| aggregation-render-unspecified.test.js | 5 |
| pages/transaction-detail-page.test.js | 3 |
| pages/transaction-detail-included-typing.test.js | 1 |
| pages/transaction-detail-unreachable-included-price.test.js | 1 |
| pages/transaction-detail-hidden-category.test.js | 2 |
| pages/transaction-detail-product-suggest.test.js | 6 |
| pages/transaction-management-page.test.js | 4 |
| pages/user-management-page.test.js | 4 |
| pages/user-management-delete-last-user.test.js | 1 |
| pages/index-logout-hides-user-setup.test.js | 3 |
| pages/recurring-rule-page.test.js | 4 |
| pages/recurring-rule-double-submit.test.js | 1 |
| pages/recurring-rule-period-range.test.js | 5 |
| pages/recurring-rule-anchor-follows-start.test.js | 2 |
| pages/product-management-edit-manufacturer-roundtrip.test.js | 1 |
| pages/recurring-rule-derived-total.test.js | 2 |
| pages/recurring-rule-cycle-options.test.js | 3 |
| pages/recurring-rule-date-order.test.js | 5 |
| pages/recurring-rule-reset.test.js | 1 |
| pages/recurring-rule-rounding-recalc.test.js | 2 |
| pages/menu-i18n-seed.test.js | 2 |
| pages/i18n-literal-user-text.test.js | 7 |
| pages/dashboard-balance-header.test.js | 1 |
| pages/transaction-list-none-account-label.test.js | 1 |
| pages/transaction-detail-none-account-label.test.js | 1 |
| pages/dashboard-balance-sign.test.js | 3 |
| pages/dashboard-default-period.test.js | 1 |
| pages/dashboard-stale-reload.test.js | 1 |
| single-flight.test.js | 4 |
| pages/product-management-page.test.js | 1 |
| pages/product-management-link-draft.test.js | 5 |
| pages/shop-management-disabled.test.js | 4 |
| pages/transaction-management-disabled-shop.test.js | 2 |
| pages/account-management-disabled.test.js | 4 |
| pages/account-management-save-error-keeps-form.test.js | 2 |
| pages/account-management-validation-i18n.test.js | 5 |
| pages/account-management-code-max-length.test.js | 4 |
| pages/transaction-management-disabled-account.test.js | 2 |
| pages/transaction-management-category1-has-details.test.js | 1 |
| modal-open-awaits-onopen.test.js | 2 |
| pages/transaction-management-restore-draft.test.js | 1 |
| pages/transaction-management-shop-roundtrip-draft.test.js | 3 |
| pages/transaction-management-rejected-save-keeps-form.test.js | 3 |
| pages/transaction-management-restore-disabled-shop.test.js | 1 |
| pages/transaction-management-restore-reopened.test.js | 1 |
| pages/aggregation-monthly-page.test.js | 5 |
| pages/aggregation-yearly-total-count.test.js | 1 |
| pages/aggregation-default-period-monthly.test.js | 1 |
| pages/aggregation-default-period-yearly.test.js | 1 |
| pages/aggregation-monthly-stale.test.js | 4 |
| pages/aggregation-daily-stale.test.js | 4 |
| pages/aggregation-weekly-stale.test.js | 4 |
| pages/aggregation-period-stale.test.js | 4 |
| pages/aggregation-yearly-stale.test.js | 4 |
| pages/dashboard-bar-top10.test.js | 1 |
| pages/index-setup-page.test.js | 3 |
| pages/category-management-page.test.js | 2 |
| pages/category-management-move-buttons.test.js | 2 |
| pages/transaction-management-filter-hidden-category.test.js | 1 |
| pages/transaction-management-save-before-details.test.js | 5 |
| pages/user-management-password-page.test.js | 2 |
| pages/user-management-nonadmin-page.test.js | 3 |
| pages/index-setup-password-length.test.js | 2 |
| **Aggregation Tests** | **115** |
| aggregation-daily.test.js | 16 |
| aggregation-weekly.test.js | 22 |
| aggregation-monthly.test.js | 33 |
| aggregation-yearly.test.js | 21 |
| aggregation-period.test.js | 23 |
| **Total (jest)** | **958** |

Grand total is Screen + Feature + Aggregation (Common Test Suites are helper
libraries invoked from Screen-Specific files and their assertions are already
counted in those screen totals).

---

## How to Run Tests

### Run all tests

```bash
cd res/tests
npm test
```

### Run specific test file

```bash
npm test admin-setup.test.js
npm test login.test.js
npm test user-deletion.test.js
```

### Run specific test case

```bash
npm test -- --testNamePattern="Empty Password"
npm test -- --testNamePattern="Username Validation"
```

### Generate coverage report

```bash
npm run test:coverage
```

### Standalone tests (Node.js, no dependencies)

```bash
node login-test-standalone.js
node backend-validation-standalone.js
```

### Refreshing the authoritative counts

```bash
cd res/tests
node --experimental-vm-modules node_modules/jest/bin/jest.js --json > /tmp/jest.json
# Per-file counts:
node -e "const j=JSON.parse(require('fs').readFileSync('/tmp/jest.json','utf8')); \
  j.testResults.map(r=>({f:r.name.replace(/^.*\\//,''),n:r.assertionResults.length})) \
  .sort((a,b)=>a.f.localeCompare(b.f)).forEach(r=>console.log(String(r.n).padStart(4)+'  '+r.f)); \
  console.log('total:',j.numTotalTests);"
```

---

## Related Documents

- [Backend Test Index](BACKEND_TEST_INDEX.md) - Complete list of Rust tests
- [Test Overview](TEST_OVERVIEW.md) - Test strategy and execution guide
- [Test Design](TEST_DESIGN.md) - Test architecture and design philosophy
- [Test Results](TEST_RESULTS.md) - Latest test execution results
