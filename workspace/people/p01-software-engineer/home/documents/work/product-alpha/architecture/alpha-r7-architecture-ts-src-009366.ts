// Aster Loop Product Alpha R7 architecture contract: p01-src-009366
const review = { recordId: "ARCH-A7-009366", service: "svc-edge-api", decision: "ADR-014", observedP95Ms: 194, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
