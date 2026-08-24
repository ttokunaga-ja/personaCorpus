// Synthetic Product Alpha R7 architecture contract: p01-src-000029
const observation = { recordId: "ARCH-A7-000029", service: "svc-contract-registry", decisionId: "ADR-030", observedOn: "2026-08-11", p95Ms: 186, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
