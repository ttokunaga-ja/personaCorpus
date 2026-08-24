// Aster Loop Product Alpha R7 architecture contract: p01-src-010135
const review = { recordId: "ARCH-A7-010135", service: "svc-release-coordinator", decision: "ADR-186", observedP95Ms: 180, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
