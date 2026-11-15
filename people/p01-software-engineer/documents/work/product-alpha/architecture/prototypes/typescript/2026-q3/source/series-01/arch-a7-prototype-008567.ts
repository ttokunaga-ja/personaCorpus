// Aster Loop Product Alpha R7 architecture contract: p01-src-008567
const review = { recordId: "ARCH-A7-008567", service: "svc-contract-registry", decision: "ADR-011", observedP95Ms: 178, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
