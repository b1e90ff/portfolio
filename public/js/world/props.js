import * as THREE from 'three';
import { angDiff, at, hash } from './engine.js';

export function house(ctx, parent, x, y, z, ry = 0) {
    const h = new THREE.Group();
    h.position.set(x, y, z);
    h.rotation.y = ry;
    parent.add(h);
    for (const [i, cx] of [-.5, .5].entries()) {
        ctx.place('town/wall-wood', cx, 0, 0, Math.PI / 2, 1, h);
        ctx.place(i === 0 ? 'town/wall-wood-window-glass' : 'town/wall-wood-door', cx, 0, 0, -Math.PI / 2, 1, h);
        ctx.place('town/roof-gable', cx, 1, 0, Math.PI / 2, 1, h);
    }
    ctx.place('town/wall-wood-window-glass', -.5, 0, 0, Math.PI, 1, h);
    ctx.place('town/wall-wood', .5, 0, 0, 0, 1, h);
    ctx.place('town/chimney', .5, 1, -.1, 0, 1, h);
    ctx.glowQuad(h, -.5, .55, .515, 0, .32, .34);
    ctx.glowQuad(h, -1.015, .55, 0, -Math.PI / 2, .32, .34);
    ctx.windowLight(-.5, .55, .95, h);
    ctx.windowLight(-1.4, .55, 0, h);
    smoke(ctx, h, .82, 2.05, -.1);
    return h;
}

function smoke(ctx, parent, x, y, z) {
    const mat = ctx.flat('#b9b4ae', { transparent: true, opacity: .75 });
    const puffs = Array.from({ length: 5 }, (_, i) => {
        const m = new THREE.Mesh(new THREE.IcosahedronGeometry(.1, 0), mat.clone());
        m.userData.o = i / 5;
        parent.add(m);
        return m;
    });
    ctx.onFrame((t) => puffs.forEach((m) => {
        const p = ctx.reduced ? m.userData.o : (t * .00016 + m.userData.o) % 1;
        m.position.set(x + Math.sin(p * 5) * .12, y + p * 1.6, z);
        m.scale.setScalar(.7 + p * 1.6);
        m.material.opacity = .75 * (1 - p);
    }));
}

export function mast(ctx, parent, x, y, z) {
    const m = new THREE.Group();
    m.position.set(x, y, z);
    parent.add(m);
    const steel = ctx.flat('#8a8f9c');
    ctx.add(new THREE.BoxGeometry(.7, .12, .7), ctx.flat('#6d6a70'), 0, .06, 0, m);
    for (const [lx, lz] of [[-.22, -.22], [.22, -.22], [.22, .22], [-.22, .22]]) {
        const leg = ctx.add(new THREE.CylinderGeometry(.025, .03, 2.6, 5), steel, lx * .55, 1.35, lz * .55, m);
        leg.rotation.z = -lx * .12;
        leg.rotation.x = lz * .12;
    }
    for (let i = 0; i < 6; i++) {
        const yy = .35 + i * .38;
        const w = .44 - i * .045;
        for (const b of [w / 2, -w / 2]) ctx.add(new THREE.BoxGeometry(w, .025, .025), steel, 0, yy, b, m);
        for (const a of [w / 2, -w / 2]) ctx.add(new THREE.BoxGeometry(.025, .025, w), steel, a, yy, 0, m);
    }
    ctx.blinkers.push({ m: ctx.add(new THREE.SphereGeometry(.07, 8, 6), ctx.emissive('#ff4a3a', 2.5), 0, 2.72, 0, m), rate: .0035, ph: 0 });
    const dish = ctx.add(new THREE.SphereGeometry(.34, 10, 5, 0, Math.PI * 2, 0, Math.PI / 2.6), ctx.flat('#d9d6d0', { side: THREE.DoubleSide }), .12, 1.7, .28, m);
    dish.rotation.set(-1.2, .5, 0);
    return m;
}

export function shed(ctx, parent, x, y, z, ry = 0) {
    const s = new THREE.Group();
    s.position.set(x, y, z);
    s.rotation.y = ry;
    parent.add(s);
    ctx.add(new THREE.BoxGeometry(1, .75, .75), ctx.flat('#3b4250'), 0, .375, 0, s);
    ctx.add(new THREE.BoxGeometry(1.12, .08, .9), ctx.flat('#262a33'), 0, .79, 0, s).rotation.x = .08;
    ctx.add(new THREE.BoxGeometry(.3, .5, .02), ctx.flat('#262a33'), -.28, .25, .38, s);
    for (let i = 0; i < 4; i++) {
        const led = ctx.add(new THREE.BoxGeometry(.05, .03, .02), ctx.emissive(i % 2 ? '#6fd08c' : '#e2be6a', 2.5), .08 + i * .08, .55, .385, s);
        ctx.blinkers.push({ m: led, rate: .006 + i * .0013, ph: i * 1.7 });
    }
    ctx.add(new THREE.BoxGeometry(.4, .05, .02), ctx.flat('#11141a'), .2, .45, .385, s);
    ctx.add(new THREE.BoxGeometry(.4, .05, .02), ctx.flat('#11141a'), .2, .38, .385, s);
    ctx.glowSpot(.2, .6, .7, 0x8fe0a8, .8, 1.6, s);
    return s;
}

