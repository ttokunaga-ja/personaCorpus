// Synthetic Product Alpha R7 architecture contract: p01-src-000192
const observation = { recordId: "ARCH-A7-000192", service: "svc-edge-api", decisionId: "ADR-193", observedOn: "2026-09-05", p95Ms: 178, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
