// Aster Loop Product Alpha R7 architecture contract: p01-src-004476
const review = { recordId: "ARCH-A7-004476", service: "svc-edge-api", decision: "ADR-099", observedP95Ms: 191, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