export function golden(ctx, parent, x, y, z, ry = 0, s = 1) {
    const g = new THREE.Group();
    g.position.set(x, y, z);
    g.rotation.y = ry;
    g.scale.setScalar(s);
    parent.add(g);
    const fur = ctx.flat('#d9a441');
    const furLight = ctx.flat('#ecc374');
    const dark = ctx.flat('#1a1210');
    const lift = .2;
    const body = new THREE.Group();
    g.add(body);
    ctx.add(new THREE.BoxGeometry(.62, .26, .3), fur, 0, .15 + lift, 0, body);
    ctx.add(new THREE.BoxGeometry(.2, .24, .32), furLight, .28, .18 + lift, 0, body);
    const head = new THREE.Group();
    head.position.set(.42, .36 + lift, 0);
    body.add(head);
    ctx.add(new THREE.BoxGeometry(.24, .22, .24), fur, 0, 0, 0, head);
    ctx.add(new THREE.BoxGeometry(.13, .1, .14), furLight, .16, -.04, 0, head);
    ctx.add(new THREE.BoxGeometry(.04, .04, .06), dark, .23, 0, 0, head);
    for (const side of [-1, 1]) {
        ctx.add(new THREE.BoxGeometry(.1, .16, .04), ctx.flat('#c08a32'), -.02, -.05, side * .14, head).rotation.x = side * .25;
        ctx.add(new THREE.BoxGeometry(.012, .035, .035), dark, .121, .05, side * .065, head);
    }
    const legs = [[.2, -.1], [.2, .1], [-.22, -.1], [-.22, .1]].map(([lx, lz]) => {
        const hip = new THREE.Group();
        hip.position.set(lx, .28, lz);
        g.add(hip);
        ctx.add(new THREE.BoxGeometry(.08, .28, .08), fur, 0, -.14, 0, hip);
        return hip;
    });
    const tail = new THREE.Group();
    tail.position.set(-.3, .2 + lift, 0);
    body.add(tail);
    ctx.add(new THREE.BoxGeometry(.3, .09, .1), furLight, -.14, 0, 0, tail).rotation.z = .6;
    ctx.wags.push({ o: tail, rate: .005, amp: .35 });
    g.userData = { legs, body, head };
    return g;
}

/* Cardea: a small, fox-like husky mix with one blue and one brown eye. */
export function husky(ctx, parent, x, y, z, ry = 0, s = 1) {
    const g = new THREE.Group();
    g.position.set(x, y, z);
    g.rotation.y = ry;
    g.scale.setScalar(s);
    parent.add(g);
    const { fur, furShade, tan, dark } = huskyCoat(ctx);
    ctx.add(new THREE.BoxGeometry(.34, .16, .17), fur, 0, .24, 0, g);
    ctx.add(new THREE.BoxGeometry(.1, .15, .15), furShade, .14, .23, 0, g);
    const legs = [[.11, -.05], [.11, .05], [-.11, -.05], [-.11, .05]].map(([lx, lz]) => {
        const hip = new THREE.Group();
        hip.position.set(lx, .16, lz);
        g.add(hip);
        ctx.add(new THREE.BoxGeometry(.05, .16, .05), furShade, 0, -.08, 0, hip);
        return hip;
    });
    const head = new THREE.Group();
    head.position.set(.22, .37, 0);
    g.add(head);
    ctx.add(new THREE.BoxGeometry(.16, .14, .15), fur, 0, 0, 0, head);
    ctx.add(new THREE.BoxGeometry(.09, .06, .07), furShade, .115, -.035, 0, head);
    ctx.add(new THREE.BoxGeometry(.025, .025, .035), dark, .165, -.02, 0, head);
    for (const side of [-1, 1]) {
        ctx.add(new THREE.ConeGeometry(.038, .11, 4), fur, -.02, .12, side * .05, head).rotation.x = -side * .12;
        ctx.add(new THREE.ConeGeometry(.02, .05, 4), tan, -.02, .155, side * .054, head).rotation.x = -side * .12;
    }
    ctx.add(new THREE.BoxGeometry(.01, .02, .022), ctx.flat('#5d88a8'), .081, .032, -.038, head);
    ctx.add(new THREE.BoxGeometry(.01, .02, .022), ctx.flat('#4a3222'), .081, .032, .038, head);
    const tail = new THREE.Group();
    tail.position.set(-.17, .3, 0);
    g.add(tail);
    [[0, .04, .07, fur], [.02, .1, .07, fur], [.07, .13, .065, fur], [.12, .12, .05, tan]].forEach(([tx, ty, size, mat]) => {
        ctx.add(new THREE.BoxGeometry(size, size, size), mat, tx, ty, 0, tail);
    });
    ctx.wags.push({ o: tail, rate: .016, amp: .35, axis: 'x' }, { o: head, rate: .0023, amp: .25 });
    g.userData = { legs, head };
    return g;
}

function huskyCoat(ctx) {
    ctx.huskyCoat ??= { fur: ctx.flat('#f4f1ec'), furShade: ctx.flat('#e2dcd2'), tan: ctx.flat('#e3b683'), dark: ctx.flat('#1a1210') };
    return ctx.huskyCoat;
}

