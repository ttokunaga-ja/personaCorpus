// Aster Loop Product Alpha R7 architecture contract: p01-src-008026
const review = { recordId: "ARCH-A7-008026", service: "svc-observability", decision: "ADR-067", observedP95Ms: 177, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
