// Aster Loop Product Alpha R7 architecture contract: p01-src-009399
const review = { recordId: "ARCH-A7-009399", service: "svc-event-bus", decision: "ADR-047", observedP95Ms: 173, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
