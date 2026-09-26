import assert from 'node:assert/strict';
import { test } from 'node:test';

import { ASIDE, ISLANDS, cameraState } from '../../public/js/world/camera.js';

const DESKTOP = { width: 1440, height: 900 };
const PHONE = { width: 390, height: 844 };

test('every island and view resolves to a finite camera state', () => {
    for (const key of [...Object.keys(ISLANDS), 'overview', ASIDE]) {
        for (const size of [DESKTOP, PHONE]) {
            const s = cameraState(key, size);
            for (const n of [...s.target, s.dist, s.height, s.yaw, s.offsetX, s.offsetY]) assert.ok(Number.isFinite(n), key);
        }
    }
});

test('portrait screens pull the overview camera back', () => {
    assert.ok(cameraState('overview', PHONE).dist > cameraState('overview', DESKTOP).dist);
});

test('landing frames the island left of the desktop panel', () => {
    const s = cameraState('projects', DESKTOP);
    assert.deepEqual(s.target, [12, 3.3, -6]);
    assert.equal(s.offsetX, -.2);
});

test('legal pages shift the overview aside for the panel', () => {
    assert.ok(cameraState(ASIDE, DESKTOP).offsetX < 0);
    assert.ok(cameraState('overview', DESKTOP).offsetX > 0);
});

test('collapsing the panel only reframes narrow screens', () => {
    assert.deepEqual(cameraState('about', { ...DESKTOP, collapsed: true }), cameraState('about', DESKTOP));
    const open = cameraState('about', PHONE);
    const folded = cameraState('about', { ...PHONE, collapsed: true });
    assert.ok(folded.offsetY < open.offsetY);
    assert.ok(folded.dist < open.dist);
});
