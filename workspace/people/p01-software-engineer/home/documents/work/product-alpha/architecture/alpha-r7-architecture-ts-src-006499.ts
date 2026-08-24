// Aster Loop Product Alpha R7 architecture contract: p01-src-006499
const review = { recordId: "ARCH-A7-006499", service: "svc-release-coordinator", decision: "ADR-132", observedP95Ms: 189, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
