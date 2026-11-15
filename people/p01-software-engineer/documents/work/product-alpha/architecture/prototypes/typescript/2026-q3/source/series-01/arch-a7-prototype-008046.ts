// Aster Loop Product Alpha R7 architecture contract: p01-src-008046
const review = { recordId: "ARCH-A7-008046", service: "svc-edge-api", decision: "ADR-087", observedP95Ms: 170, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
