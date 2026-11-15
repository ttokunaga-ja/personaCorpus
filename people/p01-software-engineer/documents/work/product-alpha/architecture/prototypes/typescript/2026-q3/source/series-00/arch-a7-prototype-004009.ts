// Aster Loop Product Alpha R7 architecture contract: p01-src-004009
const review = { recordId: "ARCH-A7-004009", service: "svc-release-coordinator", decision: "ADR-030", observedP95Ms: 183, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
