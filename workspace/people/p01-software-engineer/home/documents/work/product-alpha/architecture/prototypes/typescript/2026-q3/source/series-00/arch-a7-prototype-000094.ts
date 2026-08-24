// Synthetic Product Alpha R7 architecture contract: p01-src-000094
const observation = { recordId: "ARCH-A7-000094", service: "svc-observability", decisionId: "ADR-095", observedOn: "2026-08-07", p95Ms: 194, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
