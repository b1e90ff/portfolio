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
    ctx.add(new THREE.BoxGeometry(.2, .24, .3), furLight, .28, .18 + lift, 0, body);
    const head = new THREE.Group();
    head.position.set(.42, .36 + lift, 0);
    body.add(head);
    ctx.add(new THREE.BoxGeometry(.24, .22, .24), fur, 0, 0, 0, head);
    ctx.add(new THREE.BoxGeometry(.13, .1, .14), furLight, .16, -.04, 0, head);
    ctx.add(new THREE.BoxGeometry(.04, .04, .06), dark, .23, 0, 0, head);
    for (const side of [-1, 1]) {
        ctx.add(new THREE.BoxGeometry(.1, .16, .04), ctx.flat('#c08a32'), -.02, -.05, side * .14, head).rotation.x = side * .25;
        ctx.add(new THREE.BoxGeometry(.03, .03, .02), dark, .09, .05, side * .09, head);
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

export function whiteDog(ctx, parent, x, y, z, ry = 0, s = 1) {
    const g = new THREE.Group();
    g.position.set(x, y, z);
    g.rotation.y = ry;
    g.scale.setScalar(s);
    parent.add(g);
    const fur = ctx.flat('#f2efe9');
    const furShade = ctx.flat('#dcd7ce');
    const dark = ctx.flat('#1a1210');
    ctx.add(new THREE.BoxGeometry(.36, .18, .2), fur, 0, .24, 0, g);
    const legs = [[.12, -.06], [.12, .06], [-.12, -.06], [-.12, .06]].map(([lx, lz]) => {
        const hip = new THREE.Group();
        hip.position.set(lx, .16, lz);
        g.add(hip);
        ctx.add(new THREE.BoxGeometry(.06, .16, .06), furShade, 0, -.08, 0, hip);
        return hip;
    });
    const head = new THREE.Group();
    head.position.set(.23, .38, 0);
    g.add(head);
    ctx.add(new THREE.BoxGeometry(.18, .16, .17), fur, 0, 0, 0, head);
    ctx.add(new THREE.BoxGeometry(.08, .07, .1), furShade, .11, -.03, 0, head);
    ctx.add(new THREE.BoxGeometry(.03, .03, .04), dark, .16, -.01, 0, head);
    for (const side of [-1, 1]) {
        ctx.add(new THREE.ConeGeometry(.04, .1, 4), fur, -.02, .12, side * .055, head).rotation.x = -side * .15;
        ctx.add(new THREE.BoxGeometry(.02, .025, .02), dark, .09, .03, side * .045, head);
    }
    const tail = new THREE.Group();
    tail.position.set(-.19, .3, 0);
    g.add(tail);
    ctx.add(new THREE.BoxGeometry(.05, .16, .05), fur, 0, .07, 0, tail).rotation.z = -.4;
    ctx.wags.push({ o: tail, rate: .018, amp: .6, axis: 'x' }, { o: head, rate: .0023, amp: .25 });
    g.userData = { legs, head };
    return g;
}

function gait(dog, t, speed, rate = .02) {
    const phase = [0, Math.PI, Math.PI, 0];
    dog.userData.legs.forEach((l, i) => { l.rotation.z = Math.sin(t * rate + phase[i]) * .7 * speed; });
    if (dog.userData.body) dog.userData.body.position.y = Math.abs(Math.sin(t * rate)) * .04 * speed;
}

/* Walkable area in island space: level outline minus reserved circles, clipped to a zone. */
export function area(lvl, occ, o = {}) {
    const margin = o.margin ?? .45;
    const zone = o.zone;
    const limit = (x, z, r) => lvl.fn(Math.atan2(z - lvl.c.z, x - lvl.c.x)) - margin - r;
    const valid = (x, z, r = .2) => Math.hypot(x - lvl.c.x, z - lvl.c.z) < limit(x, z, r)
        && (!zone || Math.hypot(x - zone.x, z - zone.z) < zone.r - r)
        && !occ.some((c) => Math.hypot(x - c.x, z - c.z) < c.r + r);
    const sample = (near, minD, maxD, r = .25) => {
        for (let i = 0; i < 80; i++) {
            const a = Math.random() * Math.PI * 2;
            const d = minD + Math.random() * (maxD - minD);
            const x = near.x + Math.cos(a) * d;
            const z = near.z + Math.sin(a) * d;
            if (valid(x, z, r)) return new THREE.Vector3(x, 0, z);
        }
        return near.clone();
    };
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
        clampCircle(p, lvl.c.x, lvl.c.z, limit(p.x, p.z, r), true);
        if (zone) clampCircle(p, zone.x, zone.z, zone.r - r, true);
    };
    return { valid, sample, resolve, occ };
}

