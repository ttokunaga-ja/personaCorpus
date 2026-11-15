// Aster Loop Product Alpha R7 architecture contract: p01-src-003189
const review = { recordId: "ARCH-A7-003189", service: "svc-event-bus", decision: "ADR-006", observedP95Ms: 173, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
