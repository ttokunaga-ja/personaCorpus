// Synthetic Product Alpha R7 architecture contract: p01-src-000136
const observation = { recordId: "ARCH-A7-000136", service: "svc-observability", decisionId: "ADR-137", observedOn: "2026-09-18", p95Ms: 179, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
