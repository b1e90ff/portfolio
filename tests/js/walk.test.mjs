import assert from 'node:assert/strict';
import { test } from 'node:test';
import * as THREE from 'three';

import { radiusFn } from '../../public/js/world/engine.js';
import { area, playFetch, playTogether } from '../../public/js/world/props.js';

const level = { fn: radiusFn({ r0: 4, amp: 0 }), c: { x: 0, z: 0 } };
const inner = { fn: radiusFn({ r0: 1, amp: 0 }), c: { x: -2, z: 0 } };
const bridge = { x: 1.5, z: 0, sx: 0, sz: 1, cx: 1, cz: 0, halfLength: .6, halfWidth: .3, height: .2 };
const barrier = { ax: 1.5, az: -3, bx: 1.5, bz: 3, gapFrom: .45, gapTo: .55, gapX: 1.5, gapZ: 0 };

const walkArea = (occ = []) => area(level, occ, { margin: .2, hole: { lvl: inner, pad: .1 }, bridge, barrier });

test('valid respects outline, hole and obstacles', () => {
    const walk = walkArea([{ x: 2, z: 2, r: .5 }]);
    assert.ok(walk.valid(0, 2, .2));
    assert.ok(!walk.valid(3.9, 0, .2), 'outside the outline');
    assert.ok(!walk.valid(-2, 0, .2), 'inside the hole');
    assert.ok(!walk.valid(2, 2, .2), 'inside an obstacle');
});

test('resolve pushes points back into the walkable area', () => {
    const walk = walkArea([{ x: 2, z: 2, r: .5 }]);
    for (const p of [new THREE.Vector3(5, 0, 0), new THREE.Vector3(-2, 0, .1), new THREE.Vector3(2, 0, 2.1)]) {
        walk.resolve(p, .2);
        assert.ok(walk.valid(p.x, p.z, .19), `${p.x},${p.z}`);
    }
});

test('route detours through the barrier gap only when the straight line crosses it', () => {
    const walk = walkArea();
    const out = new THREE.Vector3();
    walk.route(new THREE.Vector3(0, 0, 2), new THREE.Vector3(3, 0, 2), out);
    assert.deepEqual([out.x, out.z], [1.5, 0]);
    walk.route(new THREE.Vector3(0, 0, 2), new THREE.Vector3(0, 0, -2), out);
    assert.deepEqual([out.x, out.z], [0, -2]);
});

test('bridge lifts walkers only on its deck', () => {
    const walk = walkArea();
    assert.ok(Math.abs(walk.heightAt(1.5, 0) - .2) < 1e-9);
    assert.equal(walk.heightAt(0, 0), 0);
});

test('samples and centres are always walkable', () => {
    const walk = walkArea([{ x: 2, z: 2, r: .5 }]);
    for (let i = 0; i < 200; i++) {
        const p = walk.sample(new THREE.Vector3(0, 0, 2), .5, 2, .25);
        assert.ok(walk.valid(p.x, p.z, .25));
    }
    assert.ok(walk.valid(walk.center.x, walk.center.z, .3));
    assert.ok(walk.valid(walk.openCenter.x, walk.openCenter.z, .9));
});

function fakeStage() {
    const frames = [];
    const ctx = {
        reduced: false,
        onFrame: (fn) => frames.push(fn),
        add: (geo, mat, x = 0, y = 0, z = 0, parent) => {
            const m = new THREE.Mesh(geo, mat);
            m.position.set(x, y, z);
            parent?.add(m);
            return m;
        },
    };
    const run = (seconds) => {
        for (let t = 0; t < seconds * 1000; t += 16) frames.forEach((fn) => fn(t, .016));
    };
    return { ctx, run };
}

/* Deterministic generator so a failing simulation can be replayed. */
function seeded(seed) {
    let a = seed;
    return () => {
        a = (a + 0x6d2b79f5) | 0;
        let t = Math.imul(a ^ (a >>> 15), 1 | a);
        t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
        return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
    };
}

function fakeDog(x, z) {
    const dog = new THREE.Group();
    dog.position.set(x, 0, z);
    const head = new THREE.Group();
    head.position.x = .2;
    dog.add(head);
    dog.userData = { legs: [0, 1, 2, 3].map(() => new THREE.Group()), head };
    return dog;
}

test('playing dogs never leave the walkable area', (t) => {
    t.mock.method(Math, 'random', seeded(7));
    const { ctx, run } = fakeStage();
    const walk = walkArea([{ x: 2, z: 2, r: .5 }]);
    const a = fakeDog(0, 2);
    const b = fakeDog(-.5, 2.8);
    playTogether(ctx, walk, [[a, { r: .24, speed: 1.1 }], [b, { r: .16, speed: 1.3 }]], 0);
    for (let step = 0; step < 60; step++) {
        run(.5);
        for (const [dog, r] of [[a, .24], [b, .16]]) {
            assert.ok(Number.isFinite(dog.position.x) && Number.isFinite(dog.position.z));
            assert.ok(walk.valid(dog.position.x, dog.position.z, r - .02), `left the area at ${dog.position.x},${dog.position.z}`);
        }
    }
});

test('the fetched ball never leaves the walkable area, even while carried', (t) => {
    const small = area({ fn: radiusFn({ r0: 1.6, amp: 0 }), c: { x: 0, z: 0 } }, [], { margin: .2 });
    for (const seed of [1, 2, 3]) {
        t.mock.method(Math, 'random', seeded(seed));
        const { ctx, run } = fakeStage();
        const parent = new THREE.Group();
        const dog = fakeDog(0, 0);
        parent.add(dog);
        playFetch(ctx, parent, small, dog, 0);
        const ball = parent.children.find((c) => c.isMesh);
        let outside = 0;
        ctx.onFrame(() => { if (!small.valid(ball.position.x, ball.position.z, .05)) outside++; });
        run(60);
        assert.equal(outside, 0, `seed ${seed}`);
        t.mock.restoreAll();
    }
});
