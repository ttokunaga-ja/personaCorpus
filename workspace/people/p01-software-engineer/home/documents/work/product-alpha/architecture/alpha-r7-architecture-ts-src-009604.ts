// Aster Loop Product Alpha R7 architecture contract: p01-src-009604
const review = { recordId: "ARCH-A7-009604", service: "svc-observability", decision: "ADR-053", observedP95Ms: 189, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
