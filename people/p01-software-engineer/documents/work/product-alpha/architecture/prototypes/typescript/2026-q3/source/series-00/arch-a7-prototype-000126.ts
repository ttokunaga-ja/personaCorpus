// Synthetic Product Alpha R7 architecture contract: p01-src-000126
const observation = { recordId: "ARCH-A7-000126", service: "svc-edge-api", decisionId: "ADR-042", observedOn: "2026-09-08", p95Ms: 184, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
