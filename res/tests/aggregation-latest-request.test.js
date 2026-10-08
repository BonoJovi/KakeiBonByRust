/**
 * aggregation-common.js — createLatestRequestGuard (latent-audit scan2-A4)
 *
 * The aggregation screens start a request per Execute and use the guard to
 * drop a slower, older request's result, error and loading-state change.
 * Pinned: only the most recent request reports itself as latest, guards are
 * independent of each other, and a request stays latest until the next one
 * starts. The per-screen behaviour is pinned in pages/aggregation-*-stale.
 */

import { jest } from '@jest/globals';

jest.unstable_mockModule('@tauri-apps/api/core', () => ({ invoke: jest.fn() }));
jest.unstable_mockModule('../js/i18n.js', () => ({
    default: { t: (key) => key, updateUI: () => {}, init: async () => {} },
}));

const { createLatestRequestGuard } = await import('../js/aggregation-common.js');

describe('createLatestRequestGuard (latent scan2-A4)', () => {
    test('should treat only the most recent request as latest when several were started (scan2-A4)', () => {
        const nextRequest = createLatestRequestGuard();
        const first = nextRequest();
        expect(first()).toBe(true);

        const second = nextRequest();
        expect(first()).toBe(false);
        expect(second()).toBe(true);
        // Asking again does not change the answer.
        expect(second()).toBe(true);
    });

    test('should count only its own requests when there are several guards (scan2-A4)', () => {
        const monthly = createLatestRequestGuard();
        const yearly = createLatestRequestGuard();
        const monthlyRequest = monthly();
        yearly();
        yearly();
        expect(monthlyRequest()).toBe(true);
    });
});
