import * as THREE from 'three';
import { angDiff, at, hash } from './engine.js';

const PINES = ['nature/tree_pineTallA_detailed', 'nature/tree_pineDefaultA', 'nature/tree_pineTallB_detailed', 'nature/tree_pineRoundA', 'nature/tree_pineTallC_detailed'];

const ROCK_TONES = ['#5a4a3e', '#645344', '#4e4036', '#6d5c4c'];

const TUFTS = ['nature/grass_large', 'nature/grass_leafs', 'nature/plant_bushSmall', 'nature/flower_yellowB', 'nature/flower_purpleB', 'nature/grass_leafsLarge', 'nature/flower_redA', 'nature/mushroom_redGroup'];

export function trees(ctx, parent, lvl, angles, inset = .45, scale = 1, avoid = () => false) {
    for (const [i, a] of angles.entries()) {
        const [x, z] = at(lvl.fn, lvl.c, a, inset + (i % 2) * .25);
        if (avoid(x, z)) continue;
        parent.userData.reserve?.(x, z, .22 * scale);
        ctx.place(PINES[i % PINES.length], x, lvl.top, z, a * 7, scale * (1 + (i % 3) * .15), parent);
    }
}

function rockGeometry(seed) {
    const geo = new THREE.IcosahedronGeometry(1, 0);
    const p = geo.attributes.position;
    for (let i = 0; i < p.count; i++) {
        const x = p.getX(i);
        const y = p.getY(i);
        const z = p.getZ(i);
        // Scaling by position keeps the duplicated vertices of neighbouring faces welded.
        const k = .75 + hash(Math.round(x * 100), Math.round(y * 100) + seed * 17, Math.round(z * 100)) * .5;
        p.setXYZ(i, x * k, y * k, z * k);
    }
    geo.computeVertexNormals();
    return geo;
}

export function rockKit(ctx) {
    ctx.rockKit ??= { geos: [0, 1, 2].map(rockGeometry), mats: ROCK_TONES.map((c) => ctx.flat(c)) };
    return ctx.rockKit;
}

/* Boulders set into the cliff wall, with pebbles along the rim, so the edge reads as rock. */
export function cliffRocks(ctx, parent, lvl, top, height, n, skip = null) {
    const { geos, mats } = rockKit(ctx);
    for (let i = 0; i < n; i++) {
        const a = (i + hash(i, n, 3) * .6) / n * Math.PI * 2;
        if (skip !== null && Math.abs(angDiff(a, skip)) < .4) continue;
        const k = hash(i, n, 7);
        const size = (.16 + k * .2) * Math.min(1, height * 1.4);
        const [x, z] = at(lvl.fn, lvl.c, a, -size * .3);
        const rock = ctx.add(geos[i % 3], mats[(i * 3) % 4], x, top - height * (.3 + hash(i, 2, n) * .5), z, parent);
        rock.scale.set(size * (1.1 + k * .5), size * (.7 + hash(n, i, 1) * .35), size * (.9 + k * .3));
        rock.rotation.set(hash(i, 1, 1) * 3, a + hash(i, 4, 4), hash(i, 5, 5) * 3);
        if (i % 3 === 0 && (skip === null || Math.abs(angDiff(a + .1, skip)) >= .4)) {
            const pebble = .06 + k * .05;
            const [px, pz] = at(lvl.fn, lvl.c, a + .1, .04);
            ctx.add(geos[(i + 1) % 3], mats[(i + 1) % 4], px, top - .02, pz, parent).scale.set(pebble * 1.3, pebble * .7, pebble);
        }
    }
}

export function tufts(ctx, parent, lvl, n, maxInset, avoid = () => false) {
    for (let i = 0; i < n; i++) {
        const a = i * 2.39 + lvl.top;
        const [x, z] = at(lvl.fn, lvl.c, a, .25 + (i % 5) / 5 * maxInset);
        if (avoid(x, z)) continue;
        ctx.place(TUFTS[i % TUFTS.length], x, lvl.top, z, i * 1.3, 1.25, parent);
    }
}
