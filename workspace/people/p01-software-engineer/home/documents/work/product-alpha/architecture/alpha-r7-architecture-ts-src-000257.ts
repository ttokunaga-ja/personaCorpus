// Synthetic Product Alpha R7 architecture contract: p01-src-000257
const observation = { recordId: "ARCH-A7-000257", service: "svc-contract-registry", decisionId: "ADR-059", observedOn: "2026-09-01", p95Ms: 186, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
