// Aster Loop Product Alpha R7 architecture contract: p01-src-008717
const review = { recordId: "ARCH-A7-008717", service: "svc-contract-registry", decision: "ADR-161", observedP95Ms: 193, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
