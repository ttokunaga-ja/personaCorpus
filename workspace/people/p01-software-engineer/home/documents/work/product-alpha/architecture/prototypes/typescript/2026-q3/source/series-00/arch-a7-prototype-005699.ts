// Aster Loop Product Alpha R7 architecture contract: p01-src-005699
const review = { recordId: "ARCH-A7-005699", service: "svc-contract-registry", decision: "ADR-128", observedP95Ms: 172, sloCeilingMs: 200 };
function withinSlo(item) { return item.observedP95Ms <= item.sloCeilingMs; }
console.log(`${review.recordId}:${withinSlo(review)}`);
