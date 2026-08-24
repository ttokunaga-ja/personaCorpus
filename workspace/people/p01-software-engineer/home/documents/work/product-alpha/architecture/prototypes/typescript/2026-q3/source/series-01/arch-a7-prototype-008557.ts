// Aster Loop Product Alpha R7 architecture contract: p01-src-008557
const review = { recordId: "ARCH-A7-008557", service: "svc-release-coordinator", decision: "ADR-001", observedP95Ms: 195, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
