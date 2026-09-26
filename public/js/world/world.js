import * as THREE from 'three';
import { angDiff, createStage } from './engine.js';
import { airship } from './props.js';
import { basecamp, mini } from './scenes.js';

const OVERVIEW_YAW = -.5;
const FLIGHT_MS = 1900;
const NARROW = 860;
const ASIDE = 'overview-aside';

const ISLANDS = {
    basecamp: { pos: [0, 0, 0], pin: 5.6, land: { t: [0, 1.3, .8], dist: 16.5, height: 7, yaw: .2 } },
    projects: { pos: [12, 2.4, -6], r: 2.2, seed: 2, plateau: true, front: OVERVIEW_YAW + .1 },
    about: { pos: [-12.5, 1.6, -6], r: 2.5, seed: 4, plateau: true, plateauTrees: false, front: OVERVIEW_YAW - .15 },
    experience: { pos: [-9, -1.4, 9], r: 2, seed: 8, front: OVERVIEW_YAW - .1 },
    contact: { pos: [9.5, -1.2, 9], r: 2, seed: 6, front: OVERVIEW_YAW + .15 },
};

const CLOUDS = [[-18, 6, -30, 1.3], [16, 9, -36, 1.7], [30, 2, -12, 1.1], [-34, 1, -4, 1.4], [4, 12, -50, 2]];

const ease = (k) => (k < .5 ? 4 * k * k * k : 1 - (-2 * k + 2) ** 3 / 2);