/* Curled-up sleeper that breathes and lets z's drift up; its nose points to local +x. */
export function sleepingHusky(ctx, parent, x, y, z, ry = 0, s = 1) {
    const g = new THREE.Group();
    g.position.set(x, y, z);
    g.rotation.y = ry;
    g.scale.setScalar(s);
    parent.add(g);
    const { fur, furShade, tan, dark } = huskyCoat(ctx);
    const body = new THREE.Group();
    g.add(body);
    ctx.add(new THREE.BoxGeometry(.26, .12, .15), fur, 0, .06, 0, body);
    ctx.add(new THREE.BoxGeometry(.09, .04, .04), furShade, .16, .02, .02, g);
    ctx.add(new THREE.BoxGeometry(.09, .04, .04), furShade, .16, .02, .08, g);
    const head = new THREE.Group();
    head.position.set(.17, .08, .06);
    head.rotation.y = -.55;
    g.add(head);
    ctx.add(new THREE.BoxGeometry(.14, .1, .12), fur, 0, 0, 0, head);
    ctx.add(new THREE.BoxGeometry(.07, .05, .07), furShade, .09, -.02, 0, head);
    ctx.add(new THREE.BoxGeometry(.025, .025, .035), dark, .13, -.01, 0, head);
    for (const side of [-1, 1]) {
        ctx.add(new THREE.ConeGeometry(.03, .08, 4), fur, -.035, .06, side * .04, head).rotation.z = .9;
        ctx.add(new THREE.ConeGeometry(.016, .035, 4), tan, -.065, .085, side * .04, head).rotation.z = .9;
        ctx.add(new THREE.BoxGeometry(.01, .007, .025), dark, .07, .02, side * .032, head);
    }
    ctx.add(new THREE.BoxGeometry(.18, .045, .045), fur, -.02, .03, .1, g).rotation.y = -.5;
    const zTex = zTexture(ctx);
    const zs = [0, 1, 2].map((i) => {
        const sprite = new THREE.Sprite(new THREE.SpriteMaterial({ map: zTex, transparent: true, depthWrite: false }));
        sprite.userData.offset = i / 3;
        g.add(sprite);
        return sprite;
    });
    ctx.onFrame((t) => {
        const breath = ctx.reduced ? 0 : Math.sin(t * .0022);
        body.scale.y = 1 + breath * .06;
        head.position.y = .08 + breath * .004;
        zs.forEach((sprite) => {
            const k = (t * .00025 + sprite.userData.offset) % 1;
            sprite.visible = !ctx.reduced;
            sprite.position.set(.2 + k * .12, .2 + k * .45, .06 + Math.sin(k * 6) * .04);
            sprite.scale.setScalar(.06 + k * .08);
            sprite.material.opacity = Math.sin(k * Math.PI) * .9;
        });
    });
    return g;
}

function zTexture(ctx) {
    const c = document.createElement('canvas');
    c.width = 64;
    c.height = 64;
    const tex = new THREE.CanvasTexture(c);
    const font = '700 52px "Bricolage Grotesque", sans-serif';
    ctx.track(document.fonts.load(font).catch(() => []).then(() => {
        const g = c.getContext('2d');
        g.fillStyle = '#f5f1ea';
        g.font = font;
        g.textAlign = 'center';
        g.textBaseline = 'middle';
        g.fillText('z', 32, 34);
        tex.needsUpdate = true;
    }));
    return tex;
}

const LEG_PHASE = [0, Math.PI, Math.PI, 0];

/* Accumulates the stride phase so changing speed never makes the legs jump. */
function gait(dog, dt, speed, rate = .02) {
    const d = dog.userData;
    d.stride = ((d.stride || 0) + dt * 1000 * rate * Math.max(speed, .15)) % (Math.PI * 2);
    d.legs.forEach((l, i) => { l.rotation.z = Math.sin(d.stride + LEG_PHASE[i]) * .7 * speed; });
    if (d.body) d.body.position.y = Math.abs(Math.sin(d.stride)) * .04 * speed;
}

