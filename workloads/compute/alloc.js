// Allocate a great many short lived objects and keep a few of them.
//
// This is the collector axis. Almost everything allocated here dies within a few thousand
// allocations, which is the generational hypothesis in its purest form, so a runtime with a
// decent young generation should be close to free and one without should show it clearly.
//
// Every object is stored into a ring buffer, and that is not decoration. The first version of
// this workload only kept one object in a thousand, and V8 and JavaScriptCore both noticed that
// the other nine hundred and ninety nine never escaped the loop body, replaced them with plain
// locals and deleted the allocation entirely. Three million allocations completed in ten
// milliseconds, which is about three nanoseconds each, which is not a speed any allocator
// reaches. Writing every object into a live array is what makes it escape, and the ring buffer
// is what stops all three million of them staying alive at once.
//
// This workload is also why the suite reports peak resident memory next to the timing. A runtime
// can win on time here by simply not collecting, and the memory column is what catches that.

function work() {
  const survivors = new Array(8192);
  let sum = 0;

  for (let i = 0; i < 10000000; i++) {
    const point = { x: i, y: i + 1, label: null };
    sum += point.x + point.y;
    survivors[i % survivors.length] = point;
  }

  let kept = 0;
  for (let i = 0; i < survivors.length; i++) {
    if (survivors[i] !== undefined) kept++;
  }

  return sum + kept;
}

const started = performance.now();
const checksum = work();
const elapsed = performance.now() - started;
console.log('katsu-bench ' + JSON.stringify({ workload: 'alloc', compute_ms: elapsed, checksum: String(checksum) }));
