// Holds the process open with nothing to do, so that resident memory can be sampled at a
// genuine idle rather than mid startup. The runner sends SIGTERM once it has its reading.
setInterval(() => {}, 1000);
