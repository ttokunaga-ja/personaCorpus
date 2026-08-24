// Aster Loop Product Alpha R7 architecture contract: p01-src-005569
const review = { recordId: "ARCH-A7-005569", service: "svc-release-coordinator", decision: "ADR-197", observedP95Ms: 177, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