/* Walkable area in island space: level outline minus reserved circles, clipped to a zone. */
export function area(lvl, occ, o = {}) {
    const margin = o.margin ?? .45;
    const zone = o.zone;
    const hole = o.hole;
    const limit = (x, z, r) => lvl.fn(Math.atan2(z - lvl.c.z, x - lvl.c.x)) - margin - r;
    const holeLimit = (x, z, r) => hole.lvl.fn(Math.atan2(z - hole.lvl.c.z, x - hole.lvl.c.x)) + hole.pad + r;
    const valid = (x, z, r = .2) => Math.hypot(x - lvl.c.x, z - lvl.c.z) < limit(x, z, r)
        && (!zone || Math.hypot(x - zone.x, z - zone.z) < zone.r - r)
        && (!hole || Math.hypot(x - hole.lvl.c.x, z - hole.lvl.c.z) > holeLimit(x, z, r))
        && !occ.some((c) => Math.hypot(x - c.x, z - c.z) < c.r + r);
    const bounds = zone || { x: lvl.c.x, z: lvl.c.z, r: lvl.fn(0) };
    const anywhere = (r) => {
        for (let i = 0; i < 60; i++) {
            const a = Math.random() * Math.PI * 2;
            const d = Math.sqrt(Math.random()) * bounds.r;
            const x = bounds.x + Math.cos(a) * d;
            const z = bounds.z + Math.sin(a) * d;
            if (valid(x, z, r)) return new THREE.Vector3(x, 0, z);
        }
        return null;
    };
    const sample = (near, minD, maxD, r = .25) => {
        for (let i = 0; i < 60; i++) {
            const a = Math.random() * Math.PI * 2;
            const d = minD + Math.random() * (maxD - minD);
            const x = near.x + Math.cos(a) * d;
            const z = near.z + Math.sin(a) * d;
            if (valid(x, z, r)) return new THREE.Vector3(x, 0, z);
        }
        return anywhere(r) || near.clone();
    };
    const findCenter = (r) => {
        for (let d = 0; d <= bounds.r; d += .15) {
            for (let k = 0; k < 12; k++) {
                const a = k / 12 * Math.PI * 2;
                const x = bounds.x + Math.cos(a) * d;
                const z = bounds.z + Math.sin(a) * d;
                if (valid(x, z, r)) return new THREE.Vector3(x, 0, z);
            }
        }
        return null;
    };
    const center = findCenter(.3) || new THREE.Vector3(bounds.x, 0, bounds.z);
    const openCenter = findCenter(.9) || center;
    const bridge = o.bridge;
    const heightAt = (x, z) => {
        if (!bridge) return 0;
        const dx = x - bridge.x;
        const dz = z - bridge.z;
        const across = dx * bridge.cx + dz * bridge.cz;
        const along = dx * bridge.sx + dz * bridge.sz;
        if (Math.abs(along) > bridge.halfWidth || Math.abs(across) > bridge.halfLength) return 0;
        return bridge.height * Math.cos(across / bridge.halfLength * Math.PI / 2);
    };
    const crossesBarrier = (from, to) => {
        const b = o.barrier;
        if (!b) return false;
        const rx = to.x - from.x;
        const rz = to.z - from.z;
        const sx = b.bx - b.ax;
        const sz = b.bz - b.az;
        const den = rx * sz - rz * sx;
        if (Math.abs(den) < 1e-6) return false;
        const t = ((b.ax - from.x) * sz - (b.az - from.z) * sx) / den;
        const u = ((b.ax - from.x) * rz - (b.az - from.z) * rx) / den;
        return t > 0 && t < 1 && u > 0 && u < 1 && (u < b.gapFrom || u > b.gapTo);
    };
    /* Detours through the barrier gap when the straight line to the target would cross it. */
    const route = (from, to, out) => (crossesBarrier(from, to) ? out.set(o.barrier.gapX, 0, o.barrier.gapZ) : out.copy(to));
    const clampCircle = (p, cx, cz, rad, inside) => {
        const dx = p.x - cx;
        const dz = p.z - cz;
        const d = Math.hypot(dx, dz);
        if (d < 1e-4) return;
        if (inside ? d > rad : d < rad) {
            p.x = cx + dx / d * rad;
            p.z = cz + dz / d * rad;
        }
    };
    const resolve = (p, r) => {
        occ.forEach((c) => clampCircle(p, c.x, c.z, c.r + r, false));
        if (hole) clampCircle(p, hole.lvl.c.x, hole.lvl.c.z, holeLimit(p.x, p.z, r), false);
        clampCircle(p, lvl.c.x, lvl.c.z, limit(p.x, p.z, r), true);
        if (zone) clampCircle(p, zone.x, zone.z, zone.r - r, true);
    };
    return { valid, sample, resolve, occ, center, openCenter, heightAt, route };
}

function agent(dog, walk, o) {
    return {
        dog,
        area: walk,
        pos: dog.position.clone(),
        vel: new THREE.Vector3(),
        want: new THREE.Vector3(),
        r: o.r,
        base: o.speed,
        max: o.speed,
        heading: dog.rotation.y,
        y: o.y,
        pitch: 0,
        pitchTarget: 0,
        lift: 0,
        vy: 0,
        wander: Math.random() * Math.PI * 2,
        look: null,
        pause: 0,
        burst: 0,
    };
}

function flee(runner, from) {
    runner.wander = Math.atan2(runner.pos.z - from.pos.z, runner.pos.x - from.pos.x);
    runner.burst = 1.1;
}

function hop(a, strength = 1.6) {
    if (a.lift === 0) a.vy = strength;
}

/* Seek with arrival, separation and obstacle repulsion, then a hard collision resolve. */
function steer(a, target, dt, others = [], arrive = .35) {
    const want = a.want.set(target.x - a.pos.x, 0, target.z - a.pos.z);
    const d = want.length();
    if (d > 1e-3) want.multiplyScalar(Math.min(1, d / arrive) * a.max / d);
    for (const b of others) {
        const dx = a.pos.x - b.pos.x;
        const dz = a.pos.z - b.pos.z;
        const dd = Math.hypot(dx, dz);
        const min = a.r + b.r + .3;
        if (dd < min && dd > 1e-4) {
            want.x += dx / dd * (min - dd) * 5;
            want.z += dz / dd * (min - dd) * 5;
        }
    }
    for (const c of a.area.occ) {
        const dx = a.pos.x - c.x;
        const dz = a.pos.z - c.z;
        const dd = Math.hypot(dx, dz);
        const near = c.r + a.r + .35;
        if (dd < near && dd > 1e-4) {
            const k = (near - dd) / .35 * a.max;
            want.x += dx / dd * k;
            want.z += dz / dd * k;
        }
    }
    a.vel.lerp(want, Math.min(1, dt * 4));
    if (a.vel.length() > a.max) a.vel.setLength(a.max);
    a.pos.addScaledVector(a.vel, dt);
    a.area.resolve(a.pos, a.r);
    const speed = a.vel.length();
    if (speed > .06) a.heading += angDiff(Math.atan2(-a.vel.z, a.vel.x), a.heading) * Math.min(1, dt * 7);
    else if (a.look) a.heading += angDiff(Math.atan2(-(a.look.z - a.pos.z), a.look.x - a.pos.x), a.heading) * Math.min(1, dt * 5);
    a.vy -= 9 * dt;
    a.lift = Math.max(0, a.lift + a.vy * dt);
    if (a.lift === 0) a.vy = 0;
    a.pitch += (a.pitchTarget - a.pitch) * Math.min(1, dt * 8);
    a.dog.position.set(a.pos.x, a.y + a.lift + a.area.heightAt(a.pos.x, a.pos.z), a.pos.z);
    a.dog.rotation.y = a.heading;
    a.dog.rotation.z = a.pitch;
    return { d, speed: speed / a.base };
}

