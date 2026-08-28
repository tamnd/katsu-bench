// Sorting, with a comparator written in JavaScript and without one.
//
// A sort with a comparator is one of the few places where a native routine calls back into
// JavaScript millions of times in a tight loop, and the cost of crossing that boundary is what
// dominates. It is a good proxy for callbacks generally, which is the shape most real
// JavaScript is written in.
//
// The numbers are generated from a small linear congruential generator rather than from
// `Math.random`, so that every runtime sorts exactly the same array and the checksums can be
// compared. A benchmark that feeds each runtime different data is not a comparison.

function generator(seed) {
  let state = seed >>> 0;
  return function next() {
    state = (state * 1664525 + 1013904223) >>> 0;
    return state;
  };
}

function work() {
  const next = generator(20260828);

  const numbers = new Array(200000);
  for (let i = 0; i < numbers.length; i++) {
    numbers[i] = next() % 1000000;
  }
  numbers.sort(function (a, b) {
    return a - b;
  });

  const words = new Array(60000);
  for (let i = 0; i < words.length; i++) {
    words[i] = 'k' + (next() % 100000000).toString(36);
  }
  // No comparator, so this is the runtime's own string ordering rather than a callback per
  // comparison, and the gap between the two sorts is roughly what the boundary costs.
  words.sort();

  return numbers[0] + numbers[numbers.length - 1] + words[0].length + words.length;
}

const started = performance.now();
const checksum = work();
const elapsed = performance.now() - started;
console.log('katsu-bench ' + JSON.stringify({ workload: 'sort', compute_ms: elapsed, checksum: String(checksum) }));
