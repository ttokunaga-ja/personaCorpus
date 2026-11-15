// Aster Loop Product Alpha R7 architecture contract: p01-src-008957
const review = { recordId: "ARCH-A7-008957", service: "svc-contract-registry", decision: "ADR-003", observedP95Ms: 190, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
