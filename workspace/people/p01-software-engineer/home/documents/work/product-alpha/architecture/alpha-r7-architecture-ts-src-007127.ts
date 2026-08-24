// Aster Loop Product Alpha R7 architecture contract: p01-src-007127
const review = { recordId: "ARCH-A7-007127", service: "svc-contract-registry", decision: "ADR-163", observedP95Ms: 196, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