/* Smooth random roaming: the travel direction drifts randomly and turns back inside near the edge. */
function roam(a, out, dt, jitter = 2.5) {
    a.wander += (Math.random() - .5) * jitter * dt * 2;
    const dx = Math.cos(a.wander);
    const dz = Math.sin(a.wander);
    if (!a.area.valid(a.pos.x + dx * .8, a.pos.z + dz * .8, a.r)) {
        let best = Math.atan2(a.area.center.z - a.pos.z, a.area.center.x - a.pos.x);
        let bestTurn = Infinity;
        for (let k = 0; k < 16; k++) {
            const cand = a.wander + (k / 16 - .5) * Math.PI * 2;
            const turn = Math.abs(angDiff(cand, a.wander));
            if (turn < bestTurn && a.area.valid(a.pos.x + Math.cos(cand) * .8, a.pos.z + Math.sin(cand) * .8, a.r)) {
                best = cand;
                bestTurn = turn;
            }
        }
        a.wander += angDiff(best, a.wander) * Math.min(1, dt * 6);
    }
    return out.set(a.pos.x + Math.cos(a.wander) * 1.4, 0, a.pos.z + Math.sin(a.wander) * 1.4);
}

const PLAY_NEXT = [['bow', .5], ['circle', .2], ['sniff', .18], ['rest', .12]];

/* Play loop: a play bow starts a chase, a catch swaps roles; circling, sniffing and rests in between. */
export function playTogether(ctx, walk, specs, y) {
    const agents = specs.map(([dog, o]) => agent(dog, walk, { ...o, y }));
    const targets = agents.map(() => new THREE.Vector3());
    const chaseGoal = new THREE.Vector3();
    const mid = new THREE.Vector3();
    let mode = 'bow';
    let leader = 0;
    let timer = 1;
    let tagCooldown = 0;
    let orbitAngle = 0;
    let orbitDir = 1;
    const enter = (next) => {
        mode = next;
        agents.forEach((a) => { a.look = null; a.pitchTarget = 0; });
        if (next === 'bow') {
            leader = Math.random() < .5 ? 0 : 1;
            timer = .9;
        } else if (next === 'chase') {
            timer = 6 + Math.random() * 5;
            tagCooldown = 1;
            flee(agents[leader], agents[1 - leader]);
        } else if (next === 'circle') {
            timer = 3 + Math.random() * 2;
            orbitDir = Math.random() < .5 ? -1 : 1;
            mid.lerpVectors(agents[0].pos, agents[1].pos, .5);
            if (!walk.valid(mid.x, mid.z, .9)) mid.copy(walk.openCenter);
            orbitAngle = Math.atan2(agents[0].pos.z - mid.z, agents[0].pos.x - mid.x);
        } else if (next === 'sniff') {
            timer = 3 + Math.random() * 2.5;
        } else {
            timer = 1.8 + Math.random() * 1.6;
        }
    };
    const afterwards = () => {
        if (mode === 'bow') return 'chase';
        if (mode === 'rest') return 'bow';
        let roll = Math.random();
        for (const [name, weight] of PLAY_NEXT) {
            roll -= weight;
            if (roll <= 0) return name;
        }
        return 'bow';
    };
    ctx.onFrame((t, dt) => {
        if (ctx.reduced) return;
        timer -= dt;
        tagCooldown -= dt;
        if (timer <= 0) enter(afterwards());
        const lead = agents[leader];
        const chaser = agents[1 - leader];
        if (mode === 'chase' && tagCooldown <= 0 && lead.pos.distanceTo(chaser.pos) < lead.r + chaser.r + .38) {
            leader = 1 - leader;
            tagCooldown = 2.4;
            hop(lead);
            hop(chaser, 1.2);
            lead.pause = .7;
            flee(chaser, lead);
        }
        if (mode === 'circle') orbitAngle += orbitDir * dt * 1.6;
        agents.forEach((a, i) => {
            const other = agents[1 - i];
            const target = targets[i];
            a.pause = Math.max(0, a.pause - dt);
            a.burst = Math.max(0, a.burst - dt);
            if (mode === 'bow') {
                target.copy(a.pos);
                a.max = a.base * .2;
                a.look = other.pos;
                a.pitchTarget = i === leader ? -.3 : -.12;
                if (i !== leader && Math.random() < dt * 1.5) hop(a, 1);
            } else if (mode === 'chase' && i === leader) {
                roam(a, target, dt);
                a.max = a.base * (a.burst > 0 ? 1.4 : 1.05);
            } else if (mode === 'chase') {
                chaseGoal.copy(other.pos).addScaledVector(other.vel, .45);
                walk.route(a.pos, chaseGoal, target);
                a.max = a.base * 1.15;
            } else if (mode === 'circle') {
                const ang = orbitAngle + i * Math.PI;
                chaseGoal.set(mid.x + Math.cos(ang) * .6, 0, mid.z + Math.sin(ang) * .6);
                walk.route(a.pos, chaseGoal, target);
                a.max = a.base * .9;
            } else if (mode === 'sniff') {
                roam(a, target, dt, 6);
                a.max = a.base * .3;
                a.pitchTarget = -.1;
            } else {
                target.copy(a.pos);
                a.max = a.base * .2;
                a.look = other.pos;
                a.pitchTarget = .16;
            }
            if (a.pause > 0) a.max = a.base * .15;
            const s = steer(a, target, dt, [other]);
            gait(a.dog, dt, Math.min(1, s.speed * 1.4), .012 + .01 * Math.min(1, s.speed));
        });
    });
}

