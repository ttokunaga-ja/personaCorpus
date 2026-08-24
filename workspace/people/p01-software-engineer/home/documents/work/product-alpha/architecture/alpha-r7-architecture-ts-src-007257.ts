// Aster Loop Product Alpha R7 architecture contract: p01-src-007257
const review = { recordId: "ARCH-A7-007257", service: "svc-event-bus", decision: "ADR-094", observedP95Ms: 191, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
