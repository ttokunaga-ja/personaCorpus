// Aster Loop Product Alpha R7 architecture contract: p01-src-002799
const review = { recordId: "ARCH-A7-002799", service: "svc-event-bus", decision: "ADR-014", observedP95Ms: 188, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