/* Ball arcs to a free spot and rolls out; the dog chases it, carries it off and drops it. */
export function playFetch(ctx, parent, walk, dog, y) {
    const a = agent(dog, walk, { r: .22, speed: 1.35, y });
    const G = 9.5;
    const R = .06;
    const ball = ctx.add(new THREE.SphereGeometry(R, 14, 10), new THREE.MeshStandardMaterial({ color: '#d7e24a', roughness: .55 }), 0, 0, 0, parent);
    const b = { p: dog.position.clone().add(new THREE.Vector3(.35, 0, 0)), v: new THREE.Vector3(), h: R, vh: 0 };
    const landing = new THREE.Vector3();
    const mouth = new THREE.Vector3();
    let state = 'wait';
    let timer = .8;
    let drop = null;
    const toss = () => {
        landing.copy(walk.sample(b.p, 1.3, 2.4, a.r));
        const tf = .95;
        b.v.set((landing.x - b.p.x) / tf, 0, (landing.z - b.p.z) / tf);
        b.h = .3;
        b.vh = G * tf / 2 - .24 / tf;
        state = 'fly';
        timer = 0;
    };
    const physics = (dt) => {
        b.vh -= G * dt;
        b.p.addScaledVector(b.v, dt);
        b.h += b.vh * dt;
        if (b.h < R) {
            b.h = R;
            if (Math.abs(b.vh) > .7) {
                b.vh = -b.vh * .45;
                b.v.multiplyScalar(.72);
            } else {
                b.vh = 0;
                b.v.multiplyScalar(Math.max(0, 1 - dt * 2.2));
            }
            if (state === 'fly') {
                state = 'chase';
                timer = 0;
            }
        }
        for (const c of walk.occ) {
            const dx = b.p.x - c.x;
            const dz = b.p.z - c.z;
            const d = Math.hypot(dx, dz);
            const m = c.r + a.r;
            if (d < m && d > 1e-4) {
                const nx = dx / d;
                const nz = dz / d;
                const vn = b.v.x * nx + b.v.z * nz;
                b.p.x = c.x + nx * m;
                b.p.z = c.z + nz * m;
                if (vn < 0) {
                    b.v.x -= 1.7 * vn * nx;
                    b.v.z -= 1.7 * vn * nz;
                }
            }
        }
        walk.resolve(b.p, a.r);
    };
    const look = (p) => {
        a.heading += angDiff(Math.atan2(-(p.z - a.pos.z), p.x - a.pos.x), a.heading) * .08;
        a.dog.rotation.y = a.heading;
    };
    const run = (dt, s, rate) => gait(a.dog, dt, Math.min(1, s.speed * 1.3), rate * .65);
    ctx.onFrame((t, dt) => {
        if (ctx.reduced) {
            ball.position.set(b.p.x, y + R, b.p.z);
            return;
        }
        if (state === 'fly' || state === 'chase') physics(dt);
        if (state === 'wait') {
            a.pitchTarget = -.25;
            if (Math.random() < dt * 2) hop(a, 1.1);
            steer(a, a.pos, dt);
            look(b.p);
            gait(a.dog, dt, 0);
            timer -= dt;
            if (timer <= 0) {
                a.pitchTarget = 0;
                toss();
            }
        } else if (state === 'fly') {
            timer += dt;
            run(dt, timer > .25 ? steer(a, landing, dt, [], .2) : steer(a, a.pos, dt), .034);
        } else if (state === 'chase') {
            timer += dt;
            run(dt, steer(a, b.p, dt, [], .12), .034);
            if (timer > 5) {
                state = 'wait';
                timer = .4;
                b.p.copy(a.pos);
                b.v.set(0, 0, 0);
            } else if (a.pos.distanceTo(b.p) < .4 && b.h < .1) {
                state = 'carry';
                drop = walk.sample(a.pos, 1, 1.8, a.r);
            }
        } else if (state === 'carry') {
            a.max = a.base * .7;
            const s = steer(a, drop, dt, [], .3);
            run(dt, s, .026);
            if (s.d < .15) {
                state = 'drop';
                timer = .9;
                a.max = a.base;
            }
        } else {
            steer(a, a.pos, dt);
            gait(a.dog, dt, 0);
            timer -= dt;
            if (timer <= 0) {
                state = 'wait';
                timer = .4 + Math.random() * .5;
            }
        }
        if (state === 'carry' || (state === 'drop' && timer > .6)) {
            a.dog.userData.head.localToWorld(mouth.set(.2, -.07, 0));
            parent.worldToLocal(mouth);
            b.p.set(mouth.x, 0, mouth.z);
            b.h = mouth.y - y;
            b.v.set(0, 0, 0);
            b.vh = 0;
        } else if (state === 'drop' || state === 'wait') {
            b.h = Math.max(R, b.h - dt * 1.5);
        }
        ball.position.set(b.p.x, y + b.h, b.p.z);
    });
}

export function campfire(ctx, parent, x, y, z) {
    const f = new THREE.Group();
    f.position.set(x, y, z);
    parent.add(f);
    ctx.place('survival/campfire-pit', 0, 0, 0, 0, 2.4, f);
    const flames = [[.13, .34, '#ff5a1f'], [.09, .26, '#ff9a2e'], [.05, .17, '#ffe07a']].map(([rad, h, c]) => {
        const m = new THREE.Mesh(new THREE.ConeGeometry(rad, h, 5), ctx.emissive(c, 2.2));
        m.position.y = h / 2 + .05;
        f.add(m);
        return m;
    });
    const light = ctx.glowSpot(0, .5, 0, 0xff8a3d, 2.4, 6, f);
    const base = light.intensity;
    ctx.onFrame((t) => {
        const k = ctx.reduced ? 1 : .85 + .15 * Math.sin(t * .011) * Math.sin(t * .027);
        flames.forEach((m, i) => {
            m.scale.set(1, k + i * .06, 1);
            m.rotation.y = t * .002 * (i + 1);
        });
        light.intensity = base * k;
    });
    return f;
}

