import * as THREE from 'three';
import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';

// Hex colours stay in the working space, matching the palette the scenes were tuned against.
THREE.ColorManagement.enabled = false;

// Intensities are authored in legacy units; physical lighting needs them scaled by PI.
const LIGHT = Math.PI;
// The bevelled grass cap rises this far above the rock body, so props stand on the grass.
const GRASS = .06;

export const hash = (x, y, z) => {
    const s = Math.sin(x * 12.9898 + y * 78.233 + z * 37.719) * 43758.5453;
    return s - Math.floor(s);
};
export const angDiff = (a, b) => Math.atan2(Math.sin(a - b), Math.cos(a - b));

export const MODES = {
    dusk: { top: '#1f1840', mid: '#4a2a55', low: '#b8604a', hemi: [0xffb896, 0x241610, .55], sun: [0xff9f70, 1.05], warm: .45, exp: 1.08, stars: .35, cloud: '#c98a8f', night: true },
    night: { top: '#1d1a45', mid: '#100f28', low: '#07070f', hemi: [0x5d6aa8, 0x120d0a, .28], sun: [0x9aabff, .6], warm: .12, exp: 1, stars: 1, cloud: '#3d3a63', night: true },
    aurora: { top: '#0a2433', mid: '#0f3a44', low: '#1f6b5c', hemi: [0x7fd6c0, 0x0c1414, .42], sun: [0xa8ffe8, .75], warm: .18, exp: 1.02, stars: .8, cloud: '#2f5a63', night: true },
    peach: { top: '#2e2d6b', mid: '#8a6aa8', low: '#f4b98e', hemi: [0xffd2c0, 0x2a1a24, .62], sun: [0xffc8a8, 1.15], warm: .5, exp: 1.08, stars: .15, cloud: '#e8b5c0', night: true },
    day: { top: '#6aa7da', mid: '#b4dcef', low: '#e9f1f3', hemi: [0xdfefff, 0x6a5a3a, .85], sun: [0xfff1d6, 1.6], warm: .3, exp: 1.05, stars: 0, cloud: '#ffffff', night: false },
};

const SKY_VERTEX = 'varying vec3 vP; void main(){ vP = normalize(position); gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0); }';
const SKY_FRAGMENT = 'uniform vec3 top; uniform vec3 mid; uniform vec3 low; varying vec3 vP; void main(){ float k = smoothstep(-0.75, 0.25, vP.y); vec3 c = k < 0.45 ? mix(low, mid, smoothstep(0.0, 0.45, k)) : mix(mid, top, smoothstep(0.45, 1.0, k)); gl_FragColor = vec4(c, 1.0); }';

