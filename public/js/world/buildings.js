import * as THREE from 'three';
import { hash } from './engine.js';
import { rockKit } from './nature.js';

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
    chimney(ctx, h, .74, -.16);
    ctx.glowQuad(h, -.5, .55, .515, 0, .32, .34);
    ctx.glowQuad(h, -1.015, .55, 0, -Math.PI / 2, .32, .34);
    ctx.windowLight(-.5, .55, .95, h);
    ctx.windowLight(-1.4, .55, 0, h);
    smoke(ctx, h, .74, 1.72, -.16);
    return h;
}

function chimney(ctx, parent, x, z) {
    const stone = ctx.flat('#8a7f76');
    const dark = ctx.flat('#5c534c');
    ctx.add(new THREE.BoxGeometry(.17, .62, .17), stone, x, 1.33, z, parent);
    ctx.add(new THREE.BoxGeometry(.215, .05, .215), dark, x, 1.665, z, parent);
    ctx.add(new THREE.BoxGeometry(.11, .012, .11), ctx.flat('#1c1714'), x, 1.694, z, parent);
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
    const door = ctx.add(new THREE.BoxGeometry(.3, .5, .02), ctx.flat('#262a33'), -.28, .25, .38, s);
    s.userData.door = door.position;
    for (let i = 0; i < 4; i++) {
        const led = ctx.add(new THREE.BoxGeometry(.05, .03, .02), ctx.emissive(i % 2 ? '#6fd08c' : '#e2be6a', 2.5), .08 + i * .08, .55, .385, s);
        ctx.blinkers.push({ m: led, rate: .006 + i * .0013, ph: i * 1.7 });
    }
    ctx.add(new THREE.BoxGeometry(.4, .05, .02), ctx.flat('#11141a'), .2, .45, .385, s);
    ctx.add(new THREE.BoxGeometry(.4, .05, .02), ctx.flat('#11141a'), .2, .38, .385, s);
    return s;
}

