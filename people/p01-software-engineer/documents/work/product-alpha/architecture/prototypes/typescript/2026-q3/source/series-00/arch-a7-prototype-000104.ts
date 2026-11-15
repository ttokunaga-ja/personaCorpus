// Synthetic Product Alpha R7 architecture contract: p01-src-000104
const observation = { recordId: "ARCH-A7-000104", service: "svc-decision-store", decisionId: "ADR-105", observedOn: "2026-08-17", p95Ms: 185, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
