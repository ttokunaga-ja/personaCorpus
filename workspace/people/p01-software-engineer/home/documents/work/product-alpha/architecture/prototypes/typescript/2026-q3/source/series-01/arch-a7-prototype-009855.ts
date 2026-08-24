// Aster Loop Product Alpha R7 architecture contract: p01-src-009855
const review = { recordId: "ARCH-A7-009855", service: "svc-event-bus", decision: "ADR-105", observedP95Ms: 170, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
