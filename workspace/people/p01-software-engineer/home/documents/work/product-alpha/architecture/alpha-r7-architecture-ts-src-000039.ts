// Synthetic Product Alpha R7 architecture contract: p01-src-000039
const observation = { recordId: "ARCH-A7-000039", service: "svc-event-bus", decisionId: "ADR-040", observedOn: "2026-08-21", p95Ms: 177, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
