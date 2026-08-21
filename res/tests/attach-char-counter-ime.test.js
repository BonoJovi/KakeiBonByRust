/**
 * attachCharCounter — IME composition guard tests (Fable-5 #D1)
 *
 * `attachCharCounter` truncates `inputEl.value` to `max` code points on
 * every `input` event. In IME sessions (kana/kanji, hangul, pinyin) each
 * pre-conversion candidate keystroke also fires `input`; writing back to
 * `.value` mid-composition cancels or corrupts the IME buffer, breaking
 * Japanese input around the `max` boundary. Before this fix the stray
 * `maxlength=128` attribute on the affected fields masked the bug by
 * keeping input under the boundary; the maxlength removal
 * (product/shop/manufacturer HTML) exposed it.
 *
 * The guard tracks `compositionstart` / `compositionend` and skips
 * truncation while a composition is active; truncation applies once
 * on `compositionend`.
 */

import { jest } from '@jest/globals';
import { attachCharCounter } from '../js/validation-display.js';

function makeInput(initialValue = '') {
    document.body.innerHTML = `<div><input id="in" type="text" /></div>`;
    const input = document.getElementById('in');
    input.value = initialValue;
    return input;
}

function fireInput(input) {
    input.dispatchEvent(new Event('input', { bubbles: true }));
}

function fireCompositionStart(input) {
    input.dispatchEvent(new Event('compositionstart', { bubbles: true }));
}

function fireCompositionEnd(input) {
    input.dispatchEvent(new Event('compositionend', { bubbles: true }));
}

afterEach(() => {
    document.body.innerHTML = '';
});

describe('attachCharCounter — non-IME baseline behaviour still holds', () => {
    test('renders "actual / max" counter next to the input', () => {
        const input = makeInput('ab');
        attachCharCounter(input, 5);
        const counter = input.nextElementSibling;
        expect(counter.classList.contains('char-counter')).toBe(true);
        expect(counter.textContent).toBe('2 / 5');
    });

    test('truncates plain typing beyond max on input event', () => {
        const input = makeInput('');
        attachCharCounter(input, 3);
        input.value = 'abcdef';
        fireInput(input);
        expect(input.value).toBe('abc');
        expect(input.nextElementSibling.textContent).toBe('3 / 3');
    });

    test('counts multibyte characters by code point, not UTF-16 unit', () => {
        const input = makeInput('あいうえおか'); // 6 code points
        attachCharCounter(input, 4);
        // Initial call already runs update; oversize gets truncated even
        // on the initial value.
        expect(input.value).toBe('あいうえ');
        expect(input.nextElementSibling.textContent).toBe('4 / 4');
    });
});

describe('attachCharCounter — IME composition guard (Fable-5 #D1)', () => {
    test('input events fired inside a composition do NOT truncate value', () => {
        const input = makeInput('');
        attachCharCounter(input, 5);

        fireCompositionStart(input);
        // Simulate the IME writing over-length pre-conversion text into
        // .value across several candidate keystrokes.
        input.value = 'あいうえおかきくけこ'; // 10 code points
        fireInput(input);
        expect(input.value).toBe('あいうえおかきくけこ'); // untouched
        input.value = 'あいうえおかきくけこさし';
        fireInput(input);
        expect(input.value).toBe('あいうえおかきくけこさし'); // still untouched

        // The counter is allowed to reflect the composing length as a
        // visual signal, but the .value itself must not be rewritten
        // — that is the write that corrupts the IME buffer.
    });

    test('compositionend commits + applies truncation', () => {
        const input = makeInput('');
        attachCharCounter(input, 5);

        fireCompositionStart(input);
        input.value = 'あいうえおかきくけこ'; // 10 code points during composition
        fireInput(input);
        expect(input.value.length).toBe(10);

        fireCompositionEnd(input);
        expect([...input.value].length).toBe(5); // truncated at commit
        expect(input.nextElementSibling.textContent).toBe('5 / 5');
    });

    test('a fresh composition after commit does not carry the composing flag', () => {
        const input = makeInput('');
        attachCharCounter(input, 5);

        fireCompositionStart(input);
        input.value = 'あいうえおか'; // 6 code points
        fireInput(input);
        fireCompositionEnd(input);
        expect([...input.value].length).toBe(5);

        // Direct typing (no composition) after the previous commit should
        // still truncate on input as usual.
        input.value = 'あいうえおかき'; // append 2 more
        fireInput(input);
        expect([...input.value].length).toBe(5);
    });

    test('idempotent: a second attach detaches prior listeners so counting is not doubled', () => {
        const input = makeInput('ab');
        attachCharCounter(input, 5);
        attachCharCounter(input, 5);

        // Second attach must not add a second counter sibling.
        const counters = document.querySelectorAll('.char-counter');
        expect(counters.length).toBe(1);

        // And there is only one listener active — one input event still
        // truncates once (a stacked listener would still truncate once
        // because truncation is idempotent, but two counter updates would
        // race; check via composition path where behaviour differs).
        fireCompositionStart(input);
        input.value = 'あいうえおかきくけこ';
        fireInput(input);
        // If the old (pre-fix) handler were still attached, this line
        // would have truncated to 5. With the guard active on both
        // (hypothetical stacked) handlers, it should stay at 10.
        expect(input.value.length).toBe(10);
        fireCompositionEnd(input);
        expect([...input.value].length).toBe(5);
    });

    test('detach() removes all three listeners (input, compositionstart, compositionend)', () => {
        const input = makeInput('');
        const detach = attachCharCounter(input, 3);
        detach();

        // With listeners removed, truncation no longer fires on input.
        input.value = 'abcdef';
        fireInput(input);
        expect(input.value).toBe('abcdef');

        // Counter element should also be gone.
        expect(document.querySelector('.char-counter')).toBeNull();
    });
});
