import assert from 'node:assert/strict';
import { test } from 'node:test';

import { ASIDE, ISLANDS, cameraState, layoutFor } from '../../public/js/world/camera.js';

const DESKTOP = { width: 1440, height: 900 };
const PHONE = { width: 390, height: 844 };
const PHONE_LANDSCAPE = { width: 844, height: 390 };
const TABLET_LANDSCAPE = { width: 1180, height: 820 };

test('every island and view resolves to a finite camera state', () => {
    for (const key of [...Object.keys(ISLANDS), 'overview', ASIDE]) {
        for (const size of [DESKTOP, PHONE, PHONE_LANDSCAPE]) {
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

test('short landscape screens get their own layout, taller ones keep theirs', () => {
    const layout = ({ width, height }) => layoutFor(width, height);
    assert.equal(layout(PHONE), 'narrow');
    assert.equal(layout(PHONE_LANDSCAPE), 'short');
    assert.equal(layout({ width: 863, height: 360 }), 'short');
    assert.equal(layout({ width: 800, height: 600 }), 'narrow');
    assert.equal(layout(TABLET_LANDSCAPE), 'wide');
    assert.equal(layout(DESKTOP), 'wide');
});

test('layoutFor boundaries sit on the 3/2 aspect and the 500px height', () => {
    assert.equal(layoutFor(800, 500), 'short');
    assert.equal(layoutFor(800, 501), 'narrow');
    assert.equal(layoutFor(750, 500), 'short');
    assert.equal(layoutFor(749, 500), 'narrow');
    assert.equal(layoutFor(500, 500), 'narrow');
    assert.equal(layoutFor(412, 380), 'narrow');
});

test('short landscape frames the island left of the side panel', () => {
    const s = cameraState('about', PHONE_LANDSCAPE);
    assert.ok(s.offsetX < 0);
    assert.equal(s.offsetY, 0);
    assert.deepEqual(cameraState('about', { ...PHONE_LANDSCAPE, collapsed: true }), s);
    assert.ok(cameraState('overview', PHONE_LANDSCAPE).offsetX > cameraState('overview', DESKTOP).offsetX);
});
