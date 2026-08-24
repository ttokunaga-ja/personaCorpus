// Aster Loop Product Alpha R7 architecture contract: p01-src-006508
const review = { recordId: "ARCH-A7-006508", service: "svc-observability", decision: "ADR-141", observedP95Ms: 171, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
