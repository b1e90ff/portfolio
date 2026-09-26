import * as THREE from 'three';
import { angDiff, at, hash, island, radiusFn } from './engine.js';
import * as P from './props.js';

const BAY = 1.05;

function reserver(group) {
    const occ = [];
    group.userData.occ = occ;
    group.userData.reserve = (x, z, r) => occ.push({ x, z, r });
    return occ;
}

export function basecamp(ctx, o) {
    const isl = island(ctx, {
        x: o.x,
        y: o.y,
        z: o.z,
        depth: 3.2,
        levels: [
            { r0: 3.45, amp: .2, seed: .5, h: .91, bay: { a: BAY, depth: 1.25, width: .3 } },
            { c: { x: -.75, z: -1.1 }, r0: 1.95, amp: .15, seed: 2, h: 1 },
        ],
    });
    const g = isl.group;
    const [T, U] = isl.levels;
    const occ = reserver(g);
    occ.push({ x: U.c.x, z: U.c.z, r: 2.05 });

    const [sx, sz] = at(U.fn, U.c, .95, -.02);
    const [ex, ez] = at(T.fn, T.c, BAY, -.05);
    P.waterfall(ctx, g, sx, sz, U.top - .02, T.top + .02, .95, .5);
    P.waterfall(ctx, g, ex, ez, T.top + .02, -2.4, BAY, .62);
    const len = Math.hypot(ex - sx, ez - sz);
    const water = new THREE.MeshStandardMaterial({ color: '#8fc4e6', emissive: '#1f5a80', emissiveIntensity: .3, roughness: .2, transparent: true, opacity: .9 });
    const stream = new THREE.Mesh(new THREE.PlaneGeometry(len, .5), water);
    stream.rotation.x = -Math.PI / 2;
    stream.rotation.z = -Math.atan2(ez - sz, ex - sx);
    stream.position.set((sx + ex) / 2, T.top + .07, (sz + ez) / 2);
    g.add(stream);
    for (let i = 0; i <= 6; i++) {
        const k = i / 6;
        occ.push({ x: sx + (ex - sx) * k, z: sz + (ez - sz) * k, r: .38 });
    }

    P.house(ctx, g, -1.25, U.top, -1.35, .35);
    P.mast(ctx, g, .2, U.top, -2.1);
    const [hx, hz] = at(T.fn, T.c, -.25, .9);
    P.shed(ctx, g, hx, T.top, hz, -1.9);
    occ.push({ x: hx, z: hz, r: .7 });
    ctx.place('nature/bridge_wood', (sx + ex) / 2, T.top - .12, (sz + ez) / 2, -Math.atan2(ez - sz, ex - sx) + Math.PI / 2, .9, g);
    P.edgeRocks(ctx, g, T, 26, -.2, BAY);
    P.edgeRocks(ctx, g, U, 12, T.top - .05);
    P.trees(ctx, g, T, [2.3, 2.7, 3.1, 3.6, 4.2, 4.6, 5.2, 5.8, .3]);
    P.trees(ctx, g, U, [3.4, 4.1, 4.9, 2.6], .35, 1.1);
    const onPlateau = (x, z) => Math.hypot(x - U.c.x, z - U.c.z) < U.fn(Math.atan2(z - U.c.z, x - U.c.x)) + .1;
    P.tufts(ctx, g, T, 16, 1.6, (x, z) => onPlateau(x, z)
        || Math.abs(angDiff(Math.atan2(z, x), BAY)) < .35
        || Math.hypot(x + .2, z - 1.45) < .95
        || Math.hypot(x + 1.35, z - 2.05) < .6);
    P.tufts(ctx, g, U, 7, .9);
    for (const [x, z, ry] of [[-.6, -.45, .3], [0, -.2, .6]]) ctx.place('nature/path_stone', x, U.top + .005, z, ry, 1, g);
    for (const [x, z, ry] of [[-1.3, 1.1, .5], [-.4, 1.2, .1], [.4, 1.25, -.2]]) ctx.place('nature/path_stone', x, T.top + .005, z, ry, 1, g);

    const fx = -1.35;
    const fz = 2.05;
    P.campfire(ctx, g, fx, T.top, fz);
    occ.push({ x: fx, z: fz, r: .5 });
    for (const [dx, dz, ry] of [[-.75, -.25, .3], [.3, .7, 1.9]]) {
        ctx.place('nature/log', fx + dx, T.top, fz + dz, ry, .9, g);
        occ.push({ x: fx + dx, z: fz + dz, r: .32 });
    }
    ctx.place('survival/barrel', fx - 1, T.top, fz + .55, 0, 1.6, g);
    occ.push({ x: fx - 1, z: fz + .55, r: .22 });
    for (const [x, y, z] of [[.35, T.top, 2.6], [-.2, U.top, -.35], [-2.2, T.top, 1.1]]) P.lantern(ctx, g, x, y, z);

    const gold = P.golden(ctx, g, -.1, T.top, 1.4, 0, .8);
    const white = P.whiteDog(ctx, g, -.7, T.top, 1.9, 0, .95);
    const walk = P.area(T, occ, { margin: .5, zone: { x: -.35, z: 1.55, r: 1.45 } });
    P.playTogether(ctx, walk, [[gold, { r: .24, speed: 1.1 }], [white, { r: .17, speed: 1.25 }]], T.top);
    bob(ctx, g, o.y, .0008, .08, 0);
    return isl;
}

