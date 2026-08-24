// Aster Loop Product Alpha R7 architecture contract: p01-src-008317
const review = { recordId: "ARCH-A7-008317", service: "svc-release-coordinator", decision: "ADR-159", observedP95Ms: 171, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
