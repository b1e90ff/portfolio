import * as THREE from 'three';

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