export function lantern(ctx, parent, x, y, z) {
    parent.userData.reserve?.(x, z, .16);
    ctx.place('town/lantern', x, y, z, 0, .8, parent);
    ctx.add(new THREE.SphereGeometry(.07, 8, 6), ctx.emissive('#ffc46b', 3), x, y + 1.18, z, parent);
    ctx.glowSpot(x, y + 1.1, z, 0xffb35a, 2, 3.4, parent);
}

export function signpost(ctx, parent, x, y, z, labels, ry = 0) {
    const s = new THREE.Group();
    s.position.set(x, y, z);
    s.rotation.y = ry;
    parent.add(s);
    const wood = ctx.flat('#7a5236');
    ctx.add(new THREE.BoxGeometry(.08, 1.3, .08), wood, 0, .65, 0, s);
    const font = '700 64px "Bricolage Grotesque", sans-serif';
    ctx.track(document.fonts.load(font).catch(() => []).then(() => drawBoards(ctx, s, wood, font, labels)));
    return s;
}

function drawBoards(ctx, s, wood, font, labels) {
    const probe = document.createElement('canvas').getContext('2d');
    probe.font = font;
    labels.forEach((txt, i) => {
        const c = document.createElement('canvas');
        c.width = Math.ceil(probe.measureText(txt).width + 64);
        c.height = 112;
        const g2 = c.getContext('2d');
        g2.fillStyle = '#8a5d3b';
        g2.fillRect(0, 0, c.width, c.height);
        g2.fillStyle = 'rgba(0,0,0,.12)';
        g2.fillRect(0, c.height - 12, c.width, 12);
        g2.fillStyle = '#f5e6c8';
        g2.font = font;
        g2.textBaseline = 'middle';
        g2.fillText(txt, 32, c.height / 2 + 2);
        const tex = new THREE.CanvasTexture(c);
        tex.anisotropy = ctx.r.capabilities.getMaxAnisotropy();
        const h = .19;
        const w = Math.max(.5, h * c.width / c.height);
        const face = new THREE.MeshStandardMaterial({ map: tex, roughness: .8 });
        const board = new THREE.Mesh(new THREE.BoxGeometry(w, h, .04), [wood, wood, wood, wood, face, wood]);
        const side = i % 2 ? -1 : 1;
        board.position.set(side * (w / 2 - .02), 1.02 - i * .26, 0);
        board.rotation.y = side * -.18;
        board.castShadow = true;
        s.add(board);
    });
}

export function mailbox(ctx, parent, x, y, z, ry = 0) {
    const m = new THREE.Group();
    m.position.set(x, y, z);
    m.rotation.y = ry;
    parent.add(m);
    const brass = ctx.flat('#b8913d', { metalness: .4, roughness: .45 });
    ctx.add(new THREE.BoxGeometry(.08, .7, .08), ctx.flat('#7a5236'), 0, .35, 0, m);
    ctx.add(new THREE.BoxGeometry(.4, .26, .26), brass, 0, .82, 0, m);
    ctx.add(new THREE.CylinderGeometry(.13, .13, .4, 12, 1, false, 0, Math.PI), brass, 0, .95, 0, m).rotation.z = Math.PI / 2;
    m.userData.flag = ctx.add(new THREE.BoxGeometry(.03, .18, .08), ctx.flat('#c0392b'), .22, .95, .06, m);
    return m;
}

export function pier(ctx, parent, x, y, z, n = 3) {
    const p = new THREE.Group();
    p.position.set(x, y, z);
    parent.add(p);
    const wood = ctx.flat('#7a5236');
    for (let i = 0; i < n; i++) {
        ctx.add(new THREE.BoxGeometry(.6, .06, .5), wood, 0, 0, i * .52, p);
        ctx.add(new THREE.BoxGeometry(.06, .5, .06), wood, .27, -.22, i * .52, p);
        ctx.add(new THREE.BoxGeometry(.06, .5, .06), wood, -.27, -.22, i * .52, p);
    }
    return p;
}

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

