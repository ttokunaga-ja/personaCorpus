// Aster Loop Product Alpha R7 architecture contract: p01-src-004119
const review = { recordId: "ARCH-A7-004119", service: "svc-event-bus", decision: "ADR-140", observedP95Ms: 185, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
