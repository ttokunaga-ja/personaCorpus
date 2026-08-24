// Aster Loop Product Alpha R7 architecture contract: p01-src-006618
const review = { recordId: "ARCH-A7-006618", service: "svc-edge-api", decision: "ADR-052", observedP95Ms: 173, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
