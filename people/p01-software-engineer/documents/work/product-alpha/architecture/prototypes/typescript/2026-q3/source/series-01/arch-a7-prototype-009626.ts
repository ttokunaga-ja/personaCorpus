// Aster Loop Product Alpha R7 architecture contract: p01-src-009626
const review = { recordId: "ARCH-A7-009626", service: "svc-decision-store", decision: "ADR-075", observedP95Ms: 184, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
