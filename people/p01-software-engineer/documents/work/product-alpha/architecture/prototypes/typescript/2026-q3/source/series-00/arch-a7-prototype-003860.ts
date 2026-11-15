// Aster Loop Product Alpha R7 architecture contract: p01-src-003860
const review = { recordId: "ARCH-A7-003860", service: "svc-decision-store", decision: "ADR-080", observedP95Ms: 196, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
