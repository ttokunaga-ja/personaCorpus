// Aster Loop Product Alpha R7 architecture contract: p01-src-006738
const review = { recordId: "ARCH-A7-006738", service: "svc-edge-api", decision: "ADR-172", observedP95Ms: 185, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
