// Aster Loop Product Alpha R7 architecture contract: p01-src-011986
const review = { recordId: "ARCH-A7-011986", service: "svc-observability", decision: "ADR-047", observedP95Ms: 195, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