export function createStage(canvas, opts) {
    const reduced = matchMedia('(prefers-reduced-motion: reduce)').matches;
    const r = new THREE.WebGLRenderer({ canvas, antialias: true });
    r.setPixelRatio(Math.min(devicePixelRatio || 1, 2));
    r.toneMapping = THREE.ACESFilmicToneMapping;
    r.shadowMap.enabled = true;
    r.shadowMap.type = THREE.PCFShadowMap;

    const scene = new THREE.Scene();
    scene.fog = new THREE.Fog(0x1b1640, 40, 120);
    const cam = new THREE.PerspectiveCamera(30, 1, .1, 400);
    const hemi = new THREE.HemisphereLight();
    const sun = new THREE.DirectionalLight();
    sun.position.set(-8, 14, 6);
    sun.castShadow = true;
    sun.shadow.mapSize.set(2048, 2048);
    sun.shadow.radius = 3;
    Object.assign(sun.shadow.camera, { left: -18, right: 18, top: 18, bottom: -18, near: 1, far: 60 });
    sun.shadow.bias = -.0004;
    // Islands bob through the shadow map; a larger normal bias stops foliage acne from shimmering.
    sun.shadow.normalBias = .05;
    const warm = new THREE.DirectionalLight(0xffc27a);
    warm.position.set(7, 5, 8);
    const world = new THREE.Group();
    scene.add(hemi, sun, sun.target, warm, world);

    const ctx = { r, scene, cam, world, reduced, mode: opts.mode, blinkers: [], wags: [], windows: [], frames: [], clouds: [] };

    ctx.flat = (c, o = {}) => new THREE.MeshStandardMaterial({ color: c, roughness: .85, flatShading: true, ...o });
    ctx.emissive = (c, i = 2) => new THREE.MeshStandardMaterial({ color: c, emissive: c, emissiveIntensity: i });
    ctx.add = (geo, mat, x = 0, y = 0, z = 0, parent = world) => {
        const m = new THREE.Mesh(geo, mat);
        m.position.set(x, y, z);
        m.castShadow = true;
        m.receiveShadow = true;
        parent.add(m);
        return m;
    };
    ctx.glowSpot = (x, y, z, color, intensity, dist, parent = world) => {
        const l = new THREE.PointLight(color, intensity * LIGHT * .3, dist, 2);
        l.userData.base = l.intensity;
        l.position.set(x, y, z);
        parent.add(l);
        return l;
    };
    ctx.windowLight = (x, y, z, parent) => {
        const l = ctx.glowSpot(x, y, z, 0xffb35a, 1.6, 2.6, parent);
        l.intensity = MODES[ctx.mode].night ? l.userData.base : 0;
        ctx.windows.push(l);
        return l;
    };
    ctx.glowQuad = (parent, x, y, z, ry, w, h) => {
        const q = new THREE.Mesh(new THREE.PlaneGeometry(w, h), ctx.emissive('#ffc46b', 2.4));
        q.position.set(x, y, z);
        q.rotation.y = ry;
        parent.add(q);
        return q;
    };
    ctx.onFrame = (fn) => ctx.frames.push(fn);

    const loader = new GLTFLoader();
    const cache = new Map();
    const load = (path) => {
        if (!cache.has(path)) {
            cache.set(path, loader.loadAsync(`${opts.models}/${path}.glb`).then((g) => g.scene, (err) => {
                console.warn(`model ${path} failed to load`, err);
                return null;
            }));
        }
        return cache.get(path);
    };
    const pending = [];
    ctx.track = (promise) => pending.push(promise);
    ctx.settled = () => Promise.all(pending);
    /* Returns a placed slot at once and fills it on arrival so models load in parallel. */
    ctx.place = (path, x, y, z, ry = 0, s = 1, parent = world) => {
        const slot = new THREE.Group();
        slot.position.set(x, y, z);
        slot.rotation.y = ry;
        slot.scale.setScalar(s);
        parent.add(slot);
        ctx.track(load(path).then((model) => {
            if (model) slot.add(prepare(model.clone(true)));
        }));
        return slot;
    };
    const prepare = (o) => {
        o.traverse((m) => {
            if (!m.isMesh) return;
            m.castShadow = true;
            m.receiveShadow = true;
            [].concat(m.material).forEach((mt) => {
                if (!mt.color || mt.userData.graded) return;
                mt.userData.graded = true;
                const hsl = {};
                mt.color.getHSL(hsl);
                if (hsl.h > .28 && hsl.h < .56 && hsl.s > .25) mt.color.setHSL(.3 + (hsl.h - .3) * .35, hsl.s * .75, hsl.l * .62);
            });
        });
        return o;
    };

    const sky = new THREE.Mesh(
        new THREE.SphereGeometry(180, 32, 16),
        new THREE.ShaderMaterial({
            side: THREE.BackSide,
            depthWrite: false,
            fog: false,
            uniforms: { top: { value: new THREE.Color() }, mid: { value: new THREE.Color() }, low: { value: new THREE.Color() } },
            vertexShader: SKY_VERTEX,
            fragmentShader: SKY_FRAGMENT,
        }),
    );
    scene.add(sky);

    const starPos = [];
    for (let i = 0; i < 900; i++) {
        const t = hash(i, 1, 2) * Math.PI * 2;
        const p = Math.acos(1 - hash(i, 3, 4) * 2);
        starPos.push(Math.cos(t) * Math.sin(p) * 160, Math.cos(p) * 160, Math.sin(t) * Math.sin(p) * 160);
    }
    const starGeo = new THREE.BufferGeometry();
    starGeo.setAttribute('position', new THREE.Float32BufferAttribute(starPos, 3));
    const stars = new THREE.Points(starGeo, new THREE.PointsMaterial({ color: 0xffffff, size: .7, fog: false, transparent: true }));
    scene.add(stars);

    ctx.cloud = (x, y, z, s = 1) => {
        const g = new THREE.Group();
        const mat = ctx.flat(MODES[ctx.mode].cloud, { transparent: true, opacity: .95 });
        [[0, 0, 0, 1], [1.1, -.25, .2, .8], [-1, -.3, -.1, .75], [.3, .45, -.3, .7], [-.4, .2, .5, .6]].forEach(([dx, dy, dz, rr]) => {
            const b = new THREE.Mesh(new THREE.IcosahedronGeometry(rr, 0), mat);
            b.position.set(dx, dy, dz);
            g.add(b);
        });
        g.position.set(x, y, z);
        g.scale.setScalar(s);
        g.userData = { mat, speed: .15 + hash(x, y, z) * .2, base: x };
        scene.add(g);
        ctx.clouds.push(g);
        return g;
    };

    ctx.setMode = (m) => {
        ctx.mode = MODES[m] ? m : 'dusk';
        const d = MODES[ctx.mode];
        const u = sky.material.uniforms;
        u.top.value.set(d.top);
        u.mid.value.set(d.mid);
        u.low.value.set(d.low);
        scene.fog.color.set(d.mid);
        hemi.color.set(d.hemi[0]);
        hemi.groundColor.set(d.hemi[1]);
        hemi.intensity = d.hemi[2] * LIGHT;
        sun.color.set(d.sun[0]);
        sun.intensity = d.sun[1] * LIGHT;
        warm.intensity = d.warm * LIGHT;
        r.toneMappingExposure = d.exp;
        stars.material.opacity = d.stars;
        stars.visible = d.stars > 0;
        ctx.clouds.forEach((c) => c.userData.mat.color.set(d.cloud));
        ctx.windows.forEach((w) => { w.intensity = d.night ? w.userData.base : 0; });
    };

    ctx.view = { yaw: opts.yaw, target: new THREE.Vector3(), dist: 50, height: 20, offsetX: 0, offsetY: 0 };
    let targetYaw = ctx.view.yaw;
    let dragging = false;
    let lastX = 0;
    canvas.addEventListener('pointerdown', (e) => {
        dragging = true;
        lastX = e.clientX;
        canvas.setPointerCapture(e.pointerId);
    });
    canvas.addEventListener('pointerup', () => { dragging = false; });
    canvas.addEventListener('pointercancel', () => { dragging = false; });
    canvas.addEventListener('pointermove', (e) => {
        if (!dragging) return;
        targetYaw -= (e.clientX - lastX) * .006;
        lastX = e.clientX;
    });
    ctx.lookYaw = (y) => { targetYaw = y; };
    ctx.getTargetYaw = () => targetYaw;
    ctx.snapYaw = (y) => {
        targetYaw = y;
        ctx.view.yaw = y;
    };

    let size = { w: 1, h: 1 };
    const applyOffset = () => {
        const { w, h } = size;
        cam.setViewOffset(w, h, -w * ctx.view.offsetX, h * ctx.view.offsetY, w, h);
        cam.updateProjectionMatrix();
    };
    const resize = () => {
        const b = canvas.getBoundingClientRect();
        size = { w: Math.max(1, b.width), h: Math.max(1, b.height) };
        r.setSize(size.w, size.h, false);
        cam.aspect = size.w / size.h;
        applyOffset();
    };
    ctx.size = () => size;
    resize();
    addEventListener('resize', resize);

    let last = performance.now();
    const frame = (t) => {
        const dt = Math.max(0, Math.min(.05, (t - last) / 1000));
        last = t;
        ctx.view.yaw += (targetYaw - ctx.view.yaw) * (reduced ? 1 : .08);
        applyOffset();
        const v = ctx.view;
        cam.position.set(v.target.x + Math.sin(v.yaw) * v.dist, v.target.y + v.height, v.target.z + Math.cos(v.yaw) * v.dist);
        cam.lookAt(v.target);
        scene.fog.near = Math.max(40, v.dist * .8);
        scene.fog.far = Math.max(120, v.dist * 2.4);
        sky.position.copy(cam.position);
        stars.position.copy(cam.position);
        if (!reduced) {
            stars.rotation.y = t * .000008;
            ctx.clouds.forEach((c) => {
                c.position.x += dt * c.userData.speed;
                if (c.position.x > c.userData.base + 30) c.position.x = c.userData.base - 30;
            });
        }
        ctx.wags.forEach((w) => {
            const val = reduced ? 0 : Math.sin(t * w.rate) * w.amp;
            if (w.axis === 'x') w.o.rotation.x = val;
            else w.o.rotation.y = val;
        });
        ctx.blinkers.forEach((b) => {
            b.m.material.emissiveIntensity = reduced ? 2 : (Math.sin(t * b.rate + b.ph) > .1 ? 2.8 : .25);
        });
        ctx.frames.forEach((fn) => fn(t, dt));
        r.render(scene, cam);
        requestAnimationFrame(frame);
    };
    ctx.start = () => requestAnimationFrame(frame);
    ctx.setMode(opts.mode);
    return ctx;
}

