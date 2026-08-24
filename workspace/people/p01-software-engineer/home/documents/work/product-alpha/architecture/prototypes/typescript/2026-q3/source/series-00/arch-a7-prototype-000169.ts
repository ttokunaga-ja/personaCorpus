// Synthetic Product Alpha R7 architecture contract: p01-src-000169
const observation = { recordId: "ARCH-A7-000169", service: "svc-release-coordinator", decisionId: "ADR-170", observedOn: "2026-08-13", p95Ms: 193, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
