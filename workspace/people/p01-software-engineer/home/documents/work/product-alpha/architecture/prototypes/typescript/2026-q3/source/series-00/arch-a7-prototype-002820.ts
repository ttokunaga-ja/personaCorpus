// Aster Loop Product Alpha R7 architecture contract: p01-src-002820
const review = { recordId: "ARCH-A7-002820", service: "svc-edge-api", decision: "ADR-035", observedP95Ms: 182, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