/* Motifs face local +z because that is the side the camera lands on. */
export function mini(ctx, o) {
    const r = o.r;
    const levels = [{ r0: r, amp: .12, seed: o.seed, h: .5 }];
    if (o.plateau) levels.push({ c: { x: -.1, z: -r * .56 }, r0: r * .34, amp: .06, seed: o.seed + 3, h: .38 });
    const isl = island(ctx, { x: o.x, y: o.y, z: o.z, depth: 1.9, levels });
    const g = isl.group;
    const [L, U] = isl.levels;
    g.rotation.y = o.front;
    const occ = reserver(g);
    const blocked = (pad) => (x, z) => occ.some((c) => Math.hypot(x - c.x, z - c.z) < c.r + pad);
    if (U) occ.push({ x: U.c.x, z: U.c.z, r: U.fn(0) });
    MOTIFS[o.motif](ctx, g, { y: L.top, yu: U?.top, r, seed: o.seed, labels: o.labels });
    P.edgeRocks(ctx, g, L, 11, -.15);
    if (U) {
        P.edgeRocks(ctx, g, U, 7, L.top - .05, Math.PI / 2);
        P.tufts(ctx, g, U, 4, .3);
    }
    P.tufts(ctx, g, L, 12, r * .55, blocked(.12));
    const backTrees = [3.5, 4.2, 4.9, 5.6, .15, 2.95];
    if (U && o.plateauTrees !== false) P.trees(ctx, g, U, backTrees.slice(0, 2), .15, .7);
    P.trees(ctx, g, L, U ? backTrees.slice(2) : backTrees, .32, .78, blocked(.3));
    for (const [x, z, ry] of [[0, r * .8, .2], [.12, r * .55, -.4]]) {
        if (!blocked(.05)(x, z)) ctx.place('nature/path_stone', x, L.top + .005, z, ry, .8, g);
    }
    bob(ctx, g, o.y, .0009, .12, hash(o.x, o.z, 1) * 6);
    return isl;
}

function bob(ctx, group, baseY, rate, amp, phase) {
    ctx.onFrame((t) => {
        if (!ctx.reduced) group.position.y = baseY + Math.sin(t * rate + phase) * amp;
    });
}

