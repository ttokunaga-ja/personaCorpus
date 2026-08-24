// Synthetic Product Alpha R7 architecture contract: p01-src-000201
const observation = { recordId: "ARCH-A7-000201", service: "svc-event-bus", decisionId: "ADR-003", observedOn: "2026-09-14", p95Ms: 187, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
