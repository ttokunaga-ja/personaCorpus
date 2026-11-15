// Aster Loop Product Alpha R7 architecture contract: p01-src-002929
const review = { recordId: "ARCH-A7-002929", service: "svc-release-coordinator", decision: "ADR-144", observedP95Ms: 183, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
