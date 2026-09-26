import * as THREE from 'three';
import { hash } from './engine.js';

function streakTexture(base, count, seed) {
    const c = document.createElement('canvas');
    c.width = 64;
    c.height = 256;
    const g = c.getContext('2d');
    g.fillStyle = base;
    g.fillRect(0, 0, 64, 256);
    for (let i = 0; i < count; i++) {
        g.fillStyle = `rgba(255,255,255,${.25 + hash(i, seed, 3) * .55})`;
        g.fillRect(hash(i, 5, seed) * 64, hash(i, 7, 9 + seed) * 256, 2 + hash(i, 1, seed) * 4, 14 + hash(i, 3, seed) * 48);
    }
    const t = new THREE.CanvasTexture(c);
    t.wrapS = THREE.RepeatWrapping;
    t.wrapT = THREE.RepeatWrapping;
    return t;
}

function fadeTexture(fadeFrom) {
    const c = document.createElement('canvas');
    c.width = 4;
    c.height = 128;
    const g = c.getContext('2d');
    const grad = g.createLinearGradient(0, 0, 0, 128);
    grad.addColorStop(0, '#fff');
    grad.addColorStop(fadeFrom, '#fff');
    grad.addColorStop(1, '#000');
    g.fillStyle = grad;
    g.fillRect(0, 0, 4, 128);
    return new THREE.CanvasTexture(c);
}

/* One sheet with no seams: flat over the lip, a rounded bend at the edge, then the drop. */
export function fallSheet(w, spill, bend, drop, arc, wTop = w) {
    const bendLen = bend * Math.PI / 2;
    const total = spill + bendLen + drop;
    const pointAt = (u) => {
        if (u <= spill) return [.02, u - spill];
        if (u <= spill + bendLen) {
            const th = (u - spill) / bend;
            return [.02 - bend * (1 - Math.cos(th)), bend * Math.sin(th)];
        }
        const d = u - spill - bendLen;
        return [.02 - bend - d, bend + arc * Math.sin(d / drop * Math.PI / 2)];
    };
    // Each section gets its own rows so the short bend stays smooth on tall falls.
    const stations = [];
    const section = (from, length, count) => {
        for (let i = 0; i < count; i++) stations.push(from + length * i / count);
    };
    section(0, spill, 2);
    section(spill, bendLen, 8);
    section(spill + bendLen, drop, 18);
    stations.push(total);
    const rows = stations.length - 1;
    const positions = [];
    const uvs = [];
    const index = [];
    for (let i = 0; i <= rows; i++) {
        const u = stations[i];
        const [y, z] = pointAt(u);
        const k = Math.min(1, Math.max(0, (u - spill) / (bendLen + drop * .3)));
        const half = (wTop + (w - wTop) * k * k * (3 - 2 * k)) / 2;
        positions.push(-half, y, z, half, y, z);
        uvs.push(0, 1 - u / total, 1, 1 - u / total);
        if (i < rows) index.push(i * 2, i * 2 + 1, i * 2 + 2, i * 2 + 1, i * 2 + 3, i * 2 + 2);
    }
    const geo = new THREE.BufferGeometry();
    geo.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
    geo.setAttribute('uv', new THREE.Float32BufferAttribute(uvs, 2));
    geo.setIndex(index);
    geo.computeVertexNormals();
    return geo;
}

export function waterfall(ctx, parent, x, z, top, bottom, a, w = .6, spill = 0, o = {}) {
    const h = top - bottom;
    const bend = .12;
    const drop = h - bend;
    const arc = o.arc ?? Math.min(.35, h * .12);
    const fade = o.mist ? fadeTexture(.55) : null;
    const back = streakTexture('#5fa8d6', 40, 2);
    const front = streakTexture('#8fd0f0', 26, 5);
    back.repeat.set(1, h / 1.2);
    front.repeat.set(.8, h / 1.6);
    const layer = (map, opacity, emissive) => new THREE.MeshStandardMaterial({
        map, alphaMap: fade, transparent: true, opacity, emissive, emissiveIntensity: .35, roughness: .2, side: THREE.DoubleSide, depthWrite: false,
    });
    const group = new THREE.Group();
    group.position.set(x, top, z);
    group.rotation.y = Math.PI / 2 - a;
    parent.add(group);
    const wTop = o.topWidth ?? w;
    group.add(new THREE.Mesh(fallSheet(w, spill, bend, drop, arc, wTop), layer(back, .88, '#2c6f9a')));
    const veil = new THREE.Mesh(fallSheet(w * .78, spill, bend, drop, arc, wTop * .78), layer(front, .55, '#4f9cc4'));
    veil.position.set(0, .004, .025);
    group.add(veil);
    const foam = [];
    if (o.foam || o.mist) {
        const mat = ctx.flat('#f4fbff', { transparent: true, opacity: .85, depthWrite: false });
        for (let i = 0; i < 7; i++) {
            const puff = new THREE.Mesh(new THREE.IcosahedronGeometry(1, 0), mat.clone());
            puff.userData = { k: i / 7, x: (hash(i, 3, 1) - .5) * w, spin: hash(i, 9, 2) * 6 };
            group.add(puff);
            foam.push(puff);
        }
    }
    const base = -h;
    const edge = bend + arc;
    ctx.onFrame((t) => {
        if (!ctx.reduced) {
            back.offset.y = (t * .0009) % 1;
            front.offset.y = (t * .0016) % 1;
        }
        for (const puff of foam) {
            const k = ctx.reduced ? puff.userData.k : (t * .0005 + puff.userData.k) % 1;
            const u = puff.userData;
            if (o.mist) {
                puff.position.set(u.x * (1 + k), base + h * .3 - k * h * .35, edge + .1 + k * .3);
                puff.scale.setScalar(.06 + k * .16);
                puff.material.opacity = .6 * (1 - k);
            } else {
                puff.position.set(u.x, base + k * .18, edge + .05 + k * .12);
                puff.scale.setScalar(.05 + Math.sin(k * Math.PI) * .07);
                puff.material.opacity = .85 * (1 - k);
            }
            puff.rotation.y = u.spin + k * 2;
        }
    });
    return group;
}

export function stream(ctx, parent, sx, sz, ex, ez, y, width) {
    const len = Math.hypot(ex - sx, ez - sz);
    const tex = streakTexture('#7fbfe4', 22, 8);
    tex.center.set(.5, .5);
    tex.rotation = Math.PI / 2;
    tex.repeat.set(width / .6, len / 1.2);
    const mat = new THREE.MeshStandardMaterial({ map: tex, emissive: '#1f5a80', emissiveIntensity: .3, roughness: .2, transparent: true, opacity: .92 });
    const m = new THREE.Mesh(new THREE.PlaneGeometry(len, width), mat);
    m.rotation.x = -Math.PI / 2;
    m.rotation.z = -Math.atan2(ez - sz, ex - sx);
    m.position.set((sx + ex) / 2, y, (sz + ez) / 2);
    parent.add(m);
    ctx.onFrame((t) => { if (!ctx.reduced) tex.offset.y = -(t * .0005) % 1; });
    return m;
}
