// Stringify and parse a nested structure, over and over.
//
// This is the workload closest to what a real server spends its day doing, and it is also the
// one that measures the least JavaScript. `JSON.stringify` and `JSON.parse` are native in every
// runtime here, so what is being compared is the quality of three C++ or Zig or Rust
// implementations plus the cost of getting the values in and out of them.
//
// It is in the suite precisely because of that. A runtime that is brilliant at arithmetic and
// slow at its own JSON codec will lose to Node on the workload the reader actually has, and a
// benchmark suite that only measures the parts we are good at is marketing.

function record(i) {
  return {
    id: i,
    name: 'record-' + i,
    active: (i & 1) === 0,
    score: i * 1.5,
    tags: ['alpha', 'beta', 'gamma'],
    nested: { depth: 1, child: { depth: 2, label: 'leaf-' + (i % 17) } },
  };
}

function work() {
  const document = { generated: 'katsu-bench', records: [] };
  for (let i = 0; i < 2000; i++) {
    document.records.push(record(i));
  }

  let total = 0;
  for (let round = 0; round < 100; round++) {
    const text = JSON.stringify(document);
    const parsed = JSON.parse(text);
    total += parsed.records.length + text.length;
  }
  return total;
}

const started = performance.now();
const checksum = work();
const elapsed = performance.now() - started;
console.log('katsu-bench ' + JSON.stringify({ workload: 'json', compute_ms: elapsed, checksum: String(checksum) }));
