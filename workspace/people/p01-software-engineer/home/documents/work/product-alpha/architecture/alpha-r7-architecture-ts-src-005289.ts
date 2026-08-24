// Aster Loop Product Alpha R7 architecture contract: p01-src-005289
const review = { recordId: "ARCH-A7-005289", service: "svc-event-bus", decision: "ADR-116", observedP95Ms: 194, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
