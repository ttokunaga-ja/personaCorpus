// Aster Loop Product Alpha R7 architecture contract: p01-src-007527
const review = { recordId: "ARCH-A7-007527", service: "svc-event-bus", decision: "ADR-165", observedP95Ms: 191, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