export function lantern(ctx, parent, x, y, z) {
    parent.userData.reserve?.(x, z, .16);
    ctx.place('town/lantern', x, y, z, 0, .8, parent);
    ctx.add(new THREE.SphereGeometry(.07, 8, 6), ctx.emissive('#ffc46b', 3), x, y + 1.18, z, parent);
    // A glow sprite instead of a point light: every light costs every lit fragment on mobile GPUs.
    const halo = new THREE.Sprite(glowKit(ctx).halo);
    halo.position.set(x, y + 1.18, z);
    halo.scale.setScalar(.7);
    parent.add(halo);
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

/* The deck follows bridge.height, the same profile walk areas use to lift the dogs. */
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

export function campfire(ctx, parent, x, y, z) {
    const f = new THREE.Group();
    f.position.set(x, y, z);
    parent.add(f);
    const kit = rockKit(ctx);
    for (let i = 0; i < 9; i++) {
        const a = i / 9 * Math.PI * 2;
        const stone = ctx.add(kit.geos[i % 3], kit.mats[i % 4], Math.cos(a) * .24, .03, Math.sin(a) * .24, f);
        stone.scale.set(.07, .05, .06);
        stone.rotation.y = a;
    }
    const bark = ctx.flat('#6b4428');
    const cut = ctx.flat('#b88a5a');
    for (let i = 0; i < 4; i++) {
        const a = i / 4 * Math.PI * 2 + .4;
        const log = new THREE.Group();
        log.position.set(Math.cos(a) * .08, .08, Math.sin(a) * .08);
        log.rotation.set(0, -a, .9);
        f.add(log);
        ctx.add(new THREE.CylinderGeometry(.028, .034, .26, 6), bark, 0, 0, 0, log);
        ctx.add(new THREE.CylinderGeometry(.029, .029, .004, 6), cut, 0, -.131, 0, log);
    }
    const flames = [
        [0, 0, .11, .4, '#e8430f'], [.06, .03, .075, .28, '#f2621a'], [-.06, -.02, .075, .3, '#f2621a'],
        [.02, -.06, .06, .26, '#f7881f'], [0, 0, .055, .26, '#ffb52e'], [0, 0, .03, .16, '#ffe08a'],
    ].map(([fx, fz, rad, h, color], i) => {
        // Flames skip tone mapping so the orange stays saturated instead of washing out to white.
        const m = new THREE.Mesh(fireKit(ctx).flame, new THREE.MeshBasicMaterial({ color, toneMapped: false }));
        m.position.set(fx, .05, fz);
        m.userData = { rad, h, phase: i * 1.7, rate: .009 + i * .0017 };
        f.add(m);
        return m;
    });
    const glow = new THREE.Mesh(new THREE.CircleGeometry(.55, 24), new THREE.MeshBasicMaterial({
        map: glowKit(ctx).texture, transparent: true, depthWrite: false, blending: THREE.AdditiveBlending,
    }));
    glow.rotation.x = -Math.PI / 2;
    glow.position.y = .01;
    f.add(glow);
    const sparks = Array.from({ length: 8 }, (_, i) => {
        const spark = new THREE.Mesh(fireKit(ctx).spark, new THREE.MeshBasicMaterial({ color: '#ffb347', toneMapped: false, transparent: true }));
        spark.userData = { offset: hash(i, 4, 9), drift: (hash(i, 2, 7) - .5) * .18, turn: hash(i, 8, 1) * 6 };
        f.add(spark);
        return spark;
    });
    const light = ctx.glowSpot(0, .45, 0, 0xff8a3d, 2.4, 6, f);
    const base = light.intensity;
    ctx.onFrame((t) => {
        const calm = ctx.reduced;
        flames.forEach((m) => {
            const u = m.userData;
            const k = calm ? 1 : .8 + .2 * Math.sin(t * u.rate + u.phase) + .08 * Math.sin(t * u.rate * 2.3 + u.phase);
            m.scale.set(u.rad * (1.1 - k * .1), u.h / 2 * k, u.rad * (1.1 - k * .1));
            m.rotation.set(calm ? 0 : Math.sin(t * .003 + u.phase) * .12, calm ? u.phase : t * .0015 + u.phase, 0);
        });
        sparks.forEach((spark) => {
            const u = spark.userData;
            const k = (t * .00045 + u.offset) % 1;
            spark.visible = !calm;
            spark.position.set(Math.sin(u.turn + k * 4) * u.drift, .2 + k * .9, Math.cos(u.turn + k * 4) * u.drift);
            spark.material.opacity = 1 - k;
            spark.rotation.set(k * 6, k * 4, 0);
        });
        const flicker = calm ? 1 : .85 + .15 * Math.sin(t * .011) * Math.sin(t * .027);
        light.intensity = base * flicker;
        glow.material.opacity = .55 + .25 * flicker;
    });
    return f;
}

function fireKit(ctx) {
    if (!ctx.fireKit) {
        const flame = new THREE.OctahedronGeometry(1, 0);
        flame.translate(0, 1, 0);
        ctx.fireKit = { flame, spark: new THREE.BoxGeometry(.018, .018, .018) };
    }
    return ctx.fireKit;
}

function glowKit(ctx) {
    if (!ctx.glowKit) {
        const texture = glowTexture();
        ctx.glowKit = {
            texture,
            halo: new THREE.SpriteMaterial({ map: texture, depthWrite: false, blending: THREE.AdditiveBlending }),
        };
    }
    return ctx.glowKit;
}

function glowTexture() {
    const c = document.createElement('canvas');
    c.width = 64;
    c.height = 64;
    const g = c.getContext('2d');
    const grad = g.createRadialGradient(32, 32, 0, 32, 32, 32);
    grad.addColorStop(0, 'rgba(255,170,90,.55)');
    grad.addColorStop(.5, 'rgba(255,120,50,.18)');
    grad.addColorStop(1, 'rgba(255,100,40,0)');
    g.fillStyle = grad;
    g.fillRect(0, 0, 64, 64);
    return new THREE.CanvasTexture(c);
}
