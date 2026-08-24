// Aster Loop Product Alpha R7 architecture contract: p01-src-004139
const review = { recordId: "ARCH-A7-004139", service: "svc-contract-registry", decision: "ADR-160", observedP95Ms: 178, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