export function createWorld(canvas, { mood, island, labels, models, onPick }) {
    const ctx = createStage(canvas, { mode: mood, models, yaw: OVERVIEW_YAW });
    CLOUDS.forEach(([x, y, z, s]) => ctx.cloud(x, y, z, s));
    airship(ctx, { radius: 17, height: 8, speed: .05 });

    /* Portrait screens pull the camera back so the archipelago or landed island still fits. */
    const computeState = (key) => {
        const { w, h } = ctx.size();
        const narrow = w <= NARROW;
        const aspect = w / h;
        if (!ISLANDS[key]) {
            const fit = Math.min(3, Math.max(1, 1.25 / aspect));
            const aside = key === ASIDE && !(narrow && panelCollapsed);
            return {
                t: new THREE.Vector3(0, 0, 1),
                dist: 52 * fit,
                height: 23 * fit,
                yaw: OVERVIEW_YAW,
                offsetX: narrow ? 0 : aside ? -.18 : .16,
                offsetY: narrow ? (aside ? .28 : -.06) : 0,
            };
        }
        const v = ISLANDS[key];
        const land = v.land || {};
        const fit = narrow ? Math.min(2.3, Math.max(1, (panelCollapsed ? .8 : .95) / aspect)) : 1;
        const collapsed = narrow && panelCollapsed;
        return {
            t: land.t ? new THREE.Vector3(...land.t) : new THREE.Vector3(v.pos[0], v.pos[1] + .9, v.pos[2]),
            dist: (land.dist || 12.5) * fit,
            height: (land.height || 5.4) * fit,
            yaw: land.yaw ?? v.front,
            offsetX: narrow ? 0 : -.2,
            offsetY: narrow ? (collapsed ? .06 : .24) : 0,
        };
    };

    const states = new Map();
    let sizeKey = '';
    let panelCollapsed = false;
    const stateFor = (key) => {
        const { w, h } = ctx.size();
        if (sizeKey !== `${w}x${h}`) {
            sizeKey = `${w}x${h}`;
            states.clear();
        }
        const cacheKey = `${key}|${panelCollapsed}`;
        if (!states.has(cacheKey)) states.set(cacheKey, computeState(key));
        return states.get(cacheKey);
    };

    let current = island;
    let flight = null;
    const apply = (s) => {
        ctx.view.target.copy(s.t);
        ctx.view.dist = s.dist;
        ctx.view.height = s.height;
        ctx.view.offsetX = s.offsetX;
        ctx.view.offsetY = s.offsetY;
    };
    const jump = (key) => {
        flight?.done();
        flight = null;
        current = key;
        const s = stateFor(key);
        apply(s);
        ctx.snapYaw(s.yaw);
    };

    const fly = (key) => new Promise((resolve) => {
        flight?.done();
        const v = ctx.view;
        const from = { t: v.target.clone(), dist: v.dist, height: v.height, offsetX: v.offsetX, offsetY: v.offsetY };
        const fromYaw = ctx.getTargetYaw();
        current = key;
        if (ctx.reduced) {
            jump(key);
            resolve();
            return;
        }
        const toYaw = fromYaw + angDiff(stateFor(key).yaw, fromYaw);
        flight = { from, fromYaw, toYaw, start: performance.now(), done: resolve };
    });

    const settle = (s, dt) => {
        const k = ctx.reduced ? 1 : 1 - Math.exp(-dt * 5);
        const v = ctx.view;
        v.target.lerp(s.t, k);
        v.dist += (s.dist - v.dist) * k;
        v.height += (s.height - v.height) * k;
        v.offsetX += (s.offsetX - v.offsetX) * k;
        v.offsetY += (s.offsetY - v.offsetY) * k;
    };

    ctx.onFrame((t, dt) => {
        const to = stateFor(current);
        if (!flight) {
            settle(to, dt);
            return;
        }
        const k = Math.min(1, (t - flight.start) / FLIGHT_MS);
        const e = ease(k);
        const { from } = flight;
        const v = ctx.view;
        v.target.lerpVectors(from.t, to.t, e);
        v.dist = from.dist + (to.dist - from.dist) * e;
        v.height = from.height + (to.height - from.height) * e + Math.sin(e * Math.PI) * 6;
        v.offsetX = from.offsetX + (to.offsetX - from.offsetX) * e;
        v.offsetY = from.offsetY + (to.offsetY - from.offsetY) * e;
        ctx.lookYaw(flight.fromYaw + (flight.toYaw - flight.fromYaw) * e);
        if (k >= 1) {
            const { done } = flight;
            flight = null;
            done();
        }
    });

    const pinTracks = [];
    const projected = new THREE.Vector3();
    ctx.onFrame(() => {
        if (current !== 'overview') return;
        const { w, h } = ctx.size();
        for (const { el, anchor } of pinTracks) {
            projected.copy(anchor).project(ctx.cam);
            const visible = projected.z < 1;
            el.style.visibility = visible ? '' : 'hidden';
            if (visible) el.style.transform = `translate(${(projected.x * .5 + .5) * w}px, ${(-projected.y * .5 + .5) * h}px) translate(-50%, -100%)`;
        }
    });
    const trackPins = (elements) => {
        for (const el of elements) {
            const v = ISLANDS[el.dataset.pin];
            if (v) pinTracks.push({ el, anchor: new THREE.Vector3(v.pos[0], v.pos[1] + (v.pin || 2.2), v.pos[2]) });
        }
    };

    const groups = {};
    const raycaster = new THREE.Raycaster();
    const ndc = new THREE.Vector2();
    let down = null;
    canvas.addEventListener('pointerdown', (e) => { down = [e.clientX, e.clientY]; });
    canvas.addEventListener('pointerup', (e) => {
        if (!down || Math.hypot(e.clientX - down[0], e.clientY - down[1]) > 6 || ISLANDS[current]) return;
        const b = canvas.getBoundingClientRect();
        ndc.set((e.clientX - b.left) / b.width * 2 - 1, -((e.clientY - b.top) / b.height) * 2 + 1);
        raycaster.setFromCamera(ndc, ctx.cam);
        let best = null;
        for (const [key, g] of Object.entries(groups)) {
            const hit = raycaster.intersectObject(g, true)[0];
            if (hit && (!best || hit.distance < best.distance)) best = { key, distance: hit.distance };
        }
        if (best) onPick(best.key);
    });

    const plane = new THREE.Mesh(new THREE.ConeGeometry(.12, .34, 3), ctx.flat('#fbf2dc'));
    plane.rotation.z = -Math.PI / 2;
    plane.visible = false;
    ctx.scene.add(plane);
    let planeT = -1;
    const origin = new THREE.Vector3();
    ctx.onFrame((t, dt) => {
        const mailbox = groups.contact?.userData.mailbox;
        if (!mailbox) return;
        mailbox.userData.flag.rotation.x = planeT >= 0 ? -1.2 : 0;
        if (planeT < 0) return;
        planeT += dt * .35;
        mailbox.getWorldPosition(origin);
        plane.position.set(origin.x - planeT * 6, origin.y + 1 + Math.sin(planeT * 3) * .6 + planeT * 5, origin.z - planeT * 5);
        plane.rotation.x = planeT * 6;
        if (planeT > 1.3) {
            planeT = -1;
            plane.visible = false;
        }
    });
    const celebrate = () => {
        if (ctx.reduced) return;
        planeT = 0;
        plane.visible = true;
    };

    jump(island);
    ctx.start();

    for (const [key, v] of Object.entries(ISLANDS)) {
        const [x, y, z] = v.pos;
        const isl = key === 'basecamp'
            ? basecamp(ctx, { x, y, z })
            : mini(ctx, { x, y, z, r: v.r, seed: v.seed, motif: key, plateau: v.plateau, plateauTrees: v.plateauTrees, front: v.front, labels });
        groups[key] = isl.group;
    }
    const ready = ctx.settled();

    const setPanelCollapsed = (collapsed) => { panelCollapsed = collapsed; };

    return { ready, fly, trackPins, celebrate, setPanelCollapsed, setMood: ctx.setMode };
}
