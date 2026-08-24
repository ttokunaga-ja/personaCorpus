// Aster Loop Product Alpha R7 architecture contract: p01-src-009226
const review = { recordId: "ARCH-A7-009226", service: "svc-observability", decision: "ADR-073", observedP95Ms: 189, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
