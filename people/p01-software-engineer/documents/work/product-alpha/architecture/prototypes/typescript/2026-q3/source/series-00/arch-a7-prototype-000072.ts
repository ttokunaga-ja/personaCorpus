// Synthetic Product Alpha R7 architecture contract: p01-src-000072
const observation = { recordId: "ARCH-A7-000072", service: "svc-edge-api", decisionId: "ADR-073", observedOn: "2026-07-16", p95Ms: 191, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