/* Boulders set into the cliff wall, with pebbles along the rim, so the edge reads as rock. */
export function cliffRocks(ctx, parent, lvl, top, height, n, skip = null) {
    ctx.rockKit ??= { geos: [0, 1, 2].map(rockGeometry), mats: ROCK_TONES.map((c) => ctx.flat(c)) };
    const { geos, mats } = ctx.rockKit;
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

function fallTexture() {
    const c = document.createElement('canvas');
    c.width = 64;
    c.height = 256;
    const g = c.getContext('2d');
    g.fillStyle = '#6fb4dc';
    g.fillRect(0, 0, 64, 256);
    for (let i = 0; i < 70; i++) {
        g.fillStyle = `rgba(255,255,255,${.25 + hash(i, 2, 3) * .5})`;
        g.fillRect(hash(i, 5, 1) * 64, hash(i, 7, 9) * 256, 2 + hash(i, 1, 1) * 4, 10 + hash(i, 3, 3) * 40);
    }
    const t = new THREE.CanvasTexture(c);
    t.wrapS = THREE.RepeatWrapping;
    t.wrapT = THREE.RepeatWrapping;
    return t;
}

export function waterfall(ctx, parent, x, z, top, bottom, a, w = .6, spill = 0) {
    const tex = fallTexture();
    const mat = new THREE.MeshStandardMaterial({ map: tex, transparent: true, opacity: .9, emissive: '#2c6f9a', emissiveIntensity: .35, roughness: .2, side: THREE.DoubleSide });
    const m = new THREE.Mesh(new THREE.PlaneGeometry(w, top - bottom), mat);
    m.position.set(x, (top + bottom) / 2, z);
    m.rotation.y = Math.PI / 2 - a;
    parent.add(m);
    ctx.onFrame((t) => { if (!ctx.reduced) tex.offset.y = (t * .0012) % 1; });
    if (spill > 0) {
        const lip = new THREE.Mesh(new THREE.PlaneGeometry(spill, w), mat);
        lip.rotation.set(-Math.PI / 2, 0, -a);
        lip.position.set(x - Math.cos(a) * spill / 2, top + .02, z - Math.sin(a) * spill / 2);
        parent.add(lip);
    }
    return m;
}

/* Small blimp that circles the archipelago on a slow, gently rising and falling loop. */
export function airship(ctx, { radius, height, speed }) {
    const ship = new THREE.Group();
    ctx.world.add(ship);
    const hull = ctx.flat('#efe6d2');
    const trim = new THREE.MeshStandardMaterial({ color: '#e2be6a', metalness: .6, roughness: .35 });
    const wood = ctx.flat('#7a5236');
    const envelope = ctx.add(new THREE.SphereGeometry(1, 10, 6), hull, 0, 0, 0, ship);
    envelope.scale.set(1.6, .62, .62);
    ctx.add(new THREE.CylinderGeometry(.64, .64, .12, 10), trim, 0, 0, 0, ship).rotation.z = Math.PI / 2;
    for (const [rx, h] of [[0, 1], [Math.PI / 2, 1], [Math.PI, 1], [-Math.PI / 2, .8]]) {
        const fin = ctx.add(new THREE.BoxGeometry(.42, .38 * h, .04), hull, -1.45, 0, 0, ship);
        fin.rotation.x = rx;
        fin.geometry.translate(0, .22 * h, 0);
    }
    ctx.add(new THREE.BoxGeometry(.6, .2, .26), wood, 0, -.78, 0, ship);
    ctx.add(new THREE.BoxGeometry(.5, .06, .28), trim, 0, -.66, 0, ship);
    for (const x of [-.22, .22]) ctx.add(new THREE.CylinderGeometry(.012, .012, .2, 4), wood, x, -.58, 0, ship);
    const lamp = ctx.add(new THREE.BoxGeometry(.06, .06, .06), ctx.emissive('#ffc46b', 2.4), .34, -.8, 0, ship);
    ctx.blinkers.push({ m: lamp, rate: .004, ph: 0 });
    const prop = new THREE.Group();
    prop.position.set(-.36, -.78, 0);
    ship.add(prop);
    for (const r of [0, Math.PI / 2]) ctx.add(new THREE.BoxGeometry(.02, .3, .05), wood, 0, 0, 0, prop).rotation.x = r;
    ship.scale.setScalar(.7);
    let angle = 2.2;
    ctx.onFrame((t, dt) => {
        if (!ctx.reduced) {
            angle += dt * speed;
            prop.rotation.x += dt * 14;
        }
        ship.position.set(Math.cos(angle) * radius, height + Math.sin(angle * 2) * .6, Math.sin(angle) * radius * .7);
        ship.rotation.y = -angle - Math.PI / 2;
        ship.rotation.z = ctx.reduced ? 0 : Math.sin(t * .0007) * .04;
    });
    return ship;
}

/* Arched plank bridge whose deck follows the same height profile the dogs walk on. */
export function woodBridge(ctx, parent, bridge, y) {
    const b = new THREE.Group();
    b.position.set(bridge.x, y, bridge.z);
    b.rotation.y = -Math.atan2(bridge.cz, bridge.cx);
    parent.add(b);
    const plank = ctx.flat('#8a5d3b');
    const beam = ctx.flat('#6b4428');
    const L = bridge.halfLength;
    const W = bridge.halfWidth;
    const deck = (x) => bridge.height * Math.cos(x / L * Math.PI / 2);
    const slope = (x) => -bridge.height * Math.PI / (2 * L) * Math.sin(x / L * Math.PI / 2);
    const n = 9;
    for (let i = 0; i < n; i++) {
        const x = -L + (i + .5) * (2 * L / n);
        const p = ctx.add(new THREE.BoxGeometry(2 * L / n * .86, .035, W * 2), i % 2 ? plank : beam, x, deck(x) + .02, 0, b);
        p.rotation.z = Math.atan(slope(x));
    }
    for (const side of [-1, 1]) {
        const z = side * (W - .02);
        for (const x of [-L, 0, L]) ctx.add(new THREE.BoxGeometry(.035, .2, .035), beam, x * .96, deck(x * .96) + .12, z, b);
        const segs = 6;
        for (let i = 0; i < segs; i++) {
            const x0 = -L * .96 + i * (2 * L * .96 / segs);
            const x1 = x0 + 2 * L * .96 / segs;
            const y0 = deck(x0) + .21;
            const y1 = deck(x1) + .21;
            const rail = ctx.add(new THREE.BoxGeometry(Math.hypot(x1 - x0, y1 - y0) + .01, .03, .03), plank, (x0 + x1) / 2, (y0 + y1) / 2, z, b);
            rail.rotation.z = Math.atan2(y1 - y0, x1 - x0);
        }
    }
    return b;
}
