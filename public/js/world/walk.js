import * as THREE from 'three';

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
