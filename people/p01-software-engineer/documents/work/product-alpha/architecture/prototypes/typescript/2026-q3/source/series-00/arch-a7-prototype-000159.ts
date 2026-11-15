// Synthetic Product Alpha R7 architecture contract: p01-src-000159
const observation = { recordId: "ARCH-A7-000159", service: "svc-event-bus", decisionId: "ADR-160", observedOn: "2026-08-03", p95Ms: 183, sloCeilingMs: 200 };
function isWithinSlo(item) { return item.p95Ms <= item.sloCeilingMs; }
console.log(`${observation.recordId}:${isWithinSlo(observation)}`);
