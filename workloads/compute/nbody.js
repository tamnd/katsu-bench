// A gravity integration over a handful of bodies, which is floating point arithmetic and
// property access on ordinary objects and nothing else.
//
// This is the workload that says whether property access is going through an inline cache or
// through a hash lookup. Every one of the innermost expressions reads a named field off an
// object whose shape never changes, which is the single case every optimising runtime is
// supposed to make free, so a large number here means the shape machinery is not working.
//
// The integrator is a plain symplectic Euler step and it is not trying to be accurate. It is
// trying to do a lot of multiply and add on values that live behind field names.

function bodies() {
  return [
    { x: 0, y: 0, z: 0, vx: 0, vy: 0, vz: 0, mass: 39.47 },
    { x: 4.84, y: -1.16, z: -0.1, vx: 0.6, vy: 2.81, vz: -0.02, mass: 0.037 },
    { x: 8.34, y: 4.12, z: -0.4, vx: -1.01, vy: 1.82, vz: 0.008, mass: 0.011 },
    { x: 12.89, y: -15.11, z: -0.22, vx: 1.08, vy: 0.86, vz: -0.01, mass: 0.0017 },
    { x: 15.37, y: -25.91, z: 0.17, vx: 0.97, vy: 0.59, vz: -0.03, mass: 0.002 },
  ];
}

function advance(system, dt) {
  for (let i = 0; i < system.length; i++) {
    const a = system[i];
    for (let j = i + 1; j < system.length; j++) {
      const b = system[j];
      const dx = a.x - b.x;
      const dy = a.y - b.y;
      const dz = a.z - b.z;
      const distance = Math.sqrt(dx * dx + dy * dy + dz * dz);
      const magnitude = dt / (distance * distance * distance);
      a.vx -= dx * b.mass * magnitude;
      a.vy -= dy * b.mass * magnitude;
      a.vz -= dz * b.mass * magnitude;
      b.vx += dx * a.mass * magnitude;
      b.vy += dy * a.mass * magnitude;
      b.vz += dz * a.mass * magnitude;
    }
  }
  for (let i = 0; i < system.length; i++) {
    const body = system[i];
    body.x += dt * body.vx;
    body.y += dt * body.vy;
    body.z += dt * body.vz;
  }
}

function energy(system) {
  let total = 0;
  for (let i = 0; i < system.length; i++) {
    const a = system[i];
    total += 0.5 * a.mass * (a.vx * a.vx + a.vy * a.vy + a.vz * a.vz);
    for (let j = i + 1; j < system.length; j++) {
      const b = system[j];
      const dx = a.x - b.x;
      const dy = a.y - b.y;
      const dz = a.z - b.z;
      total -= (a.mass * b.mass) / Math.sqrt(dx * dx + dy * dy + dz * dz);
    }
  }
  return total;
}

function work() {
  const system = bodies();
  for (let step = 0; step < 2000000; step++) {
    advance(system, 0.01);
  }
  // Rounded, because the last few bits of a float differ between a runtime that keeps an
  // intermediate in a wider register and one that does not, and disagreeing on those would
  // report a correctness failure where there is only a rounding difference.
  return energy(system).toFixed(9);
}

const started = performance.now();
const checksum = work();
const elapsed = performance.now() - started;
console.log('katsu-bench ' + JSON.stringify({ workload: 'nbody', compute_ms: elapsed, checksum: String(checksum) }));
