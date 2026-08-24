// Aster Loop Product Alpha R7 architecture contract: p01-src-003599
const review = { recordId: "ARCH-A7-003599", service: "svc-contract-registry", decision: "ADR-018", observedP95Ms: 178, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
