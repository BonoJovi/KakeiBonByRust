/**
 * The app bundle must not contain the frontend tests.
 *
 * Tauri bundles everything under `build.frontendDist` in tauri.conf.json
 * into the release app. When the tests lived inside that folder, the
 * release app carried every test file and the whole Jest `node_modules`.
 * These tests walk the bundled folder and fail on any test file or
 * `node_modules` found there.
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

// Walk up from this file to the repository root (where tauri.conf.json is).
function findRepoRoot(start) {
    let dir = start;
    while (!fs.existsSync(path.join(dir, 'tauri.conf.json'))) {
        const parent = path.dirname(dir);
        if (parent === dir) throw new Error('tauri.conf.json not found');
        dir = parent;
    }
    return dir;
}

const ROOT = findRepoRoot(path.dirname(fileURLToPath(import.meta.url)));
const conf = JSON.parse(fs.readFileSync(path.join(ROOT, 'tauri.conf.json'), 'utf8'));
const DIST = path.resolve(ROOT, conf.build.frontendDist);

// Paths under DIST (relative) that match `isUnwanted`. Symbolic links are
// reported by name and not followed.
function findUnder(dir, isUnwanted) {
    const found = [];
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
        const full = path.join(dir, entry.name);
        if (isUnwanted(entry)) {
            found.push(path.relative(DIST, full));
        } else if (entry.isDirectory()) {
            found.push(...findUnder(full, isUnwanted));
        }
    }
    return found;
}

describe('release bundle — frontend tests', () => {
    test('should find no test files when the bundled frontend folder is walked', () => {
        // `*.test.js` (Jest) and `*-tests.js` (shared suites such as
        // password-validation-tests.js).
        expect(findUnder(DIST, (e) => /(\.test|-tests)\.(c?js|html)$/.test(e.name))).toEqual([]);
    });

    test('should find no node_modules folder when the bundled frontend folder is walked', () => {
        expect(findUnder(DIST, (e) => e.name === 'node_modules')).toEqual([]);
    });
});
