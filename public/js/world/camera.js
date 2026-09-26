export const OVERVIEW_YAW = -.5;
export const NARROW = 860;
export const ASIDE = 'overview-aside';

export const ISLANDS = {
    basecamp: { pos: [0, 0, 0], pin: 5.6, land: { t: [0, 1.3, .8], dist: 16.5, height: 7, yaw: .2 } },
    projects: { pos: [12, 2.4, -6], r: 2.2, seed: 2, plateau: true, front: OVERVIEW_YAW + .1 },
    about: { pos: [-12.5, 1.6, -6], r: 2.5, seed: 4, plateau: true, plateauTrees: false, front: OVERVIEW_YAW - .15 },
    experience: { pos: [-9, -1.4, 9], r: 2, seed: 8, front: OVERVIEW_YAW - .1 },
    contact: { pos: [9.5, -1.2, 9], r: 2, seed: 6, front: OVERVIEW_YAW + .15 },
};

/* Portrait screens pull the camera back so the archipelago or landed island still fits. */
export function cameraState(key, { width, height, collapsed = false }) {
    const narrow = width <= NARROW;
    const aspect = width / height;
    const folded = narrow && collapsed;
    if (!ISLANDS[key]) {
        const fit = Math.min(3, Math.max(1, 1.25 / aspect));
        const aside = key === ASIDE && !folded;
        return {
            target: [0, 0, 1],
            dist: 52 * fit,
            height: 23 * fit,
            yaw: OVERVIEW_YAW,
            offsetX: narrow ? 0 : aside ? -.18 : .16,
            offsetY: narrow ? (aside ? .28 : -.06) : 0,
        };
    }
    const v = ISLANDS[key];
    const land = v.land || {};
    const fit = narrow ? Math.min(2.3, Math.max(1, (folded ? .8 : .95) / aspect)) : 1;
    return {
        target: land.t || [v.pos[0], v.pos[1] + .9, v.pos[2]],
        dist: (land.dist || 12.5) * fit,
        height: (land.height || 5.4) * fit,
        yaw: land.yaw ?? v.front,
        offsetX: narrow ? 0 : -.2,
        offsetY: narrow ? (folded ? .06 : .24) : 0,
    };
}
