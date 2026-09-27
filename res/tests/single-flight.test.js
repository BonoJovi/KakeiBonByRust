import { jest } from '@jest/globals';
import { singleFlight } from '../js/single-flight.js';

// singleFlight (latent-audit M19): a second submit while the first handler
// is still running is ignored; the guard is released once it settles,
// whether it resolved or threw.

function deferred() {
    let resolve;
    let reject;
    const promise = new Promise((res, rej) => { resolve = res; reject = rej; });
    return { promise, resolve, reject };
}

const fakeEvent = () => ({ preventDefault: jest.fn() });

describe('singleFlight', () => {
    test('ignores a second call while the first is in flight', async () => {
        const pending = deferred();
        const handler = jest.fn(() => pending.promise);
        const listener = singleFlight(handler);

        const first = listener(fakeEvent());
        const second = listener(fakeEvent());
        await second;
        expect(handler).toHaveBeenCalledTimes(1);

        pending.resolve();
        await first;
    });

    test('calls preventDefault on every submit, including ignored ones', async () => {
        const pending = deferred();
        const listener = singleFlight(() => pending.promise);
        const e1 = fakeEvent();
        const e2 = fakeEvent();

        const first = listener(e1);
        await listener(e2);
        expect(e1.preventDefault).toHaveBeenCalledTimes(1);
        expect(e2.preventDefault).toHaveBeenCalledTimes(1);

        pending.resolve();
        await first;
    });

    test('accepts a new call after the previous one resolved', async () => {
        const handler = jest.fn(async () => {});
        const listener = singleFlight(handler);

        await listener(fakeEvent());
        await listener(fakeEvent());
        expect(handler).toHaveBeenCalledTimes(2);
    });

    test('releases the guard when the handler throws', async () => {
        const handler = jest.fn()
            .mockImplementationOnce(async () => { throw new Error('boom'); })
            .mockImplementationOnce(async () => {});
        const listener = singleFlight(handler);

        await expect(listener(fakeEvent())).rejects.toThrow('boom');
        await listener(fakeEvent());
        expect(handler).toHaveBeenCalledTimes(2);
    });
});
