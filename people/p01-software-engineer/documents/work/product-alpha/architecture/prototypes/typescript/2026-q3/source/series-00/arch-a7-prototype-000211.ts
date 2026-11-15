// Synthetic Product Alpha R7 architecture contract: p01-src-000211
const observation = { recordId: "ARCH-A7-000211", service: "svc-release-coordinator", decisionId: "ADR-013", observedOn: "2026-07-17", p95Ms: 178, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
