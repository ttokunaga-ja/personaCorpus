// Aster Loop Product Alpha R7 architecture contract: p01-src-008199
const review = { recordId: "ARCH-A7-008199", service: "svc-event-bus", decision: "ADR-041", observedP95Ms: 188, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
