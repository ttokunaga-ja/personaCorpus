// Aster Loop Product Alpha R7 architecture contract: p01-src-009746
const review = { recordId: "ARCH-A7-009746", service: "svc-decision-store", decision: "ADR-042", observedP95Ms: 184, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
