// Recursive Fibonacci, which is a call benchmark wearing an arithmetic benchmark's clothes.
//
// Almost all of the time here is the call sequence: push a frame, pass one argument, compare,
// two recursive calls, add, return. There is one addition and one comparison per call and
// nothing else, so a runtime that is slow at calls cannot hide it behind anything.
//
// Thirty two is chosen so that the work lands somewhere around a tenth of a second on a current
// machine, which is long enough that process startup does not swamp it and short enough that a
// full run of the suite finishes while you are still looking at it.

function fib(n) {
  if (n < 2) return n;
  return fib(n - 1) + fib(n - 2);
}

function work() {
  return fib(35);
}

const started = performance.now();
const checksum = work();
const elapsed = performance.now() - started;
console.log('katsu-bench ' + JSON.stringify({ workload: 'fib', compute_ms: elapsed, checksum: String(checksum) }));
