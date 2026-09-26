import * as THREE from 'three';
import { angDiff } from './engine.js';

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
            walk.resolve(b.p, R);
            b.h = mouth.y - y;
            b.v.set(0, 0, 0);
            b.vh = 0;
        } else if (state === 'drop' || state === 'wait') {
            b.h = Math.max(R, b.h - dt * 1.5);
        }
        ball.position.set(b.p.x, y + b.h, b.p.z);
    });
}
