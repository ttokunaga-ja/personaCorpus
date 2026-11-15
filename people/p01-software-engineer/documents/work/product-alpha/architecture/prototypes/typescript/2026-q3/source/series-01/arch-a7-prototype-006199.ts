// Aster Loop Product Alpha R7 architecture contract: p01-src-006199
const review = { recordId: "ARCH-A7-006199", service: "svc-release-coordinator", decision: "ADR-031", observedP95Ms: 186, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
