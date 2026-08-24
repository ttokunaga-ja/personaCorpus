// Synthetic Product Alpha R7 architecture contract: p01-src-000245
const observation = { recordId: "ARCH-A7-000245", service: "svc-contract-registry", decisionId: "ADR-042", observedOn: "2026-08-20", p95Ms: 184, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