function agent(dog, walk, o) {
    return { dog, area: walk, pos: dog.position.clone(), vel: new THREE.Vector3(), r: o.r, base: o.speed, max: o.speed, heading: dog.rotation.y, y: o.y };
}

/* Seek with arrival, separation and obstacle repulsion, then a hard collision resolve. */
function steer(a, target, dt, others = [], arrive = .35) {
    const want = new THREE.Vector3(target.x - a.pos.x, 0, target.z - a.pos.z);
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
    a.dog.position.set(a.pos.x, a.y, a.pos.z);
    a.dog.rotation.y = a.heading;
    return { d, speed: speed / a.base };
}

/* Two dogs switch between chasing (random leader), sniffing and short rests. */
export function playTogether(ctx, walk, specs, y) {
    const agents = specs.map(([dog, o]) => agent(dog, walk, { ...o, y }));
    const goal = [null, null];
    let mode = 'chase';
    let leader = 0;
    let timer = 5;
    const next = () => {
        const roll = Math.random();
        mode = roll < .55 ? 'chase' : roll < .82 ? 'sniff' : 'rest';
        if (mode === 'chase' && Math.random() < .5) leader = 1 - leader;
        timer = mode === 'rest' ? 1.5 + Math.random() * 2 : 5 + Math.random() * 6;
        goal[0] = null;
        goal[1] = null;
    };
    const wander = (a, i, minD, maxD) => {
        if (!goal[i] || a.pos.distanceTo(goal[i]) < .3) goal[i] = walk.sample(a.pos, minD, maxD, a.r);
        return goal[i];
    };
    ctx.onFrame((t, dt) => {
        if (ctx.reduced) return;
        timer -= dt;
        if (timer <= 0) next();
        agents.forEach((a, i) => {
            let target;
            if (mode === 'chase' && i === leader) {
                target = wander(a, i, .9, 2.2);
                a.max = a.base;
            } else if (mode === 'chase') {
                const lead = agents[leader];
                const back = lead.vel.clone();
                if (back.lengthSq() > 1e-4) back.setLength(.8);
                target = lead.pos.clone().sub(back);
                a.max = a.base * 1.12;
            } else if (mode === 'sniff') {
                target = wander(a, i, .3, 1);
                a.max = a.base * .35;
            } else {
                target = a.pos;
                a.max = a.base * .2;
            }
            const s = steer(a, target, dt, [agents[1 - i]]);
            gait(a.dog, t, Math.min(1, s.speed * 1.4), .02 + .016 * Math.min(1, s.speed));
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
    let timer = 1.2;
    let drop = null;
    const toss = () => {
        landing.copy(walk.sample(b.p, 1.1, 2, .15));
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
            if (state === 'fly') state = 'chase';
        }
        for (const c of walk.occ) {
            const dx = b.p.x - c.x;
            const dz = b.p.z - c.z;
            const d = Math.hypot(dx, dz);
            const m = c.r + R;
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
        walk.resolve(b.p, R);
    };
    const look = (p) => {
        a.heading += angDiff(Math.atan2(-(p.z - a.pos.z), p.x - a.pos.x), a.heading) * .08;
        a.dog.rotation.y = a.heading;
    };
    const run = (t, s, rate) => gait(a.dog, t, Math.min(1, s.speed * 1.3), rate);
    ctx.onFrame((t, dt) => {
        if (ctx.reduced) {
            ball.position.set(b.p.x, y + R, b.p.z);
            return;
        }
        if (state === 'fly' || state === 'chase') physics(dt);
        if (state === 'wait') {
            steer(a, a.pos, dt);
            look(b.p);
            gait(a.dog, t, 0);
            a.dog.position.y = y + Math.abs(Math.sin(t * .012)) * .05;
            timer -= dt;
            if (timer <= 0) toss();
        } else if (state === 'fly') {
            timer += dt;
            run(t, timer > .25 ? steer(a, landing, dt, [], .2) : steer(a, a.pos, dt), .034);
        } else if (state === 'chase') {
            run(t, steer(a, b.p, dt, [], .12), .034);
            if (a.pos.distanceTo(b.p) < .34 && b.h < .1) {
                state = 'carry';
                drop = walk.sample(a.pos, 1, 1.8, a.r);
            }
        } else if (state === 'carry') {
            a.max = a.base * .7;
            const s = steer(a, drop, dt, [], .3);
            run(t, s, .026);
            if (s.d < .15) {
                state = 'drop';
                timer = .9;
                a.max = a.base;
            }
        } else {
            steer(a, a.pos, dt);
            gait(a.dog, t, 0);
            timer -= dt;
            if (timer <= 0) {
                state = 'wait';
                timer = .8 + Math.random() * .8;
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
const ROCKS = ['nature/rock_tallA', 'nature/rock_tallB', 'nature/rock_largeB', 'nature/rock_tallC', 'nature/rock_largeD', 'nature/rock_largeA'];
const TUFTS = ['nature/grass_large', 'nature/grass_leafs', 'nature/plant_bushSmall', 'nature/flower_yellowB', 'nature/flower_purpleB', 'nature/grass_leafsLarge', 'nature/flower_redA', 'nature/mushroom_redGroup'];

export function trees(ctx, parent, lvl, angles, inset = .45, scale = 1, avoid = () => false) {
    for (const [i, a] of angles.entries()) {
        const [x, z] = at(lvl.fn, lvl.c, a, inset + (i % 2) * .25);
        if (avoid(x, z)) continue;
        parent.userData.reserve?.(x, z, .22 * scale);
        ctx.place(PINES[i % PINES.length], x, lvl.top, z, a * 7, scale * (1 + (i % 3) * .15), parent);
    }
}

export function edgeRocks(ctx, parent, lvl, n, y, skip = null) {
    for (let i = 0; i < n; i++) {
        const a = i / n * Math.PI * 2;
        if (skip !== null && Math.abs(angDiff(a, skip)) < .45) continue;
        const [x, z] = at(lvl.fn, lvl.c, a, -.12);
        ctx.place(ROCKS[i % ROCKS.length], x, y, z, a * 3, .85 + (i % 4) * .12, parent);
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

export function waterfall(ctx, parent, x, z, top, bottom, a, w = .6) {
    const tex = fallTexture();
    const mat = new THREE.MeshStandardMaterial({ map: tex, transparent: true, opacity: .9, emissive: '#2c6f9a', emissiveIntensity: .35, roughness: .2, side: THREE.DoubleSide });
    const m = new THREE.Mesh(new THREE.PlaneGeometry(w, top - bottom), mat);
    m.position.set(x, (top + bottom) / 2, z);
    m.rotation.y = Math.PI / 2 - a;
    parent.add(m);
    ctx.onFrame((t) => { if (!ctx.reduced) tex.offset.y = (t * .0012) % 1; });
    return m;
}
