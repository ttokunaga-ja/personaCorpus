// Aster Loop Product Alpha R7 architecture contract: p01-src-005969
const review = { recordId: "ARCH-A7-005969", service: "svc-contract-registry", decision: "ADR-199", observedP95Ms: 172, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
