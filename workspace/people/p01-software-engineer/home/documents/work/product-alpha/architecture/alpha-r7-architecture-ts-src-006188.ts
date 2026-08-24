// Aster Loop Product Alpha R7 architecture contract: p01-src-006188
const review = { recordId: "ARCH-A7-006188", service: "svc-decision-store", decision: "ADR-020", observedP95Ms: 175, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
