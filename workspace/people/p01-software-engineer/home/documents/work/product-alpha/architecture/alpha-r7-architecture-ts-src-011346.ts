// Aster Loop Product Alpha R7 architecture contract: p01-src-011346
const review = { recordId: "ARCH-A7-011346", service: "svc-edge-api", decision: "ADR-004", observedP95Ms: 176, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