const MOTIFS = {
    projects(ctx, g, { y, yu }) {
        const R = g.userData.reserve;
        ctx.place('survival/workbench', 0, y, -.05, 0, 1.9, g);
        R(0, -.05, .55);
        ctx.place('survival/tool-hammer', .1, y + .38, -.05, .4, 1.8, g);
        ctx.place('survival/box-large', -.8, y, .25, .3, 1.7, g);
        ctx.place('survival/box', -.78, y + .38, .28, .8, 1.5, g);
        R(-.8, .25, .35);
        ctx.place('survival/box', -1.12, y, .7, -.3, 1.4, g);
        R(-1.12, .7, .25);
        ctx.place('town/cart', .85, y, .45, -.7, .95, g);
        R(.85, .45, .5);
        P.lantern(ctx, g, 1.05, y, -.25);
        if (yu) {
            ctx.place('survival/barrel', -.35, yu, -1.2, 0, 1.5, g);
            ctx.place('survival/barrel', .05, yu, -1.35, 0, 1.4, g);
            ctx.place('survival/resource-planks', .3, yu, -.95, .5, 1.4, g);
        }
    },
    about(ctx, g, { y, yu, r, seed }) {
        const R = g.userData.reserve;
        if (yu) ctx.place('survival/tent-canvas', -.1, yu, -1.22, 0, 2, g);
        P.campfire(ctx, g, -.35, y, .35);
        R(-.35, .35, .45);
        for (const [x, z, ry] of [[-.95, .2, .2], [-.3, -.2, 1.6]]) {
            ctx.place('nature/log', x, y, z, ry, .8, g);
            R(x, z, .3);
        }
        P.whiteDog(ctx, g, -1.05, y, .8, .35, .9);
        R(-1.05, .8, .3);
        P.lantern(ctx, g, -1.25, y, -.35);
        if (yu) ctx.place('survival/resource-wood', .45, yu, -1.05, .3, 1.4, g);
        const dog = P.golden(ctx, g, .9, y, .5, 2.6, .8);
        const ground = { fn: radiusFn({ r0: r, amp: .12, seed }), c: { x: 0, z: 0 } };
        P.playFetch(ctx, g, P.area(ground, g.userData.occ, { margin: .5, zone: { x: .45, z: .55, r: 1.45 } }), dog, y);
    },
    experience(ctx, g, { y, yu, labels }) {
        const R = g.userData.reserve;
        P.signpost(ctx, g, 0, y, -.25, labels, 0);
        R(0, -.25, .8);
        ctx.place('survival/chest', .7, y, .1, -.5, 1.5, g);
        R(.7, .1, .3);
        for (const [x, z, ry] of [[-.55, .3, .6], [-.8, -.1, 1.1]]) ctx.place('nature/path_stone', x, y + .005, z, ry, .8, g);
        P.lantern(ctx, g, -.75, y, -.5);
        R(.8, -.55, .25);
        ctx.place('nature/statue_obelisk', .8, y, -.55, .4, .9, g);
        if (yu) ctx.place('survival/signpost-single', -.2, yu, -.9, .3, 1.6, g);
    },
    contact(ctx, g, { y, yu }) {
        const R = g.userData.reserve;
        g.userData.mailbox = P.mailbox(ctx, g, 0, y, .15, 0);
        R(0, .15, .3);
        R(.55, 1.6, .5);
        P.pier(ctx, g, .55, y - .02, 1.1, 3);
        ctx.place('town/stall-bench', -.65, y, -.25, .3, 1, g);
        R(-.65, -.25, .45);
        P.lantern(ctx, g, .7, y, -.35);
        for (const [i, n] of ['nature/flower_redA', 'nature/flower_yellowB', 'nature/flower_purpleB'].entries()) {
            ctx.place(n, -.3 + hash(i, 4, 2) * .6, y, .5 + hash(i, 9, 5) * .3, hash(i, 1, 7) * 6, 1.3, g);
        }
        if (yu) ctx.place('survival/bucket', -.1, yu, -.85, 0, 1.6, g);
    },
};