export const radiusFn = ({ r0, amp = .2, seed = 0, bay = null }) => (a) => {
    let v = r0 + amp * (1.1 * Math.sin(3 * a + seed) + .7 * Math.sin(5 * a + 1.3 + seed * 2) + .35 * Math.sin(9 * a + 2 + seed * 3));
    if (bay) v -= bay.depth * Math.exp(-(angDiff(a, bay.a) ** 2) / (2 * (bay.width || .3) ** 2));
    return v;
};

export const at = (fn, c, a, inset = 0) => {
    const rr = fn(a) - inset;
    return [c.x + Math.cos(a) * rr, c.z + Math.sin(a) * rr];
};

function blob(ctx, fn, c, y0, h, rockCol, grassCol, parent) {
    const shape = new THREE.Shape();
    const N = 56;
    for (let i = 0; i <= N; i++) {
        const [x, z] = at(fn, c, i / N * Math.PI * 2);
        if (i === 0) shape.moveTo(x, -z);
        else shape.lineTo(x, -z);
    }
    const geo = new THREE.ExtrudeGeometry(shape, { depth: h, steps: 3, bevelEnabled: false });
    geo.rotateX(-Math.PI / 2);
    const p = geo.attributes.position;
    for (let i = 0; i < p.count; i++) {
        const x = p.getX(i);
        const y = p.getY(i);
        const z = p.getZ(i);
        if (y > h - .01) continue;
        const k = y < .01 ? .6 : 1;
        const dx = x - c.x;
        const dz = z - c.z;
        const d = Math.hypot(dx, dz) || 1;
        const push = (hash(Math.round(x * 20), Math.round(y * 20), Math.round(z * 20)) - .35) * .28 * k;
        p.setXYZ(i, x + dx / d * push, y + (hash(z, x, y) - .5) * .05 * k, z + dz / d * push);
    }
    geo.computeVertexNormals();
    const rock = new THREE.Mesh(geo, [ctx.flat(grassCol), ctx.flat(rockCol)]);
    rock.position.y = y0;
    rock.castShadow = true;
    rock.receiveShadow = true;
    parent.add(rock);
    const cap = new THREE.ExtrudeGeometry(shape, { depth: .1, bevelEnabled: true, bevelThickness: .04, bevelSize: .07, bevelSegments: 1 });
    cap.rotateX(-Math.PI / 2);
    const lip = new THREE.Mesh(cap, ctx.flat(grassCol));
    lip.position.y = y0 + h - .08;
    lip.receiveShadow = true;
    lip.castShadow = true;
    parent.add(lip);
}

