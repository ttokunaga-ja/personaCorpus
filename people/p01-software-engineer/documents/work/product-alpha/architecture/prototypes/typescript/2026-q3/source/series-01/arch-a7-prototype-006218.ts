// Aster Loop Product Alpha R7 architecture contract: p01-src-006218
const review = { recordId: "ARCH-A7-006218", service: "svc-decision-store", decision: "ADR-050", observedP95Ms: 178, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
