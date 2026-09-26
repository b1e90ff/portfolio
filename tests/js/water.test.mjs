import assert from 'node:assert/strict';
import { test } from 'node:test';

import { fallSheet } from '../../public/js/world/water.js';

const SHEET = { w: .62, spill: .1, bend: .12, drop: 4.3, arc: .5, wTop: .5 };

function rows(geo) {
    const p = geo.attributes.position;
    const out = [];
    for (let i = 0; i < p.count; i += 2) out.push({ x: p.getX(i + 1), y: p.getY(i), z: p.getZ(i) });
    return out;
}

const make = () => fallSheet(SHEET.w, SHEET.spill, SHEET.bend, SHEET.drop, SHEET.arc, SHEET.wTop);

test('sheet starts at the stream width on the lip and ends at full width below the drop', () => {
    const r = rows(make());
    assert.ok(Math.abs(r[0].x * 2 - SHEET.wTop) < 1e-6);
    assert.ok(Math.abs(r.at(-1).x * 2 - SHEET.w) < 1e-6);
    assert.ok(Math.abs(r.at(-1).y - (.02 - SHEET.bend - SHEET.drop)) < 1e-6);
    assert.ok(Math.abs(r.at(-1).z - (SHEET.bend + SHEET.arc)) < 1e-6);
});

test('consecutive segments never turn sharply, so the bend reads as a curve', () => {
    const r = rows(make());
    let prev = null;
    for (let i = 1; i < r.length; i++) {
        const dir = Math.atan2(r[i].y - r[i - 1].y, r[i].z - r[i - 1].z);
        if (prev !== null) assert.ok(Math.abs(dir - prev) < .35, `turn of ${(dir - prev).toFixed(2)} rad at row ${i}`);
        prev = dir;
    }
});

test('a fall without a flat head still has no degenerate rows', () => {
    const r = rows(fallSheet(.5, 0, .12, 2, .3));
    for (let i = 1; i < r.length; i++) {
        assert.ok(Math.hypot(r[i].y - r[i - 1].y, r[i].z - r[i - 1].z) > 1e-6, `row ${i} repeats`);
    }
});