function underside(ctx, fn, c, depth, rockCol, parent) {
    const N = 40;
    const rings = [[1.04, .03], [.84, -.35], [.56, -.6], [.28, -.85]];
    const ring = rings.map(([s, dy], ri) => Array.from({ length: N }, (_, i) => {
        const a = i / N * Math.PI * 2;
        const rr = fn(a) * s * (ri ? .9 + hash(i, ri, 7) * .2 : 1);
        return new THREE.Vector3(c.x + Math.cos(a) * rr, dy * depth + (ri ? (hash(ri, i, 3) - .5) * .25 : 0), c.z + Math.sin(a) * rr);
    }));
    const tip = new THREE.Vector3(c.x + .2, -depth, c.z - .1);
    const verts = [];
    for (let ri = 0; ri < rings.length - 1; ri++) {
        for (let i = 0; i < N; i++) {
            const a = ring[ri][i];
            const b = ring[ri][(i + 1) % N];
            const cc = ring[ri + 1][i];
            const d = ring[ri + 1][(i + 1) % N];
            verts.push(a, b, cc, b, d, cc);
        }
    }
    const lastRing = ring[rings.length - 1];
    for (let i = 0; i < N; i++) verts.push(lastRing[i], lastRing[(i + 1) % N], tip);
    const geo = new THREE.BufferGeometry().setFromPoints(verts);
    geo.computeVertexNormals();
    const m = new THREE.Mesh(geo, ctx.flat(rockCol, { side: THREE.DoubleSide }));
    m.castShadow = true;
    m.receiveShadow = true;
    parent.add(m);
}

export function island(ctx, o) {
    const g = new THREE.Group();
    g.position.set(o.x || 0, o.y || 0, o.z || 0);
    ctx.world.add(g);
    const info = { group: g, levels: [] };
    let y = 0;
    o.levels.forEach((L, i) => {
        const fn = radiusFn(L);
        const c = L.c || { x: 0, z: 0 };
        blob(ctx, fn, c, y, L.h, i ? '#665446' : '#5d4c40', i ? '#436f40' : '#3d6a3c', g);
        if (i === 0) underside(ctx, fn, c, o.depth || 3, '#4f4038', g);
        info.levels.push({ fn, c, top: y + L.h + GRASS });
        y += L.h;
    });
    return info;
}
