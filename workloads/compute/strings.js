// Building, slicing, comparing and joining strings.
//
// Three things are being measured here and they are worth separating in your head when you read
// the number. Building a string by repeated concatenation is the case ropes exist for, and a
// runtime without them copies the whole accumulated string every time. Slicing is the case
// substring views exist for. Comparison and sorting are the case where the internal encoding
// shows up, because a comparison between a Latin-1 string and a UTF-16 one has to widen as it
// goes rather than reading two byte arrays.
//
// The sizes are kept modest on purpose. A concatenation loop that runs long enough will measure
// the allocator and the collector rather than the string representation, and there is a
// separate workload for that.

function work() {
  let built = '';
  for (let i = 0; i < 100000; i++) {
    built += 'katsu-' + (i % 97) + ';';
  }

  let slices = 0;
  for (let i = 0; i + 32 < built.length; i += 32) {
    const piece = built.slice(i, i + 16);
    if (piece.charCodeAt(0) === 107) slices++;
  }

  const words = built.split(';');
  words.sort();

  let joined = 0;
  for (let round = 0; round < 4; round++) {
    joined += words.join(',').length;
  }

  // A wide string as well as a narrow one, because the interesting comparison is the mixed
  // encoding case and a suite that only ever handles ASCII never reaches it.
  const wide = 'かつ丼' + built.slice(0, 4096);
  let wideHits = 0;
  for (let i = 0; i < words.length; i++) {
    if (words[i] < wide) wideHits++;
  }

  return built.length + slices + words.length + joined + wideHits;
}

const started = performance.now();
const checksum = work();
const elapsed = performance.now() - started;
console.log('katsu-bench ' + JSON.stringify({ workload: 'strings', compute_ms: elapsed, checksum: String(checksum) }));
