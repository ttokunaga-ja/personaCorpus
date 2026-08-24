// Aster Loop Product Alpha R7 architecture contract: p01-src-010676
const review = { recordId: "ARCH-A7-010676", service: "svc-decision-store", decision: "ADR-130", observedP95Ms: 181, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
