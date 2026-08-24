// Aster Loop Product Alpha R7 architecture contract: p01-src-007376
const review = { recordId: "ARCH-A7-007376", service: "svc-decision-store", decision: "ADR-014", observedP95Ms: 175, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
