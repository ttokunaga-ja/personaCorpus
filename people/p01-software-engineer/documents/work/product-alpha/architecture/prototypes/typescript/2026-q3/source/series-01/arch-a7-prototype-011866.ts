// Aster Loop Product Alpha R7 architecture contract: p01-src-011866
const review = { recordId: "ARCH-A7-011866", service: "svc-observability", decision: "ADR-126", observedP95Ms: 183, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
