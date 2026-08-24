// Synthetic Product Alpha R7 architecture contract: p01-src-000051
const observation = { recordId: "ARCH-A7-000051", service: "svc-event-bus", decisionId: "ADR-052", observedOn: "2026-09-02", p95Ms: 189, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
